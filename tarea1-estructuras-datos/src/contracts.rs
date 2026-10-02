//! Shared behavior: callers can switch implementations without changing operations.

/// A last-in, first-out collection.
pub trait Stack<T>: Default {
    fn push(&mut self, value: T);
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn len(&self) -> usize;
    fn clear(&mut self);
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A first-in, first-out collection.
pub trait Queue<T>: Default {
    fn enqueue(&mut self, value: T);
    fn dequeue(&mut self) -> Option<T>;
    fn front(&self) -> Option<&T>;
    fn len(&self) -> usize;
    fn clear(&mut self);
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Unique keys, replacement on duplicate insertion, and sorted traversal.
pub trait Dictionary<K: Ord, V>: Default {
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
    fn visit<F: FnMut(&K, &V)>(&self, visitor: F);
    fn len(&self) -> usize;
    fn clear(&mut self);
    fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
