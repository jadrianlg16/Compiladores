//! The same behavioral checks run against every implementation.
use rust_structures_assignment::contracts::{Dictionary, Queue, Stack};
use rust_structures_assignment::manual_map::ManualDictionary;
use rust_structures_assignment::manual_queue::ManualQueue;
use rust_structures_assignment::manual_stack::ManualStack;
use rust_structures_assignment::std_impl::{
    LibraryDictionary, LibraryHashDictionary, LibraryQueue, LibraryStack,
};

fn stack_empty<S: Stack<i32>>() {
    let mut s = S::default();
    assert!(s.is_empty());
    assert_eq!(s.len(), 0);
    assert_eq!(s.peek(), None);
    assert_eq!(s.pop(), None);
    assert_eq!(s.len(), 0);
}
fn stack_order<S: Stack<i32>>() {
    let mut s = S::default();
    for value in [10, 20, 30] {
        s.push(value);
    }
    assert_eq!(s.peek(), Some(&30));
    assert_eq!(s.len(), 3);
    for value in [30, 20, 10] {
        assert_eq!(s.pop(), Some(value));
    }
    assert!(s.is_empty());
}
fn stack_clear<S: Stack<i32>>() {
    let mut s = S::default();
    s.push(1);
    s.push(1);
    assert_eq!(s.len(), 2);
    s.clear();
    s.clear();
    assert_eq!(s.pop(), None);
    s.push(9);
    assert_eq!(s.pop(), Some(9));
}
fn stack_owned<S: Stack<String>>() {
    let mut s = S::default();
    s.push(String::from("owned"));
    assert_eq!(s.peek().map(String::as_str), Some("owned"));
    assert_eq!(s.pop(), Some(String::from("owned")));
}
fn queue_empty<Q: Queue<i32>>() {
    let mut q = Q::default();
    assert!(q.is_empty());
    assert_eq!(q.len(), 0);
    assert_eq!(q.front(), None);
    assert_eq!(q.dequeue(), None);
}
fn queue_order<Q: Queue<i32>>() {
    let mut q = Q::default();
    for value in [10, 20, 30] {
        q.enqueue(value);
    }
    assert_eq!(q.front(), Some(&10));
    assert_eq!(q.len(), 3);
    for value in [10, 20, 30] {
        assert_eq!(q.dequeue(), Some(value));
    }
    assert!(q.is_empty());
}
fn queue_interleaved<Q: Queue<i32>>() {
    let mut q = Q::default();
    q.enqueue(1);
    q.enqueue(2);
    assert_eq!(q.dequeue(), Some(1));
    q.enqueue(3);
    assert_eq!(q.dequeue(), Some(2));
    assert_eq!(q.dequeue(), Some(3));
    q.enqueue(4);
    assert_eq!(q.dequeue(), Some(4));
}
fn queue_clear<Q: Queue<i32>>() {
    let mut q = Q::default();
    q.enqueue(1);
    q.enqueue(1);
    assert_eq!(q.len(), 2);
    q.clear();
    q.clear();
    assert!(q.is_empty());
    q.enqueue(9);
    assert_eq!(q.dequeue(), Some(9));
}
fn queue_owned<Q: Queue<String>>() {
    let mut q = Q::default();
    q.enqueue(String::from("owned"));
    assert_eq!(q.front().map(String::as_str), Some("owned"));
    assert_eq!(q.dequeue(), Some(String::from("owned")));
}
fn dictionary_empty<D: Dictionary<i32, i32>>() {
    let mut d = D::default();
    assert!(d.is_empty());
    assert_eq!(d.get(&1), None);
    assert!(!d.contains_key(&1));
    assert_eq!(d.remove(&1), None);
    let mut calls = 0;
    d.visit(|_, _| calls += 1);
    assert_eq!(calls, 0);
}
fn dictionary_update<D: Dictionary<i32, i32>>() {
    let mut d = D::default();
    assert_eq!(d.insert(2, 20), None);
    assert_eq!(d.insert(2, 99), Some(20));
    assert_eq!(d.len(), 1);
    assert_eq!(d.get(&2), Some(&99));
    assert!(d.contains_key(&2));
    assert_eq!(d.remove(&2), Some(99));
    assert_eq!(d.remove(&2), None);
    assert!(d.is_empty());
}
fn dictionary_order<D: Dictionary<i32, i32>>() {
    let mut d = D::default();
    for key in [4, 2, 6, 1, 3, 5, 7] {
        d.insert(key, key * 10);
    }
    let mut entries = Vec::new();
    d.visit(|key, value| entries.push((*key, *value)));
    assert_eq!(entries, (1..=7).map(|k| (k, k * 10)).collect::<Vec<_>>());
    assert_eq!(d.len(), 7);
}
fn dictionary_removal<D: Dictionary<i32, i32>>() {
    let mut d = D::default();
    for key in [4, 2, 6, 1, 3, 5, 7] {
        d.insert(key, key * 10);
    }
    // Leaf, one-child, and two-child deletions, including the root.
    for (index, key) in [1, 2, 4, 6, 3, 5, 7].into_iter().enumerate() {
        assert_eq!(d.remove(&key), Some(key * 10));
        assert_eq!(d.get(&key), None);
        assert_eq!(d.len(), 6 - index);
    }
    assert!(d.is_empty());
}
fn dictionary_successor<D: Dictionary<i32, i32>>() {
    let mut d = D::default();
    for key in [8, 4, 12, 10, 11, 14] {
        d.insert(key, key);
    }
    assert_eq!(d.remove(&8), Some(8));
    let mut keys = Vec::new();
    d.visit(|key, _| keys.push(*key));
    assert_eq!(keys, [4, 10, 11, 12, 14]);
    assert_eq!(d.len(), 5);
}
fn dictionary_clear<D: Dictionary<i32, i32>>() {
    let mut d = D::default();
    for key in 0..128 {
        d.insert(key, key);
    }
    d.clear();
    d.clear();
    assert_eq!(d.len(), 0);
    assert_eq!(d.get(&127), None);
    d.insert(9, 90);
    assert_eq!(d.remove(&9), Some(90));
}
fn dictionary_owned<D: Dictionary<String, String>>() {
    let mut d = D::default();
    d.insert(String::from("name"), String::from("string"));
    assert_eq!(
        d.get(&String::from("name")).map(String::as_str),
        Some("string")
    );
    assert_eq!(
        d.remove(&String::from("name")),
        Some(String::from("string"))
    );
}

