# Assignment 1: Data Structures in Rust

This project demonstrates a stack (LIFO), a queue (FIFO), and a dictionary in two ways: adapters around Rust's standard collections, and manually implemented linked structures. The dictionary has a third version backed by a hash table (`HashMap`). All code, comments, tests, and documentation are in English.

## Run it

Install a Rust toolchain with Cargo using https://rustup.rs/ if needed. Open a terminal in this project directory:

```sh
cargo run
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

The project has no external dependencies. `src/lib.rs` contains the reusable library; `src/main.rs` is the console demonstration; `tests/behavior.rs` validates every implementation.

## What the assignment asks for

The supplied assignment permits public libraries or implementation from scratch. It requests source files, a small demonstration program, descriptions of test cases, storage in Git, and the exact AI tools and prompts used. See `TEST_CASES.md` and `AI_USAGE.md`.

`TABLE/HASH/DICTIONARY (order)` is implemented as a key-value dictionary with unique keys whose traversal follows **sorted key order**. It is provided three ways: a balanced search tree (`BTreeMap`), a hash table (`HashMap`), and a manual binary search tree. All three satisfy the same contract and pass the same tests.

## Implementations and their shared contract

| Concept | Standard-collection adapter | Manual implementation |
|---|---|---|
| Stack | `LibraryStack<T>` wraps `Vec<T>` | `ManualStack<T>` links heap-allocated nodes |
| Queue | `LibraryQueue<T>` wraps `VecDeque<T>` | `ManualQueue<T>` links heap-allocated nodes |
| Dictionary (tree) | `LibraryDictionary<K, V>` wraps `BTreeMap<K, V>` | `ManualDictionary<K, V>` is an unbalanced binary search tree |
| Dictionary (hash table) | `LibraryHashDictionary<K, V>` wraps `HashMap<K, V>` | — |

`contracts.rs` defines three traits. A trait describes required behavior, much like an interface. A `struct` defines stored data, and an `impl` block provides methods. Rust does not use traditional class declarations or class inheritance here.

The same generic demonstration and test scenarios call these traits. If a manual implementation violates the agreed behavior, the same assertions that validate the library version should fail for the manual version.

### Stack operations

`push(value)` takes ownership of a value. `pop()` removes and returns the most recent value as `Some(value)`, or returns `None` when empty. `peek()` borrows the most recent value without removing it. `len()`, `is_empty()`, and `clear()` inspect or reset the stack. Duplicate values are allowed.

### Queue operations

`enqueue(value)` takes ownership of a value at the back. `dequeue()` removes and returns the earliest remaining value. `front()` borrows that value without removing it. `len()`, `is_empty()`, and `clear()` behave as for the stack. Duplicate values are allowed.

### Dictionary operations

`insert(key, value)` adds a new key and returns `None`, or replaces an existing key's value and returns `Some(old_value)` without changing the count. `get(&key)` borrows a stored value. `remove(&key)` transfers a removed value to the caller. `contains_key(&key)` checks membership. `visit(visitor)` calls a closure for each key/value pair in ascending key order. `len()`, `is_empty()`, and `clear()` inspect or reset the dictionary.

Keys require `Ord`, so any two keys can be compared; the hash-table version also requires `Hash`. Values do not need `Clone` or `Copy`.

`HashMap` stores entries in no fixed order. To keep the shared contract, `LibraryHashDictionary::visit` collects references to the entries and sorts them by key before calling the visitor. Insert, lookup, and removal keep the hash table's average O(1) cost; only full traversal pays for the sort. This is the structure a compiler typically uses for a symbol table, where lookups by name dominate. Traversal uses a callback instead of returning a `Vec`, keeping the manual collection modules free of built-in collection storage.

## What "from scratch" means here

The files `manual_stack.rs`, `manual_queue.rs`, and `manual_map.rs` use no `Vec`, `VecDeque`, `HashMap`, `BTreeMap`, external collection crate, raw pointer, or `unsafe` block. They implement all links and operations themselves.

They still use basic Rust facilities: `Option`, `Box`, `Ordering`, and `std::mem::replace`. `Box` supplies heap allocation and ownership; it does not implement a stack, queue, or dictionary for us. Avoiding these facilities entirely would turn this exercise into an allocator and unsafe-memory exercise.

The tests use `Vec` to collect observed output for comparison. That use is outside the manual storage implementation. Tests also use `Rc<Cell<usize>>` to count destruction of tracked values; those types do not implement any of the submitted structures.

## Performance and design limits

Here `n` is the number of entries and `h` is the manual tree's height.

| Operation | Standard version | Manual version | HashMap dictionary |
|---|---|---|---|
| Stack push | O(1) amortized | O(1) | — |
| Stack pop / peek | O(1) | O(1) | — |
| Queue enqueue | O(1) amortized | O(n): walks to the tail | — |
| Queue dequeue / front | O(1) | O(1) | — |
| Dictionary insert / get / remove | O(log n) | O(h), worst-case O(n) | O(1) average |
| Dictionary full traversal | O(n) | O(n), with O(h) recursive call depth | O(n log n): sorts by key |
| Size / empty checks | O(1) | O(1) | O(1) |
| Clear | O(n) | O(n) | O(n) |

The manual queue favors simple, safe ownership over a tail-pointer optimization. The manual tree is not balanced: inserting sorted keys can create a chain. Recursive tree traversal and deletion can exhaust the call stack for sufficiently deep trees. These are educational implementations, not replacements for production collections. Stack/queue cleanup and dictionary cleanup are iterative to avoid recursively dropping a chain of owned nodes. Dictionary cleanup uses rotations to preserve all remaining nodes until they are detached and dropped.

Identical behavior does not imply identical algorithms or running time. The tests check correctness, not benchmark claims.

## Expected demonstration

All three headings (standard library, manual nodes, and standard library with the HashMap dictionary) should show the same operations and results:

- Pushing/enqueuing `10, 20, 30` yields stack removal `30 20 10` and queue removal `10 20 30`.
- Empty removal returns `None`.
- `total` changes from `int` to `float`, with `Some("int")` returned on replacement.
- Removing `name` returns its previous value.
- The remaining dictionary entries print `active` before `total`.
- All three empty checks print `true` after clearing.

## Folder layout

```text
tarea1-estructuras-datos/
├── Cargo.toml, Cargo.lock
├── Entrega_Tarea1.pdf submission document for Canvas (Spanish): Git link, approach, AI use, test cases
├── README.md          this overview
├── TEST_CASES.md      test-case descriptions (deliverable)
├── AI_USAGE.md        AI tools and exact prompts used (deliverable)
├── src/               library (contracts, std adapters, manual structures) + demo in main.rs
├── tests/behavior.rs  behavioral test suite
└── docs/
    ├── interactive_guide.html              interactive guide: playground, annotated code, quiz
    ├── Rust_Data_Structures_Explained.pdf  line-by-line learning guide
    ├── LINE_BY_LINE.json                    data behind the guide
    ├── SOURCE_MANIFEST.json                 SHA-256 of the sources the guide describes
    ├── VERIFICATION.md                      verification report
    ├── DEMO_OUTPUT.txt                      captured `cargo run` output
    └── TEST_OUTPUT.txt                      captured `cargo test` output
