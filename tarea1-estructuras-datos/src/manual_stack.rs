//! A manually linked stack. Each node owns the node below it.
use crate::contracts::Stack;

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}
pub struct ManualStack<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}
impl<T> Default for ManualStack<T> {
    fn default() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }
}
impl<T> Stack<T> for ManualStack<T> {
    fn push(&mut self, value: T) {
        let next = self.head.take();
        self.head = Some(Box::new(Node { value, next }));
        self.size += 1;
    }
    fn pop(&mut self) -> Option<T> {
        let node = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }
    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }
    fn len(&self) -> usize {
        self.size
    }
    fn clear(&mut self) {
        while self.pop().is_some() {}
    }
}
impl<T> Drop for ManualStack<T> {
    fn drop(&mut self) {
        self.clear();
    }
}
