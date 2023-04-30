use demo_integrity::LinkTypes;
use hdk::prelude::*;
use prefix_index::PrefixSearchIndex;

#[hdk_extern]
pub fn add_to_index_a(text: String) -> ExternResult<()> {
    let index = PrefixSearchIndex::new("demo_index_a".into(), LinkTypes::PrefixIndex, 3, 3)?;

    index.add_result(text)?;

    Ok(())
}

#[hdk_extern]
pub fn remove_from_index_a(text: String) -> ExternResult<()> {
    let index = PrefixSearchIndex::new("demo_index_a".into(), LinkTypes::PrefixIndex, 3, 3)?;

    index.remove_result(text)?;

    Ok(())
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct SearchIndexInput {
    query: String,
    limit: usize,
}
#[hdk_extern]
pub fn search_index_a(input: SearchIndexInput) -> ExternResult<Vec<String>> {
    let index = PrefixSearchIndex::new("demo_index_a".into(), LinkTypes::PrefixIndex, 3, 3)?;

    index.get_results(input.query, input.limit)
}

#[hdk_extern]
pub fn add_to_index_b(text: String) -> ExternResult<()> {
    let index: PrefixSearchIndex =
        PrefixSearchIndex::new("demo_index_b".into(), LinkTypes::PrefixIndex, 3, 5)?;

    index.add_result(text)?;

    Ok(())
}

#[hdk_extern]
pub fn search_index_b(input: SearchIndexInput) -> ExternResult<Vec<String>> {
    let index = PrefixSearchIndex::new("demo_index_b".into(), LinkTypes::PrefixIndex, 3, 5)?;

    index.get_results(input.query, input.limit)
}

#[hdk_extern]
pub fn add_to_index_c(text: String) -> ExternResult<()> {
    let index: PrefixSearchIndex =
        PrefixSearchIndex::new("demo_index_c".into(), LinkTypes::PrefixIndex, 4, 2)?;

    index.add_result(text)?;

    Ok(())
}

#[hdk_extern]
pub fn search_index_c(input: SearchIndexInput) -> ExternResult<Vec<String>> {
    let index = PrefixSearchIndex::new("demo_index_c".into(), LinkTypes::PrefixIndex, 4, 2)?;

    index.get_results(input.query, input.limit)
}
