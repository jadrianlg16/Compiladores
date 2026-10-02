//! An unbalanced binary search tree: left keys < key < right keys.
use crate::contracts::Dictionary;
use std::cmp::Ordering;

type Link<K, V> = Option<Box<Node<K, V>>>;
struct Node<K, V> {
    key: K,
    value: V,
    left: Link<K, V>,
    right: Link<K, V>,
}
pub struct ManualDictionary<K, V> {
    root: Link<K, V>,
    size: usize,
}
impl<K, V> Default for ManualDictionary<K, V> {
    fn default() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }
}

// Detach the smallest node; its right child takes its former position.
fn take_min<K, V>(link: &mut Link<K, V>) -> Box<Node<K, V>> {
    if link.as_ref().expect("nonempty subtree").left.is_some() {
        return take_min(&mut link.as_mut().unwrap().left);
    }
    let mut node = link.take().expect("nonempty subtree");
    *link = node.right.take();
    node
}

fn remove_node<K: Ord, V>(link: &mut Link<K, V>, key: &K) -> Option<V> {
    match key.cmp(&link.as_ref()?.key) {
        Ordering::Less => remove_node(&mut link.as_mut()?.left, key),
        Ordering::Greater => remove_node(&mut link.as_mut()?.right, key),
        Ordering::Equal => {
            let mut node = link.take()?;
            match (node.left.take(), node.right.take()) {
                (None, right) => {
                    *link = right;
                    Some(node.value)
                }
                (left, None) => {
                    *link = left;
                    Some(node.value)
                }
                (left, right) => {
                    let mut subtree = right;
                    let successor = take_min(&mut subtree);
                    node.key = successor.key;
                    let old = std::mem::replace(&mut node.value, successor.value);
                    node.left = left;
                    node.right = subtree;
                    *link = Some(node);
                    Some(old)
                }
            }
        }
    }
}

fn visit_node<K, V, F: FnMut(&K, &V)>(link: &Link<K, V>, visitor: &mut F) {
    if let Some(node) = link {
        visit_node(&node.left, visitor);
        visitor(&node.key, &node.value);
        visit_node(&node.right, visitor);
    }
}

impl<K: Ord, V> Dictionary<K, V> for ManualDictionary<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut link = &mut self.root;
        loop {
            match link {
                None => {
                    *link = Some(Box::new(Node {
                        key,
                        value,
                        left: None,
                        right: None,
                    }));
                    self.size += 1;
                    return None;
                }
                Some(node) => match key.cmp(&node.key) {
                    Ordering::Less => link = &mut node.left,
                    Ordering::Greater => link = &mut node.right,
                    Ordering::Equal => {
                        return Some(std::mem::replace(&mut node.value, value));
                    }
                },
            }
        }
    }
    fn get(&self, key: &K) -> Option<&V> {
        let mut link = self.root.as_deref();
        while let Some(node) = link {
            match key.cmp(&node.key) {
                Ordering::Less => link = node.left.as_deref(),
                Ordering::Greater => link = node.right.as_deref(),
                Ordering::Equal => return Some(&node.value),
            }
        }
        None
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        let removed = remove_node(&mut self.root, key);
        if removed.is_some() {
            self.size -= 1;
        }
        removed
    }
    fn visit<F: FnMut(&K, &V)>(&self, mut visitor: F) {
        visit_node(&self.root, &mut visitor);
    }
    fn len(&self) -> usize {
        self.size
    }
    fn clear(&mut self) {
        self.clear_nodes();
    }
}

impl<K, V> ManualDictionary<K, V> {
    // Rotate left children upward, then drop child-free nodes iteratively.
    fn clear_nodes(&mut self) {
        while let Some(mut node) = self.root.take() {
            if let Some(mut left) = node.left.take() {
                node.left = left.right.take();
                left.right = Some(node);
                self.root = Some(left);
            } else {
                self.root = node.right.take();
            }
        }
        self.size = 0;
    }
}
impl<K, V> Drop for ManualDictionary<K, V> {
    fn drop(&mut self) {
        self.clear_nodes();
    }
}
