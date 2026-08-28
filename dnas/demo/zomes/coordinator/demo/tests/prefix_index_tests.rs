use demo::SearchIndexInput;
use hdk::prelude::*;
use holochain::{
    prelude::{random_network_seed, DnaFile},
    sweettest::*,
};

static DNA_BUNDLE: tokio::sync::OnceCell<DnaFile> = tokio::sync::OnceCell::const_new();

/// The demo DNA with a fresh network seed, so concurrently running tests form
/// disjoint DHT networks instead of cross-contaminating each other's indexes.
/// The bundle itself is decoded from disk once for the whole suite.
pub async fn load_dna() -> DnaFile {
    let dna = DNA_BUNDLE
        .get_or_init(|| async {
            let dna_path = std::env::current_dir()
                .unwrap()
                .join("../../../workdir/demo.dna");
            SweetDnaFile::from_bundle(&dna_path).await.unwrap()
        })
        .await;

    dna.update_modifiers(DnaModifiersOpt::none().with_network_seed(random_network_seed()))
}

/// Two conductors sharing a local rendezvous, each with the demo app installed.
async fn setup_two_agents() -> (SweetConductorBatch, SweetCell, SweetCell) {
    let dna = load_dna().await;
    let mut conductors = SweetConductorBatch::standard(2).await;
    let ((alice,), (bob,)) = conductors
        .setup_app("demo", &[dna])
        .await
        .unwrap()
        .into_tuples();
    conductors.exchange_peer_info().await;
    (conductors, alice, bob)
}

/// The four strings every search test seeds into an index.
const SUPER_RESULTS: &[&str] = &[
    "superdupercool",
    "superdupercrazy",
    "supercomputing",
    "supersaturates",
];

/// The labeled (hashtag/cashtag) variants used by the label tests.
const TAGGED_RESULTS: &[&str] = &[
    "#superdupercool",
    "#superdupercrazy",
    "#supercomputing",
    "#supersaturates",
    "$supercomputing",
    "$supersaturates",
];

/// Assert that every expected string is present in the results, naming the
/// missing string on failure.
fn assert_contains_all(results: &[String], expected: &[&str]) {
    for item in expected {
        assert!(
            results.iter().any(|r| r == item),
            "expected {item:?} in results, got {results:?}"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn search_prefix_index_with_width_3_and_depth_3() {
    let (conductors, alice, bob) = setup_two_agents().await;

    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_to_index_a",
            String::from("superdupercool"),
        )
        .await;
    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_to_index_a",
            String::from("superdupercrazy"),
        )
        .await;
    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_to_index_a",
            String::from("supercomputing"),
        )
        .await;

    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_to_index_a",
            String::from("supersaturates"),
        )
        .await;

    await_consistency_s(60, [&alice, &bob]).await.unwrap();

    // Read through bob to verify the index actually propagated over the DHT,
    // not just that alice can read her own chain.
    let results: Vec<String> = conductors[1]
        .call(
            &bob.zome("demo"),
            "search_index_a",
            SearchIndexInput {
                query: "sup".into(),
                limit: 4,
            },
        )
        .await;

    assert_contains_all(&results, SUPER_RESULTS);

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_a",
            SearchIndexInput {
                query: "sup".into(),
                limit: 1,
            },
        )
        .await;

    assert!(results.contains(&String::from("supercomputing")));

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_a",
            SearchIndexInput {
                query: "super".into(),
                limit: 5,
            },
        )
        .await;

    assert_contains_all(&results, SUPER_RESULTS);
    assert_eq!(results[0], String::from("supercomputing"));

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_a",
            SearchIndexInput {
                query: "superdupe".into(),
                limit: 5,
            },
        )
        .await;

    assert_contains_all(&results, SUPER_RESULTS);
    assert_eq!(results[0], String::from("superdupercool"));

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_a",
            SearchIndexInput {
                query: String::from("superdupercool"),
                limit: 5,
            },
        )
        .await;

    assert_contains_all(&results, SUPER_RESULTS);
    assert_eq!(results[0], String::from("superdupercool"));

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_a",
            SearchIndexInput {
                query: "superduperbad".into(),
                limit: 5,
            },
        )
        .await;

    assert_contains_all(&results, SUPER_RESULTS);
    assert_eq!(results[0], String::from("superdupercool"));

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_a",
            SearchIndexInput {
                query: "supersaturday".into(),
                limit: 5,
            },
        )
        .await;

    assert_contains_all(&results, SUPER_RESULTS);
    assert_eq!(results[0], String::from("supersaturates"));

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_a",
            SearchIndexInput {
                query: "cow".into(),
                limit: 5,
            },
        )
        .await;

    assert_eq!(results.len(), 0);
}

