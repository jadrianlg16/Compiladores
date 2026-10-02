//! A simple linked FIFO queue: enqueue is O(n), dequeue is O(1).
use crate::contracts::Queue;

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}
pub struct ManualQueue<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}
impl<T> Default for ManualQueue<T> {
    fn default() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }
}
impl<T> Queue<T> for ManualQueue<T> {
    fn enqueue(&mut self, value: T) {
        // Walk through mutable links until reaching the empty tail link.
        let mut link = &mut self.head;
        while let Some(node) = link {
            link = &mut node.next;
        }
        *link = Some(Box::new(Node { value, next: None }));
        self.size += 1;
    }
    fn dequeue(&mut self) -> Option<T> {
        let node = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }
    fn front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }
    fn len(&self) -> usize {
        self.size
    }
    fn clear(&mut self) {
        while self.dequeue().is_some() {}
    }
}
impl<T> Drop for ManualQueue<T> {
    fn drop(&mut self) {
        self.clear();
    }
}
