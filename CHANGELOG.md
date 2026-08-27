# Changelog

All notable changes to this project will be documented in this file.

## Unreleased

## [0.15.0] - 2026-08-26

### Build

- Bump to holochain 0.7 (hdi 0.8 / hdk 0.7, holonix main-0.7)

### Changed

- **Breaking:** `PrefixIndex::validate_create_link` / `validate_delete_link` now take the 0.7 `TypedAction<CreateLinkData>` / `TypedAction<DeleteLinkData>` produced by `FlatOp::Link(OpLink::…)` match arms
- Link tags that fail to decode as a path component are now rejected as `Invalid` instead of erroring the validate callback
- Replaced the second-component width check with a target/tag coherence check: a link under the index root must target the path its tag names. The old check rejected results shorter than the index width, which shard to zero components and so hang directly off the root — they could never be indexed
- **Breaking:** `remove_result` / `remove_result_with_label` return the number of links deleted, and delete only the calling agent's own links. Deleting another agent's link fails validation and rolls the whole call back, so a result added by two agents was previously unremovable by either. Because path creation is deduplicated across authors, the second agent to add a result authors no links for it — a count of `0` tells that caller the result is still indexed by someone else rather than silently reporting success
- **Breaking:** `PrefixIndex::validate_delete_link` is an associated function taking no `self` — the rule reads no index configuration, so callers no longer build a `PrefixIndex` (and pay its `zome_info()` host call) to validate a delete
- `GetStrategy` now reaches `Path::exists()` / `ensure()` as well as `get_links`, including for child paths walked during a search, so `GetStrategy::Local` no longer makes network calls
- happ manifest moved to the strict 0.7 schema; demo sweettest suite migrated (`SweetConductorBatch::standard`, `await_consistency_s`, per-test network seeds) and gained negative tests for both validation rules

### Notes

- An index name containing `.` has never worked: names are parsed through the path DSL, so a dot splits them into multiple components. Use dot-free index names.

## [0.14.0] - 2026-01-21

### Build

- Bump to holochain 0.6

### Changed

- **Breaking:** `get_results`, `get_random_results`, `remove_result`, and `remove_result_with_label` take a new required `GetStrategy` argument controlling whether index reads go to the network or only to local storage

## [0.13.0] - 2025-02-25

## Build

- Bump holochain to 0.4.1

## [0.12.0] - 2024-08-27

## Build

- Bump holochain to 0.3.2

## [0.11.0] - 2024-07-27

## Build

- Bump holochain to 0.3.1

## [0.10.0] - 2024-02-07

### Documentation

- Add license and description
- Fix readme crate name

### Build

- Bump holochain to 0.2.6

## [0.9.0] - 2023-12-16

### Documentation

- Changelog
- Update readme

### Miscellaneous Tasks

- Move package dir to match name

### Refactor

- Rename package 'hc_prefix_index'

### Build

- Bump package version

## [0.8.0] - 2023-12-16

### Documentation

- Add changelog
- Changelog

### Miscellaneous Tasks

- Fmt + clippy

### Refactor

- Migrate tests from tryoama to sweettest

### Build

- Holochain-0.2.3-beta-rc.1
- Upgrade holochain to 0.2.4
- Bump version to 0.6.0
- Package lock

## [0.7.0] - 2023-09-27

### Build

- Bump deps to hdk 0.2.2 + hdi 0.3.2
- Bump version to 0.6.0
- Actually it should be 0.7.0

## [0.6.0] - 2023-07-26

### Miscellaneous Tasks

- Bump flake lock
- Better caching in github action
- Clippy + fmt

### Refactor

- [**breaking**] Entry hashes from linkable
- Unstable is_ok_and

### Build

- Bump holochain 0.2.1-beta-dev.0 & update for compatibility
- [**breaking**] Upgrade to holochain v0.3.0-beta-dev.8
- Upgrade to holochain 0.3.0-beta-dev.9
- Upgrade client & tryorama
- Bump holochain deps to 0.2.1
- Update nix flake for 0.2.1, fix github action commands

## [0.5.0] - 2023-05-16

### Documentation

- Note letter casing in README
- Title gotchas

### Features

- Add function to get random results from index

### Build

- Bump version to 0.5.0

## [0.4.0] - 2023-05-11

### Features

- Ignore letter case in index, preserve letter case in result

### Build

- Bump version to 0.4.0

## [0.3.0] - 2023-05-04

### Features

- Support for indexing text with a distinct leaf text

### Miscellaneous Tasks

- File rename for clarity

## [0.2.0] - 2023-05-03

### Features

- Return TypedPath to result in add_result()

### Miscellaneous Tasks

- Remove vitest results file from git repo
- Remove warn logs

### Refactor

- Make validation functions callable via PrefixIndex trait

## [0.1.0] - 2023-05-02

### Documentation

- Basic explanation
- Readme code fix
- Add future desired features to README

### Features

- Search prefix index working w/ tests

### Miscellaneous Tasks

- Rename PrefixSearchIndex to PrefixIndex
- Readme markdown
- License under CAL
- Split out utils into their own file

### Testing

- Tests for prefixes width different width + depths

<!-- generated by git-cliff -->
