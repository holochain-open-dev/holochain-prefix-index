# Prefix Index

A library crate to implement a prefix index in your holochain zomes.

A prefix index is useful for typeahead "search" functionality.

## Usage

1. Include the prefix_index crate in your coordinator zome's Cargo.toml:

```toml
[dependencies]
...
prefix_index = { git = "https://github.com/mattyg/holochain-prefix-index" module = "prefix_index" }
```

2. Include the prefix_index crate in your integrity zome's Cargo.toml and create a link type for the index:
```rust
#[hdk_link_types]
enum LinkTypes {
    PrefixIndex
}
```

3. Add the prefix index validation functions to your validate match arm for CreateLink and DeleteLink on LinkTypes::PrefixIndex

3. Setup a prefix index

```rust
use prefix_index::PrefixIndex;

let index = PrefixIndex::new("demo_index".into(), LinkTypes::PrefixIndex, 3, 3)?;
```

4. Add some results to the index:
```rust
index.add_result("superdupercool");
index.add_result("superdupercrazy");
index.add_result("supercomputing");
index.add_result("supersaturates");
```

5. This will generate an index of links for the strings as follows:

```mermaid
flowchart LR
    A[demo_index] --> B[sup]
    B --> C[erd] --> D[upe] --> E[superdupercool]
    D --> F[superdupercrazy]
    B --> G[erc] --> H[omp] --> I[supercomputing]
    B --> J[ers] --> K[atu] --> L[supersaturates]
```

6. Now you can search the index.
```rust
index.get_results("sup", 10);
// [
//   "supercomputing",
//   "superdupercool",
//   "superdupercrazy",
//   "supersaturates",
// ]

let res2 = index.get_results("superduper", 10);
// [
//   "superdupercool",
//   "superdupercrazy",
//   "supercomputing",
//   "supersaturates",
// ]

let res2 = index.get_results("superduper", 2);
// [
//   "superdupercool",
//   "superdupercrazy",
// ]

let res2 = index.get_results("walrus", 10);
// [ ]
```

*Note that all strings beneath the top-level prefix of your search query will be returned (ordered by similarity, then alphabetically). Make sure to specify a "limit" to reduce the number of returned results.*

## Todo
- [ ] Custom Elements for typeahead search using prefix index 

## Running the tests

```bash
nix-shell
npm install
npm test
```
