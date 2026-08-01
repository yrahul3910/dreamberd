use dreamberd::{scanner::scan_tokens, tokens::TokenType};

/// Scan `src` and assert it lexed cleanly.
pub fn toks(src: &str) -> Vec<TokenType> {
    let result = scan_tokens(src, "test.gom");
    assert!(
        result.errors.is_empty(),
        "unexpected errors for \u{1b}[32m{src:?}\u{1b}[0m: {:?}",
        result.errors
    );
    result.tokens
}

/// Scan `$src` and assert the token stream equals `$expected` exactly.
///
/// On failure the panic message includes the source that was scanned, and
/// the reported location is the test rather than this helper.
macro_rules! assert_tokens {
    ($src:expr, $expected:expr $(,)?) => {{
        let result = dreamberd::scanner::scan_tokens($src, "test.gom");
        assert!(
            result.errors.is_empty(),
            "\n  while scanning \u{1b}[32m{:?}\u{1b}[0m: {:?}",
            $src,
            result.errors
        );
        assert_eq!(
            result.tokens, $expected,
            "\n  while scanning \u{1b}[32m{:?}\u{1b}[0m",
            $src
        );
    }};
}

pub(crate) use assert_tokens;

/// Scan `$src` and assert it lexes with no errors and ends in `Eof`.
///
/// Unlike [`assert_tokens`], this makes no claim about which tokens are
/// produced: it exists to pin "valid syntax from the spec scans cleanly".
macro_rules! assert_scans {
    ($src:expr $(,)?) => {{
        let result = dreamberd::scanner::scan_tokens($src, "test.gom");
        assert!(
            result.errors.is_empty(),
            "\n  while scanning \u{1b}[32m{:?}\u{1b}[0m: {:?}",
            $src,
            result.errors
        );
        assert_eq!(
            result.tokens.last(),
            Some(&dreamberd::tokens::TokenType::Eof),
            "\n  while scanning \u{1b}[32m{:?}\u{1b}[0m: no trailing Eof",
            $src
        );
    }};
}

pub(crate) use assert_scans;
