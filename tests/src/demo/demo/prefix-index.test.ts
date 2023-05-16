import { pause, runScenario } from "@holochain/tryorama";
import { assert, test } from "vitest";

test("search prefix index with width=3, depth=3", async () => {
  await runScenario(
    async (scenario) => {
      // Set up the app to be installed
      const appSource = { appBundleSource: { path: "../workdir/prefix-index.happ"}};

      // Add 2 players with the test app to the Scenario. The returned players
      // can be destructured.
      const [alice] = await scenario.addPlayersWithApps([appSource]);

      // Shortcut peer discovery through gossip and register all agents in every
      // conductor of the scenario.
      await scenario.shareAllAgents();

      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_to_index_a",
        payload: "superdupercool",
      });
      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_to_index_a",
        payload: "superdupercrazy",
      });
      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_to_index_a",
        payload: "supercomputing",
      });
      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_to_index_a",
        payload: "supersaturates",
      });

      await pause(1000);


      let results: string[] = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_a",
        payload: {
          query: "sup",
          limit: 4,
        }
      });
      
      assert.sameMembers(results, [
        'superdupercool',
        'superdupercrazy',
        'supercomputing',
        'supersaturates',
      ]);

      results = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_a",
        payload: {
          query: "sup",
          limit: 1,
        }
      });
      
      assert.sameMembers(results, [
        'supercomputing',
      ]);

      results = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_a",
        payload: {
          query: "super",
          limit: 5,
        }
      });
      
      assert.sameMembers(results, [
        'superdupercool',
        'superdupercrazy',
        'supercomputing',
        'supersaturates',
      ]);
      assert.equal(results[0], 'supercomputing');

      results = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_a",
        payload: {
          query: "superdupe",
          limit: 5
        }
      });
      
      assert.sameMembers(results, [
        'superdupercool',
        'superdupercrazy',
        'supercomputing',
        'supersaturates',
      ]);
      assert.equal(results[0], 'superdupercool');

      results = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_a",
        payload: {
          query: "superdupercool",
          limit: 5
        }
      });
      
      assert.sameMembers(results, [
        'superdupercool',
        'superdupercrazy',
        'supercomputing',
        'supersaturates',
      ]);
      assert.equal(results[0], 'superdupercool');


      results = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_a",
        payload: {
          query: "superduperbad",
          limit: 5
        }
      });
      
      assert.sameMembers(results, [
        'superdupercool',
        'superdupercrazy',
        'supercomputing',
        'supersaturates',
      ]);
      assert.equal(results[0], 'superdupercool');

      results = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_a",
        payload: {
          query: "supersaturday",
          limit: 5
        }
      });
      assert.sameMembers(results, [
        'superdupercool',
        'superdupercrazy',
        'supercomputing',
        'supersaturates',
      ]);
      assert.equal(results[0], 'supersaturates');

      results = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_a",
        payload: {
          query: "cow",
          limit: 5
        }
      });
      assert.lengthOf(results, 0);
    },
    true,
    { timeout: 100000 }
  );
});