// Generate named test wrappers while sharing each scenario's actual assertions.
macro_rules! dictionary_suite {
    ($d:ident) => {
        #[test]
        fn empty_dictionary() {
            dictionary_empty::<$d<i32, i32>>();
        }
        #[test]
        fn updated_dictionary() {
            dictionary_update::<$d<i32, i32>>();
        }
        #[test]
        fn sorted_dictionary() {
            dictionary_order::<$d<i32, i32>>();
        }
        #[test]
        fn deleted_dictionary() {
            dictionary_removal::<$d<i32, i32>>();
        }
        #[test]
        fn successor_dictionary() {
            dictionary_successor::<$d<i32, i32>>();
        }
        #[test]
        fn reusable_dictionary() {
            dictionary_clear::<$d<i32, i32>>();
        }
        #[test]
        fn owned_dictionary() {
            dictionary_owned::<$d<String, String>>();
        }
    };
}
macro_rules! suite {
    ($s:ident, $q:ident, $d:ident) => {
        #[test]
        fn empty_stack() {
            stack_empty::<$s<i32>>();
        }
        #[test]
        fn lifo_stack() {
            stack_order::<$s<i32>>();
        }
        #[test]
        fn reusable_stack() {
            stack_clear::<$s<i32>>();
        }
        #[test]
        fn owned_stack() {
            stack_owned::<$s<String>>();
        }
        #[test]
        fn empty_queue() {
            queue_empty::<$q<i32>>();
        }
        #[test]
        fn fifo_queue() {
            queue_order::<$q<i32>>();
        }
        #[test]
        fn mixed_queue() {
            queue_interleaved::<$q<i32>>();
        }
        #[test]
        fn reusable_queue() {
            queue_clear::<$q<i32>>();
        }
        #[test]
        fn owned_queue() {
            queue_owned::<$q<String>>();
        }
        dictionary_suite!($d);
    };
}
mod library {
    use super::*;
    suite!(LibraryStack, LibraryQueue, LibraryDictionary);
}
mod manual {
    use super::*;
    suite!(ManualStack, ManualQueue, ManualDictionary);
}
mod hash_table {
    use super::*;
    dictionary_suite!(LibraryHashDictionary);
}

