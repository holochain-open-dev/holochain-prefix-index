use crate::utils::component_from_link_tag;
use crate::PrefixIndex;
use hdi::hash_path::path::root_hash;
use hdk::hdi::prelude::TypedAction;
use hdk::prelude::*;

pub fn validate_create_link_prefix_index(
    action: TypedAction<CreateLinkData>,
    prefix_index: &PrefixIndex,
) -> ExternResult<ValidateCallbackResult> {
    let CreateLinkData {
        base_address,
        target_address,
        tag,
        ..
    } = action.data;

    // The tag is author-controlled data: bytes that don't decode to a string
    // path component are the author's fault, so reject rather than error.
    let Some(tag_component) = component_from_link_tag(&tag) else {
        return Ok(ValidateCallbackResult::Invalid(
            "PrefixIndex link tag must be a string path component".into(),
        ));
    };
    let Ok(tag_string) = String::try_from(&tag_component) else {
        return Ok(ValidateCallbackResult::Invalid(
            "PrefixIndex link tag must be a string path component".into(),
        ));
    };

    // Every link in the index targets a Path entry.
    let Some(target_entry_hash) = target_address.into_entry_hash() else {
        return Ok(ValidateCallbackResult::Invalid(
            "PrefixIndex link target address must be an entry hash".into(),
        ));
    };

    // First component: DHT root -> index name.
    if base_address == root_hash()? {
        if tag_string != prefix_index.index_name {
            return Ok(ValidateCallbackResult::Invalid(
                "PrefixIndex first component: tag string must be index name".into(),
            ));
        }
        let index_root = Path::from(vec![tag_component]);
        if target_entry_hash != index_root.path_entry_hash()? {
            return Ok(ValidateCallbackResult::Invalid(
                "PrefixIndex first component: target address must be the index name path".into(),
            ));
        }

        return Ok(ValidateCallbackResult::Valid);
    }

    // Second component: index name -> child. The base tells us the parent path
    // here (it is the index root, whose hash we can compute), so the child path
    // is exactly [index_name, tag] and the target must hash to it. This accepts
    // both shard components and result leaves shorter than the index width,
    // which shard to zero components and so hang directly off the index root.
    let index_name_component = Component::from(prefix_index.index_name.clone());
    let index_root = Path::from(vec![index_name_component.clone()]);
    if base_address == AnyLinkableHash::from(index_root.path_entry_hash()?) {
        let child = Path::from(vec![index_name_component, tag_component]);
        if target_entry_hash != child.path_entry_hash()? {
            return Ok(ValidateCallbackResult::Invalid(
                "PrefixIndex second component: target address must be the path named by the tag"
                    .into(),
            ));
        }
    }

    // Third or later component: unvalidatable from this link alone. We know
    // neither the parent's components (a hash cannot be inverted) nor that this
    // link's author wrote the parent link, so must_get_agent_activity is no help.

    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_prefix_index(
    action: TypedAction<DeleteLinkData>,
    original_action: TypedAction<CreateLinkData>,
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
