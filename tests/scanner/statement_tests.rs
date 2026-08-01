//! Example statements from docs/SPECIFICATION.md (grouped by spec section),
//! pinned to the exact token stream the scanner should produce. One assert
//! per statement.

use crate::common::{assert_scans, assert_tokens};
use dreamberd::tokens::TokenType::*;

#[test]
fn exclamation_marks() {
    assert_tokens!(
        r#"print("Hello world")!"#,
        vec![
            Identifier("print".into()),
            LeftParen,
            Quote("\"".into()),
            Identifier("Hello".into()),
            Space(1),
            Identifier("world".into()),
            Quote("\"".into()),
            RightParen,
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        r#"print("Hello world")!!!"#,
        vec![
            Identifier("print".into()),
            LeftParen,
            Quote("\"".into()),
            Identifier("Hello".into()),
            Space(1),
            Identifier("world".into()),
            Quote("\"".into()),
            RightParen,
            Bang(3),
            Eof,
        ]
    );
    assert_tokens!(
        r#"print("Hello world")?"#,
        vec![
            Identifier("print".into()),
            LeftParen,
            Quote("\"".into()),
            Identifier("Hello".into()),
            Space(1),
            Identifier("world".into()),
            Quote("\"".into()),
            RightParen,
            QuestionMark,
            Eof,
        ]
    );
    // `if` isn't a keyword; `;` is the `not` operator.
    assert_tokens!(
        "if (;false) {",
        vec![
            Identifier("if".into()),
            Space(1),
            LeftParen,
            Semicolon,
            False,
            RightParen,
            Space(1),
            LeftBrace,
            Eof,
        ]
    );
}

#[test]
// The point is the literal source text, not the constant.
#[allow(clippy::approx_constant)]
fn declarations() {
    assert_tokens!(
        r#"const const name = "Luke"!"#,
        vec![
            ConstConst,
            Space(1),
            Identifier("name".into()),
            Space(1),
            Assignment,
            Space(1),
            Quote("\"".into()),
            Identifier("Luke".into()),
            Quote("\"".into()),
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "const var name = \"Luke\"!",
        vec![
            ConstVar,
            Space(1),
            Identifier("name".into()),
            Space(1),
            Assignment,
            Space(1),
            Quote("\"".into()),
            Identifier("Luke".into()),
            Quote("\"".into()),
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "name.pop()!",
        vec![
            Identifier("name".into()),
            Dot,
            Identifier("pop".into()),
            LeftParen,
            RightParen,
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "var const name = \"Luke\"!",
        vec![
            VarConst,
            Space(1),
            Identifier("name".into()),
            Space(1),
            Assignment,
            Space(1),
            Quote("\"".into()),
            Identifier("Luke".into()),
            Quote("\"".into()),
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "name = \"Lu\"!",
        vec![
            Identifier("name".into()),
            Space(1),
            Assignment,
            Space(1),
            Quote("\"".into()),
            Identifier("Lu".into()),
            Quote("\"".into()),
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "name.push(\"k\")!",
        vec![
            Identifier("name".into()),
            Dot,
            Identifier("push".into()),
            LeftParen,
            Quote("\"".into()),
            Identifier("k".into()),
            Quote("\"".into()),
            RightParen,
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "const const const pi = 3.14!",
        vec![
            ConstConstConst,
            Space(1),
            Identifier("pi".into()),
            Space(1),
            Assignment,
            Space(1),
            Float(3.14),
            Bang(1),
            Eof,
        ]
    );
}

#[test]
fn naming_allows_any_unicode_or_number() {
    assert_tokens!(
        "const const letter = 'A'!",
        vec![
            ConstConst,
            Space(1),
            Identifier("letter".into()),
            Space(1),
            Assignment,
            Space(1),
            Quote("'".into()),
            Identifier("A".into()),
            Quote("'".into()),
            Bang(1),
            Eof,
        ]
    );
    // NB: spec writes `True`; scans as an identifier since keywords are case-sensitive.
    assert_tokens!(
        "var const 👍 = True!",
        vec![
            VarConst,
            Space(1),
            Identifier("👍".into()),
            Space(1),
            Assignment,
            Space(1),
            Identifier("True".into()),
            Bang(1),
            Eof,
        ]
    );
    // U+1F1E3 is a digit followed by two combining chars: the `1` scans as a
    // number, the combining chars become (zero-quote-string) identifiers.
    assert_tokens!(
        "var var 1\u{fe0f}\u{20e3} = 1!",
        vec![
            VarVar,
            Space(1),
            Float(1.0),
            Identifier("\u{fe0f}".into()),
            Identifier("\u{20e3}".into()),
            Space(1),
            Assignment,
            Space(1),
            Float(1.0),
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "const const 5 = 4!",
        vec![
            ConstConst,
            Space(1),
            Float(5.0),
            Space(1),
            Assignment,
            Space(1),
            Float(4.0),
            Bang(1),
            Eof,
        ]
    );
}

#[test]
fn arrays() {
    assert_tokens!(
        "const const scores = [3, 2, 5]!",
        vec![
            ConstConst,
            Space(1),
            Identifier("scores".into()),
            Space(1),
            Assignment,
            Space(1),
            LeftBracket,
            Float(3.0),
            Comma,
            Space(1),
            Float(2.0),
            Comma,
            Space(1),
            Float(5.0),
            RightBracket,
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "print(scores[-1])!",
        vec![
            Identifier("print".into()),
            LeftParen,
            Identifier("scores".into()),
            LeftBracket,
            Minus,
            Float(1.0),
            RightBracket,
            RightParen,
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "scores[0.5] = 4!",
        vec![
            Identifier("scores".into()),
            LeftBracket,
            Float(0.5),
            RightBracket,
            Space(1),
            Assignment,
            Space(1),
            Float(4.0),
            Bang(1),
            Eof,
        ]
    );
}

#[test]
fn when_blocks() {
    assert_tokens!(
        "const var health = 10!",
        vec![
            ConstVar,
            Space(1),
            Identifier("health".into()),
            Space(1),
            Assignment,
            Space(1),
            Float(10.0),
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "when (health = 0) {",
        vec![
            When,
            Space(1),
            LeftParen,
            Identifier("health".into()),
            Space(1),
            Assignment,
            Space(1),
            Float(0.0),
            RightParen,
            Space(1),
            LeftBrace,
            Eof,
        ]
    );
}

#[test]
fn lifetimes() {
    assert_tokens!(
        "const const name<2> = \"Luke\"!",
        vec![
            ConstConst,
            Space(1),
            Identifier("name".into()),
            LeftAngular,
            Float(2.0),
            RightAngular,
            Space(1),
            Assignment,
            Space(1),
            Quote("\"".into()),
            Identifier("Luke".into()),
            Quote("\"".into()),
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "const const name<20s> = \"Luke\"!",
        vec![
            ConstConst,
            Space(1),
            Identifier("name".into()),
            LeftAngular,
            Float(20.0),
            Identifier("s".into()),
            RightAngular,
            Space(1),
            Assignment,
            Space(1),
            Quote("\"".into()),
            Identifier("Luke".into()),
            Quote("\"".into()),
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "const const name<Infinity> = \"Luke\"!",
        vec![
            ConstConst,
            Space(1),
            Identifier("name".into()),
            LeftAngular,
            Infinity,
            RightAngular,
            Space(1),
            Assignment,
            Space(1),
            Quote("\"".into()),
            Identifier("Luke".into()),
            Quote("\"".into()),
            Bang(1),
            Eof,
        ]
    );
    // Trailing comment is dropped, but the space before it is not.
    assert_tokens!(
        "print(name)! //Luke",
        vec![
            Identifier("print".into()),
            LeftParen,
            Identifier("name".into()),
            RightParen,
            Bang(1),
            Space(1),
            Eof,
        ]
    );
    assert_tokens!(
        "const const name<-1> = \"Luke\"!",
        vec![
            ConstConst,
            Space(1),
            Identifier("name".into()),
            LeftAngular,
            Minus,
            Float(1.0),
            RightAngular,
            Space(1),
            Assignment,
            Space(1),
            Quote("\"".into()),
            Identifier("Luke".into()),
            Quote("\"".into()),
            Bang(1),
            Eof,
        ]
    );
}

#[test]
fn booleans() {
    assert_tokens!(
        "const var keys = {}!",
        vec![
            ConstVar,
            Space(1),
            Identifier("keys".into()),
            Space(1),
            Assignment,
            Space(1),
            LeftBrace,
            RightBrace,
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "addEventListener(\"keydown\", (e) => keys[e.key] = true)!",
        vec![
            Identifier("addEventListener".into()),
            LeftParen,
            Quote("\"".into()),
            Identifier("keydown".into()),
            Quote("\"".into()),
            Comma,
            Space(1),
            LeftParen,
            Identifier("e".into()),
            RightParen,
            Space(1),
            Arrow,
            Space(1),
            Identifier("keys".into()),
            LeftBracket,
            Identifier("e".into()),
            Dot,
            Identifier("key".into()),
            RightBracket,
            Space(1),
            Assignment,
            Space(1),
            True,
            RightParen,
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "function isKeyDown(key) => {",
        vec![
            Function,
            Space(1),
            Identifier("isKeyDown".into()),
            LeftParen,
            Identifier("key".into()),
            RightParen,
            Space(1),
            Arrow,
            Space(1),
            LeftBrace,
            Eof,
        ]
    );
    assert_tokens!(
        "   if (keys[key] = undefined) {",
        vec![
            Space(3),
            Identifier("if".into()),
            Space(1),
            LeftParen,
            Identifier("keys".into()),
            LeftBracket,
            Identifier("key".into()),
            RightBracket,
            Space(1),
            Assignment,
            Space(1),
            Undefined,
            RightParen,
            Space(1),
            LeftBrace,
            Eof,
        ]
    );
    assert_tokens!(
        "      return maybe!",
        vec![Space(6), Return, Space(1), Maybe, Bang(1), Eof]
    );
    assert_tokens!(
        "   return keys[key]!",
        vec![
            Space(3),
            Return,
            Space(1),
            Identifier("keys".into()),
            LeftBracket,
            Identifier("key".into()),
            RightBracket,
            Bang(1),
            Eof,
        ]
    );
}

#[test]
fn arithmetic() {
    assert_tokens!(
        "print(1 + 2*3)!",
        vec![
            Identifier("print".into()),
            LeftParen,
            Float(1.0),
            Space(1),
            Plus,
            Space(1),
            Float(2.0),
            Asterisk,
            Float(3.0),
            RightParen,
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "print(1+2 * 3)!",
        vec![
            Identifier("print".into()),
            LeftParen,
            Float(1.0),
            Plus,
            Float(2.0),
            Space(1),
            Asterisk,
            Space(1),
            Float(3.0),
            RightParen,
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "const const half = 1/2!",
        vec![
            ConstConst,
            Space(1),
            Identifier("half".into()),
            Space(1),
            Assignment,
            Space(1),
            Float(1.0),
            ForwardSlash,
            Float(2.0),
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "print(one + two)!",
        vec![
            Identifier("print".into()),
            LeftParen,
            Identifier("one".into()),
            Space(1),
            Plus,
            Space(1),
            Identifier("two".into()),
            RightParen,
            Bang(1),
            Eof,
        ]
    );
}

// SPEC: indents are 3 spaces; -3 spaces is also allowed.
#[test]
fn indents_of_three_spaces() {
    assert_tokens!(
        "function main() => {",
        vec![
            Function,
            Space(1),
            Identifier("main".into()),
            LeftParen,
            RightParen,
            Space(1),
            Arrow,
            Space(1),
            LeftBrace,
            Eof,
        ]
    );
    assert_tokens!(
        "   print(\"Gulf of Mexico is the future\")!",
        vec![
            Space(3),
            Identifier("print".into()),
            LeftParen,
            Quote("\"".into()),
            Identifier("Gulf".into()),
            Space(1),
            Identifier("of".into()),
            Space(1),
            Identifier("Mexico".into()),
            Space(1),
            Identifier("is".into()),
            Space(1),
            Identifier("the".into()),
            Space(1),
            Identifier("future".into()),
            Quote("\"".into()),
            RightParen,
            Bang(1),
            Eof,
        ]
    );
    // -3 spaces: the enclosing line is indented, the body is not.
    assert_tokens!(
        "   function main() => {",
        vec![
            Space(3),
            Function,
            Space(1),
            Identifier("main".into()),
            LeftParen,
            RightParen,
            Space(1),
            Arrow,
            Space(1),
            LeftBrace,
            Eof,
        ]
    );
}

#[test]
// The point is the literal source text, not the constant.
#[allow(clippy::approx_constant)]
fn equality() {
    assert_tokens!(
        "3.14 == \"3.14\"!",
        vec![
            Float(3.14),
            Space(1),
            LooseCheck,
            Space(1),
            Quote("\"".into()),
            Float(3.14),
            Quote("\"".into()),
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "3.14 === \"3.14\"!",
        vec![
            Float(3.14),
            Space(1),
            PreciseCheck,
            Space(1),
            Quote("\"".into()),
            Float(3.14),
            Quote("\"".into()),
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "print(pi ==== pi)!",
        vec![
            Identifier("print".into()),
            LeftParen,
            Identifier("pi".into()),
            Space(1),
            MorePreciseCheck,
            Space(1),
            Identifier("pi".into()),
            RightParen,
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "3.14 ==== 3.14!",
        vec![
            Float(3.14),
            Space(1),
            MorePreciseCheck,
            Space(1),
            Float(3.14),
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "3 = 3.14!",
        vec![Float(3.0), Space(1), Assignment, Space(1), Float(3.14), Bang(1), Eof]
    );
}

#[test]
fn functions() {
    assert_tokens!(
        "function add(a, b) => a + b!",
        vec![
            Function,
            Space(1),
            Identifier("add".into()),
            LeftParen,
            Identifier("a".into()),
            Comma,
            Space(1),
            Identifier("b".into()),
            RightParen,
            Space(1),
            Arrow,
            Space(1),
            Identifier("a".into()),
            Space(1),
            Plus,
            Space(1),
            Identifier("b".into()),
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "func multiply(a, b) => a * b!",
        vec![
            Function,
            Space(1),
            Identifier("multiply".into()),
            LeftParen,
            Identifier("a".into()),
            Comma,
            Space(1),
            Identifier("b".into()),
            RightParen,
            Space(1),
            Arrow,
            Space(1),
            Identifier("a".into()),
            Space(1),
            Asterisk,
            Space(1),
            Identifier("b".into()),
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "fun subtract(a, b) => a - b!",
        vec![
            Function,
            Space(1),
            Identifier("subtract".into()),
            LeftParen,
            Identifier("a".into()),
            Comma,
            Space(1),
            Identifier("b".into()),
            RightParen,
            Space(1),
            Arrow,
            Space(1),
            Identifier("a".into()),
            Space(1),
            Minus,
            Space(1),
            Identifier("b".into()),
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "fn divide(a, b) => a / b!",
        vec![
            Function,
            Space(1),
            Identifier("divide".into()),
            LeftParen,
            Identifier("a".into()),
            Comma,
            Space(1),
            Identifier("b".into()),
            RightParen,
            Space(1),
            Arrow,
            Space(1),
            Identifier("a".into()),
            Space(1),
            ForwardSlash,
            Space(1),
            Identifier("b".into()),
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "functi power(a, b) => a ^ b!",
        vec![
            Function,
            Space(1),
            Identifier("power".into()),
            LeftParen,
            Identifier("a".into()),
            Comma,
            Space(1),
            Identifier("b".into()),
            RightParen,
            Space(1),
            Arrow,
            Space(1),
            Identifier("a".into()),
            Space(1),
            Caret,
            Space(1),
            Identifier("b".into()),
            Bang(1),
            Eof,
        ]
    );
    // SPEC: `f` is a valid function keyword ("any letters from `function` in
    // order"), but it currently lexes as an identifier; see the basic syntax
    // tests. Pinning the deviation until that is fixed.
    assert_tokens!(
        "f inverse(a) => 1/a!",
        vec![
            Identifier("f".into()),
            Space(1),
            Identifier("inverse".into()),
            LeftParen,
            Identifier("a".into()),
            RightParen,
            Space(1),
            Arrow,
            Space(1),
            Float(1.0),
            ForwardSlash,
            Identifier("a".into()),
            Bang(1),
            Eof,
        ]
    );
}

#[test]
fn dividing_by_zero() {
    assert_tokens!(
        "print(3 / 0)!",
        vec![
            Identifier("print".into()),
            LeftParen,
            Float(3.0),
            Space(1),
            ForwardSlash,
            Space(1),
            Float(0.0),
            RightParen,
            Bang(1),
            Eof,
        ]
    );
}

#[test]
fn strings_with_any_number_of_quotes() {
    assert_tokens!(
        "const const name = 'Lu'!",
        vec![
            ConstConst,
            Space(1),
            Identifier("name".into()),
            Space(1),
            Assignment,
            Space(1),
            Quote("'".into()),
            Identifier("Lu".into()),
            Quote("'".into()),
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "const const name = '''Lu'''!",
        vec![
            ConstConst,
            Space(1),
            Identifier("name".into()),
            Space(1),
            Assignment,
            Space(1),
            Quote("'''".into()),
            Identifier("Lu".into()),
            Quote("'''".into()),
            Bang(1),
            Eof,
        ]
    );
    // Mixed quote types within a run coalesce too; matching them up (by
    // reverse run) is the parser's job (DEVIATIONS).
    assert_tokens!(
        "const const name = \"'Lu'\"!",
        vec![
            ConstConst,
            Space(1),
            Identifier("name".into()),
            Space(1),
            Assignment,
            Space(1),
            Quote("\"'".into()),
            Identifier("Lu".into()),
            Quote("'\"".into()),
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "const const name = \"\"\"\"Luke\"\"\"\"!",
        vec![
            ConstConst,
            Space(1),
            Identifier("name".into()),
            Space(1),
            Assignment,
            Space(1),
            Quote("\"\"\"\"".into()),
            Identifier("Luke".into()),
            Quote("\"\"\"\"".into()),
            Bang(1),
            Eof,
        ]
    );
    // Zero quotes at all: the bare identifier is a zero-quote string's start.
    assert_tokens!(
        "const const name = Luke!",
        vec![
            ConstConst,
            Space(1),
            Identifier("name".into()),
            Space(1),
            Assignment,
            Space(1),
            Identifier("Luke".into()),
            Bang(1),
            Eof,
        ]
    );
}

#[test]
// Known limitation: the scanner doesn't track string state, so the `//`
// inside the quotes lexes as a comment and swallows the rest of the line.
// Expected below is the *spec* behavior; this test fails until the scanner
// becomes string-aware (see DEVIATIONS, Unimplemented features).
fn comment_marker_inside_a_string_stays_string_content() {
    assert_tokens!(
        r#"print("a // b")!"#,
        vec![
            Identifier("print".into()),
            LeftParen,
            Quote("\"".into()),
            Identifier("a".into()),
            Space(1),
            ForwardSlash,
            ForwardSlash,
            Space(1),
            Identifier("b".into()),
            Quote("\"".into()),
            RightParen,
            Bang(1),
            Eof,
        ]
    );
}

#[test]
fn string_interpolation_with_regional_currency() {
    assert_tokens!(
        "print(\"Hello ${name}!\")!",
        vec![
            Identifier("print".into()),
            LeftParen,
            Quote("\"".into()),
            Identifier("Hello".into()),
            Space(1),
            Identifier("$".into()),
            LeftBrace,
            Identifier("name".into()),
            RightBrace,
            Bang(1),
            Quote("\"".into()),
            RightParen,
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "print(\"Hello \u{a3}{name}!\")!",
        vec![
            Identifier("print".into()),
            LeftParen,
            Quote("\"".into()),
            Identifier("Hello".into()),
            Space(1),
            Identifier("\u{a3}".into()),
            LeftBrace,
            Identifier("name".into()),
            RightBrace,
            Bang(1),
            Quote("\"".into()),
            RightParen,
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "print(\"Hello {name}\u{20ac}!\")!",
        vec![
            Identifier("print".into()),
            LeftParen,
            Quote("\"".into()),
            Identifier("Hello".into()),
            Space(1),
            LeftBrace,
            Identifier("name".into()),
            RightBrace,
            Identifier("\u{20ac}".into()),
            Bang(1),
            Quote("\"".into()),
            RightParen,
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "print(\"Hello {player$name}!\")!",
        vec![
            Identifier("print".into()),
            LeftParen,
            Quote("\"".into()),
            Identifier("Hello".into()),
            Space(1),
            LeftBrace,
            Identifier("player".into()),
            Identifier("$".into()),
            Identifier("name".into()),
            RightBrace,
            Bang(1),
            Quote("\"".into()),
            RightParen,
            Bang(1),
            Eof,
        ]
    );
}

#[test]
fn types() {
    assert_tokens!(
        "const var age: Int = 28!",
        vec![
            ConstVar,
            Space(1),
            Identifier("age".into()),
            Colon,
            Space(1),
            IntT,
            Space(1),
            Assignment,
            Space(1),
            Float(28.0),
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "String == Char[]!",
        vec![
            StringT,
            Space(1),
            LooseCheck,
            Space(1),
            CharT,
            LeftBracket,
            RightBracket,
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "Int == Digit[]!",
        vec![
            IntT,
            Space(1),
            LooseCheck,
            Space(1),
            DigitT,
            LeftBracket,
            RightBracket,
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "const var age: Int9 = 28!",
        vec![
            ConstVar,
            Space(1),
            Identifier("age".into()),
            Colon,
            Space(1),
            Int9T,
            Space(1),
            Assignment,
            Space(1),
            Float(28.0),
            Bang(1),
            Eof,
        ]
    );
}

// The spec's RegExp example is enormous; the exact token stream would be
// hundreds of mostly zero-quote-string tokens. Pin that it scans cleanly
// rather than enumerating it.
#[test]
fn regular_expressions() {
    assert_scans!(r#"const const email: RegExp<(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])> = "mymail@mail.com"!"#);
    // No Regexp spelled differently is a keyword other than the documented ones.
    assert_tokens!(
        "const const r: RegExp = email!",
        vec![
            ConstConst,
            Space(1),
            Identifier("r".into()),
            Colon,
            Space(1),
            RegexpT,
            Space(1),
            Assignment,
            Space(1),
            Identifier("email".into()),
            Bang(1),
            Eof,
        ]
    );
}

#[test]
fn previous_next_and_current() {
    assert_tokens!(
        "const var score = 5!",
        vec![
            ConstVar,
            Space(1),
            Identifier("score".into()),
            Space(1),
            Assignment,
            Space(1),
            Float(5.0),
            Bang(1),
            Eof,
        ]
    );
    // `++` scans as two Plus tokens (adjacency, not a fused
    // increment operator, is the parser's signal).
    assert_tokens!(
        "score++!",
        vec![Identifier("score".into()), Plus, Plus, Bang(1), Eof]
    );
    assert_tokens!(
        "print(previous score)!",
        vec![
            Identifier("print".into()),
            LeftParen,
            Previous,
            Space(1),
            Identifier("score".into()),
            RightParen,
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "addEventListener(\"click\", () => score++)!",
        vec![
            Identifier("addEventListener".into()),
            LeftParen,
            Quote("\"".into()),
            Identifier("click".into()),
            Quote("\"".into()),
            Comma,
            Space(1),
            LeftParen,
            RightParen,
            Space(1),
            Arrow,
            Space(1),
            Identifier("score".into()),
            Plus,
            Plus,
            RightParen,
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "print(await next score)!",
        vec![
            Identifier("print".into()),
            LeftParen,
            Await,
            Space(1),
            Next,
            Space(1),
            Identifier("score".into()),
            RightParen,
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "print(current score)!",
        vec![
            Identifier("print".into()),
            LeftParen,
            Current,
            Space(1),
            Identifier("score".into()),
            RightParen,
            Bang(1),
            Eof,
        ]
    );
}

#[test]
fn file_structure() {
    assert_tokens!(
        "=====================",
        vec![FileDelim, Eof]
    );
    // Named file: first run of 5+ equals opens it, trailing run is another.
    assert_tokens!(
        "======= add.gom =======",
        vec![
            FileDelim,
            Space(1),
            Identifier("add".into()),
            Dot,
            Identifier("gom".into()),
            Space(1),
            FileDelim,
            Eof,
        ]
    );
}

#[test]
fn exporting_and_importing() {
    assert_tokens!(
        "export add to \"main.gom\"!",
        vec![
            Export,
            Space(1),
            Identifier("add".into()),
            Space(1),
            To,
            Space(1),
            Quote("\"".into()),
            Identifier("main".into()),
            Dot,
            Identifier("gom".into()),
            Quote("\"".into()),
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "import add!",
        vec![Import, Space(1), Identifier("add".into()), Bang(1), Eof]
    );
    assert_tokens!(
        "add(3, 2)!",
        vec![
            Identifier("add".into()),
            LeftParen,
            Float(3.0),
            Comma,
            Space(1),
            Float(2.0),
            RightParen,
            Bang(1),
            Eof,
        ]
    );
}

#[test]
fn classes() {
    assert_tokens!(
        "class Player {",
        vec![Class, Space(1), Identifier("Player".into()), Space(1), LeftBrace, Eof]
    );
    assert_tokens!(
        "const var player1 = new Player()!",
        vec![
            ConstVar,
            Space(1),
            Identifier("player1".into()),
            Space(1),
            Assignment,
            Space(1),
            New,
            Space(1),
            Identifier("Player".into()),
            LeftParen,
            RightParen,
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "function makePlayer() => {",
        vec![
            Function,
            Space(1),
            Identifier("makePlayer".into()),
            LeftParen,
            RightParen,
            Space(1),
            Arrow,
            Space(1),
            LeftBrace,
            Eof,
        ]
    );
    assert_tokens!(
        "const var player1 = playerMaker.makePlayer()!",
        vec![
            ConstVar,
            Space(1),
            Identifier("player1".into()),
            Space(1),
            Assignment,
            Space(1),
            Identifier("playerMaker".into()),
            Dot,
            Identifier("makePlayer".into()),
            LeftParen,
            RightParen,
            Bang(1),
            Eof,
        ]
    );
    // className aliases class.
    assert_tokens!(
        "className Player {",
        vec![Class, Space(1), Identifier("Player".into()), Space(1), LeftBrace, Eof]
    );
}

#[test]
fn time() {
    assert_tokens!(
        "Date.now()!",
        vec![
            Identifier("Date".into()),
            Dot,
            Identifier("now".into()),
            LeftParen,
            RightParen,
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "Date.now() -= 3600000!",
        vec![
            Identifier("Date".into()),
            Dot,
            Identifier("now".into()),
            LeftParen,
            RightParen,
            Space(1),
            Minus,
            Assignment,
            Space(1),
            Float(3_600_000.0),
            Bang(1),
            Eof,
        ]
    );
}

#[test]
fn delete() {
    assert_tokens!(
        "delete 3!",
        vec![Delete, Space(1), Float(3.0), Bang(1), Eof]
    );
    assert_tokens!(
        "print(2 + 1)!",
        vec![
            Identifier("print".into()),
            LeftParen,
            Float(2.0),
            Space(1),
            Plus,
            Space(1),
            Float(1.0),
            RightParen,
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "delete class!",
        vec![Delete, Space(1), Class, Bang(1), Eof]
    );
    assert_tokens!(
        "delete delete!",
        vec![Delete, Space(1), Delete, Bang(1), Eof]
    );
    assert_tokens!(
        "class Player {}",
        vec![
            Class,
            Space(1),
            Identifier("Player".into()),
            Space(1),
            LeftBrace,
            RightBrace,
            Eof,
        ]
    );
}

#[test]
fn overloading_and_priority() {
    assert_tokens!(
        "const const name = \"Luke\"!",
        vec![
            ConstConst,
            Space(1),
            Identifier("name".into()),
            Space(1),
            Assignment,
            Space(1),
            Quote("\"".into()),
            Identifier("Luke".into()),
            Quote("\"".into()),
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "const const name = \"Lu\"!!",
        vec![
            ConstConst,
            Space(1),
            Identifier("name".into()),
            Space(1),
            Assignment,
            Space(1),
            Quote("\"".into()),
            Identifier("Lu".into()),
            Quote("\"".into()),
            Bang(2),
            Eof,
        ]
    );
    assert_tokens!(
        "const const name = \"Luke\"\u{a1}",
        vec![
            ConstConst,
            Space(1),
            Identifier("name".into()),
            Space(1),
            Assignment,
            Space(1),
            Quote("\"".into()),
            Identifier("Luke".into()),
            Quote("\"".into()),
            Bang(-1),
            Eof,
        ]
    );
}

#[test]
fn semantic_naming() {
    assert_tokens!(
        "const const sName = \"Lu\"!",
        vec![
            ConstConst,
            Space(1),
            Identifier("sName".into()),
            Space(1),
            Assignment,
            Space(1),
            Quote("\"".into()),
            Identifier("Lu".into()),
            Quote("\"".into()),
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "const const g_fScore = 4.5!",
        vec![
            ConstConst,
            Space(1),
            Identifier("g_fScore".into()),
            Space(1),
            Assignment,
            Space(1),
            Float(4.5),
            Bang(1),
            Eof,
        ]
    );
}

#[test]
fn reversing() {
    assert_tokens!("reverse!", vec![Reverse, Bang(1), Eof]);
}

#[test]
fn asynchronous_functions() {
    assert_tokens!(
        "async funct count() => {",
        vec![
            Async,
            Space(1),
            Function,
            Space(1),
            Identifier("count".into()),
            LeftParen,
            RightParen,
            Space(1),
            Arrow,
            Space(1),
            LeftBrace,
            Eof,
        ]
    );
    assert_tokens!(
        "async func count() => {",
        vec![
            Async,
            Space(1),
            Function,
            Space(1),
            Identifier("count".into()),
            LeftParen,
            RightParen,
            Space(1),
            Arrow,
            Space(1),
            LeftBrace,
            Eof,
        ]
    );
    assert_tokens!("   noop!", vec![Space(3), Noop, Bang(1), Eof]);
    assert_tokens!(
        "count()!",
        vec![Identifier("count".into()), LeftParen, RightParen, Bang(1), Eof]
    );
}

#[test]
fn signals() {
    assert_tokens!(
        "const var score = use(0)!",
        vec![
            ConstVar,
            Space(1),
            Identifier("score".into()),
            Space(1),
            Assignment,
            Space(1),
            Use,
            LeftParen,
            Float(0.0),
            RightParen,
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "score(9)!",
        vec![Identifier("score".into()), LeftParen, Float(9.0), RightParen, Bang(1), Eof]
    );
    assert_tokens!(
        "score()?",
        vec![Identifier("score".into()), LeftParen, RightParen, QuestionMark, Eof]
    );
    assert_tokens!(
        "const var [getScore, setScore] = use(0)!",
        vec![
            ConstVar,
            Space(1),
            LeftBracket,
            Identifier("getScore".into()),
            Comma,
            Space(1),
            Identifier("setScore".into()),
            RightBracket,
            Space(1),
            Assignment,
            Space(1),
            Use,
            LeftParen,
            Float(0.0),
            RightParen,
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "const var [[[getScore, setScore], setScore], setScore] = use(0)!",
        vec![
            ConstVar,
            Space(1),
            LeftBracket,
            LeftBracket,
            LeftBracket,
            Identifier("getScore".into()),
            Comma,
            Space(1),
            Identifier("setScore".into()),
            RightBracket,
            Comma,
            Space(1),
            Identifier("setScore".into()),
            RightBracket,
            Comma,
            Space(1),
            Identifier("setScore".into()),
            RightBracket,
            Space(1),
            Assignment,
            Space(1),
            Use,
            LeftParen,
            Float(0.0),
            RightParen,
            Bang(1),
            Eof,
        ]
    );
}

// SPEC: unclosed quotes/brackets are valid (AQMI / ABI / AI). Note the
// trailing comments swallow everything after the `//`, comments included.
#[test]
fn unclosed_constructs_are_valid() {
    assert_tokens!(
        "print(\"Hello world\") // This is fine",
        vec![
            Identifier("print".into()),
            LeftParen,
            Quote("\"".into()),
            Identifier("Hello".into()),
            Space(1),
            Identifier("world".into()),
            Quote("\"".into()),
            RightParen,
            Space(1),
            Eof,
        ]
    );
    assert_tokens!(
        "print(\"Hello world\" // This is also fine",
        vec![
            Identifier("print".into()),
            LeftParen,
            Quote("\"".into()),
            Identifier("Hello".into()),
            Space(1),
            Identifier("world".into()),
            Quote("\"".into()),
            Space(1),
            Eof,
        ]
    );
    assert_tokens!(
        "print(\"Hello world // This is fine as well",
        vec![
            Identifier("print".into()),
            LeftParen,
            Quote("\"".into()),
            Identifier("Hello".into()),
            Space(1),
            Identifier("world".into()),
            Space(1),
            Eof,
        ]
    );
    assert_tokens!(
        "print( // This is probably fine",
        vec![Identifier("print".into()), LeftParen, Space(1), Eof]
    );
    assert_tokens!(
        "(add (3, (add (5, 6)!",
        vec![
            LeftParen,
            Identifier("add".into()),
            Space(1),
            LeftParen,
            Float(3.0),
            Comma,
            Space(1),
            LeftParen,
            Identifier("add".into()),
            Space(1),
            LeftParen,
            Float(5.0),
            Comma,
            Space(1),
            Float(6.0),
            RightParen,
            Bang(1),
            Eof,
        ]
    );
}

#[test]
fn parentheses_do_nothing() {
    assert_tokens!(
        "add)3, 2(!",
        vec![
            Identifier("add".into()),
            RightParen,
            Float(3.0),
            Comma,
            Space(1),
            Float(2.0),
            LeftParen,
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "add 3, 2!",
        vec![
            Identifier("add".into()),
            Space(1),
            Float(3.0),
            Comma,
            Space(1),
            Float(2.0),
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "(add (3, (add (5, 6))))!",
        vec![
            LeftParen,
            Identifier("add".into()),
            Space(1),
            LeftParen,
            Float(3.0),
            Comma,
            Space(1),
            LeftParen,
            Identifier("add".into()),
            Space(1),
            LeftParen,
            Float(5.0),
            Comma,
            Space(1),
            Float(6.0),
            RightParen,
            RightParen,
            RightParen,
            RightParen,
            Bang(1),
            Eof,
        ]
    );
}

#[test]
fn main_gom_statements() {
    // "unc" is an in-order subsequence of "function".
    assert_tokens!(
        "unc le(left, right) => left - right!",
        vec![
            Function,
            Space(1),
            Identifier("le".into()),
            LeftParen,
            Identifier("left".into()),
            Comma,
            Space(1),
            Identifier("right".into()),
            RightParen,
            Space(1),
            Arrow,
            Space(1),
            Identifier("left".into()),
            Space(1),
            Minus,
            Space(1),
            Identifier("right".into()),
            Bang(1),
            Eof,
        ]
    );
    // "union" and "in" also match the function-keyword letters.
    assert_tokens!(
        "union station(choo) => choo--choo!",
        vec![
            Function,
            Space(1),
            Identifier("station".into()),
            LeftParen,
            Identifier("choo".into()),
            RightParen,
            Space(1),
            Arrow,
            Space(1),
            Identifier("choo".into()),
            Minus,
            Minus,
            Identifier("choo".into()),
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "      in deep() => {",
        vec![
            Space(6),
            Function,
            Space(1),
            Identifier("deep".into()),
            LeftParen,
            RightParen,
            Space(1),
            Arrow,
            Space(1),
            LeftBrace,
            Eof,
        ]
    );
    assert_tokens!(
        "password[0.5] = quote[station(56)]!",
        vec![
            Identifier("password".into()),
            LeftBracket,
            Float(0.5),
            RightBracket,
            Space(1),
            Assignment,
            Space(1),
            Identifier("quote".into()),
            LeftBracket,
            Identifier("station".into()),
            LeftParen,
            Float(56.0),
            RightParen,
            RightBracket,
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "(0..4).forEach((i) => {",
        vec![
            LeftParen,
            Float(0.0),
            Range,
            Float(4.0),
            RightParen,
            Dot,
            Identifier("forEach".into()),
            LeftParen,
            LeftParen,
            Identifier("i".into()),
            RightParen,
            Space(1),
            Arrow,
            Space(1),
            LeftBrace,
            Eof,
        ]
    );
    assert_tokens!(
        "password.push(exclamation[i])!",
        vec![
            Identifier("password".into()),
            Dot,
            Identifier("push".into()),
            LeftParen,
            Identifier("exclamation".into()),
            LeftBracket,
            Identifier("i".into()),
            RightBracket,
            RightParen,
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "password.push(quote[-1] + 0x20)!",
        vec![
            Identifier("password".into()),
            Dot,
            Identifier("push".into()),
            LeftParen,
            Identifier("quote".into()),
            LeftBracket,
            Minus,
            Float(1.0),
            RightBracket,
            Space(1),
            Plus,
            Space(1),
            Float(32.0),
            RightParen,
            Bang(1),
            Eof,
        ]
    );
    assert_tokens!(
        "print(password)!!",
        vec![
            Identifier("print".into()),
            LeftParen,
            Identifier("password".into()),
            RightParen,
            Bang(2),
            Eof,
        ]
    );
    assert_tokens!(
        "const const const lucky<-Infinity> = 13!!",
        vec![
            ConstConstConst,
            Space(1),
            Identifier("lucky".into()),
            LeftAngular,
            Minus,
            Infinity,
            RightAngular,
            Space(1),
            Assignment,
            Space(1),
            Float(13.0),
            Bang(2),
            Eof,
        ]
    );
}

#[test]
fn main_gom_scans_cleanly() {
    let src = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/main.gom"))
        .expect("main.gom should exist");
    assert_scans!(src.as_str());
}
