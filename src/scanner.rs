//! The DreamBerd scanner (lexer).

use std::str::FromStr;

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

/// Attempt to match one of the multi-character tokens at `pos`, in order.
///
/// `pos` is a *char* index into `chars`; the whole scanner works in char
/// indices so multi-byte input doesn't desync it.
///
/// # Arguments
///
/// * `chars` - A slice of chars to process
/// * `pos` - The position to start processing `chars` at
///
/// # Returns
///
/// A tuple (`len`, `tt`), where `len` is the length of the parsed token, and `tt` is the
/// [`TokenType`] that was just parsed. If the token at the given position is not identified as a
/// multi-char token, `None` is returned instead.
fn match_multi_char(chars: &[char], pos: usize) -> Option<(usize, TokenType)> {
    let rest = &chars[pos..];

    // File delimiters can be 5 or more = (with no bound), so we check that separately
    if rest.starts_with(&['=', '=', '=', '=', '=']) {
        let len = rest.iter().take_while(|&&c| c == '=').count();
        return Some((len, TokenType::FileDelim));
    }

    let table = [
        ("const const const", TokenType::ConstConstConst),
        ("const const", TokenType::ConstConst),
        ("const var", TokenType::ConstVar),
        ("var const", TokenType::VarConst),
        ("var var", TokenType::VarVar),
        ("====", TokenType::MorePreciseCheck),
        ("===", TokenType::PreciseCheck),
        ("==", TokenType::LooseCheck),
        ("//", TokenType::Comment),
        ("=>", TokenType::Arrow),
    ];
    table
        .into_iter()
        .find(|(pat, _)| {
            rest.iter()
                .copied()
                .take(pat.chars().count())
                .eq(pat.chars())
        })
        .map(|(pat, tok)| (pat.chars().count(), tok))
}

