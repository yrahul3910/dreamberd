use dreamberd::scanner::scan_tokens;
use dreamberd::tokens::TokenType::*;

#[test]
fn scans_a_simple_declaration() {
    let result = scan_tokens("const const x = 5!");
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
    let result = scan_tokens("return maybe foo");
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
    let result = scan_tokens("==== === == =");
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
    let result = scan_tokens("x // a comment\ny");
    assert!(result.tokens.contains(&Newline));
    assert!(!result.tokens.iter().any(|t| matches!(t, Comment)));
    assert!(result.tokens.contains(&Identifier("y".to_string())));
}

#[test]
fn function_abbreviations_map_to_function() {
    for spelling in ["function", "func", "fun", "fn"] {
        let result = scan_tokens(spelling);
        assert_eq!(result.tokens, vec![Function, Eof], "failed for {spelling}");
    }
}

#[test]
fn unicode_identifiers_scan_without_byte_errors() {
    // Char positions, not byte offsets: a multi-byte identifier char must not
    // desync the scanner.
    let result = scan_tokens("café = 1!");
    assert!(result.errors.is_empty(), "unexpected errors: {:?}", result.errors);
    assert!(result.tokens.contains(&Identifier("café".to_string())));
}
