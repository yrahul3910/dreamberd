//! Token definitions for the DreamBerd scanner.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // whitespace
    Space(u32), // run length; leading indentation is coalesced, inner gaps stay 1
    Newline,
    // basics
    Bang(u32),    // end of statement, carries priority (count of `!` - count of `¡`)
    Semicolon,    // the `not` operator
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
    Integer(i64),
    Float(f64),
    Infinity, // usable in lifetimes
    // misc
    Undefined, // like JS; unset dict keys return this; also 1/0
    // eof
    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenType::*;
        match self {
            Space(n) => write!(f, "SPACE({n})"),
            Newline => writeln!(f),
            Bang(n) => write!(f, "BANG({n})"),
            Semicolon => write!(f, "SEMICOLON"),
            QuestionMark => write!(f, "QUESTION_MARK"),
            LeftParen => write!(f, "LEFT_PAREN"),
            RightParen => write!(f, "RIGHT_PAREN"),
            LeftBrace => write!(f, "LEFT_BRACE"),
            RightBrace => write!(f, "RIGHT_BRACE"),
            LeftBracket => write!(f, "LEFT_BRACKET"),
            RightBracket => write!(f, "RIGHT_BRACKET"),
            Dot => write!(f, "DOT"),
            Quote(q) => write!(f, "QUOTE({q})"),
            LeftAngular => write!(f, "LEFT_ANGULAR"),
            RightAngular => write!(f, "RIGHT_ANGULAR"),
            True => write!(f, "TRUE"),
            False => write!(f, "FALSE"),
            Maybe => write!(f, "MAYBE"),
            Plus => write!(f, "PLUS"),
            Minus => write!(f, "MINUS"),
            Asterisk => write!(f, "ASTERISK"),
            ForwardSlash => write!(f, "FORWARD_SLASH"),
            Caret => write!(f, "CARET"),
            IncrementOp => write!(f, "INCREMENT_OP"),
            Comment => write!(f, "COMMENT"),
            When => write!(f, "WHEN"),
            Use => write!(f, "USE"),
            ConstConst => write!(f, "CONST_CONST"),
            ConstVar => write!(f, "CONST_VAR"),
            VarConst => write!(f, "VAR_CONST"),
            VarVar => write!(f, "VAR_VAR"),
            ConstConstConst => write!(f, "CONST_CONST_CONST"),
            Assignment => write!(f, "ASSIGNMENT"),
            MuchLooserCheck => write!(f, "MUCH_LOOSER_CHECK"),
            LooseCheck => write!(f, "LOOSE_CHECK"),
            PreciseCheck => write!(f, "PRECISE_CHECK"),
            MorePreciseCheck => write!(f, "MORE_PRECISE_CHECK"),
            FileDelim => write!(f, "FILE_DELIM"),
            Function => write!(f, "FUNCTION"),
            Arrow => write!(f, "ARROW"),
            Return => write!(f, "RETURN"),
            Comma => write!(f, "COMMA"),
            Colon => write!(f, "COLON"),
            IntT => write!(f, "INT_T"),
            StringT => write!(f, "STRING_T"),
            CharT => write!(f, "CHAR_T"),
            DigitT => write!(f, "DIGIT_T"),
            Int9T => write!(f, "INT9_T"),
            Int99T => write!(f, "INT99_T"),
            RegexpT => write!(f, "REGEXP_T"),
            Previous => write!(f, "PREVIOUS"),
            Next => write!(f, "NEXT"),
            Current => write!(f, "CURRENT"),
            Import => write!(f, "IMPORT"),
            Export => write!(f, "EXPORT"),
            To => write!(f, "TO"),
            Class => write!(f, "CLASS"),
            New => write!(f, "NEW"),
            Delete => write!(f, "DELETE"),
            Async => write!(f, "ASYNC"),
            Await => write!(f, "AWAIT"),
            Noop => write!(f, "NOOP"),
            Reverse => write!(f, "REVERSE"),
            Identifier(s) => write!(f, "IDENTIFIER({s})"),
            Str(s) => write!(f, "STRING({s})"),
            Integer(n) => write!(f, "INTEGER({n})"),
            Float(x) => write!(f, "FLOAT({x})"),
            Infinity => write!(f, "INFINITY"),
            Undefined => write!(f, "UNDEFINED"),
            Eof => write!(f, "EOF"),
        }
    }
}
