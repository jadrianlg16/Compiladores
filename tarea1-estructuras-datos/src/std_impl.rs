//! Adapters around Rust's standard collections; no external crates are needed.
use crate::contracts::{Dictionary, Queue, Stack};
use std::collections::{BTreeMap, HashMap, VecDeque};

pub struct LibraryStack<T> {
    items: Vec<T>,
}
impl<T> Default for LibraryStack<T> {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}
impl<T> Stack<T> for LibraryStack<T> {
    fn push(&mut self, value: T) {
        self.items.push(value);
    }
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    fn len(&self) -> usize {
        self.items.len()
    }
    fn clear(&mut self) {
        self.items.clear();
    }
}

pub struct LibraryQueue<T> {
    items: VecDeque<T>,
}
impl<T> Default for LibraryQueue<T> {
    fn default() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }
}
impl<T> Queue<T> for LibraryQueue<T> {
    fn enqueue(&mut self, value: T) {
        self.items.push_back(value);
    }
    fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }
    fn front(&self) -> Option<&T> {
        self.items.front()
    }
    fn len(&self) -> usize {
        self.items.len()
    }
    fn clear(&mut self) {
        self.items.clear();
    }
}

pub struct LibraryDictionary<K, V> {
    items: BTreeMap<K, V>,
}
impl<K, V> Default for LibraryDictionary<K, V> {
    fn default() -> Self {
        Self {
            items: BTreeMap::new(),
        }
    }
}
impl<K: Ord, V> Dictionary<K, V> for LibraryDictionary<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.items.insert(key, value)
    }
    fn get(&self, key: &K) -> Option<&V> {
        self.items.get(key)
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        self.items.remove(key)
    }
    fn visit<F: FnMut(&K, &V)>(&self, mut visitor: F) {
        for (key, value) in &self.items {
            visitor(key, value);
        }
    }
    fn len(&self) -> usize {
        self.items.len()
    }
    fn clear(&mut self) {
        self.items.clear();
    }
}

/// A hash table: average O(1) insert/get/remove. HashMap iterates in no fixed
/// order, so `visit` sorts the entries by key to honor the shared contract.
pub struct LibraryHashDictionary<K, V> {
    items: HashMap<K, V>,
}
impl<K, V> Default for LibraryHashDictionary<K, V> {
    fn default() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
}
impl<K: Ord + std::hash::Hash, V> Dictionary<K, V> for LibraryHashDictionary<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.items.insert(key, value)
    }
    fn get(&self, key: &K) -> Option<&V> {
        self.items.get(key)
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        self.items.remove(key)
    }
    fn visit<F: FnMut(&K, &V)>(&self, mut visitor: F) {
        let mut entries: Vec<(&K, &V)> = self.items.iter().collect();
        entries.sort_unstable_by_key(|&(key, _)| key);
        for (key, value) in entries {
            visitor(key, value);
        }
    }
    fn len(&self) -> usize {
        self.items.len()
    }
    fn clear(&mut self) {
        self.items.clear();
    }
}
