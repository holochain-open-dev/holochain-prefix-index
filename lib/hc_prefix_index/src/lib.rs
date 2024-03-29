//! Holochain Prefix Index
//!
//! A crate for prefix indexing of labelled hashes
//!
//! Useful for type-ahead "search" or autocomplete features.
pub mod prefix_index;
pub use crate::prefix_index::PrefixIndex;
mod utils;
mod validate;
