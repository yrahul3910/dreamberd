//! The DreamBerd scanner (lexer).

use miette::{Diagnostic, NamedSource, SourceSpan};
use regex::Regex;
use thiserror::Error;

use crate::tokens::TokenType;

/// Width of a single indentation level, in spaces (see the Indents section of
/// SPECIFICATION.md). Leading whitespace on a line must be a multiple of this.
const INDENT_WIDTH: usize = 3;

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
    pub tokens: Vec<TokenType>,
    /// Diagnostics for any input the scanner could not tokenise.
    pub errors: Vec<LexerError>,
}

/// Attempt to match one of the multi-character operators at `pos`, in order.
///
/// The `=` family is listed longest-first so `====` wins over `==`, and `//`
/// precedes `/` so comments aren't mistaken for division.
fn match_multi(chars: &[char], pos: usize) -> Option<(usize, TokenType)> {
    let table: [(&str, TokenType); 6] = [
        ("====", TokenType::MorePreciseCheck),
        ("===", TokenType::PreciseCheck),
        ("==", TokenType::LooseCheck),
        ("=", TokenType::Assignment),
        ("//", TokenType::Comment),
        ("/", TokenType::ForwardSlash),
    ];
    table
        .into_iter()
        .find(|(pat, _)| pat[pos..].starts_with(chars))
        .map(|(pat, tok)| (pat.chars().count(), tok))
}

/// Single-character tokens. Letters, digits, `=` and `/` are intentionally
/// absent — they are handled by [`match_multi`] or [`parse_token`].
fn single_char(c: char) -> Option<TokenType> {
    use TokenType::*;
    Some(match c {
        ' ' => Space(1),
        '!' => Bang(1),
        ';' => Semicolon,
        '?' => QuestionMark,
        '(' => LeftParen,
        ')' => RightParen,
        '{' => LeftBrace,
        '}' => RightBrace,
        '[' => LeftBracket,
        ']' => RightBracket,
        '<' => LeftAngular,
        '>' => RightAngular,
        '\'' => Quote("'".to_string()),
        '"' => Quote("\"".to_string()),
        '+' => Plus,
        '-' => Minus,
        '*' => Asterisk,
        '^' => Caret,
        ',' => Comma,
        '.' => Dot,
        ':' => Colon,
        '\r' | '\n' => Newline,
        _ => return None,
    })
}

/// Read a run of digit-ish characters (`0-9`, `.`, `-`) at `pos` and parse it
/// as an `f64`. Returns the parse result together with the next position, so
/// the caller can always make progress even when the run doesn't parse.
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
///
/// Note: `const` / `var` are absent on purpose. They only exist as the paired
/// declaration forms (`const const`, `var var`, ...), which is a parser-level
/// concern.
fn keyword(word: &str) -> Option<TokenType> {
    use TokenType::*;
    Some(match word {
        "true" => True,
        "false" => False,
        "maybe" => Maybe,
        "when" => When,
        "use" => Use,
        "return" => Return,
        "previous" => Previous,
        "next" => Next,
        "current" => Current,
        "import" => Import,
        "export" => Export,
        "to" => To,
        // `className` is the JS-compat alias for `class` (SPECIFICATION.md).
        "class" | "className" => Class,
        "new" => New,
        "delete" => Delete,
        "async" => Async,
        "await" => Await,
        "noop" => Noop,
        "reverse" => Reverse,
        "Infinity" => Infinity,
        "undefined" => Undefined,
        // Type names. Annotations are no-ops per the spec but still tokenized.
        "Int" => IntT,
        "String" => StringT,
        "Char" => CharT,
        "Digit" => DigitT,
        "Int9" => Int9T,
        "Int99" => Int99T,
        "Regex" | "RegExp" | "RegularExpression" => RegexpT,
        _ => return None,
    })
}

/// Spellings of `function` that the spec shows explicitly.
fn is_function_keyword(word: &str) -> bool {
    const MIN_FUNCTION_KW_LEN: usize = 2;
    let re = Regex::new("f?u?n?c?t?i?o?n?").unwrap();

    word.len() > MIN_FUNCTION_KW_LEN && re.is_match(word)
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
    let chars: Vec<char> = source.chars().collect();
    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    let mut pos = 0;

    // True at the start of the file and just after each newline, so the next
    // space run can be treated as indentation rather than an inter-token gap.
    let mut at_line_start = true;

    while pos < chars.len() {
        // 0. leading indentation: coalesce the space run starting a line into a
        //    single Space(n) and check it is a multiple of INDENT_WIDTH. The
        //    relative +3-per-level / -3-outdent rules are left to the parser.
        if at_line_start {
            at_line_start = false;
            if chars[pos] == ' ' {
                let spaces = chars[pos..].iter().take_while(|&&c| c == ' ').count();
                let end = pos + spaces;

                // A run followed by a newline or EOF is a blank line, not an
                // indent, so it is coalesced but not checked.
                let blank_line = matches!(chars.get(end), None | Some(&('\n' | '\r')));
                if !blank_line && spaces % INDENT_WIDTH != 0 {
                    errors.push(LexerError {
                        src: NamedSource::new(source_name, source.to_string()),
                        span: byte_span(&chars, pos, pos + spaces),
                        hint: format!("not a multiple of {INDENT_WIDTH}"),
                        message: format!(
            "indentation must be a multiple of {INDENT_WIDTH} spaces (found {spaces})"
        ),
                        advice: Some(format!(
                            "Gulf of Mexico indents are {INDENT_WIDTH} spaces per level"
                        )),
                    });
                }
                tokens.push(TokenType::Space(u32::try_from(spaces).unwrap_or(u32::MAX)));
                pos = end;
                continue;
            }
        }

        // 1. multi-character operators
        if let Some((len, tok)) = match_multi(&chars, pos) {
            if tok == TokenType::Comment {
                // comments run to end-of-line and are dropped, leaving a newline
                match chars[pos..].iter().position(|&c| c == '\n') {
                    Some(offset) => {
                        tokens.push(TokenType::Newline);
                        at_line_start = true;
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

        // 2. single-character tokens
        if let Some(tok) = single_char(chars[pos]) {
            if tok == TokenType::Newline {
                at_line_start = true;
            }
            tokens.push(tok);
            pos += 1;
            continue;
        }

        // 3. numbers, keywords, identifiers
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
