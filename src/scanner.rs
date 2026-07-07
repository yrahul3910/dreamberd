//! The DreamBerd scanner (lexer).

use miette::{Diagnostic, NamedSource, SourceSpan};
use regex::Regex;
use thiserror::Error;

use crate::tokens::TokenType;

/// A single problem the scanner found in the source.
#[derive(Debug, Error, Diagnostic)]
#[error("{message}")]
#[diagnostic(code(dreamberd::scanner))]
pub struct LexerError {
    /// The full source under scan, so the error renders in context.
    #[source_code]
    src: NamedSource<String>,

    /// Byte span of the offending input within `src`.
    #[label("{hint}")]
    span: SourceSpan,

    /// Short annotation rendered on the highlighted span.
    hint: String,

    /// Full description rendered as the error line.
    message: String,

    /// Optional suggestion for fixing the problem.
    #[help]
    advice: Option<String>,
}

/// All [`LexerError`]s from a single scan, grouped so they report together.
#[derive(Debug, Error, Diagnostic)]
#[error("scanning failed with {} error(s)", .errors.len())]
pub struct ScanErrors {
    #[related]
    errors: Vec<LexerError>,
}

impl ScanErrors {
    /// Group scan errors into a single reportable diagnostic.
    pub fn new(errors: Vec<LexerError>) -> Self {
        Self { errors }
    }
}

/// Result of scanning a source string.
#[derive(Debug, Default)]
pub struct ScanResult {
    /// Tokens emitted by the lexer
    pub tokens: Vec<TokenType>,
    /// Diagnostics for any input the scanner could not tokenise.
    pub errors: Vec<LexerError>,
}

/// Attempt to match one of the multi-character operators at `pos`, in order.
///
/// The `=` family is listed longest-first so `====` wins over `==`, and `//`
/// precedes `/` so comments aren't mistaken for division.
fn match_multi(chars: &str, pos: usize) -> Option<(usize, TokenType)> {
    // File delimiters can be 5 or more = (with no bound), so we check that separately
    if chars[pos..].starts_with("=====") {
        return chars[pos..]
            .find(|c: char| c.is_whitespace())
            .map(|p| (p, TokenType::FileDelim));
    }

    let table = [
        ("====", TokenType::MorePreciseCheck),
        ("===", TokenType::PreciseCheck),
        ("==", TokenType::LooseCheck),
        ("//", TokenType::Comment),
        ("=>", TokenType::Arrow),
        ("..", TokenType::Range), // TODO: doesn't quite work yet
    ];
    table
        .into_iter()
        .find(|(pat, _)| chars[pos..].starts_with(pat))
        .map(|(pat, tok)| (pat.chars().count(), tok))
}

/// Single-character tokens. Letters and digits are intentionally
/// absent: they are handled by [`parse_token`].
fn single_char(c: char) -> Option<TokenType> {
    Some(match c {
        '=' => TokenType::Assignment,
        '/' => TokenType::ForwardSlash,
        ' ' => TokenType::Space(1),
        '!' => TokenType::Bang(1),
        ';' => TokenType::Semicolon,
        '?' => TokenType::QuestionMark,
        '(' => TokenType::LeftParen,
        ')' => TokenType::RightParen,
        '{' => TokenType::LeftBrace,
        '}' => TokenType::RightBrace,
        '[' => TokenType::LeftBracket,
        ']' => TokenType::RightBracket,
        '<' => TokenType::LeftAngular,
        '>' => TokenType::RightAngular,
        '\'' => TokenType::Quote("'".to_string()),
        '"' => TokenType::Quote("\"".to_string()),
        '+' => TokenType::Plus,
        '-' => TokenType::Minus,
        '*' => TokenType::Asterisk,
        '^' => TokenType::Caret,
        ',' => TokenType::Comma,
        '.' => TokenType::Dot,
        ':' => TokenType::Colon,
        '\r' | '\n' => TokenType::Newline,
        _ => return None,
    })
}

/// Read a run of digit-ish characters (`0-9`, `.`, `-`) at `pos` and parse it
/// as an `f64`. Returns the parse result together with the next position, so
/// the caller can always make progress even when the run doesn't parse.
///
/// TODO: This doesn't handle hex: 0x20, for example
///
/// TODO: the accepted char set is loose and will happily consume e.g. `1-2` into
/// a single failing run
fn parse_digit(chars: &[char], pos: usize) -> (Result<f64, ()>, usize) {
    let end = chars[pos..]
        .iter()
        .take_while(|&&c| c.is_ascii_digit() || c == '.' || c == '-')
        .count();
    let s: String = chars[pos..pos + end].iter().collect();
    (s.parse::<f64>().map_err(|_| ()), pos + end)
}

