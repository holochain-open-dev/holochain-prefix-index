use hc_prefix_index::PrefixIndex;
use hdi::prelude::*;

pub const PREFIX_INDEX_A_NAME: &str = "prefix_index_a";
pub const PREFIX_INDEX_A_WIDTH: usize = 3;
pub const PREFIX_INDEX_A_DEPTH: usize = 3;

pub const PREFIX_INDEX_B_NAME: &str = "prefix_index_b";
pub const PREFIX_INDEX_B_WIDTH: usize = 3;
pub const PREFIX_INDEX_B_DEPTH: usize = 5;

pub const PREFIX_INDEX_C_NAME: &str = "prefix_index_c";
pub const PREFIX_INDEX_C_WIDTH: usize = 4;
pub const PREFIX_INDEX_C_DEPTH: usize = 2;

#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    PrefixIndexA,
    PrefixIndexB,
    PrefixIndexC,
}

// Validation you perform during the genesis process. Nobody else on the network performs it, only you.
// There *is no* access to network calls in this callback
#[hdk_extern]
pub fn genesis_self_check(_data: GenesisSelfCheckData) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

// Validation the network performs when you try to join, you can't perform this validation yourself as you are not a member yet.
// There *is* access to network calls in this function
pub fn validate_agent_joining(
    _agent_pub_key: AgentPubKey,
    _membrane_proof: &Option<MembraneProof>,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

// This is the unified validation callback for all entries and link types in this integrity zome
// Below is a match template for all of the variants of `DHT Ops` and entry and link types
//
// Holochain has already performed the following validation for you:
// - The action signature matches on the hash of its content and is signed by its author
// - The previous action exists, has a lower timestamp than the new action, and incremented sequence number
// - The previous action author is the same as the new action author
// - The timestamp of each action is after the DNA's origin time
// - AgentActivity authorities check that the agent hasn't forked their chain
// - The entry hash in the action matches the entry content
// - The entry type in the action matches the entry content
// - The entry size doesn't exceed the maximum entry size (currently 4MB)
// - Private entry types are not included in the Op content, and public entry types are
// - If the `Op` is an update or a delete, the original action exists and is a `Create` or `Update` action
// - If the `Op` is an update, the original entry exists and is of the same type as the new one
// - If the `Op` is a delete link, the original action exists and is a `CreateLink` action
// - Link tags don't exceed the maximum tag size (currently 1KB)
// - Countersigned entries include an action from each required signer
//
// You can read more about validation here: https://docs.rs/hdi/latest/hdi/index.html#data-validation
#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    let prefix_index_a = PrefixIndex::new(
        PREFIX_INDEX_A_NAME.into(),
        LinkTypes::PrefixIndexA,
        PREFIX_INDEX_A_WIDTH,
        PREFIX_INDEX_A_DEPTH,
    )?;

    let prefix_index_b = PrefixIndex::new(
        PREFIX_INDEX_B_NAME.into(),
        LinkTypes::PrefixIndexB,
        PREFIX_INDEX_B_WIDTH,
        PREFIX_INDEX_B_DEPTH,
    )?;

    let prefix_index_c = PrefixIndex::new(
        PREFIX_INDEX_C_NAME.into(),
        LinkTypes::PrefixIndexC,
        PREFIX_INDEX_C_WIDTH,
        PREFIX_INDEX_C_DEPTH,
    )?;

    match op.flattened::<(), LinkTypes>()? {
        FlatOp::StoreEntry(store_entry) => match store_entry {
            OpEntry::CreateEntry {
                app_entry: _,
                action: _,
            } => Ok(ValidateCallbackResult::Invalid(
                "There are no entry types in this integrity zome".to_string(),
            )),
            OpEntry::UpdateEntry { .. } => Ok(ValidateCallbackResult::Invalid(
                "There are no entry types in this integrity zome".to_string(),
            )),
            _ => Ok(ValidateCallbackResult::Valid),
        },
        FlatOp::RegisterUpdate(update_entry) => match update_entry {
            OpUpdate::Entry {
                original_action: _,
                original_app_entry: _,
                app_entry: _,
                action: _,
            } => Ok(ValidateCallbackResult::Invalid(
                "There are no entry types in this integrity zome".to_string(),
            )),
            _ => Ok(ValidateCallbackResult::Valid),
        },
        FlatOp::RegisterDelete(delete_entry) => match delete_entry {
            OpDelete::Entry {
                original_action: _,
                original_app_entry: _,
                action: _,
            } => Ok(ValidateCallbackResult::Invalid(
                "There are no entry types in this integrity zome".to_string(),
            )),
            _ => Ok(ValidateCallbackResult::Valid),
        },
        FlatOp::RegisterCreateLink {
            link_type,
            base_address: _,
            target_address: _,
            tag: _,
            action,
        } => match link_type {
            LinkTypes::PrefixIndexA => prefix_index_a.validate_create_link(action),
            LinkTypes::PrefixIndexB => prefix_index_b.validate_create_link(action),
            LinkTypes::PrefixIndexC => prefix_index_c.validate_create_link(action),
        },
        FlatOp::RegisterDeleteLink {
            link_type,
            base_address: _,
            target_address: _,
            tag: _,
            original_action,
            action,
        } => match link_type {
            LinkTypes::PrefixIndexA => prefix_index_a.validate_delete_link(action, original_action),
            LinkTypes::PrefixIndexB => prefix_index_b.validate_delete_link(action, original_action),
            LinkTypes::PrefixIndexC => prefix_index_c.validate_delete_link(action, original_action),
        },
        FlatOp::StoreRecord(store_record) => match store_record {
            // Complementary validation to the `StoreEntry` Op, in which the record itself is validated
            // If you want to optimize performance, you can remove the validation for an entry type here and keep it in `StoreEntry`
            // Notice that doing so will cause `must_get_valid_record` for this record to return a valid record even if the `StoreEntry` validation failed
            OpRecord::CreateEntry {
                app_entry: _,
                action: _,
            } => Ok(ValidateCallbackResult::Invalid(
                "There are no entry types in this integrity zome".to_string(),
            )),
            // Complementary validation to the `RegisterUpdate` Op, in which the record itself is validated
            // If you want to optimize performance, you can remove the validation for an entry type here and keep it in `StoreEntry` and in `RegisterUpdate`
            // Notice that doing so will cause `must_get_valid_record` for this record to return a valid record even if the other validations failed
            OpRecord::UpdateEntry { .. } => Ok(ValidateCallbackResult::Invalid(
                "There are no entry types in this integrity zome".to_string(),
            )),
            // Complementary validation to the `RegisterDelete` Op, in which the record itself is validated
            // If you want to optimize performance, you can remove the validation for an entry type here and keep it in `RegisterDelete`
            // Notice that doing so will cause `must_get_valid_record` for this record to return a valid record even if the `RegisterDelete` validation failed
            OpRecord::DeleteEntry { .. } => Ok(ValidateCallbackResult::Invalid(
                "There are no entry types in this integrity zome".to_string(),
            )),
            // Complementary validation to the `RegisterCreateLink` Op, in which the record itself is validated
            // If you want to optimize performance, you can remove the validation for an entry type here and keep it in `RegisterCreateLink`
            // Notice that doing so will cause `must_get_valid_record` for this record to return a valid record even if the `RegisterCreateLink` validation failed
            OpRecord::CreateLink {
                base_address: _,
                target_address: _,
                tag: _,
                link_type,
                action,
            } => match link_type {
                LinkTypes::PrefixIndexA => prefix_index_a.validate_create_link(action),
                LinkTypes::PrefixIndexB => prefix_index_b.validate_create_link(action),
                LinkTypes::PrefixIndexC => prefix_index_c.validate_create_link(action),
            },
            // Complementary validation to the `RegisterDeleteLink` Op, in which the record itself is validated
            // If you want to optimize performance, you can remove the validation for an entry type here and keep it in `RegisterDeleteLink`
            // Notice that doing so will cause `must_get_valid_record` for this record to return a valid record even if the `RegisterDeleteLink` validation failed
            OpRecord::DeleteLink {
                original_action_hash,
                base_address: _,
                action,
            } => {
                let record = must_get_valid_record(original_action_hash)?;
                let create_link = match record.action() {
                    Action::CreateLink(create_link) => create_link.clone(),
                    _ => {
                        return Ok(ValidateCallbackResult::Invalid(
                            "The action that a DeleteLink deletes must be a CreateLink".to_string(),
                        ));
                    }
                };
                let link_type =
                    match LinkTypes::from_type(create_link.zome_index, create_link.link_type)? {
                        Some(lt) => lt,
                        None => {
                            return Ok(ValidateCallbackResult::Valid);
                        }
                    };

                match link_type {
                    LinkTypes::PrefixIndexA => {
                        prefix_index_a.validate_delete_link(action, create_link)
                    }
                    LinkTypes::PrefixIndexB => {
                        prefix_index_b.validate_delete_link(action, create_link)
                    }
                    LinkTypes::PrefixIndexC => {
                        prefix_index_c.validate_delete_link(action, create_link)
                    }
                }
            }
            OpRecord::CreatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::UpdatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::CreateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::CreateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::UpdateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::UpdateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::Dna { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::OpenChain { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::CloseChain { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::InitZomesComplete { .. } => Ok(ValidateCallbackResult::Valid),
            _ => Ok(ValidateCallbackResult::Valid),
        },
        FlatOp::RegisterAgentActivity(agent_activity) => match agent_activity {
            OpActivity::CreateAgent { agent, action } => {
                let previous_action = must_get_action(action.prev_action)?;
                match previous_action.action() {
                    Action::AgentValidationPkg(AgentValidationPkg { membrane_proof, .. }) => validate_agent_joining(agent, membrane_proof),
                    _ => Ok(ValidateCallbackResult::Invalid("The previous action for a `CreateAgent` action must be an `AgentValidationPkg`".to_string()))
                }
            }
            _ => Ok(ValidateCallbackResult::Valid),
        },
    }
}