test("search prefix index with width=3, depth=5", async () => {
  await runScenario(
    async (scenario) => {
      // Set up the app to be installed
      const appSource = { appBundleSource: { path: "../workdir/prefix-index.happ"}};

      // Add 2 players with the test app to the Scenario. The returned players
      // can be destructured.
      const [alice] = await scenario.addPlayersWithApps([appSource]);

      // Shortcut peer discovery through gossip and register all agents in every
      // conductor of the scenario.
      await scenario.shareAllAgents();

      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_to_index_b",
        payload: "superdupercool",
      });
      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_to_index_b",
        payload: "superdupercrazy",
      });
      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_to_index_b",
        payload: "supercomputing",
      });
      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_to_index_b",
        payload: "supersaturates",
      });

      await pause(1000);

      let results: string[] = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_b",
        payload: {
          query: "sup",
          limit: 5,
        }
      });
      
      assert.sameMembers(results, [
        'superdupercool',
        'superdupercrazy',
        'supercomputing',
        'supersaturates',
      ]);

      results = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_b",
        payload: {
          query: "sup",
          limit: 1,
        }
      });
      
      assert.sameMembers(results, [
        'supercomputing',
      ]);

      results = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_b",
        payload: {
          query: "super",
          limit: 5,
        }
      });
      
      assert.sameMembers(results, [
        'superdupercool',
        'superdupercrazy',
        'supercomputing',
        'supersaturates',
      ]);
      assert.equal(results[0], 'supercomputing');

      results = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_b",
        payload: {
          query: "superdupe",
          limit: 5
        }
      });
      
      assert.sameMembers(results, [
        'superdupercool',
        'superdupercrazy',
        'supercomputing',
        'supersaturates',
      ]);
      assert.equal(results[0], 'superdupercool');

      results = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_b",
        payload: {
          query: "superdupercool",
          limit: 5
        }
      });
      
      assert.sameMembers(results, [
        'superdupercool',
        'superdupercrazy',
        'supercomputing',
        'supersaturates',
      ]);
      assert.equal(results[0], 'superdupercool');


      results = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_b",
        payload: {
          query: "superduperbad",
          limit: 5
        }
      });
      
      assert.sameMembers(results, [
        'superdupercool',
        'superdupercrazy',
        'supercomputing',
        'supersaturates',
      ]);
      assert.equal(results[0], 'superdupercool');

      results = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_b",
        payload: {
          query: "supersaturday",
          limit: 5
        }
      });
      
      assert.sameMembers(results, [
        'superdupercool',
        'superdupercrazy',
        'supercomputing',
        'supersaturates',
      ]);
      assert.equal(results[0], 'supersaturates');
    },
    true,
    { timeout: 100000 }
  );
});

test("search prefix index with width=4, depth=2", async () => {
  await runScenario(
    async (scenario) => {
      // Set up the app to be installed
      const appSource = { appBundleSource: { path: "../workdir/prefix-index.happ"}};

      // Add 2 players with the test app to the Scenario. The returned players
      // can be destructured.
      const [alice] = await scenario.addPlayersWithApps([appSource]);

      // Shortcut peer discovery through gossip and register all agents in every
      // conductor of the scenario.
      await scenario.shareAllAgents();

      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_to_index_c",
        payload: "superdupercool",
      });
      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_to_index_c",
        payload: "superdupercrazy",
      });
      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_to_index_c",
        payload: "supercomputing",
      });
      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_to_index_c",
        payload: "supersaturates",
      });

      await pause(1000);

      let results: string[] = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_c",
        payload: {
          query: "sup",
          limit: 5,
        }
      });
      assert.lengthOf(results, 0);

      results = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_c",
        payload: {
          query: "supe",
          limit: 5,
        }
      });
      assert.sameMembers(results, [
        'superdupercool',
        'superdupercrazy',
        'supercomputing',
        'supersaturates',
      ]);

      results = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_c",
        payload: {
          query: "sup",
          limit: 1,
        }
      });
      
      assert.lengthOf(results, 0);

      results = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_c",
        payload: {
          query: "super",
          limit: 5,
        }
      });
      
      assert.sameMembers(results, [
        'superdupercool',
        'superdupercrazy',
        'supercomputing',
        'supersaturates',
      ]);
      assert.equal(results[0], 'supercomputing');

      results = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_c",
        payload: {
          query: "superdupe",
          limit: 5
        }
      });
      
      assert.sameMembers(results, [
        'superdupercool',
        'superdupercrazy',
        'supercomputing',
        'supersaturates',
      ]);
      assert.equal(results[0], 'superdupercool');

      results = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_c",
        payload: {
          query: "superdupercool",
          limit: 5
        }
      });
      
      assert.sameMembers(results, [
        'superdupercool',
        'superdupercrazy',
        'supercomputing',
        'supersaturates',
      ]);
      assert.equal(results[0], 'superdupercool');


      results = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_c",
        payload: {
          query: "superduperbad",
          limit: 5
        }
      });
      
      assert.sameMembers(results, [
        'superdupercool',
        'superdupercrazy',
        'supercomputing',
        'supersaturates',
      ]);
      assert.equal(results[0], 'superdupercool');

      results = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_c",
        payload: {
          query: "supersaturday",
          limit: 5
        }
      });
      
      assert.sameMembers(results, [
        'superdupercool',
        'superdupercrazy',
        'supercomputing',
        'supersaturates',
      ]);
      assert.equal(results[0], 'supersaturates');
    },
    true,
    { timeout: 100000 }
  );
});