#[tokio::test(flavor = "multi_thread")]
async fn search_prefix_index_with_width_3_and_depth_5() {
    let (conductors, alice, bob) = setup_two_agents().await;

    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_to_index_b",
            String::from("superdupercool"),
        )
        .await;
    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_to_index_b",
            String::from("superdupercrazy"),
        )
        .await;
    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_to_index_b",
            String::from("supercomputing"),
        )
        .await;
    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_to_index_b",
            String::from("supersaturates"),
        )
        .await;

    await_consistency_s(60, [&alice, &bob]).await.unwrap();

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_b",
            SearchIndexInput {
                query: "sup".into(),
                limit: 4,
            },
        )
        .await;

    assert_contains_all(&results, SUPER_RESULTS);

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_b",
            SearchIndexInput {
                query: "sup".into(),
                limit: 1,
            },
        )
        .await;

    assert!(results.contains(&String::from("supercomputing")));

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_b",
            SearchIndexInput {
                query: "super".into(),
                limit: 5,
            },
        )
        .await;

    assert_contains_all(&results, SUPER_RESULTS);
    assert_eq!(results[0], String::from("supercomputing"));

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_b",
            SearchIndexInput {
                query: "superdupe".into(),
                limit: 5,
            },
        )
        .await;

    assert_contains_all(&results, SUPER_RESULTS);
    assert_eq!(results[0], String::from("superdupercool"));

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_b",
            SearchIndexInput {
                query: String::from("superdupercool"),
                limit: 5,
            },
        )
        .await;

    assert_contains_all(&results, SUPER_RESULTS);
    assert_eq!(results[0], String::from("superdupercool"));

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_b",
            SearchIndexInput {
                query: "superduperbad".into(),
                limit: 5,
            },
        )
        .await;

    assert_contains_all(&results, SUPER_RESULTS);
    assert_eq!(results[0], String::from("superdupercool"));

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_b",
            SearchIndexInput {
                query: "supersaturday".into(),
                limit: 5,
            },
        )
        .await;

    assert_contains_all(&results, SUPER_RESULTS);
    assert_eq!(results[0], String::from("supersaturates"));

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_b",
            SearchIndexInput {
                query: "cow".into(),
                limit: 5,
            },
        )
        .await;

    assert_eq!(results.len(), 0);
}

#[tokio::test(flavor = "multi_thread")]
async fn search_prefix_index_with_width_4_and_depth_2() {
    let (conductors, alice, bob) = setup_two_agents().await;

    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_to_index_c",
            String::from("superdupercool"),
        )
        .await;
    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_to_index_c",
            String::from("superdupercrazy"),
        )
        .await;
    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_to_index_c",
            String::from("supercomputing"),
        )
        .await;
    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_to_index_c",
            String::from("supersaturates"),
        )
        .await;

    await_consistency_s(60, [&alice, &bob]).await.unwrap();

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_c",
            SearchIndexInput {
                query: "sup".into(),
                limit: 4,
            },
        )
        .await;
    assert_eq!(results.len(), 0);

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_c",
            SearchIndexInput {
                query: "supe".into(),
                limit: 4,
            },
        )
        .await;

    assert_contains_all(&results, SUPER_RESULTS);

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_c",
            SearchIndexInput {
                query: "super".into(),
                limit: 5,
            },
        )
        .await;

    assert_contains_all(&results, SUPER_RESULTS);
    assert_eq!(results[0], String::from("supercomputing"));

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_c",
            SearchIndexInput {
                query: "superdupe".into(),
                limit: 5,
            },
        )
        .await;

    assert_contains_all(&results, SUPER_RESULTS);
    assert_eq!(results[0], String::from("superdupercool"));

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_c",
            SearchIndexInput {
                query: String::from("superdupercool"),
                limit: 5,
            },
        )
        .await;

    assert_contains_all(&results, SUPER_RESULTS);
    assert_eq!(results[0], String::from("superdupercool"));

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_c",
            SearchIndexInput {
                query: "superduperbad".into(),
                limit: 5,
            },
        )
        .await;

    assert_contains_all(&results, SUPER_RESULTS);
    assert_eq!(results[0], String::from("superdupercool"));

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_c",
            SearchIndexInput {
                query: "supersaturday".into(),
                limit: 5,
            },
        )
        .await;

    assert_contains_all(&results, SUPER_RESULTS);
    assert_eq!(results[0], String::from("supersaturates"));

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_c",
            SearchIndexInput {
                query: "cow".into(),
                limit: 5,
            },
        )
        .await;

    assert_eq!(results.len(), 0);
}