/// Match single-character tokens. Letters and digits are intentionally
/// absent: they are handled by [`parse_token`].
///
/// # Arguments
///
/// * `c` - The char to match
///
/// A [`TokenType`] if one matches, otherwise `None`.
fn match_single_char(c: char) -> Option<TokenType> {
    Some(match c {
        '=' => TokenType::Assignment,
        '/' => TokenType::ForwardSlash,
        ' ' => TokenType::Space(1),
        '!' => TokenType::Bang(1),
        '¡' => TokenType::Bang(-1),
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

/// Match single-word keywords.
///
/// # Arguments
///
/// * `word` - A slice containing one word; this is not enforced
///
/// # Returns
///
/// A [`TokenType`], if a reserved keyword token matches; otherwise `None`.
fn get_keyword_token(word: &str) -> Option<TokenType> {
    Some(match word {
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

/// Match whether the word is a keyword defining a function.
///
/// # Arguments
///
/// * `word` - The slice to match
///
/// # Returns
///
/// `true` if this is a function keyword.
fn is_function_keyword(word: &str) -> bool {
    const MIN_FUNCTION_KW_LEN: usize = 2;
    let re = Regex::new("f?u?n?c?t?i?o?n?").unwrap();

    word.len() >= MIN_FUNCTION_KW_LEN && re.find(word).is_some_and(|m| m.len() == word.len())
}

/// Parse a number from a string that is either a hex value, oct value, or a valid number.
fn parse_number<T: FromStr + From<i32>>(s: &str) -> Option<T> {
    if s.starts_with("0x") {
        // TODO: We don't distinguish between int and float yet
        i32::from_str_radix(&s[2..], 16).map(|i| i.into()).ok()
    } else if s.starts_with("0o") {
        i32::from_str_radix(&s[2..], 8).map(|i| i.into()).ok()
    } else {
        s.parse::<T>().ok()
    }
}

/// Scan a number, keyword, or identifier at `pos`, assuming [`match_multi_char`] and
/// [`match_single_char`] have already been tried. Anything else becomes an
/// [`TokenType::Identifier`] too: per [Deviations], there are no invalid tokens.
/// On failure (malformed numbers only) returns the position to resume at
/// (always strictly greater than `pos`, guaranteeing progress).
///
/// [Deviations]: ../docs/DEVIATIONS.md
///
/// # Arguments
///
/// * `chars` - A slice of chars to parse
/// * `pos` - A position to begin parsing `chars` from
///
/// # Returns
///
/// A tuple (`len`, `tt`), where `len` is the length of the parsed token, and `tt` is the
/// [`TokenType`] that was just parsed.
fn parse_token(chars: &[char], pos: usize) -> Result<(usize, TokenType), usize> {
    let c = chars[pos];

    if c.is_ascii_digit() {
        let end = chars[pos..]
            .iter()
            .take_while(|&&c| c.is_ascii_hexdigit() || ['.', '-', 'x', 'o'].contains(&c))
            .count();
        let s: String = chars[pos..pos + end].iter().collect();

        // Special case: ranges
        if let Some(idx) = s.find("..")
            && let Some(rstart) = parse_number::<i32>(&s[..idx])
            && let Some(rend) = parse_number::<i32>(&s[idx + 2..])
        {
            return Ok((end, TokenType::Range(rstart as i64, rend as i64)));
        }

        return match parse_number::<f64>(&s) {
            Some(value) => Ok((end, TokenType::Float(value))),
            None => Err(end), // skip the malformed numeric run
        };
    }

    if c.is_alphabetic() || c == '_' {
        let end = chars[pos..]
            .iter()
            .take_while(|&&c| c.is_alphanumeric() || c == '_')
            .count();
        let word: String = chars[pos..pos + end].iter().collect();
        let tok = if let Some(kw) = get_keyword_token(&word) {
            kw
        } else if is_function_keyword(&word) {
            TokenType::Function
        } else {
            TokenType::Identifier(word)
        };
        return Ok((end, tok));
    }

    // There is no such thing as an invalid token. Unknown input
    // becomes an identifier; the parser decides whether it names a variable
    // or forms part of a zero-quote string (undeclared identifiers are
    // zero-quote strings per the spec).
    Ok((1, TokenType::Identifier(c.to_string())))
}

/// Byte span `(offset, length)` covering the char range `[start, end)`.
///
/// The scanner works in char indices (so Unicode identifiers don't desync it),
/// but miette spans are byte offsets, so we sum the UTF-8 widths to convert.
///
/// # Arguments
///
/// * `chars` - A slice of chars
/// * `start` - Inclusive start index
/// * `end` - Exclusive end index
///
/// # Returns
///
/// A [`SourceSpan`] containing the offset and length in bytes
fn byte_span(chars: &[char], start: usize, end: usize) -> SourceSpan {
    let offset: usize = chars[..start].iter().map(|c| c.len_utf8()).sum();
    let len: usize = chars[start..end].iter().map(|c| c.len_utf8()).sum();
    (offset, len).into()
}

/// Given a stream of tokens, collapse those for which it makes sense.
///
/// For example, two `SPACE(1)` tokens are merged into one `SPACE(2)` token.
///
/// # Arguments
///
/// * `tokens` - A slice of parsed tokens
///
/// # Returns
///
/// An owned [`Vec<TokenType>`].
fn collapse_tokens(tokens: &[TokenType]) -> Vec<TokenType> {
    let mut i = 0;
    let mut collapsed = Vec::new();

    while i < tokens.len() {
        match tokens[i] {
            TokenType::Space(m) => {
                let mut total_spaces = m;
                let mut j = i + 1;

                while j < tokens.len()
                    && let TokenType::Space(n) = tokens[j]
                {
                    total_spaces += n;
                    j += 1;
                }
                collapsed.push(TokenType::Space(total_spaces));

                i = j;
            }
            TokenType::Bang(m) => {
                let mut total_bangs = m;
                let mut j = i + 1;

                while j < tokens.len()
                    && let TokenType::Bang(n) = tokens[j]
                {
                    total_bangs += n;
                    j += 1;
                }
                collapsed.push(TokenType::Bang(total_bangs));

                i = j;
            }
            TokenType::Quote(ref t) => {
                let mut quote_type = t.clone();
                let mut j = i + 1;

                while j < tokens.len()
                    && let TokenType::Quote(ref u) = tokens[j]
                {
                    quote_type.push_str(&u);
                    j += 1;
                }
                collapsed.push(TokenType::Quote(quote_type));

                i = j;
            }
            TokenType::Newline => {
                collapsed.push(TokenType::Newline);
                let mut j = i + 1;

                while j < tokens.len()
                    && let TokenType::Newline = tokens[j]
                {
                    j += 1;
                }

                i = j;
            }
            ref tok => {
                collapsed.push(tok.clone());
                i += 1;
            }
        }
    }

    collapsed
}

/// Scan `source` into a token stream terminated by [`TokenType::Eof`].
///
/// `source_name` labels the source in any [`LexerError`]s produced (e.g. the
/// file path), so diagnostics can point back at the right file.
///
/// # Arguments
///
/// * `source` - The source string to parse into tokens
/// * `source_name` - The name of the source being parsed, such as a file name, used in errors
///
/// # Returns
///
/// A [`ScanResult`] containing the tokens and any errors encountered.
pub fn scan_tokens(source: &str, source_name: &str) -> ScanResult {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    let mut pos = 0;
    // The scanner works in char indices; multi-byte input must not desync it.
    let chars: Vec<char> = source.chars().collect();

    while pos < chars.len() {
        // multi-character operators
        if let Some((len, tok)) = match_multi_char(&chars, pos) {
            if tok == TokenType::Comment {
                // comments run to end-of-line and are dropped, leaving a newline
                match chars[pos..].iter().position(|&c| c == '\n') {
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
        if let Some(tok) = match_single_char(chars[pos]) {
            tokens.push(tok);
            pos += 1;
            continue;
        }

        // numbers, keywords, identifiers
        match parse_token(&chars, pos) {
            Ok((tok, next)) => {
                tokens.push(tok);
                pos += next;
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
                pos += next;
            }
        }
    }

    tokens.push(TokenType::Eof);

    ScanResult {
        tokens: collapse_tokens(&tokens),
        errors,
    }
}
