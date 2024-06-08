//! LRU cache implementation.

use std::collections::{HashMap, VecDeque};

struct LRUCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    order: VecDeque<K>,
}

impl<K: Eq + std::hash::Hash + Clone, V> LRUCache<K, V> {
    /// Create an [LRUCache] with a specific capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        LRUCache {
            capacity,
            map: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    /// Length of the map containing keys.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns the value of a key.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            // Move the accessed key to the back of the order queue.
            self.order.retain(|k| k != key);
            self.order.push_back(key.clone());
            self.map.get(key)
        } else {
            None
        }
    }

    /// Put en entry on the cache.
    ///
    /// If the capacity is exceeded, delete the last element get.
    pub fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            // Update the existing value and move the key to the back of the order queue.
            self.map.insert(key.clone(), value);
            self.order.retain(|k| k != &key);
            self.order.push_back(key);
        } else {
            // Check if the cache is at capacity.
            if self.len() == self.capacity {
                // Remove the least recently used item from the cache.
                if let Some(lru_key) = self.order.pop_front() {
                    self.map.remove(&lru_key);
                }
            }
            // Insert the new item and update the order queue.
            self.map.insert(key.clone(), value);
            self.order.push_back(key);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eviction() {
        let mut cache = LRUCache::with_capacity(2);

        cache.put("gravitalia.com/robots.txt", "User-agent: *\nAllow: /");
        assert_eq!(
            cache.get(&"gravitalia.com/robots.txt"),
            Some(&"User-agent: *\nAllow: /")
        );

        cache.put(
            "finance.gravitalia.com/robots.txt",
            "User-agent: *\nAllow: /",
        );
        assert_eq!(cache.len(), 2);
        cache.put(
            "news.gravitalia.com/robots.txt",
            "User-agent: *\nDisallow: /",
        );
        assert_eq!(cache.len(), 2);

        assert_eq!(cache.get(&"gravitalia.com/robots.txt"), None);
    }
}