```

This folder lives inside the course repository, next to the other assignments and the mini-project.

## Interactive guide

Open `docs/interactive_guide.html` in any browser; it is a single self-contained file. It has four tabs:

- **Playground**: run any operation on any implementation and step through it one source line at a time. A memory diagram shows nodes, the Vec buffer, the VecDeque ring buffer, the binary search tree, or the hash table slots, next to the highlighted line that is running. Scripts replay `main.rs` and several tests.
- **Code**: every source file with an explanation for each line and a summary for each function.
- **Big picture**: how the traits, the seven implementations, the demo, and the tests fit together, the cost table, and the Rust concepts used, each linked to the lines that use it.
- **Quiz**: rounds of ten questions, including generated "predict the result" puzzles.

The page embeds the source code as of the commit that added it, so it does not follow later code changes. The HashMap view is a simplified teaching model (FNV-1a hash, linear probing), not the exact internals of Rust's `HashMap`.

The PDF in `docs/` explains the original two-implementation version line by line; `docs/SOURCE_MANIFEST.json` records the exact source versions it describes. The HashMap dictionary was added afterwards: `src/lib.rs`, `src/contracts.rs`, and the three `src/manual_*.rs` files still match the PDF, while `src/std_impl.rs` (new `HashMap` import and the `LibraryHashDictionary` adapter appended at the end), `src/main.rs` (third demonstration), and `tests/behavior.rs` (dictionary scenarios also run against the hash table) have changed. Those additions are described in this README and in `TEST_CASES.md`.

## References

- Standard collections: https://doc.rust-lang.org/std/collections/index.html
- Vec: https://doc.rust-lang.org/std/vec/struct.Vec.html
- VecDeque: https://doc.rust-lang.org/std/collections/struct.VecDeque.html
- BTreeMap: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
- HashMap: https://doc.rust-lang.org/std/collections/struct.HashMap.html
- Ownership: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
- Borrowing: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
- Box: https://doc.rust-lang.org/book/ch15-01-box.html
- Option: https://doc.rust-lang.org/std/option/enum.Option.html
- Testing: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
