use demo::SearchIndexInput;
use hdk::prelude::*;
use holochain::{
    prelude::{random_network_seed, DnaFile},
    sweettest::*,
};

// Each test gets its own network seed so concurrently running tests form
// disjoint DHT networks instead of cross-contaminating each other's indexes.
pub async fn load_dna() -> DnaFile {
    let dna_path = std::env::current_dir()
        .unwrap()
        .join("../../../workdir/demo.dna");
    SweetDnaFile::from_bundle_with_overrides(
        &dna_path,
        DnaModifiersOpt::<SerializedBytes>::none().with_network_seed(random_network_seed()),
    )
    .await
    .unwrap()
}

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
    let dna = load_dna().await;
    let mut conductors: SweetConductorBatch = SweetConductorBatch::standard(2).await;
    let ((alice,), (bob,)) = conductors
        .setup_app("demo", &[dna])
        .await
        .unwrap()
        .into_tuples();
    conductors.exchange_peer_info().await;

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

    assert_contains_all(
        &results,
        &[
            "superdupercool",
            "superdupercrazy",
            "supercomputing",
            "supersaturates",
        ],
    );

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

    assert_contains_all(
        &results,
        &[
            "superdupercool",
            "superdupercrazy",
            "supercomputing",
            "supersaturates",
        ],
    );
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

    assert_contains_all(
        &results,
        &[
            "superdupercool",
            "superdupercrazy",
            "supercomputing",
            "supersaturates",
        ],
    );
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

    assert_contains_all(
        &results,
        &[
            "superdupercool",
            "superdupercrazy",
            "supercomputing",
            "supersaturates",
        ],
    );
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

    assert_contains_all(
        &results,
        &[
            "superdupercool",
            "superdupercrazy",
            "supercomputing",
            "supersaturates",
        ],
    );
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

    assert_contains_all(
        &results,
        &[
            "superdupercool",
            "superdupercrazy",
            "supercomputing",
            "supersaturates",
        ],
    );
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
    let dna = load_dna().await;
    let mut conductors: SweetConductorBatch = SweetConductorBatch::standard(2).await;
    let ((alice,), (bob,)) = conductors
        .setup_app("demo", &[dna])
        .await
        .unwrap()
        .into_tuples();
    conductors.exchange_peer_info().await;

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

    assert_contains_all(
        &results,
        &[
            "superdupercool",
            "superdupercrazy",
            "supercomputing",
            "supersaturates",
        ],
    );

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

    assert_contains_all(
        &results,
        &[
            "superdupercool",
            "superdupercrazy",
            "supercomputing",
            "supersaturates",
        ],
    );
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

    assert_contains_all(
        &results,
        &[
            "superdupercool",
            "superdupercrazy",
            "supercomputing",
            "supersaturates",
        ],
    );
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

    assert_contains_all(
        &results,
        &[
            "superdupercool",
            "superdupercrazy",
            "supercomputing",
            "supersaturates",
        ],
    );
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

    assert_contains_all(
        &results,
        &[
            "superdupercool",
            "superdupercrazy",
            "supercomputing",
            "supersaturates",
        ],
    );
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

    assert_contains_all(
        &results,
        &[
            "superdupercool",
            "superdupercrazy",
            "supercomputing",
            "supersaturates",
        ],
    );
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
    let dna = load_dna().await;
    let mut conductors: SweetConductorBatch = SweetConductorBatch::standard(2).await;
    let ((alice,), (bob,)) = conductors
        .setup_app("demo", &[dna])
        .await
        .unwrap()
        .into_tuples();
    conductors.exchange_peer_info().await;

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

    assert_contains_all(
        &results,
        &[
            "superdupercool",
            "superdupercrazy",
            "supercomputing",
            "supersaturates",
        ],
    );

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

    assert_contains_all(
        &results,
        &[
            "superdupercool",
            "superdupercrazy",
            "supercomputing",
            "supersaturates",
        ],
    );
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

    assert_contains_all(
        &results,
        &[
            "superdupercool",
            "superdupercrazy",
            "supercomputing",
            "supersaturates",
        ],
    );
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

    assert_contains_all(
        &results,
        &[
            "superdupercool",
            "superdupercrazy",
            "supercomputing",
            "supersaturates",
        ],
    );
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

    assert_contains_all(
        &results,
        &[
            "superdupercool",
            "superdupercrazy",
            "supercomputing",
            "supersaturates",
        ],
    );
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

    assert_contains_all(
        &results,
        &[
            "superdupercool",
            "superdupercrazy",
            "supercomputing",
            "supersaturates",
        ],
    );
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
    let dna = load_dna().await;
    let mut conductors: SweetConductorBatch = SweetConductorBatch::standard(2).await;
    let ((alice,), (bob,)) = conductors
        .setup_app("demo", &[dna])
        .await
        .unwrap()
        .into_tuples();
    conductors.exchange_peer_info().await;

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

    let _: () = conductors[0]
        .call(
            &alice.zome("demo"),
            "remove_from_index_a",
            String::from("superduper"),
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

    assert_eq!(vec![String::from("superdupercrazy")], results);
}

#[tokio::test(flavor = "multi_thread")]
async fn add_result_with_labels() {
    let dna = load_dna().await;
    let mut conductors: SweetConductorBatch = SweetConductorBatch::standard(2).await;
    let ((alice,), (bob,)) = conductors
        .setup_app("demo", &[dna])
        .await
        .unwrap()
        .into_tuples();
    conductors.exchange_peer_info().await;

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

    assert_contains_all(
        &results,
        &[
            "#superdupercool",
            "#superdupercrazy",
            "#supercomputing",
            "#supersaturates",
            "$supercomputing",
            "$supersaturates",
        ],
    );
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

    assert_contains_all(
        &results,
        &[
            "#superdupercool",
            "#superdupercrazy",
            "#supercomputing",
            "#supersaturates",
            "$supercomputing",
            "$supersaturates",
        ],
    );
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

    assert_contains_all(
        &results,
        &[
            "#superdupercool",
            "#superdupercrazy",
            "#supercomputing",
            "#supersaturates",
            "$supercomputing",
            "$supersaturates",
        ],
    );
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

    assert_contains_all(
        &results,
        &[
            "#superdupercool",
            "#superdupercrazy",
            "#supercomputing",
            "#supersaturates",
            "$supercomputing",
            "$supersaturates",
        ],
    );
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

    assert_contains_all(
        &results,
        &[
            "#superdupercool",
            "#superdupercrazy",
            "#supercomputing",
            "#supersaturates",
            "$supercomputing",
            "$supersaturates",
        ],
    );
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

    assert_contains_all(
        &results,
        &[
            "#superdupercool",
            "#superdupercrazy",
            "#supercomputing",
            "#supersaturates",
            "$supercomputing",
            "$supersaturates",
        ],
    );
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
    let dna = load_dna().await;
    let mut conductors: SweetConductorBatch = SweetConductorBatch::standard(2).await;
    let ((alice,), (bob,)) = conductors
        .setup_app("demo", &[dna])
        .await
        .unwrap()
        .into_tuples();
    conductors.exchange_peer_info().await;

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
    let dna = load_dna().await;
    let mut conductors: SweetConductorBatch = SweetConductorBatch::standard(2).await;
    let ((alice,), (bob,)) = conductors
        .setup_app("demo", &[dna])
        .await
        .unwrap()
        .into_tuples();
    conductors.exchange_peer_info().await;

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
