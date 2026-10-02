# Test-case descriptions

Run `cargo test`. There are 16 shared scenarios instantiated for the `library` and `manual` implementations (32 named tests), the 7 dictionary scenarios instantiated again for the `hash_table` module (`LibraryHashDictionary`), one 1,000-step differential scenario, and one manual destruction scenario: **41 tests in total**.

| Named scenario (in `library` and `manual`; the `_dictionary` rows also in `hash_table`) | Input/actions | Expected result |
|---|---|---|
| `empty_stack` | Create; query size/top; pop | Empty, size 0, `None`; size stays 0 |
| `lifo_stack` | Push 10, 20, 30; peek; pop three times | Peek 30 without removal; pop 30, 20, 10 |
| `reusable_stack` | Push duplicate 1s; clear twice; push/pop 9 | Duplicates count separately; safe repeated clear; reuse works |
| `owned_stack` | Push owned `String`; borrow; pop | Content is preserved without a `Clone` requirement |
| `empty_queue` | Create; query front; dequeue | Empty, size 0, `None` |
| `fifo_queue` | Enqueue 10, 20, 30; front; dequeue | Front 10 without removal; dequeue 10, 20, 30 |
| `mixed_queue` | Add 1/2; remove 1; add 3; drain; add 4 | Removal 1, 2, 3, 4; reuse after draining |
| `reusable_queue` | Add duplicate 1s; clear twice; add/remove 9 | Count 2 before clearing; empty; reuse works |
| `owned_queue` | Enqueue owned `String`; borrow; dequeue | Content preserved without cloning |
| `empty_dictionary` | Missing get/remove; membership; visit | `None`, false; visitor never called |
| `updated_dictionary` | Insert (2,20), then (2,99); remove twice | Old value 20 returned; size remains 1; remove 99, then `None` |
| `sorted_dictionary` | Insert keys 4,2,6,1,3,5,7; visit | Pairs (1,10) through (7,70) in ascending order; size 7 |
| `deleted_dictionary` | Build that tree; remove 1,2,4,6,3,5,7 | Correct values/count after every removal; leaf, one-child, two-child and root cases exercised in manual tree |
| `successor_dictionary` | Insert 8,4,12,10,11,14; remove 8 | Keys 4,10,11,12,14 survive; successor's right child 11 is preserved |
| `reusable_dictionary` | Insert 0..127; clear twice; insert/remove (9,90) | Empty size 0; no old key remains; reuse works |
| `owned_dictionary` | Owned string key/value; get/remove | Borrowed content correct; removed owned string returned |

## Differential test

`deterministic_differential_operations` uses a fixed seed (42) and a reproducible wrapping arithmetic generator. For 1,000 steps it selects insert/add, remove, lookup, or clear operations. It compares returned values, sizes, stack tops, queue fronts, and the complete dictionary traversal after each step between the library and manual versions; the HashMap dictionary is compared against the BTreeMap dictionary in the same way. This is a broad consistency check against the library-backed implementation, not a proof of correctness or a performance benchmark.

## Hash-table ordering check

`HashMap` iterates in an arbitrary order that changes between runs. `sorted_dictionary`, `successor_dictionary`, and the differential test all compare the full traversal, so they fail if `LibraryHashDictionary::visit` stops sorting by key. This was confirmed by deleting the sort line in a scratch copy: those 3 tests failed in each of 3 runs (38 passed, 3 failed), while the unmodified code passed 41 of 41 in 5 consecutive runs.

## Destruction test

`manual_values_drop_exactly_once` inserts 20 tracked values into each manual structure (60 values), explicitly removes three, and replaces one dictionary value with a newly created value (61 values created total). Four values must have been destroyed before the structures leave scope. The final total must be 61. This checks that removals, replacement, and automatic cleanup destroy every stored value exactly once in that scenario.

## Other checks

The verification report (`docs/VERIFICATION.md`) records actual compiler/tool versions and check outcomes. `cargo fmt --check` checks formatting; Clippy checks lint issues; `cargo run` executes the demonstration. The transcript is supplied as `docs/DEMO_OUTPUT.txt`.