#[tokio::test(flavor = "multi_thread")]
async fn remove_result_from_index() {
    let (conductors, alice, bob) = setup_two_agents().await;

    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_to_index_a",
            String::from("superduper"),
        )
        .await;

    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_to_index_a",
            String::from("superdupercrazy"),
        )
        .await;

    await_consistency_s(60, [&alice, &bob]).await.unwrap();

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_a",
            SearchIndexInput {
                query: "sup".into(),
                limit: 4,
            },
        )
        .await;

    assert_contains_all(&results, &["superduper", "superdupercrazy"]);

    let deleted: usize = conductors[0]
        .call(
            &alice.zome("demo"),
            "remove_from_index_a",
            String::from("superduper"),
        )
        .await;
    assert!(deleted > 0, "alice authored this result's links");

    await_consistency_s(60, [&alice, &bob]).await.unwrap();

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_a",
            SearchIndexInput {
                query: "sup".into(),
                limit: 4,
            },
        )
        .await;

    assert_eq!(vec![String::from("superdupercrazy")], results);
}

#[tokio::test(flavor = "multi_thread")]
async fn add_result_with_labels() {
    let (conductors, alice, bob) = setup_two_agents().await;

    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_hashtag_to_index_a",
            String::from("#superdupercool"),
        )
        .await;

    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_hashtag_to_index_a",
            String::from("#superdupercrazy"),
        )
        .await;

    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_hashtag_to_index_a",
            String::from("#supercomputing"),
        )
        .await;

    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_hashtag_to_index_a",
            String::from("#supersaturates"),
        )
        .await;

    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_cashtag_to_index_a",
            String::from("$supercomputing"),
        )
        .await;

    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_cashtag_to_index_a",
            String::from("$supersaturates"),
        )
        .await;

    await_consistency_s(60, [&alice, &bob]).await.unwrap();

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_a",
            SearchIndexInput {
                query: "sup".into(),
                limit: 10,
            },
        )
        .await;

    assert_contains_all(&results, TAGGED_RESULTS);
    assert_eq!(String::from("#supercomputing"), results[0]);

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_a",
            SearchIndexInput {
                query: "super".into(),
                limit: 10,
            },
        )
        .await;

    assert_contains_all(&results, TAGGED_RESULTS);
    assert_eq!(String::from("#supercomputing"), results[0]);

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_a",
            SearchIndexInput {
                query: "superdupe".into(),
                limit: 10,
            },
        )
        .await;

    assert_contains_all(&results, TAGGED_RESULTS);
    assert_eq!(String::from("#superdupercool"), results[0]);

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_a",
            SearchIndexInput {
                query: "superdupercool".into(),
                limit: 10,
            },
        )
        .await;

    assert_contains_all(&results, TAGGED_RESULTS);
    assert_eq!(String::from("#superdupercool"), results[0]);

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_a",
            SearchIndexInput {
                query: "superduperbad".into(),
                limit: 10,
            },
        )
        .await;

    assert_contains_all(&results, TAGGED_RESULTS);
    assert_eq!(String::from("#superdupercool"), results[0]);

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_a",
            SearchIndexInput {
                query: "supersaturday".into(),
                limit: 10,
            },
        )
        .await;

    assert_contains_all(&results, TAGGED_RESULTS);
    assert_eq!(String::from("#supersaturates"), results[0]);

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_a",
            SearchIndexInput {
                query: "cow".into(),
                limit: 10,
            },
        )
        .await;
    assert_eq!(results.len(), 0);
}

