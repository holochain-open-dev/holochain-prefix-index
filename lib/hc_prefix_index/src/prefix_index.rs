use crate::utils::*;
use crate::validate::*;
use hdk::hdi::prelude::TypedAction;
use hdk::prelude::*;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, SerializedBytes)]
pub struct PrefixIndex {
    pub index_name: String,
    pub link_type: ScopedLinkType,
    pub width: usize,
    pub depth: usize,
}

impl PrefixIndex {
    /// Create a prefix index.
    ///
    /// `index_name` must not contain `.`: index names are parsed through the
    /// path DSL, where a dot splits a name into multiple path components.
    pub fn new<T, E>(
        index_name: String,
        link_type: T,
        width: usize,
        depth: usize,
    ) -> ExternResult<Self>
    where
        ScopedLinkType: TryFrom<T, Error = E>,
        WasmError: From<E>,
    {
        Ok(Self {
            index_name,
            link_type: link_type.try_into()?,
            width,
            depth,
        })
    }

    pub fn add_result(&self, text: String) -> ExternResult<TypedPath> {
        self.inner_add_result(text, None)
    }

    pub fn add_result_with_label(
        &self,
        text: String,
        full_text: String,
    ) -> ExternResult<TypedPath> {
        self.inner_add_result(text, Some(full_text))
    }

    fn inner_add_result(&self, text: String, full_text: Option<String>) -> ExternResult<TypedPath> {
        let typed_path = self
            .make_result_path(text.clone(), full_text)?
            .typed(self.link_type)?;

        typed_path.ensure()?;

        debug!(
            "Added result '{:?}' to path {:?}",
            text,
            path_to_string(typed_path.clone())
        );

        Ok(typed_path)
    }

    /// Remove a result from the index, returning how many links were deleted.
    ///
    /// Only the calling agent's own links are deleted, because validation lets
    /// an author delete only what they authored. A count of `0` means this agent
    /// authored none of the links for this result — either it was never indexed,
    /// or another agent indexed it first (path creation is deduplicated across
    /// authors, so the second agent to add a result authors no links for it).
    /// In that case the result stays in the index and remains searchable.
    pub fn remove_result(&self, text: String, strategy: GetStrategy) -> ExternResult<usize> {
        self.inner_remove_result(text, None, strategy)
    }

    /// Remove a labeled result from the index, returning how many links were
    /// deleted. See [`PrefixIndex::remove_result`] for what a count of `0` means.
    pub fn remove_result_with_label(
        &self,
        text: String,
        full_text: String,
        strategy: GetStrategy,
    ) -> ExternResult<usize> {
        self.inner_remove_result(text, Some(full_text), strategy)
    }

    fn inner_remove_result(
        &self,
        text: String,
        full_text: Option<String>,
        strategy: GetStrategy,
    ) -> ExternResult<usize> {
        let path = self
            .make_result_path(text, full_text)?
            .typed(self.link_type)?
            .with_strategy(strategy);

        self.inner_remove_result_from_path(path, strategy)
    }

    fn inner_remove_result_from_path(
        &self,
        path: TypedPath,
        strategy: GetStrategy,
    ) -> ExternResult<usize> {
        let mut deleted = 0;
        if path.exists()? {
            if let Some(parent) = path.parent() {
                // Get all children of parent of path
                let children = get_links(
                    LinkQuery::new(
                        parent.path_entry_hash()?,
                        LinkTypeFilter::single_type(
                            self.link_type.zome_index,
                            self.link_type.zome_type,
                        ),
                    ),
                    strategy,
                )?;

                let path_entry_hash = path.path_entry_hash()?;
                let result_children: Vec<Link> = children
                    .clone()
                    .into_iter()
                    .filter(|c| -> bool {
                        let maybe_eh = c.clone().target.into_entry_hash();
                        match maybe_eh {
                            Some(eh) => eh == path_entry_hash,
                            None => false,
                        }
                    })
                    .collect();

                // Delete only our own links to the current path. Validation lets
                // an author delete only their own links, so attempting to delete
                // another agent's link to a result both agents added would fail
                // inline validation and roll the whole call back — leaving the
                // result unremovable by anyone.
                let my_agent = agent_info()?.agent_initial_pubkey;
                let get_options = GetOptions::from(strategy);
                for child in result_children.clone().into_iter() {
                    if child.author == my_agent {
                        delete_link(child.create_link_hash, get_options.clone())?;
                        deleted += 1;
                    }
                }

                // Get other children of parent of path
                let mut other_children = vec![];
                for i in children.into_iter() {
                    if !result_children.contains(&i) {
                        other_children.push(i);
                    }
                }

                // If no other agent's content still hangs off the parent, prune it too
                if other_children.is_empty() && !parent.is_root() {
                    deleted += self.inner_remove_result_from_path(parent, strategy)?;
                }
            }
        }

        Ok(deleted)
    }

