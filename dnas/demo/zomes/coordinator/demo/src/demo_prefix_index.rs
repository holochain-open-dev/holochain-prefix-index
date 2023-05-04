use demo_integrity::*;
use hdk::prelude::*;
use prefix_index::PrefixIndex;

#[hdk_extern]
pub fn add_to_index_a(text: String) -> ExternResult<()> {
    let index = PrefixIndex::new(
        PREFIX_INDEX_A_NAME.into(),
        LinkTypes::PrefixIndexA,
        PREFIX_INDEX_A_WIDTH,
        PREFIX_INDEX_A_DEPTH,
    )?;

    index.add_result(text)?;

    Ok(())
}

#[hdk_extern]
pub fn remove_from_index_a(text: String) -> ExternResult<()> {
    let index = PrefixIndex::new(
        PREFIX_INDEX_A_NAME.into(),
        LinkTypes::PrefixIndexA,
        PREFIX_INDEX_A_WIDTH,
        PREFIX_INDEX_A_DEPTH,
    )?;

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
    let index = PrefixIndex::new(
        PREFIX_INDEX_A_NAME.into(),
        LinkTypes::PrefixIndexA,
        PREFIX_INDEX_A_WIDTH,
        PREFIX_INDEX_A_DEPTH,
    )?;

    index.get_results(input.query, input.limit)
}

#[hdk_extern]
pub fn add_to_index_b(text: String) -> ExternResult<()> {
    let index = PrefixIndex::new(
        PREFIX_INDEX_B_NAME.into(),
        LinkTypes::PrefixIndexB,
        PREFIX_INDEX_B_WIDTH,
        PREFIX_INDEX_B_DEPTH,
    )?;

    index.add_result(text)?;

    Ok(())
}

#[hdk_extern]
pub fn search_index_b(input: SearchIndexInput) -> ExternResult<Vec<String>> {
    let index = PrefixIndex::new(
        PREFIX_INDEX_B_NAME.into(),
        LinkTypes::PrefixIndexB,
        PREFIX_INDEX_B_WIDTH,
        PREFIX_INDEX_B_DEPTH,
    )?;

    index.get_results(input.query, input.limit)
}

#[hdk_extern]
pub fn add_to_index_c(text: String) -> ExternResult<()> {
    let index = PrefixIndex::new(
        PREFIX_INDEX_C_NAME.into(),
        LinkTypes::PrefixIndexC,
        PREFIX_INDEX_C_WIDTH,
        PREFIX_INDEX_C_DEPTH,
    )?;

    index.add_result(text)?;

    Ok(())
}

#[hdk_extern]
pub fn search_index_c(input: SearchIndexInput) -> ExternResult<Vec<String>> {
    let index = PrefixIndex::new(
        PREFIX_INDEX_C_NAME.into(),
        LinkTypes::PrefixIndexC,
        PREFIX_INDEX_C_WIDTH,
        PREFIX_INDEX_C_DEPTH,
    )?;

    index.get_results(input.query, input.limit)
}

#[hdk_extern]
pub fn add_hashtag_to_index_a(hashtag: String) -> ExternResult<()> {
    let index = PrefixIndex::new(
        PREFIX_INDEX_A_NAME.into(),
        LinkTypes::PrefixIndexA,
        PREFIX_INDEX_A_WIDTH,
        PREFIX_INDEX_A_DEPTH,
    )?;

    let index_text = make_hashtag_index_text(hashtag.clone());
    index.add_result_with_label(index_text, hashtag)?;

    Ok(())
}

#[hdk_extern]
pub fn remove_hashtag_from_index_a(hashtag: String) -> ExternResult<()> {
    let index = PrefixIndex::new(
        PREFIX_INDEX_A_NAME.into(),
        LinkTypes::PrefixIndexA,
        PREFIX_INDEX_A_WIDTH,
        PREFIX_INDEX_A_DEPTH,
    )?;

    let index_text = make_hashtag_index_text(hashtag.clone());
    index.remove_result_with_label(index_text, hashtag)?;

    Ok(())
}

fn make_hashtag_index_text(text: String) -> String {
    text
        .split('#')
        .nth(1)
        .unwrap_or(&text)
        .to_string()
}

#[hdk_extern]
pub fn add_cashtag_to_index_a(cashtag: String) -> ExternResult<()> {
    let index = PrefixIndex::new(
        PREFIX_INDEX_A_NAME.into(),
        LinkTypes::PrefixIndexA,
        PREFIX_INDEX_A_WIDTH,
        PREFIX_INDEX_A_DEPTH,
    )?;

    let index_text = make_cashtag_index_text(cashtag.clone());
    index.add_result_with_label(index_text, cashtag)?;

    Ok(())
}

#[hdk_extern]
pub fn remove_cashtag_from_index_a(cashtag: String) -> ExternResult<()> {
    let index = PrefixIndex::new(
        PREFIX_INDEX_A_NAME.into(),
        LinkTypes::PrefixIndexA,
        PREFIX_INDEX_A_WIDTH,
        PREFIX_INDEX_A_DEPTH,
    )?;

    let index_text = make_cashtag_index_text(cashtag.clone());
    index.remove_result_with_label(index_text, cashtag)?;

    Ok(())
}

fn make_cashtag_index_text(text: String) -> String {
    text
        .split('$')
        .nth(1)
        .unwrap_or(&text)
        .to_string()
}