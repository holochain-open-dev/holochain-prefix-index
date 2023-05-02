use hdk::{hash_path::path::Component, prelude::*};
use crate::utils::*;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, SerializedBytes)]
pub struct PrefixIndex {
    pub index_name: String,
    pub link_type: ScopedLinkType,
    pub width: usize,
    pub depth: usize,
}

impl PrefixIndex {
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

    pub fn add_result(&self, text: String) -> ExternResult<()> {
        let typed_path = self.make_result_path(text.clone())?.typed(self.link_type)?;

        typed_path.ensure()?;

        debug!(
            "Added result '{:?}' to path {:?}",
            text,
            path_to_string(typed_path)
        );

        Ok(())
    }

    pub fn remove_result(&self, text: String) -> ExternResult<()> {
        let path = self.make_result_path(text.clone())?.typed(self.link_type)?;

        self.inner_remove_result_from_path(path)?;

        Ok(())
    }

    fn inner_remove_result_from_path(&self, path: TypedPath) -> ExternResult<()> {
        if path.exists()? {
            if let Some(parent) = path.parent() {
                // Get all children of parent of path
                let children = get_links(
                    parent.path_entry_hash()?,
                    LinkTypeFilter::single_type(
                        self.link_type.zome_index,
                        self.link_type.zome_type,
                    ),
                    None,
                )?;

                let path_entry_hash = path.path_entry_hash()?;
                let result_children: Vec<Link> = children
                    .clone()
                    .into_iter()
                    .filter(|c| EntryHash::from(c.clone().target) == path_entry_hash)
                    .collect();

                // Delete children link corresponding to current path
                for child in result_children.clone().into_iter() {
                    if EntryHash::from(child.target) == path.path_entry_hash()? {
                        delete_link(child.create_link_hash)?;
                    }
                }

                // Get other children of parent of path
                let mut other_children = vec![];
                for i in children.clone().into_iter() {
                    if !result_children.contains(&i) {
                        other_children.push(i);
                    }
                }

                // If there are no other children of parent of path, delete parent of path
                if other_children.len() == 0 && !parent.is_root() {
                    self.inner_remove_result_from_path(parent)?;
                }
            }
        }

        Ok(())
    }

    pub fn get_results(&self, query: String, limit: usize) -> ExternResult<Vec<String>> {
        if limit == 0 {
            return Err(wasm_error!(WasmErrorInner::Guest(
                "limit must be > 0".into()
            )));
        }

        let path = self
            .make_result_path(query.clone())?
            .typed(self.link_type)?;

        debug!(
            "Searching for '{:?}', staring at path '{:?}'",
            query,
            path_to_string(path.clone())
        );

        let results = self.get_results_from_path(path, limit)?;

        let leafs: Vec<Component> = results
            .into_iter()
            .filter(|r| r.leaf().is_some())
            .map(|p| p.leaf().unwrap().clone())
            .collect();

        let strings = leafs
            .into_iter()
            .filter_map(|c| String::try_from(&c).ok())
            .collect();

        Ok(strings)
    }

    pub fn make_result_path(&self, text: String) -> ExternResult<Path> {
        Ok(Path::from(format!(
            "{}.{}:{}#{}",
            self.index_name, self.width, self.depth, text
        )))
    }

    /// Gets the deepest-most Paths that descend from `path`, or it's parents, up to limit
    fn get_results_from_path(&self, path: TypedPath, limit: usize) -> ExternResult<Vec<TypedPath>> {
        self.inner_get_results_from_path(path, limit, vec![], vec![])
    }

    fn inner_get_results_from_path(
        &self,
        path: TypedPath,
        limit: usize,
        mut visited: Vec<TypedPath>,
        mut results: Vec<TypedPath>,
    ) -> ExternResult<Vec<TypedPath>> {
        visited.push(path.clone());

        let children = get_children_paths(path.clone())?;
        match children.len() == 0 {
            true => {
                if path.exists()? && !results.contains(&path) && results.len() < limit {
                    results.push(path.clone());
                }

                match path.parent() {
                    Some(parent) => {
                        if !visited.contains(&parent) && !parent.is_root() {
                            return self
                                .inner_get_results_from_path(parent, limit, visited, results);
                        }

                        Ok(results)
                    }
                    None => Ok(results),
                }
            }
            false => {
                for child in children.into_iter() {
                    let grandchildren = self
                        .inner_get_results_from_path(
                            child.clone(),
                            limit,
                            visited.clone(),
                            results.clone(),
                        )
                        .unwrap_or(vec![]);

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
                            return self
                                .inner_get_results_from_path(parent, limit, visited, results);
                        }

                        Ok(results)
                    }
                    None => Ok(results),
                }
            }
        }
    }
}
