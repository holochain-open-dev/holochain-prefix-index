use crate::PrefixIndex;
use hdi::hash_path::path::root_hash;
use hdk::hdi::prelude::TypedAction;
use hdk::prelude::*;

pub fn validate_create_link_prefix_index(
    action: &TypedAction<CreateLinkData>,
    prefix_index: &PrefixIndex,
) -> ExternResult<ValidateCallbackResult> {
    let tag_bytes = SerializedBytes::from(UnsafeBytes::from(action.data.tag.clone().into_inner()));
    // The tag is author-controlled data: bytes that don't decode to a path
    // component are the author's fault, so reject rather than error.
    let tag_component = match Component::try_from(tag_bytes) {
        Ok(component) => component,
        Err(_) => {
            return Ok(ValidateCallbackResult::Invalid(
                "PrefixIndex link tag must be a path component".into(),
            ));
        }
    };
    let tag_string = match String::try_from(&tag_component) {
        Ok(string) => string,
        Err(_) => {
            return Ok(ValidateCallbackResult::Invalid(
                "PrefixIndex link tag must be a string path component".into(),
            ));
        }
    };

    // First Component: root hash -> index name
    let path: Path = Path::from(tag_string.clone());

    // Target is an entry hash
    let maybe_target_entryhash = action.data.target_address.clone().into_entry_hash();
    if maybe_target_entryhash.is_none() {
        return Ok(ValidateCallbackResult::Invalid(
            "PrefixIndex first component: target address must be entry hash".into(),
        ));
    }

    // first component
    if action.data.base_address == root_hash()? {
        if let Some(eh) = maybe_target_entryhash {
            if eh != path.path_entry_hash()? {
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
    }
    // second or later component
    // The component length cannot be validated from the link alone:
    // - a result string shorter than the index width shards to zero components,
    //   so the result itself sits directly under the index root
    // - labeled results replace the leaf with a label of arbitrary length
    // - third and later components are unreachable from this link's context
    //   (we can't assume this link author is also the previous link author,
    //   so we can't use must_get_agent_activity)

    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_prefix_index(
    action: &TypedAction<DeleteLinkData>,
    original_action: &TypedAction<CreateLinkData>,
) -> ExternResult<ValidateCallbackResult> {
    if action.author() != original_action.author() {
        return Ok(ValidateCallbackResult::Invalid(
            "Only the original author can delete a PrefixIndex link".into(),
        ));
    }

    if original_action.data.base_address == root_hash()? {
        return Ok(ValidateCallbackResult::Invalid(
            "Cannot delete the root component of a PrefixIndex".into(),
        ));
    }

    Ok(ValidateCallbackResult::Valid)
}
