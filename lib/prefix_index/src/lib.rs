//! Holochain Prefix Index
//!
//! A crate for prefix indexing of labelled hashes
//!
//! Useful for type-ahead "search" or autocomplete features.
use hdk::prelude::*;
pub mod prefix_index_to_hashes;
pub use prefix_index_to_hashes::*;
pub mod validate;
pub use validate::*;
