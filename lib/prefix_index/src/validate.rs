//use crate::PREFIX_INDEX_WIDTH;
use crate::PrefixIndex;
use hdk::hash_path::path::{root_hash, Component};
use hdk::prelude::*;

pub fn validate_create_link_prefix_index(
    action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    tag: LinkTag,
    prefix_index: PrefixIndex,
) -> ExternResult<ValidateCallbackResult> {
    let tag_bytes = SerializedBytes::try_from(UnsafeBytes::from(tag.into_inner()))
        .map_err(|_| wasm_error!("Failed to convert link tag to SerializedBytes"))?;
    let tag_component = Component::try_from(tag_bytes).map_err(|e| wasm_error!(e))?;
    let tag_string = String::try_from(&tag_component).map_err(|e| wasm_error!(e))?;

    // First Component: root hash -> index name
    let path: Path = Path::from(tag_string.clone());

    // first component
    if base_address == root_hash()? {
        if EntryHash::from(target_address.clone()) != path.path_entry_hash()? {
            return Ok(ValidateCallbackResult::Invalid(
                "PrefixIndex first component: target address must be index name".into(),
            ));
        }
        if tag_string != prefix_index.index_name {
            return Ok(ValidateCallbackResult::Invalid(
                "PrefixIndex first component: tag string must be index name".into(),
            ));
        }
    }
    // second component
    else if EntryHash::from(base_address)
        == Path::from(prefix_index.index_name.clone()).path_entry_hash()?
    {
        if tag_string.chars().count() != prefix_index.width {
            return Ok(ValidateCallbackResult::Invalid("PrefixIndex second component: tag string must have same number of chars as prefix index width".into()));
        }
    }
    // third or later component
    // unable to validate since we don't have any way of getting the previous links in the path
    //  (we can't assume this link author is also the previous link author, so we can't use must_get_agent_activity)

    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_prefix_index(
    action: DeleteLink,
    original_action: CreateLink,
    base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    if action.author != original_action.author {
        return Ok(ValidateCallbackResult::Invalid(
            "Only the original author can delete a PrefixIndex link".into(),
        ));
    }

    if base == root_hash()? {
        return Ok(ValidateCallbackResult::Invalid(
            "Cannot delete the root component of a PrefixIndex".into(),
        ));
    }

    Ok(ValidateCallbackResult::Valid)
}