#[tokio::test(flavor = "multi_thread")]
async fn presevere_letter_case_in_result_but_ignore_letter_case_in_index() {
    let (conductors, alice, bob) = setup_two_agents().await;

    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_hashtag_to_index_a",
            String::from("#HOLOCHAIN"),
        )
        .await;

    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_hashtag_to_index_a",
            String::from("#holosapian"),
        )
        .await;

    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_cashtag_to_index_a",
            String::from("$HOLY"),
        )
        .await;

    await_consistency_s(60, [&alice, &bob]).await.unwrap();

    let results: Vec<String> = conductors[0]
        .call(
            &alice.zome("demo"),
            "search_index_a",
            SearchIndexInput {
                query: "holo".into(),
                limit: 5,
            },
        )
        .await;

    assert_contains_all(&results, &["#HOLOCHAIN", "#holosapian", "$HOLY"]);
}

#[tokio::test(flavor = "multi_thread")]
async fn get_random_results_returns_random_results() {
    let (conductors, alice, bob) = setup_two_agents().await;

    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_hashtag_to_index_a",
            String::from("#HOLOCHAIN"),
        )
        .await;

    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_hashtag_to_index_a",
            String::from("#holosapian"),
        )
        .await;

    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_cashtag_to_index_a",
            String::from("$HOLY"),
        )
        .await;

    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_cashtag_to_index_a",
            String::from("$CAT"),
        )
        .await;

    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_cashtag_to_index_a",
            String::from("$DOGGO"),
        )
        .await;

    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_hashtag_to_index_a",
            String::from("#monkeys"),
        )
        .await;

    await_consistency_s(240, [&alice, &bob]).await.unwrap();

    let results1: Vec<String> = conductors[0]
        .call(&alice.zome("demo"), "get_random_results_index_a", 1)
        .await;

    let results2: Vec<String> = conductors[0]
        .call(&alice.zome("demo"), "get_random_results_index_a", 1)
        .await;

    let results3: Vec<String> = conductors[0]
        .call(&alice.zome("demo"), "get_random_results_index_a", 1)
        .await;

    let results4: Vec<String> = conductors[0]
        .call(&alice.zome("demo"), "get_random_results_index_a", 1)
        .await;

    let results5: Vec<String> = conductors[0]
        .call(&alice.zome("demo"), "get_random_results_index_a", 1)
        .await;

    // Assert we did not get the exact same result 5 times
    let mut unique_results = HashSet::new();
    unique_results.insert(&results1[0]);
    unique_results.insert(&results2[0]);
    unique_results.insert(&results3[0]);
    unique_results.insert(&results4[0]);
    unique_results.insert(&results5[0]);
    assert!(unique_results.len() > 1)
}