#[test]
fn deterministic_differential_operations() {
    let mut a = LibraryDictionary::<i32, i32>::default();
    let mut b = ManualDictionary::<i32, i32>::default();
    let mut c = LibraryHashDictionary::<i32, i32>::default();
    let mut s1 = LibraryStack::<i32>::default();
    let mut s2 = ManualStack::<i32>::default();
    let mut q1 = LibraryQueue::<i32>::default();
    let mut q2 = ManualQueue::<i32>::default();
    let mut seed = 42_u32;
    for step in 0..1000 {
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        let key = ((seed >> 8) % 31) as i32;
        match seed % 5 {
            0 | 1 => {
                let expected = a.insert(key, step);
                assert_eq!(b.insert(key, step), expected);
                assert_eq!(c.insert(key, step), expected);
                s1.push(key);
                s2.push(key);
                q1.enqueue(key);
                q2.enqueue(key);
            }
            2 => {
                let expected = a.remove(&key);
                assert_eq!(b.remove(&key), expected);
                assert_eq!(c.remove(&key), expected);
                assert_eq!(s1.pop(), s2.pop());
                assert_eq!(q1.dequeue(), q2.dequeue());
            }
            3 => {
                assert_eq!(b.get(&key), a.get(&key));
                assert_eq!(c.get(&key), a.get(&key));
            }
            _ => {
                a.clear();
                b.clear();
                c.clear();
                s1.clear();
                s2.clear();
                q1.clear();
                q2.clear();
            }
        }
        assert_eq!(s1.peek(), s2.peek());
        assert_eq!(q1.front(), q2.front());
        assert_eq!((a.len(), s1.len(), q1.len()), (b.len(), s2.len(), q2.len()));
        assert_eq!(c.len(), a.len());
        let mut x = Vec::new();
        let mut y = Vec::new();
        let mut z = Vec::new();
        a.visit(|k, v| x.push((*k, *v)));
        b.visit(|k, v| y.push((*k, *v)));
        c.visit(|k, v| z.push((*k, *v)));
        assert_eq!(x, y);
        assert_eq!(x, z);
    }
}

#[test]
fn manual_values_drop_exactly_once() {
    use std::cell::Cell;
    use std::rc::Rc;
    struct Tracked(Rc<Cell<usize>>);
    impl Drop for Tracked {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }
    let count = Rc::new(Cell::new(0));
    {
        let mut s = ManualStack::default();
        let mut q = ManualQueue::default();
        let mut d = ManualDictionary::default();
        for key in 0..20 {
            s.push(Tracked(Rc::clone(&count)));
            q.enqueue(Tracked(Rc::clone(&count)));
            d.insert(key, Tracked(Rc::clone(&count)));
        }
        drop(s.pop());
        drop(q.dequeue());
        drop(d.remove(&10));
        drop(d.insert(11, Tracked(Rc::clone(&count))));
        assert_eq!(count.get(), 4);
    }
    assert_eq!(count.get(), 61);
}