/// Reserved words that map to a single keyword token.
fn keyword(word: &str) -> Option<TokenType> {
    Some(match word {
        "const const const" => TokenType::ConstConstConst,
        "const const" => TokenType::ConstConst,
        "const var" => TokenType::ConstVar,
        "var const " => TokenType::VarConst,
        "var var " => TokenType::VarVar,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "maybe" => TokenType::Maybe,
        "when" => TokenType::When,
        "use" => TokenType::Use,
        "return" => TokenType::Return,
        "previous" => TokenType::Previous,
        "next" => TokenType::Next,
        "current" => TokenType::Current,
        "import" => TokenType::Import,
        "export" => TokenType::Export,
        "to" => TokenType::To,
        // `className` is the JS-compat alias for `class` (SPECIFICATION.md).
        "class" | "className" => TokenType::Class,
        "new" => TokenType::New,
        "delete" => TokenType::Delete,
        "async" => TokenType::Async,
        "await" => TokenType::Await,
        "noop" => TokenType::Noop,
        "reverse" => TokenType::Reverse,
        "Infinity" => TokenType::Infinity,
        "undefined" => TokenType::Undefined,
        // Type names. Annotations are no-ops per the spec but still tokenized.
        "Int" => TokenType::IntT,
        "String" => TokenType::StringT,
        "Char" => TokenType::CharT,
        "Digit" => TokenType::DigitT,
        "Int9" => TokenType::Int9T,
        "Int99" => TokenType::Int99T,
        "Regex" | "RegExp" | "RegularExpression" => TokenType::RegexpT,
        _ => return None,
    })
}

/// Spellings of `function` that the spec shows explicitly.
fn is_function_keyword(word: &str) -> bool {
    const MIN_FUNCTION_KW_LEN: usize = 2;
    let re = Regex::new("f?u?n?c?t?i?o?n?").unwrap();

    word.len() > MIN_FUNCTION_KW_LEN && re.find(word).is_some_and(|m| m.len() == word.len())
}

/// Scan a number, keyword, or identifier at `pos`, assuming [`try_match`] and
/// [`single_char`] have already been tried. On failure returns the position to
/// resume at (always strictly greater than `pos`, guaranteeing progress).
fn parse_token(chars: &[char], pos: usize) -> Result<(TokenType, usize), usize> {
    let c = chars[pos];

    if c.is_ascii_digit() {
        let (parsed, next) = parse_digit(chars, pos);
        return match parsed {
            Ok(value) => Ok((TokenType::Float(value), next)),
            Err(()) => Err(next), // skip the malformed numeric run
        };
    }

    if c.is_alphabetic() || c == '_' {
        let end = chars[pos..]
            .iter()
            .take_while(|&&c| c.is_alphanumeric() || c == '_')
            .count();
        let word: String = chars[pos..pos + end].iter().collect();
        let tok = if let Some(kw) = keyword(&word) {
            kw
        } else if is_function_keyword(&word) {
            TokenType::Function
        } else {
            TokenType::Identifier(word)
        };
        return Ok((tok, pos + end));
    }

    Err(pos + 1)
}

/// Byte span `(offset, length)` covering the char range `[start, end)`.
///
/// The scanner works in char indices (so Unicode identifiers don't desync it),
/// but miette spans are byte offsets, so we sum the UTF-8 widths to convert.
fn byte_span(chars: &[char], start: usize, end: usize) -> SourceSpan {
    let offset: usize = chars[..start].iter().map(|c| c.len_utf8()).sum();
    let len: usize = chars[start..end].iter().map(|c| c.len_utf8()).sum();
    (offset, len).into()
}

/// Scan `source` into a token stream terminated by [`TokenType::Eof`].
///
/// `source_name` labels the source in any [`LexerError`]s produced (e.g. the
/// file path), so diagnostics can point back at the right file.
pub fn scan_tokens(source: &str, source_name: &str) -> ScanResult {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    let mut pos = 0;

    while pos < source.len() {
        // multi-character operators
        if let Some((len, tok)) = match_multi(source, pos) {
            if tok == TokenType::Comment {
                // comments run to end-of-line and are dropped, leaving a newline
                match source[pos..].find('\n') {
                    Some(offset) => {
                        tokens.push(TokenType::Newline);
                        pos += offset + 1;
                    }
                    None => break, // trailing comment; nothing left to scan
                }
            } else {
                tokens.push(tok);
                pos += len;
            }
            continue;
        }

        // single-character tokens
        let chars: Vec<char> = source.chars().collect();
        if let Some(tok) = single_char(chars[pos]) {
            tokens.push(tok);
            pos += 1;
            continue;
        }

        // numbers, keywords, identifiers
        match parse_token(&chars, pos) {
            Ok((tok, next)) => {
                tokens.push(tok);
                pos = next;
            }
            Err(next) => {
                errors.push(LexerError {
                    src: NamedSource::new(source_name, source.to_string()),
                    span: byte_span(&chars, pos, next),
                    hint: "unexpected input".to_string(),
                    message: format!(
                        "unexpected input `{}`",
                        chars[pos..next].iter().collect::<String>()
                    ),
                    advice: Some("remove or replace the highlighted input".to_string()),
                });
                pos = next;
            }
        }
    }

    tokens.push(TokenType::Eof);
    ScanResult { tokens, errors }
}
