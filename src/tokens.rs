//! Token definitions for the DreamBerd scanner.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // whitespace
    Space(u32), // run length; leading indentation is coalesced, inner gaps stay 1
    Newline,
    // basics
    Bang(i32), // end of statement, carries priority (count of `!` minus count of `¡`; may be negative)
    Semicolon, // the `not` operator
    QuestionMark, // debug operator
    // parens
    LeftParen,
    RightParen,
    // scopes
    LeftBrace,
    RightBrace,
    // arrays
    LeftBracket,
    RightBracket,
    // member access
    Dot, // `name.push(...)`; the parser decides what the access means
    // strings
    Quote(String), // quote type (single, double, any, etc.)
    // lifetimes
    LeftAngular,
    RightAngular,
    // booleans
    True,
    False,
    Maybe,
    // arithmetic
    Plus,
    Minus,
    Asterisk,
    ForwardSlash,
    Caret,
    IncrementOp,
    DecrementOp,
    // single-line comment
    Comment,
    // signals
    When,
    Use,
    // declarations
    ConstConst,      // cannot be changed in any way
    ConstVar,        // can be edited but not re-assigned
    VarConst,        // can be re-assigned but not edited
    VarVar,          // can be edited and re-assigned
    ConstConstConst, // constant, immutable, affects all users globally forever
    Assignment,
    // comparison
    MuchLooserCheck,  // "="
    LooseCheck,       // "=="
    PreciseCheck,     // "==="
    MorePreciseCheck, // "===="
    // files
    FileDelim, // 5+ equal signs
    // functions
    Function,
    Arrow, // function foo(a, b) => ...
    Return,
    Comma,
    // types
    Colon,
    IntT,
    StringT,
    CharT,
    DigitT,
    Int9T,
    Int99T,
    RegexpT,
    // prev/next
    Previous,
    Next,
    Current,
    // imports
    Import,
    Export,
    To, // export foo to "filename.db"
    // classes
    Class, // "class" or "className"
    New,
    // delete
    Delete,
    // async/await
    Async,
    Await,
    Noop,
    // reverse
    Reverse,
    // literals
    Identifier(String),
    Str(String),
    Float(f64),
    Infinity, // usable in lifetimes
    // misc
    Undefined, // like JS; unset dict keys return this; also 1/0
    // eof
    Eof,
    // Custom extensions
    Null,
    Range,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Space(n) => {
                let s = " ".repeat(*n as usize);
                f.write_str(&s)
            }
            TokenType::Newline => writeln!(f, "\n"),
            TokenType::Bang(n) => {
                // positive priority renders as `!`s, negative as `¡`s
                let (mark, count) = if *n >= 0 { ("!", *n) } else { ("¡", -*n) };
                f.write_str(&mark.repeat(count as usize))
            }
            TokenType::Semicolon => write!(f, ";"),
            TokenType::QuestionMark => write!(f, "?"),
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::LeftBracket => write!(f, "["),
            TokenType::RightBracket => write!(f, "]"),
            TokenType::Dot => write!(f, "."),
            TokenType::Quote(q) => write!(f, "Quote({q})"),
            TokenType::LeftAngular => write!(f, "<"),
            TokenType::RightAngular => write!(f, ">"),
            TokenType::True => write!(f, "true"),
            TokenType::False => write!(f, "false"),
            TokenType::Maybe => write!(f, "maybe"),
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Asterisk => write!(f, "*"),
            TokenType::ForwardSlash => write!(f, "/"),
            TokenType::Caret => write!(f, "^"),
            TokenType::IncrementOp => write!(f, "++"),
            TokenType::DecrementOp => write!(f, "--"),
            TokenType::Comment => write!(f, "COMMENT"),
            TokenType::When => write!(f, "when"),
            TokenType::Use => write!(f, "use"),
            TokenType::ConstConst => write!(f, "const const"),
            TokenType::ConstVar => write!(f, "const var"),
            TokenType::VarConst => write!(f, "var const"),
            TokenType::VarVar => write!(f, "var var"),
            TokenType::ConstConstConst => write!(f, "const const const"),
            TokenType::Assignment => write!(f, "="),
            TokenType::MuchLooserCheck => write!(f, "="),
            TokenType::LooseCheck => write!(f, "=="),
            TokenType::PreciseCheck => write!(f, "==="),
            TokenType::MorePreciseCheck => write!(f, "===="),
            TokenType::FileDelim => write!(f, "====="),
            TokenType::Function => write!(f, "function"),
            TokenType::Arrow => write!(f, "=>"),
            TokenType::Return => write!(f, "return"),
            TokenType::Comma => write!(f, ","),
            TokenType::Colon => write!(f, ":"),
            TokenType::IntT => write!(f, "Int"),
            TokenType::StringT => write!(f, "String"),
            TokenType::CharT => write!(f, "Char"),
            TokenType::DigitT => write!(f, "Digit"),
            TokenType::Int9T => write!(f, "Int9"),
            TokenType::Int99T => write!(f, "Int99"),
            TokenType::RegexpT => write!(f, "RegExp"),
            TokenType::Previous => write!(f, "previous"),
            TokenType::Next => write!(f, "next"),
            TokenType::Current => write!(f, "current"),
            TokenType::Import => write!(f, "import"),
            TokenType::Export => write!(f, "export"),
            TokenType::To => write!(f, "to"),
            TokenType::Class => write!(f, "class"),
            TokenType::New => write!(f, "new"),
            TokenType::Delete => write!(f, "delete"),
            TokenType::Async => write!(f, "async"),
            TokenType::Await => write!(f, "await"),
            TokenType::Noop => write!(f, "noop"),
            TokenType::Reverse => write!(f, "reverse"),
            TokenType::Identifier(s) => write!(f, "IDENTIFIER({s})"),
            TokenType::Str(s) => write!(f, "STRING({s})"),
            TokenType::Float(x) => write!(f, "{x}"),
            TokenType::Infinity => write!(f, "Infinity"),
            TokenType::Undefined => write!(f, "undefined"),
            TokenType::Eof => write!(f, "EOF"),
            TokenType::Null => write!(f, "null"),
            TokenType::Range => write!(f, "RANGE"),
        }
    }
}