test("remove result from index", async () => {
  await runScenario(
    async (scenario) => {
      // Set up the app to be installed
      const appSource = { appBundleSource: { path: "../workdir/prefix-index.happ"}};

      // Add 2 players with the test app to the Scenario. The returned players
      // can be destructured.
      const [alice] = await scenario.addPlayersWithApps([appSource]);

      // Shortcut peer discovery through gossip and register all agents in every
      // conductor of the scenario.
      await scenario.shareAllAgents();

      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_to_index_a",
        payload: "superduper",
      });

      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_to_index_a",
        payload: "superdupercrazy",
      });

      await pause(1000);

      let results: string[] = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_a",
        payload: {
          query: "sup",
          limit: 4,
        }
      });
      
      assert.sameMembers(results, ['superduper', 'superdupercrazy']);

      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "remove_from_index_a",
        payload: "superduper"
      });


      await pause(1000);

      results = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_a",
        payload: {
          query: "sup",
          limit: 4,
        }
      });
      assert.sameMembers(results, ['superdupercrazy']);
    },
    true,
    { timeout: 100000 }
  );
});

test("add results with labels", async () => {
  await runScenario(
    async (scenario) => {
      // Set up the app to be installed
      const appSource = { appBundleSource: { path: "../workdir/prefix-index.happ"}};

      // Add 2 players with the test app to the Scenario. The returned players
      // can be destructured.
      const [alice] = await scenario.addPlayersWithApps([appSource]);

      // Shortcut peer discovery through gossip and register all agents in every
      // conductor of the scenario.
      await scenario.shareAllAgents();

      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_hashtag_to_index_a",
        payload: "#superdupercool",
      });
      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_hashtag_to_index_a",
        payload: "#superdupercrazy",
      });
      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_hashtag_to_index_a",
        payload: "#supercomputing",
      });
      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_hashtag_to_index_a",
        payload: "#supersaturates",
      });
      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_cashtag_to_index_a",
        payload: "$supercomputing",
      });
      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_cashtag_to_index_a",
        payload: "$supersaturates",
      });


      await pause(1000);


      let results: string[] = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_a",
        payload: {
          query: "sup",
          limit: 10,
        }
      });
      
      assert.sameMembers(results, [
        '#superdupercool',
        '#superdupercrazy',
        '#supercomputing',
        '#supersaturates',
        '$supercomputing',
        '$supersaturates',
      ]);

      results = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_a",
        payload: {
          query: "sup",
          limit: 1,
        }
      });
      
      assert.sameMembers(results, [
        '#supercomputing',
      ]);

      results = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_a",
        payload: {
          query: "super",
          limit: 10,
        }
      });
      
      assert.sameMembers(results, [
        '#superdupercool',
        '#superdupercrazy',
        '#supercomputing',
        '#supersaturates',
        '$supercomputing',
        '$supersaturates',
      ]);
      assert.equal(results[0], '#supercomputing');

      results = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_a",
        payload: {
          query: "superdupe",
          limit: 10
        }
      });
      
      assert.sameMembers(results, [
        '#superdupercool',
        '#superdupercrazy',
        '#supercomputing',
        '#supersaturates',
        '$supercomputing',
        '$supersaturates',
      ]);
      assert.equal(results[0], '#superdupercool');

      results = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_a",
        payload: {
          query: "superdupercool",
          limit: 10
        }
      });
      
      assert.sameMembers(results, [
        '#superdupercool',
        '#superdupercrazy',
        '#supercomputing',
        '#supersaturates',
        '$supercomputing',
        '$supersaturates',
      ]);
      assert.equal(results[0], '#superdupercool');


      results = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_a",
        payload: {
          query: "superduperbad",
          limit: 10
        }
      });
      
      assert.sameMembers(results, [
        '#superdupercool',
        '#superdupercrazy',
        '#supercomputing',
        '#supersaturates',
        '$supercomputing',
        '$supersaturates',
      ]);
      assert.equal(results[0], '#superdupercool');

      results = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_a",
        payload: {
          query: "supersaturday",
          limit: 10
        }
      });
      assert.sameMembers(results, [
        '#superdupercool',
        '#superdupercrazy',
        '#supercomputing',
        '#supersaturates',
        '$supercomputing',
        '$supersaturates',
      ]);
      assert.equal(results[0], '#supersaturates');

      results = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_a",
        payload: {
          query: "cow",
          limit: 5
        }
      });
      assert.lengthOf(results, 0);
    },
    true,
    { timeout: 100000 }
  );
});

