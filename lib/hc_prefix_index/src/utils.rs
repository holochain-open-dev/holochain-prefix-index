use hdk::prelude::*;

/// Duplicates of get_children from holochain TypedPath
/// but without calling ensure() on those children
pub fn get_children(path: TypedPath) -> ExternResult<Vec<Link>> {
    let mut unwrapped = get_links(
        GetLinksInputBuilder::try_new(
            path.path_entry_hash()?,
            LinkTypeFilter::single_type(path.link_type.zome_index, path.link_type.zome_type),
        )?
        .build(),
    )?;
    // Only need one of each hash to build the tree.
    unwrapped.sort_unstable_by(|a, b| a.tag.cmp(&b.tag));
    unwrapped.dedup_by(|a, b| a.tag.eq(&b.tag));
    Ok(unwrapped)
}

/// Duplicates of get_children_paths from holochain TypedPath
/// but without calling ensure() on those children
pub fn get_children_paths(path: TypedPath) -> ExternResult<Vec<TypedPath>> {
    let children = get_children(path.clone())?;
    let components: ExternResult<Vec<Option<Component>>> = children
        .into_iter()
        .map(|link| {
            let component_bytes = &link.tag.0[..];
            if component_bytes.is_empty() {
                Ok(None)
            } else {
                Ok(Some(
                    SerializedBytes::from(UnsafeBytes::from(component_bytes.to_vec()))
                        .try_into()
                        .map_err(|e: SerializedBytesError| wasm_error!(e))?,
                ))
            }
        })
        .collect();
    Ok(components?
        .into_iter()
        .map(|maybe_component| {
            let mut new_path = path.path.clone();
            if let Some(component) = maybe_component {
                new_path.append_component(component);
            }
            new_path.into_typed(path.link_type)
        })
        .collect())
}

pub fn path_to_string(path: TypedPath) -> String {
    let component_strings: Vec<String> = path
        .as_ref()
        .iter()
        .filter_map(|c| String::try_from(c).ok())
        .collect();

    component_strings.join(".")
}