    pub fn get_results(
        &self,
        query: String,
        limit: usize,
        strategy: GetStrategy,
    ) -> ExternResult<Vec<String>> {
        if limit == 0 {
            return Err(wasm_error!(WasmErrorInner::Guest(
                "limit must be > 0".into()
            )));
        }

        let path = self
            .make_result_path(query.clone(), None)?
            .typed(self.link_type)?
            .with_strategy(strategy);

        debug!(
            "Searching for '{:?}', starting at path '{:?}'",
            query,
            path_to_string(path.clone())
        );

        self.inner_get_results(path, limit, false, strategy)
    }

    pub fn get_random_results(
        &self,
        limit: usize,
        strategy: GetStrategy,
    ) -> ExternResult<Vec<String>> {
        if limit == 0 {
            return Err(wasm_error!(WasmErrorInner::Guest(
                "limit must be > 0".into()
            )));
        }

        let base_path = Path::from(self.index_name.clone())
            .typed(self.link_type)?
            .with_strategy(strategy);

        self.inner_get_results(base_path, limit, true, strategy)
    }

    /// Make a Path to the result following the ShardStrategy specified by PrefixIndex width + depth
    pub fn make_result_path(&self, text: String, full_text: Option<String>) -> ExternResult<Path> {
        let mut path_components = Path::from(format!(
            "{}.{}:{}#{}",
            self.index_name,
            self.width,
            self.depth,
            text.to_lowercase()
        ))
        .as_ref()
        .clone();

        match full_text {
            // Replace last component of path with full_text
            Some(full_text_string) => {
                path_components.pop();
                path_components.push(Component::from(full_text_string));
            }

            // Replace last component of path with original text (preserve case)
            None => {
                path_components.pop();
                path_components.push(Component::from(text));
            }
        }
        Ok(Path::from(path_components))
    }

    pub fn validate_create_link(
        self,
        action: TypedAction<CreateLinkData>,
    ) -> ExternResult<ValidateCallbackResult> {
        validate_create_link_prefix_index(action, &self)
    }

    /// Validate a delete of a prefix-index link.
    ///
    /// An associated function rather than a method: the rule reads no index
    /// configuration, so callers need not construct a `PrefixIndex` (which costs
    /// a `zome_info()` host call) to validate a delete.
    pub fn validate_delete_link(
        action: TypedAction<DeleteLinkData>,
        original_action: TypedAction<CreateLinkData>,
    ) -> ExternResult<ValidateCallbackResult> {
        validate_delete_link_prefix_index(action, original_action)
    }

    /// Gets the deepest-most Paths that descend from `path`, or it's parents, up to limit
    fn get_results_from_path(
        &self,
        path: TypedPath,
        limit: usize,
        shuffle: bool,
        strategy: GetStrategy,
    ) -> ExternResult<Vec<TypedPath>> {
        self.inner_get_results_from_path(path, limit, shuffle, vec![], vec![], strategy)
    }

    fn inner_get_results(
        &self,
        path: TypedPath,
        limit: usize,
        shuffle: bool,
        strategy: GetStrategy,
    ) -> ExternResult<Vec<String>> {
        let results = self.get_results_from_path(path, limit, shuffle, strategy)?;

        let leaf_strings: Vec<String> = results
            .into_iter()
            .filter(|r| r.leaf().is_some())
            .map(|p| p.leaf().unwrap().clone())
            .filter_map(|c| String::try_from(&c).ok())
            .collect();

        Ok(leaf_strings)
    }

    #[allow(clippy::only_used_in_recursion)]
    fn inner_get_results_from_path(
        &self,
        path: TypedPath,
        limit: usize,
        shuffle: bool,
        mut visited: Vec<TypedPath>,
        mut results: Vec<TypedPath>,
        strategy: GetStrategy,
    ) -> ExternResult<Vec<TypedPath>> {
        visited.push(path.clone());

        let mut children = get_children_paths(path.clone(), strategy)?;
        match children.is_empty() {
            true => {
                if path.exists()? && !results.contains(&path) && results.len() < limit {
                    results.push(path.clone());
                }

                match path.parent() {
                    Some(parent) => {
                        if !visited.contains(&parent) && !parent.is_root() {
                            return self.inner_get_results_from_path(
                                parent, limit, shuffle, visited, results, strategy,
                            );
                        }

                        Ok(results)
                    }
                    None => Ok(results),
                }
            }
            false => {
                if shuffle {
                    hdk_shuffle(&mut children)?;
                }

                for child in children.into_iter() {
                    let grandchildren = self
                        .inner_get_results_from_path(
                            child.clone(),
                            limit,
                            shuffle,
                            visited.clone(),
                            results.clone(),
                            strategy,
                        )
                        .unwrap_or_default();

                    for grandchild in grandchildren.into_iter() {
                        if grandchild.exists()?
                            && !results.contains(&grandchild)
                            && results.len() < limit
                        {
                            results.push(grandchild.clone());
                        }
                    }
                }

                match path.parent() {
                    Some(parent) => {
                        if !visited.contains(&parent) && !parent.is_root() {
                            return self.inner_get_results_from_path(
                                parent, limit, shuffle, visited, results, strategy,
                            );
                        }

                        Ok(results)
                    }
                    None => Ok(results),
                }
            }
        }
    }
}
