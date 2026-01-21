use hdk::prelude::*;

/// Duplicates of get_children from holochain TypedPath
/// but without calling ensure() on those children
pub fn get_children(path: TypedPath, strategy: GetStrategy) -> ExternResult<Vec<Link>> {
    let mut unwrapped = get_links(
        LinkQuery::new(
            path.path_entry_hash()?,
            LinkTypeFilter::single_type(path.link_type.zome_index, path.link_type.zome_type),
        ),
        strategy,
    )?;
    // Only need one of each hash to build the tree.
    unwrapped.sort_unstable_by(|a, b| a.tag.cmp(&b.tag));
    unwrapped.dedup_by(|a, b| a.tag.eq(&b.tag));
    Ok(unwrapped)
}

/// Duplicates of get_children_paths from holochain TypedPath
/// but without calling ensure() on those children
pub fn get_children_paths(path: TypedPath, strategy: GetStrategy) -> ExternResult<Vec<TypedPath>> {
    let children = get_children(path.clone(), strategy)?;
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

/// Fisher-Yates shuffle using HDK's random_bytes for WASM-compatible randomness
pub fn hdk_shuffle<T>(slice: &mut [T]) -> ExternResult<()> {
    let len = slice.len();
    if len <= 1 {
        return Ok(());
    }

    // Get enough random bytes for all swaps
    let random_bytes = random_bytes((len * 4) as u32)?;

    for i in (1..len).rev() {
        // Convert 4 bytes to u32, then mod by (i+1) to get index in range [0, i]
        let byte_offset = (len - 1 - i) * 4;
        let random_u32 = u32::from_le_bytes([
            random_bytes[byte_offset],
            random_bytes[byte_offset + 1],
            random_bytes[byte_offset + 2],
            random_bytes[byte_offset + 3],
        ]);
        let j = (random_u32 as usize) % (i + 1);
        slice.swap(i, j);
    }

    Ok(())
}
