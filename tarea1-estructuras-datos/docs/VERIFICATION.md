# Verification report

## Observed results

- `cargo test`: **34 passed, 0 failed** (integration tests).
- `cargo run --quiet`: completed successfully; the two demonstration bodies matched exactly after their different headings were removed.
- `cargo fmt --check`: passed.
- `cargo clippy --all-targets -- -D warnings`: passed.
- Compiler: `rustc 1.70.0 (90c541806 2023-05-31)`.
- Cargo: `cargo 1.70.0 (ec8a8a0ca 2023-04-25)`.
- External dependencies: none.
- The reusable library forbids unsafe Rust with `#![forbid(unsafe_code)]`.
- The manual collection modules use no built-in collection storage.

The zero-test results for the separate library/binary unit-test targets and doc-tests are expected. The 34 behavioral checks are in `tests/behavior.rs`.

## Guide coverage

The PDF explains all **753 nonblank lines across 774 physical lines** of the eight delivered Rust source files, including the test file. Blank lines are layout separators. `LINE_BY_LINE.json` contains each numbered source line and its explanation. `SOURCE_MANIFEST.json` records SHA-256 hashes of the final formatted source files.

The PDF includes a macro-level design overview, Rust syntax primer, ownership and borrowing explanations, operation traces, a tree diagram, function-by-function walkthroughs, test explanations, exercises, a clickable contents page, and chapter bookmarks. It was rendered and visually inspected.

## Limits of validation

Tests validate the listed scenarios and the reproducible 1,000-step differential sequence. They do not prove behavior for every possible input, measure performance, or eliminate the documented recursion-depth limit in manual tree deletion/traversal. At the time of this original report, no remote Git push or Canvas submission had been performed.

## Re-verification after moving into the course repository (2026-10-01, commit c8e96c7)

Run from `tarea1-estructuras-datos/` with `rustc 1.77.1 (7cf61ebde 2024-03-27)` and `cargo 1.77.1 (e52e36006 2024-03-26)`:

- `cargo test`: **34 passed, 0 failed** in `tests/behavior.rs`.
- `cargo fmt --check`: passed (exit 0).
- `cargo clippy --all-targets -- -D warnings`: passed (exit 0).
- `cargo run --quiet`: output identical to the `docs/DEMO_OUTPUT.txt` of that commit.
- The SHA-256 hashes in `docs/SOURCE_MANIFEST.json` matched all eight source files at that commit, so the learning PDF described the code as committed then.

## Verification after adding the HashMap dictionary (2026-10-01)

Same toolchain (`rustc 1.77.1`, `cargo 1.77.1`):

- `cargo test`: **41 passed, 0 failed** (34 previous + 7 `hash_table` scenarios), in 5 consecutive runs, because `HashMap` order varies between runs. The captured output is `docs/TEST_OUTPUT.txt`.
- Mutation check: with the key sort removed from `LibraryHashDictionary::visit` in a scratch copy, 3 tests failed in each of 3 runs (`hash_table::sorted_dictionary`, `hash_table::successor_dictionary`, `deterministic_differential_operations`).
- `cargo fmt --check`: passed (exit 0).
- `cargo clippy --all-targets -- -D warnings`: passed (exit 0).
- `cargo run --quiet`: regenerated as `docs/DEMO_OUTPUT.txt`; the third heading's body is identical to the other two.
- PDF coverage: `src/lib.rs`, `src/contracts.rs`, `src/manual_stack.rs`, `src/manual_queue.rs`, and `src/manual_map.rs` still match `docs/SOURCE_MANIFEST.json`. `src/std_impl.rs`, `src/main.rs`, and `tests/behavior.rs` changed, so the PDF and `LINE_BY_LINE.json` do not cover the HashMap additions.
