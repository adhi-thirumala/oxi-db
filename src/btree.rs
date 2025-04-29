use serde::{Deserialize, Serialize};
use std::collections::BTreeMap as StdBTreeMap;
use std::fmt::Debug;

/// A wrapper around the standard library's BTreeMap for the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BTree<K, V>
where
    K: Ord + Clone + Debug,
    V: Clone + Debug,
{
    data: StdBTreeMap<K, V>,
}

impl<K, V> BTree<K, V>
where
    K: Ord + Clone + Debug,
    V: Clone + Debug,
{
    /// Create a new empty B-tree
    pub fn new() -> Self {
        Self {
            data: StdBTreeMap::new(),
        }
    }

    /// Insert a key-value pair into the B-tree
    pub fn insert(&mut self, key: K, value: V) {
        self.data.insert(key, value);
    }

    /// Search for a key in the B-tree
    pub fn search(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    /// Get a mutable reference to a value
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.data.get_mut(key)
    }

    /// Remove a key from the B-tree
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }

    /// Traverse the B-tree in-order and apply a function to each key-value pair
    pub fn traverse<F>(&self, mut f: F)
    where
        F: FnMut(&K, &V),
    {
        for (k, v) in &self.data {
            f(k, v);
        }
    }

    /// Get all key-value pairs as a vector
    pub fn to_vec(&self) -> Vec<(K, V)> {
        self.data.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }

    /// Check if the B-tree is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get the number of key-value pairs in the B-tree
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Clear the B-tree
    pub fn clear(&mut self) {
        self.data.clear();
    }
}