#[tokio::test(flavor = "multi_thread")]
async fn rejects_a_link_whose_tag_does_not_name_its_target() {
    let (conductors, alice, _bob) = setup_two_agents().await;

    // Seed the index so its root path exists.
    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_to_index_a",
            String::from("superdupercool"),
        )
        .await;

    // A link under the index root whose tag names a different path than its
    // target must be rejected: it would inject an arbitrary search result.
    let result: Result<ActionHash, _> = conductors[0]
        .call_fallible(
            &alice.zome("demo"),
            "commit_incoherent_index_link",
            String::from("spam-the-type-ahead"),
        )
        .await;

    assert!(
        result.is_err(),
        "expected an incoherent index link to be rejected, got {result:?}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn rejects_deleting_another_agents_link() {
    let (conductors, alice, bob) = setup_two_agents().await;

    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_to_index_a",
            String::from("superdupercool"),
        )
        .await;

    await_consistency_s(60, [&alice, &bob]).await.unwrap();

    let links: Vec<Link> = conductors[1]
        .call(&bob.zome("demo"), "get_index_root_links", ())
        .await;
    let alice_link = links
        .into_iter()
        .find(|link| link.author == *alice.agent_pubkey())
        .expect("alice's index link should have reached bob");

    let result: Result<ActionHash, _> = conductors[1]
        .call_fallible(
            &bob.zome("demo"),
            "delete_index_link",
            alice_link.create_link_hash,
        )
        .await;

    assert!(
        result.is_err(),
        "expected bob's delete of alice's link to be rejected, got {result:?}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn removing_a_shared_result_reports_whose_links_were_deleted() {
    let (conductors, alice, bob) = setup_two_agents().await;

    // Alice indexes the string first, so she authors its links. Path creation
    // is deduplicated across authors, so bob's add authors nothing.
    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_to_index_a",
            String::from("superdupercool"),
        )
        .await;
    await_consistency_s(60, [&alice, &bob]).await.unwrap();
    let _: () = conductors[1]
        .call(
            &bob.zome("demo"),
            "add_to_index_a",
            String::from("superdupercool"),
        )
        .await;
    await_consistency_s(60, [&alice, &bob]).await.unwrap();

    // Bob owns none of the links, so his removal deletes nothing and must say
    // so rather than silently reporting success. Crucially it must not error:
    // attempting to delete alice's link would fail validation and roll back.
    let deleted: usize = conductors[1]
        .call(
            &bob.zome("demo"),
            "remove_from_index_a",
            String::from("superdupercool"),
        )
        .await;
    assert_eq!(deleted, 0, "bob authored none of this result's links");

    // ...and the result is still indexed, for both agents.
    await_consistency_s(60, [&alice, &bob]).await.unwrap();
    for (conductor, cell) in [(&conductors[0], &alice), (&conductors[1], &bob)] {
        let results: Vec<String> = conductor
            .call(
                &cell.zome("demo"),
                "search_index_a",
                SearchIndexInput {
                    query: "sup".into(),
                    limit: 5,
                },
            )
            .await;
        assert_contains_all(&results, &["superdupercool"]);
    }

    // Alice owns the links, so her removal deletes them and the result goes.
    let deleted: usize = conductors[0]
        .call(
            &alice.zome("demo"),
            "remove_from_index_a",
            String::from("superdupercool"),
        )
        .await;
    assert!(deleted > 0, "alice authored this result's links");

    await_consistency_s(60, [&alice, &bob]).await.unwrap();
    for (conductor, cell) in [(&conductors[0], &alice), (&conductors[1], &bob)] {
        let results: Vec<String> = conductor
            .call(
                &cell.zome("demo"),
                "search_index_a",
                SearchIndexInput {
                    query: "sup".into(),
                    limit: 5,
                },
            )
            .await;
        assert!(
            !results.contains(&String::from("superdupercool")),
            "expected the result to be gone after its author removed it, got {results:?}"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn indexes_a_result_shorter_than_the_index_width() {
    let (conductors, alice, bob) = setup_two_agents().await;

    // Shorter than index A's width of 3, so it shards to zero components and
    // hangs directly off the index root.
    let _: () = conductors[0]
        .call(&alice.zome("demo"), "add_to_index_a", String::from("ab"))
        .await;

    await_consistency_s(60, [&alice, &bob]).await.unwrap();

    let results: Vec<String> = conductors[1]
        .call(
            &bob.zome("demo"),
            "search_index_a",
            SearchIndexInput {
                query: "ab".into(),
                limit: 5,
            },
        )
        .await;

    assert_contains_all(&results, &["ab"]);
}

#[tokio::test(flavor = "multi_thread")]
async fn removal_keeps_the_chain_another_agents_link_hangs_off() {
    let (conductors, alice, bob) = setup_two_agents().await;

    // Alice indexes the result, authoring the whole chain down to the leaf.
    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "add_to_index_a",
            String::from("superdupercool"),
        )
        .await;
    await_consistency_s(60, [&alice, &bob]).await.unwrap();

    // Bob authors a second link to the same leaf, as happens when two agents
    // race an add and one already sees the parent chain.
    let _: ActionHash = conductors[1]
        .call(
            &bob.zome("demo"),
            "add_duplicate_leaf_link",
            String::from("superdupercool"),
        )
        .await;
    await_consistency_s(60, [&alice, &bob]).await.unwrap();

    // Alice removes her own links. Bob's link to the leaf survives, so the
    // chain it hangs off must survive too — pruning it would orphan Bob's link
    // and de-index the result for everyone.
    let deleted: usize = conductors[0]
        .call(
            &alice.zome("demo"),
            "remove_from_index_a",
            String::from("superdupercool"),
        )
        .await;
    assert!(deleted > 0, "alice authored this result's links");

    await_consistency_s(60, [&alice, &bob]).await.unwrap();

    for (conductor, cell) in [(&conductors[0], &alice), (&conductors[1], &bob)] {
        let results: Vec<String> = conductor
            .call(
                &cell.zome("demo"),
                "search_index_a",
                SearchIndexInput {
                    query: "sup".into(),
                    limit: 5,
                },
            )
            .await;
        assert_contains_all(&results, &["superdupercool"]);
    }
}
