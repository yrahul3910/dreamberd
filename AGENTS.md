# Context for Coding Agents

This is a work-in-progress Rust implementation of an interpreter for the DreamBerd (aka Gulf of Mexico) esolang. The specification is in `docs/SPECIFICATION.md`, but since it is at times vague or contradictory, changes to the original spec are in `docs/DEVIATIONS.md`. The current grammar is in `docs/GRAMMAR.md`.

This implementation follows the book, "Crafting Interpreters", with some changes for idiomatic Rust.

## Testing

Integration tests live in `tests/` (entry point: `tests/integration_tests.rs`, shared helpers in `tests/common/mod.rs` such as the `assert_tokens!` and `assert_scans!` macros).

**One test fails deliberately:** `scanner::statement_tests::comment_marker_inside_a_string_stays_string_content` pins the *spec* behavior for `//` inside a string literal (it should be string content, but the scanner doesn't track string state yet and lexes it as a comment). Do not "fix" it by adjusting the test; it should start passing once the scanner becomes string-aware.

**Another test is expected to fail:** `scanner::statement_tests::main_gom_statements` currently fails, because `i` is tokenized as a `Function` instead of an `Identifier`. This is correct, but `Function` will be removed later, so it's fine.
