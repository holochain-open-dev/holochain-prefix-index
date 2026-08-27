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
/// The index a link type belongs to. Constructing a `PrefixIndex` costs a
/// `zome_info()` host call, so build only the one the op needs, and only for
/// link ops.
fn prefix_index_for(link_type: LinkTypes) -> ExternResult<PrefixIndex> {
    match link_type {
        LinkTypes::PrefixIndexA => PrefixIndex::new(
            PREFIX_INDEX_A_NAME.into(),
            LinkTypes::PrefixIndexA,
            PREFIX_INDEX_A_WIDTH,
            PREFIX_INDEX_A_DEPTH,
        ),
        LinkTypes::PrefixIndexB => PrefixIndex::new(
            PREFIX_INDEX_B_NAME.into(),
            LinkTypes::PrefixIndexB,
            PREFIX_INDEX_B_WIDTH,
            PREFIX_INDEX_B_DEPTH,
        ),
        LinkTypes::PrefixIndexC => PrefixIndex::new(
            PREFIX_INDEX_C_NAME.into(),
            LinkTypes::PrefixIndexC,
            PREFIX_INDEX_C_WIDTH,
            PREFIX_INDEX_C_DEPTH,
        ),
    }
}

#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    match op.flattened::<(), LinkTypes>()? {
        FlatOp::CreateEntry(create_entry) => match create_entry {
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
        FlatOp::Update(update_entry) => match update_entry {
            OpUpdate::Entry {
                app_entry: _,
                action: _,
            } => Ok(ValidateCallbackResult::Invalid(
                "There are no entry types in this integrity zome".to_string(),
            )),
            _ => Ok(ValidateCallbackResult::Valid),
        },
        FlatOp::Delete(_delete_entry) => Ok(ValidateCallbackResult::Invalid(
            "There are no entry types in this integrity zome".to_string(),
        )),
        FlatOp::Link(op_link) => match op_link {
            OpLink::CreateLink { link_type, action } => {
                prefix_index_for(link_type)?.validate_create_link(action)
            }
            // Delete validation reads no index config, so don't build one.
            OpLink::DeleteLink {
                original_action,
                action,
                ..
            } => PrefixIndex::validate_delete_link(action, original_action),
        },
        FlatOp::CreateRecord(create_record) => match create_record {
            // Complementary validation to the `CreateEntry` Op, in which the record itself is validated
            // If you want to optimize performance, you can remove the validation for an entry type here and keep it in `CreateEntry`
            // Notice that doing so will cause `must_get_valid_record` for this record to return a valid record even if the `CreateEntry` validation failed
            OpRecord::CreateEntry {
                app_entry: _,
                action: _,
            } => Ok(ValidateCallbackResult::Invalid(
                "There are no entry types in this integrity zome".to_string(),
            )),
            // Complementary validation to the `Update` Op, in which the record itself is validated
            // If you want to optimize performance, you can remove the validation for an entry type here and keep it in `CreateEntry` and in `Update`
            // Notice that doing so will cause `must_get_valid_record` for this record to return a valid record even if the other validations failed
            OpRecord::UpdateEntry { .. } => Ok(ValidateCallbackResult::Invalid(
                "There are no entry types in this integrity zome".to_string(),
            )),
            // Complementary validation to the `Delete` Op, in which the record itself is validated
            // If you want to optimize performance, you can remove the validation for an entry type here and keep it in `Delete`
            // Notice that doing so will cause `must_get_valid_record` for this record to return a valid record even if the `Delete` validation failed
            OpRecord::DeleteEntry { .. } => Ok(ValidateCallbackResult::Invalid(
                "There are no entry types in this integrity zome".to_string(),
            )),
            // Link validation is authoritative in the `Link(OpLink::…)` arms above,
            // which every base-address authority runs (and which warrants a bad
            // author). Repeating it here would duplicate the root-hash and
            // path-hash host calls for CreateLink, and for DeleteLink would force
            // a blocking `must_get_valid_record` fetch of the original action, since
            // the record op carries no link type to scope on. The tradeoff is the
            // one hdi's scaffold documents: this action-hash authority may store a
            // link op that the base authority rejected, so don't rely on a link
            // record's validity as evidence the link itself is valid.
            OpRecord::CreateLink { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::DeleteLink { .. } => Ok(ValidateCallbackResult::Valid),
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
        FlatOp::AgentActivity(agent_activity) => match agent_activity {
            OpActivity::CreateAgent { agent, action } => {
                let prev_action_hash = action.prev_action().cloned().ok_or_else(|| {
                    wasm_error!(WasmErrorInner::Guest(
                        "CreateAgent action must have a previous action".to_string()
                    ))
                })?;
                let previous_action = must_get_action(prev_action_hash)?;
                match &previous_action.action().data {
                    ActionData::AgentValidationPkg(AgentValidationPkgData { membrane_proof }) => validate_agent_joining(agent, membrane_proof),
                    _ => Ok(ValidateCallbackResult::Invalid("The previous action for a `CreateAgent` action must be an `AgentValidationPkg`".to_string()))
                }
            }
            _ => Ok(ValidateCallbackResult::Valid),
        },
    }
}