test("preserve letter case in result, but ignore letter case in indexing", async () => {
  await runScenario(
    async (scenario) => {
      // Set up the app to be installed
      const appSource = { appBundleSource: { path: "../workdir/prefix-index.happ"}};

      // Add 2 players with the test app to the Scenario. The returned players
      // can be destructured.
      const [alice] = await scenario.addPlayersWithApps([appSource]);

      // Shortcut peer discovery through gossip and register all agents in every
      // conductor of the scenario.
      await scenario.shareAllAgents();

      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_hashtag_to_index_a",
        payload: "#HOLOCHAIN",
      });
      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_hashtag_to_index_a",
        payload: "#holosapian",
      });
      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_cashtag_to_index_a",
        payload: "$HOLY",
      });

      await pause(1000);

      let results: string[] = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "search_index_a",
        payload: {
          query: "holo",
          limit: 5,
        }
      });

      assert.sameMembers(results, [
        '#HOLOCHAIN',
        '#holosapian',
        '$HOLY'
      ]);
    }
  )
});

test("get_random_results returns random results from prefix index", async () => {
  await runScenario(
    async (scenario) => {
      // Set up the app to be installed
      const appSource = { appBundleSource: { path: "../workdir/prefix-index.happ"}};

      // Add 2 players with the test app to the Scenario. The returned players
      // can be destructured.
      const [alice] = await scenario.addPlayersWithApps([appSource]);

      // Shortcut peer discovery through gossip and register all agents in every
      // conductor of the scenario.
      await scenario.shareAllAgents();

      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_hashtag_to_index_a",
        payload: "#HOLOCHAIN",
      });
      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_hashtag_to_index_a",
        payload: "#holosapian",
      });
      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_cashtag_to_index_a",
        payload: "$HOLY",
      });
      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_cashtag_to_index_a",
        payload: "$CAT",
      });
      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_cashtag_to_index_a",
        payload: "$DOGGO",
      });
      await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "add_hashtag_to_index_a",
        payload: "#monkeys",
      });

      await pause(1000);

      const [ result1 ]: string[] = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "get_random_results_index_a",
        payload:  1
      });

      const [ result2 ]: string[] = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "get_random_results_index_a",
        payload:  1
      });

      const [ result3 ]: string[] = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "get_random_results_index_a",
        payload:  1
      });

      const [ result4 ]: string[] = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "get_random_results_index_a",
        payload:  1
      });

      const [ result5 ]: string[] = await alice.cells[0].callZome({
        zome_name: "demo",
        fn_name: "get_random_results_index_a",
        payload:  1
      });

      // Assert we did not get the exact same result 5 times
      assert.ok(new Set([result1, result2, result3, result4, result5]).size > 1);
    }
  )
});