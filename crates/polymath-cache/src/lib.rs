#![forbid(unsafe_code)]
#![deny(
    unused_imports,
    unused_mut,
    missing_docs,
    missing_debug_implementations
)]
//! Multi-cache support:
//! * Least Recently Used (LRU) cache;
//! * Time-based cache.
//!
//! Time-based cache can use `Last-Modified` and `ETag` headers.
//!
//! LRU should be used to cache recently seen URLs or robots.txt while
//! headers-based should be used to know if a page needs re-indexation.

pub mod lru;
