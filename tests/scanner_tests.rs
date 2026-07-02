use dreamberd::scanner::scan_tokens;
use dreamberd::tokens::TokenType::*;

#[test]
fn scans_a_simple_declaration() {
    let result = scan_tokens("const const x = 5!", "test.gom");
    // `const`/`x` are identifiers for now; pairing `const const` -> ConstConst
    // is deferred to the parser (see scanner.rs notes).
    assert!(result.errors.is_empty(), "unexpected errors: {:?}", result.errors);
    assert_eq!(result.tokens.last(), Some(&Eof));
    assert!(result.tokens.contains(&Assignment));
    assert!(result.tokens.contains(&Bang(1)));
    assert!(result.tokens.contains(&Float(5.0)));
}

#[test]
fn recognises_keywords_and_identifiers() {
    let result = scan_tokens("return maybe foo", "test.gom");
    assert_eq!(
        result.tokens,
        vec![
            Return,
            Space(1),
            Maybe,
            Space(1),
            Identifier("foo".to_string()),
            Eof,
        ]
    );
}

#[test]
fn equality_operators_match_longest_first() {
    let result = scan_tokens("==== === == =", "test.gom");
    assert_eq!(
        result.tokens,
        vec![
            MorePreciseCheck,
            Space(1),
            PreciseCheck,
            Space(1),
            LooseCheck,
            Space(1),
            Assignment,
            Eof,
        ]
    );
}

#[test]
fn comments_collapse_to_a_newline() {
    let result = scan_tokens("x // a comment\ny", "test.gom");
    assert!(result.tokens.contains(&Newline));
    assert!(!result.tokens.iter().any(|t| matches!(t, Comment)));
    assert!(result.tokens.contains(&Identifier("y".to_string())));
}

#[test]
fn function_abbreviations_map_to_function() {
    for spelling in ["function", "func", "fun", "fn"] {
        let result = scan_tokens(spelling, "test.gom");
        assert_eq!(result.tokens, vec![Function, Eof], "failed for {spelling}");
    }
}

#[test]
fn unicode_identifiers_scan_without_byte_errors() {
    // Char positions, not byte offsets: a multi-byte identifier char must not
    // desync the scanner.
    let result = scan_tokens("café = 1!", "test.gom");
    assert!(result.errors.is_empty(), "unexpected errors: {:?}", result.errors);
    assert!(result.tokens.contains(&Identifier("café".to_string())));
}

#[test]
fn member_access_dot_is_a_token_not_an_error() {
    let result = scan_tokens("name.push", "test.gom");
    assert!(result.errors.is_empty(), "unexpected errors: {:?}", result.errors);
    assert_eq!(
        result.tokens,
        vec![
            Identifier("name".to_string()),
            Dot,
            Identifier("push".to_string()),
            Eof,
        ]
    );
}

#[test]
fn three_space_indent_is_accepted_and_coalesced() {
    let result = scan_tokens("   print", "test.gom");
    assert!(result.errors.is_empty(), "unexpected errors: {:?}", result.errors);
    // The whole leading run becomes one Space token, not three.
    assert_eq!(result.tokens.first(), Some(&Space(3)));
}

#[test]
fn indent_that_is_not_a_multiple_of_three_is_an_error() {
    let result = scan_tokens("  print", "test.gom"); // two spaces
    assert_eq!(result.errors.len(), 1);
    assert!(
        format!("{:?}", result.errors[0]).contains("multiple of 3"),
        "error should describe the indent rule: {:?}",
        result.errors[0]
    );
}

#[test]
fn spaces_between_tokens_are_not_treated_as_indentation() {
    // A two-space gap mid-line is not a line-start indent, so it must not error.
    let result = scan_tokens("a  b", "test.gom");
    assert!(result.errors.is_empty(), "unexpected errors: {:?}", result.errors);
    assert!(result.tokens.contains(&Space(1)));
}

#[test]
fn trailing_spaces_on_a_blank_line_are_not_indentation() {
    let result = scan_tokens("  \n", "test.gom"); // two spaces, then newline
    assert!(result.errors.is_empty(), "unexpected errors: {:?}", result.errors);
}

#[test]
fn unrecognised_input_is_reported_as_a_diagnostic() {
    // `@` matches no token rule, so it becomes a single scan error while the
    // surrounding identifiers still tokenise.
    let result = scan_tokens("a @ b", "test.gom");
    assert_eq!(result.errors.len(), 1);
    // The diagnostic quotes the offending input.
    assert!(
        format!("{:?}", result.errors[0]).contains('@'),
        "error should mention the bad input: {:?}",
        result.errors[0]
    );
    // Scanning recovers: identifiers on either side are still produced.
    assert!(result.tokens.contains(&Identifier("a".to_string())));
    assert!(result.tokens.contains(&Identifier("b".to_string())));
}
