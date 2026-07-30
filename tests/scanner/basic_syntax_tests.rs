use crate::common::{assert_tokens, toks};
use dreamberd::{
    scanner::scan_tokens,
    tokens::TokenType::{self, *},
};

#[test]
fn equality_operators_match_longest_first() {
    assert_tokens!(
        "==== === == =",
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
fn five_or_more_equals_is_a_file_delimiter() {
    assert_tokens!("=====\n=====", vec![FileDelim, Newline, FileDelim, Eof]);
}

#[test]
fn file_delimiter_of_ten_equals_is_one_token() {
    assert_tokens!("==========", vec![FileDelim, Eof]);
}

#[test]
fn declaration_keyword_combinations() {
    assert_tokens!("const const const", vec![ConstConstConst, Eof]);
    assert_tokens!("const const", vec![ConstConst, Eof]);
    assert_tokens!("const var", vec![ConstVar, Eof]);
    assert_tokens!("var const", vec![VarConst, Eof]);
    assert_tokens!("var var", vec![VarVar, Eof]);
}

#[test]
fn keywords_prefixed_by_identifier_chars_stay_identifiers() {
    // "const" is only a keyword when it stands alone.
    assert_tokens!("constant", vec![Identifier("constant".into()), Eof]);
}

#[test]
fn any_ordered_subsequence_of_function_at_least_3_chars_is_the_function_keyword() {
    for spelling in [
        "function", "functio", "functin", "fuctin", "func", "fun", "fin", "fnc", "uin", "fio",
        "fn", "fu", "fnt", "union", "fuc", "in", "on",
    ] {
        assert_tokens!(spelling, vec![Function, Eof]);
    }
}

#[test]
fn too_short_or_out_of_order_letter_runs_are_identifiers() {
    for spelling in ["f", "nf", "ions"] {
        assert_tokens!(spelling, vec![Identifier(spelling.to_string()), Eof]);
    }
}

#[test]
fn comment_runs_to_end_of_line_leaving_a_newline() {
    assert_tokens!(
        "x // a comment\ny",
        vec![
            Identifier("x".into()),
            Space(1),
            Newline,
            Identifier("y".into()),
            Eof,
        ]
    );
}

#[test]
fn comment_at_eof_needs_no_newline() {
    let result = scan_tokens("x // trailing comment", "test.gom");
    assert!(result.errors.is_empty());
    assert_eq!(result.tokens.last(), Some(&Eof));
    assert!(!result.tokens.iter().any(|t| matches!(t, Comment)));
    assert!(result.tokens.contains(&Identifier("x".to_string())));
}

#[test]
fn comment_contents_are_never_tokenised() {
    assert_tokens!("// ==== const delete\n42", vec![Newline, Float(42.0), Eof]);
}

#[test]
fn integer_and_float_literals() {
    assert_tokens!("42", vec![Float(42.0), Eof]);
    assert_tokens!("3.14", vec![Float(3.14), Eof]);
}

#[test]
fn hex_and_octal_literals() {
    assert_tokens!("0xFF", vec![Float(255.0), Eof]);
    assert_tokens!("0o17", vec![Float(15.0), Eof]);
}

#[test]
fn range_literals() {
    // Custom extension: a..b (see GRAMMAR.md).
    assert_tokens!("0..4", vec![Range(0, 4), Eof]);
    assert_tokens!("0x1..0xF", vec![Range(1, 15), Eof]);
}

#[test]
fn malformed_number_is_an_error_and_scanning_continues() {
    let result = scan_tokens("1.2.3 y", "test.gom");
    assert_eq!(result.errors.len(), 1);
    // Recovery: the identifier after the bad number still tokenises.
    assert!(result.tokens.contains(&Identifier("y".into())));
}

#[test]
fn exclamation_marks_coalesce_into_one_priority() {
    assert_tokens!(
        "print(x)!!",
        vec![
            Identifier("print".into()),
            LeftParen,
            Identifier("x".into()),
            RightParen,
            Bang(2),
            Eof,
        ]
    );
}

#[test]
fn inverted_exclamation_marks_give_negative_priority() {
    // DEVIATIONS: "¡" inverted bangs subtract from the count.
    assert_tokens!(
        "print(x)¡¡",
        vec![
            Identifier("print".into()),
            LeftParen,
            Identifier("x".into()),
            RightParen,
            Bang(-2),
            Eof,
        ]
    );
}

#[test]
fn mixed_bangs_net_out() {
    assert_tokens!("x!!¡", vec![Identifier("x".into()), Bang(1), Eof]);
}

#[test]
fn boolean_and_maybe_literals() {
    assert_tokens!("true", vec![True, Eof]);
    assert_tokens!("false", vec![False, Eof]);
    assert_tokens!("maybe", vec![Maybe, Eof]);
}

#[test]
fn keyword_table() {
    let cases: &[(&str, TokenType)] = &[
        ("when", When),
        ("return", Return),
        ("previous", Previous),
        ("next", Next),
        ("current", Current),
        ("import", Import),
        ("export", Export),
        ("to", To),
        ("class", Class),
        ("className", Class),
        ("new", New),
        ("delete", Delete),
        ("async", Async),
        ("await", Await),
        ("noop", Noop),
        ("reverse", Reverse),
        ("Infinity", Infinity),
        ("undefined", Undefined),
        ("Int", IntT),
        ("String", StringT),
        ("Char", CharT),
        ("Digit", DigitT),
        ("Int9", Int9T),
        ("Int99", Int99T),
        ("Regex", RegexpT),
        ("RegExp", RegexpT),
        ("RegularExpression", RegexpT),
    ];
    for (src, expected) in cases {
        assert_tokens!(src, vec![expected.clone(), Eof]);
    }
}

#[test]
fn keywords_are_case_sensitive_except_explicit_aliases() {
    assert_tokens!("TRUE", vec![Identifier("TRUE".into()), Eof]);
    assert_tokens!("Class", vec![Identifier("Class".into()), Eof]);
}

#[test]
fn member_access_parses_as_dot() {
    assert_tokens!(
        "name.push",
        vec![
            Identifier("name".into()),
            Dot,
            Identifier("push".into()),
            Eof,
        ]
    );
}

#[test]
fn quote_kinds_are_distinct_tokens() {
    assert_tokens!(
        "'a'",
        vec![
            Quote("'".into()),
            Identifier("a".into()),
            Quote("'".into()),
            Eof
        ]
    );
    assert_tokens!(
        "\"a\"",
        vec![
            Quote("\"".into()),
            Identifier("a".into()),
            Quote("\"".into()),
            Eof
        ]
    );
}

#[test]
fn three_space_indent_is_accepted() {
    // "All indents must be 3 spaces long."
    assert_eq!(toks("   print").first(), Some(&Space(3)));
}

#[test]
fn indent_that_is_not_a_multiple_of_three_is_an_error() {
    let result = scan_tokens("  print", "test.gom"); // two spaces
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn six_space_indent_is_two_levels_and_fine() {
    assert_eq!(toks("      print").first(), Some(&Space(6)));
}

#[test]
fn spaces_between_tokens_are_not_indentation() {
    // A two-space gap mid-line is not a line-start indent, so no error.
    assert_tokens!(
        "a  b",
        vec![
            Identifier("a".into()),
            Space(2),
            Identifier("b".into()),
            Eof
        ]
    );
}

#[test]
fn blank_line_whitespace_is_not_indentation() {
    let result = scan_tokens("  \n", "test.gom");
    assert!(result.errors.is_empty());
}

#[test]
fn empty_source_is_just_eof() {
    assert_tokens!("", vec![Eof]);
}

#[test]
fn unrecognised_input_become_zero_quote_strings() {
    let result = scan_tokens("a @ b", "test.gom");
    assert_eq!(result.errors.len(), 0);
    assert!(result.tokens.contains(&Identifier("a".into())));
    assert!(result.tokens.contains(&Identifier("@".into())));
    assert!(result.tokens.contains(&Identifier("b".into())));
}

#[test]
fn unicode_source_does_not_panic_or_desync() {
    // Multi-byte chars must not break the scanner's position tracking.
    let result = scan_tokens("café = 1!", "test.gom");
    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);
    assert_eq!(
        result.tokens,
        vec![
            Identifier("café".into()),
            Space(1),
            Assignment,
            Space(1),
            Float(1.0),
            Bang(1),
            Eof,
        ]
    );
}
