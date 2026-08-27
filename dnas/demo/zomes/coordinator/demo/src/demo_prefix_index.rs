use demo_integrity::*;
use hc_prefix_index::PrefixIndex;
use hdk::prelude::*;

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

/// Remove a result, returning how many of the caller's own links were deleted.
/// Zero means the caller authored none of this result's links.
#[hdk_extern]
pub fn remove_from_index_a(text: String) -> ExternResult<usize> {
    let index = PrefixIndex::new(
        PREFIX_INDEX_A_NAME.into(),
        LinkTypes::PrefixIndexA,
        PREFIX_INDEX_A_WIDTH,
        PREFIX_INDEX_A_DEPTH,
    )?;

    index.remove_result(text, GetStrategy::Local)
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct SearchIndexInput {
    pub query: String,
    pub limit: usize,
}
#[hdk_extern]
pub fn search_index_a(input: SearchIndexInput) -> ExternResult<Vec<String>> {
    let index = PrefixIndex::new(
        PREFIX_INDEX_A_NAME.into(),
        LinkTypes::PrefixIndexA,
        PREFIX_INDEX_A_WIDTH,
        PREFIX_INDEX_A_DEPTH,
    )?;

    index.get_results(input.query, input.limit, GetStrategy::Local)
}

#[hdk_extern]
pub fn get_random_results_index_a(limit: usize) -> ExternResult<Vec<String>> {
    let index = PrefixIndex::new(
        PREFIX_INDEX_A_NAME.into(),
        LinkTypes::PrefixIndexA,
        PREFIX_INDEX_A_WIDTH,
        PREFIX_INDEX_A_DEPTH,
    )?;

    index.get_random_results(limit, GetStrategy::Local)
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

    index.get_results(input.query, input.limit, GetStrategy::Local)
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

    index.get_results(input.query, input.limit, GetStrategy::Local)
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
    index.remove_result_with_label(index_text, hashtag, GetStrategy::Local)?;

    Ok(())
}

fn make_hashtag_index_text(text: String) -> String {
    text.split('#').nth(1).unwrap_or(&text).to_string()
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
    index.remove_result_with_label(index_text, cashtag, GetStrategy::Local)?;

    Ok(())
}

fn make_cashtag_index_text(text: String) -> String {
    text.split('$').nth(1).unwrap_or(&text).to_string()
}

// --- Test-support functions -------------------------------------------------
// These bypass the PrefixIndex API to author links the library itself would
// never write, so the test suite can assert that validation rejects them.

/// Commit a link under the index root whose tag does not name its target path.
/// Validation must reject this: it would inject an arbitrary string into search
/// results at a path that doesn't exist.
#[hdk_extern]
pub fn commit_incoherent_index_link(tag: String) -> ExternResult<ActionHash> {
    let index_root = Path::from(vec![Component::from(PREFIX_INDEX_A_NAME.to_string())]);
    // Target a path that the tag does not name.
    let unrelated = Path::from(vec![
        Component::from(PREFIX_INDEX_A_NAME.to_string()),
        Component::from("not-the-tag".to_string()),
    ]);

    // Encode the tag exactly as TypedPath::make_tag does, so the link is
    // well-formed and validation must reject it on the tag/target mismatch
    // rather than on the tag decode.
    let tag = Path::from(vec![Component::from(tag)]).make_tag()?;

    create_link(
        index_root.path_entry_hash()?,
        unrelated.path_entry_hash()?,
        LinkTypes::PrefixIndexA,
        tag,
    )
}

/// Delete another agent's index link by its create-link action hash.
/// Validation must reject this: only the original author may delete a link.
#[hdk_extern]
pub fn delete_index_link(create_link_hash: ActionHash) -> ExternResult<ActionHash> {
    delete_link(create_link_hash, GetOptions::network())
}

/// The create-link actions under the index root, so a test can hand one to
/// `delete_index_link` from another agent.
#[hdk_extern]
pub fn get_index_root_links(_: ()) -> ExternResult<Vec<Link>> {
    let index_root = Path::from(vec![Component::from(PREFIX_INDEX_A_NAME.to_string())]);
    get_links(
        LinkQuery::new(
            index_root.path_entry_hash()?,
            LinkTypes::PrefixIndexA.try_into_filter()?,
        ),
        GetStrategy::Network,
    )
}
