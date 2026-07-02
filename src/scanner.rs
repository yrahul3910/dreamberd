//! The DreamBerd scanner (lexer).
//!
//! Ported from `lib/scanner.ml`, with the previously-stubbed identifier /
//! keyword branch filled in. The pipeline per position is unchanged from the
//! OCaml original: try multi-char operators, then single-char tokens, then
//! numbers / keywords / identifiers.
//!
//! Two deliberate departures from the OCaml version:
//!   * `scan_tokens` returns a [`ScanResult`] carrying both the tokens and the
//!     positions of scan errors. The OCaml threaded an `errs` list through the
//!     recursion but discarded it at the base case.
//!   * The source is scanned as a `Vec<char>`, so positions are char indices
//!     (not bytes). This keeps the Unicode identifiers the spec allows from
//!     tripping over UTF-8 byte boundaries.

use crate::tokens::TokenType;

/// Result of scanning a source string.
#[derive(Debug, Default, PartialEq)]
pub struct ScanResult {
    pub tokens: Vec<TokenType>,
    /// Char positions at which no token could be recognised.
    pub errors: Vec<usize>,
}

/// Does `chars` contain `pat` (compared char-by-char) starting at `pos`?
fn starts_with(chars: &[char], pos: usize, pat: &str) -> bool {
    pat.chars()
        .enumerate()
        .all(|(i, pc)| chars.get(pos + i) == Some(&pc))
}

/// Attempt to match one of the multi-character operators at `pos`, in order.
///
/// The `=` family is listed longest-first so `====` wins over `==`, and `//`
/// precedes `/` so comments aren't mistaken for division.
fn try_match(chars: &[char], pos: usize) -> Option<(usize, TokenType)> {
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
        .find(|(pat, _)| starts_with(chars, pos, pat))
        .map(|(pat, tok)| (pat.chars().count(), tok))
}

/// Single-character tokens. Letters, digits, `=` and `/` are intentionally
/// absent — they are handled by [`try_match`] or [`parse_token`].
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
        ':' => Colon,
        '\r' | '\n' => Newline,
        _ => return None,
    })
}

/// Read a run of digit-ish characters (`0-9`, `.`, `-`) at `pos` and parse it
/// as an `f64`. Returns the parse result together with the next position, so
/// the caller can always make progress even when the run doesn't parse.
///
/// (Faithful to the OCaml `parse_digit`; the accepted char set is loose and
/// will happily consume e.g. `1-2` into a single failing run — a refinement
/// worth revisiting once numbers are actually evaluated.)
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
/// concern — the scanner emits them as identifiers for now.
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
///
/// The full rule is "any subsequence of the letters of `function`, in order"
/// (so even `f` counts). That's ambiguous with ordinary identifiers — `f`,
/// `in`, `on`, `no` are all subsequences — so the general rule is left to the
/// parser, which has the context to disambiguate. See SPECIFICATION.md >
/// Functions. Single-letter `f` is excluded here for the same reason.
fn is_function_keyword(word: &str) -> bool {
    matches!(
        word,
        "function" | "functio" | "functi" | "funct" | "func" | "fun" | "fn"
    )
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

/// Scan `source` into a token stream terminated by [`TokenType::Eof`].
pub fn scan_tokens(source: &str) -> ScanResult {
    let chars: Vec<char> = source.chars().collect();
    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    let mut pos = 0;

    while pos < chars.len() {
        // 1. multi-character operators
        if let Some((len, tok)) = try_match(&chars, pos) {
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

        // 2. single-character tokens
        if let Some(tok) = single_char(chars[pos]) {
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
                errors.push(pos);
                pos = next;
            }
        }
    }

    tokens.push(TokenType::Eof);
    ScanResult { tokens, errors }
}
