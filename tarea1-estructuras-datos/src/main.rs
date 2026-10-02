//! Run identical examples with every implementation.
use rust_structures_assignment::contracts::{Dictionary, Queue, Stack};
use rust_structures_assignment::manual_map::ManualDictionary;
use rust_structures_assignment::manual_queue::ManualQueue;
use rust_structures_assignment::manual_stack::ManualStack;
use rust_structures_assignment::std_impl::{
    LibraryDictionary, LibraryHashDictionary, LibraryQueue, LibraryStack,
};

fn demonstrate<S, Q, D>(label: &str)
where
    S: Stack<i32>,
    Q: Queue<i32>,
    D: Dictionary<&'static str, &'static str>,
{
    println!("\n=== {label} ===");
    let mut stack = S::default();
    let mut queue = Q::default();
    for value in [10, 20, 30] {
        stack.push(value);
        queue.enqueue(value);
    }
    println!("Stack top: {:?}; size: {}", stack.peek(), stack.len());
    print!("Stack removal order:");
    while let Some(value) = stack.pop() {
        print!(" {value}");
    }
    println!();
    println!("Queue front: {:?}; size: {}", queue.front(), queue.len());
    print!("Queue removal order:");
    while let Some(value) = queue.dequeue() {
        print!(" {value}");
    }
    println!();
    println!(
        "Empty removal: stack={:?}, queue={:?}",
        stack.pop(),
        queue.dequeue()
    );
    let mut symbols = D::default();
    symbols.insert("total", "int");
    symbols.insert("active", "bool");
    symbols.insert("name", "string");
    println!("Lookup total: {:?}", symbols.get(&"total"));
    println!(
        "Update total; old type: {:?}",
        symbols.insert("total", "float")
    );
    println!("Contains name: {}", symbols.contains_key(&"name"));
    println!("Remove name: {:?}", symbols.remove(&"name"));
    println!("Missing lookup: {:?}", symbols.get(&"missing"));
    println!("Dictionary in key order ({} entries):", symbols.len());
    symbols.visit(|key, value| println!("  {key} -> {value}"));
    stack.push(99);
    queue.enqueue(99);
    stack.clear();
    queue.clear();
    symbols.clear();
    println!(
        "After clear: stack={}, queue={}, dictionary={}",
        stack.is_empty(),
        queue.is_empty(),
        symbols.is_empty()
    );
}

fn main() {
    demonstrate::<LibraryStack<i32>, LibraryQueue<i32>, LibraryDictionary<&str, &str>>(
        "STANDARD LIBRARY",
    );
    demonstrate::<ManualStack<i32>, ManualQueue<i32>, ManualDictionary<&str, &str>>("MANUAL NODES");
    demonstrate::<LibraryStack<i32>, LibraryQueue<i32>, LibraryHashDictionary<&str, &str>>(
        "STANDARD LIBRARY, HASHMAP DICTIONARY",
    );
}
