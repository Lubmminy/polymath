//! LRU cache implementation.

use std::collections::{HashMap};

pub struct Lru<T> {
    capacity: usize,
    cache: HashMap<i32, T>,
}

impl<T> Lru<T> {
    /// Create an [Lru] with a specific capacity.
    pub fn with_capacity(capacity: usize) -> Lru<T> {
        Lru {
            capacity,
            cache: HashMap::new(),
        }
    }
}
