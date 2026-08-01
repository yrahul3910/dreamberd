//! Every example statement from docs/SPECIFICATION.md (grouped by spec
//! section) must scan with no errors. These pin "valid syntax stays
//! scannable", not any particular token stream.
//!
//! Also covers the tricky statements in main.gom.

use crate::common::assert_scans;

#[test]
fn exclamation_marks() {
    let cases = [
        r#"print("Hello world")!"#,
        r#"print("Hello world")!!!"#,
        r#"print("Hello world")?"#,
        "if (;false) {\n   print(\"Hello world\")!\n}",
    ];
    for src in cases {
        assert_scans!(src);
    }
}

#[test]
fn declarations() {
    let cases = [
        "const const name = \"Luke\"!",
        "const var name = \"Luke\"!\nname.pop()!\nname.pop()!",
        "var const name = \"Luke\"!\nname = \"Lu\"!",
        "var var name = \"Luke\"!\nname = \"Lu\"!\nname.push(\"k\")!\nname.push(\"e\")!",
        "const const const pi = 3.14!",
    ];
    for src in cases {
        assert_scans!(src);
    }
}

#[test]
fn naming_allows_any_unicode_or_number() {
    let cases = [
        "const const letter = 'A'!",
        "var const 👍 = True!",
        "var var 1️⃣ = 1!",
        "const const 5 = 4!\nprint(2 + 2 === 5)! //true",
    ];
    for src in cases {
        assert_scans!(src);
    }
}

#[test]
fn arrays() {
    let cases = [
        "const const scores = [3, 2, 5]!\nprint(scores[-1])! //3\nprint(scores[0])!  //2\nprint(scores[1])!  //5",
        "const var scores = [3, 2, 5]!\nscores[0.5] = 4!\nprint(scores)! //[3, 2, 4, 5]",
    ];
    for src in cases {
        assert_scans!(src);
    }
}

#[test]
fn when_blocks() {
    assert_scans!("const var health = 10!\nwhen (health = 0) {\n   print(\"You lose\")!\n}");
}

#[test]
fn lifetimes() {
    let cases = [
        r#"const const name<2> = "Luke"! //lasts for two lines"#,
        r#"const const name<20s> = "Luke"! //lasts for 20 seconds"#,
        r#"const const name<Infinity> = "Luke"! //lasts forever"#,
        r#"print(name)! //Luke
const const name<-1> = "Luke"!"#,
    ];
    for src in cases {
        assert_scans!(src);
    }
}

#[test]
fn booleans() {
    let cases = [
        "const var keys = {}!\naddEventListener(\"keydown\", (e) => keys[e.key] = true)!\naddEventListener(\"keyup\", (e) => keys[e.key] = false)!",
        "function isKeyDown(key) => {\n   if (keys[key] = undefined) {\n      return maybe!\n   }\n   return keys[key]!\n}",
    ];
    for src in cases {
        assert_scans!(src);
    }
}

#[test]
fn arithmetic() {
    let cases = [
        "print(1 + 2*3)! //7\nprint(1+2 * 3)! //9",
        "const const half = 1/2!",
        "print(one + two)! //3",
    ];
    for src in cases {
        assert_scans!(src);
    }
}

#[test]
fn indents_of_three_spaces() {
    let cases = [
        "function main() => {\n   print(\"Gulf of Mexico is the future\")!\n}",
        "   function main() => {\nprint(\"Gulf of Mexico is the future\")!\n   }",
    ];
    for src in cases {
        assert_scans!(src);
    }
}

#[test]
fn equality() {
    let cases = [
        "3.14 == \"3.14\"! //true",
        "3.14 === \"3.14\"! //false",
        "const const pi = 3.14!\nprint(pi ==== pi)! //true\nprint(3.14 ==== 3.14)! //true\nprint(3.14 ==== pi)! //false",
        "3 = 3.14! //true",
    ];
    for src in cases {
        assert_scans!(src);
    }
}

#[test]
fn functions() {
    let cases = [
        "function add(a, b) => a + b!",
        "func multiply(a, b) => a * b!",
        "fun subtract(a, b) => a - b!",
        "fn divide(a, b) => a / b!",
        "functi power(a, b) => a ^ b!",
        "f inverse(a) => 1/a!",
    ];
    for src in cases {
        assert_scans!(src);
    }
}

#[test]
fn dividing_by_zero() {
    assert_scans!("print(3 / 0)! //undefined");
}

#[test]
fn strings_with_any_number_of_quotes() {
    let cases = [
        "const const name = 'Lu'!",
        "const const name = \"Luke\"!",
        "const const name = '''Lu'''!",
        "const const name = \"'Lu'\"!",
        "const const name = \"\"\"\"Luke\"\"\"\"!",
        "const const name = Luke!",
    ];
    for src in cases {
        assert_scans!(src);
    }
}

#[test]
fn string_interpolation_with_regional_currency() {
    let cases = [
        "const const name = \"world\"!\nprint(\"Hello ${name}!\")!",
        "print(\"Hello \u{a3}{name}!\")!",
        "print(\"Hello \u{a5}{name}!\")!",
        "print(\"Hello {name}\u{20ac}!\")!",
        "const const player = { name: \"Lu\" }!\nprint(\"Hello {player$name}!\")!",
    ];
    for src in cases {
        assert_scans!(src);
    }
}

#[test]
fn types() {
    let cases = [
        "const var age: Int = 28!",
        "String == Char[]!",
        "Int == Digit[]!",
        "const var age: Int9 = 28!",
    ];
    for src in cases {
        assert_scans!(src);
    }
}

#[test]
fn regular_expressions() {
    assert_scans!(
        r#"const const email: RegExp<(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])> = "mymail@mail.com"!"#
    );
}

#[test]
fn previous_next_and_current() {
    let cases = [
        "const var score = 5!\nscore++!\nprint(score)! //6\nprint(previous score)! //5",
        "const var score = 5!\naddEventListener(\"click\", () => score++)!\nprint(await next score)! //6 (when you click)",
        "const var score = 5!\nprint(current score)! //5",
    ];
    for src in cases {
        assert_scans!(src);
    }
}

#[test]
fn file_structure() {
    let cases = [
        "const const score = 5!\nprint(score)! //5\n\n=====================\n\nconst const score = 3!\nprint(score)! //3",
        "======= add.gom =======\nfunction add(a, b) => {\n   return a + b!\n}",
    ];
    for src in cases {
        assert_scans!(src);
    }
}

#[test]
fn exporting_and_importing() {
    assert_scans!(
        "===== add.gom ==\nfunction add(a, b) => {\n   return a + b!\n}\n\nexport add to \"main.gom\"!\n\n===== main.gom ==\nimport add!\nadd(3, 2)!"
    );
}

#[test]
fn classes() {
    let cases = [
        "class Player {\n   const var health = 10!\n}\n\nconst var player1 = new Player()!\nconst var player2 = new Player()! //Error: Can't have more than one 'Player' instance!",
        "class PlayerMaker {\n   function makePlayer() => {\n      class Player {\n         const var health = 10!\n      }\n      const const player = new Player()!\n      return player!\n   }\n}\n\nconst const playerMaker = new PlayerMaker()!\nconst var player1 = playerMaker.makePlayer()!\nconst var player2 = playerMaker.makePlayer()!",
        "className Player {\n   const var health = 10!\n}",
    ];
    for src in cases {
        assert_scans!(src);
    }
}

#[test]
fn time() {
    let cases = [
        "Date.now()!",
        "// Move the clocks back one hour\nDate.now() -= 3600000!",
    ];
    for src in cases {
        assert_scans!(src);
    }
}

#[test]
fn delete() {
    let cases = [
        "delete 3!\nprint(2 + 1)! // Error: 3 has been deleted",
        "delete class!\nclass Player {} // Error: class was deleted",
        "delete delete!",
    ];
    for src in cases {
        assert_scans!(src);
    }
}

#[test]
fn overloading_and_priority() {
    let cases = [
        "const const name = \"Luke\"!\nconst const name = \"Lu\"!\nprint(name)! // \"Lu\"",
        "const const name = \"Lu\"!!\nconst const name = \"Luke\"!\nprint(name)! // \"Lu\"",
        "const const name = \"Lu or Luke (either is fine)\"!!!!!!!!!\nprint(name)! // \"Lu or Luke (either is fine)\"",
        "const const name = \"Lu\"!\nconst const name = \"Luke\"\u{a1}\nprint(name)! // \"Lu\"",
    ];
    for src in cases {
        assert_scans!(src);
    }
}

#[test]
fn semantic_naming() {
    let cases = [
        "const const sName = \"Lu\"!\nconst const iAge = 29!\nconst const bHappy = true!",
        "const const g_fScore = 4.5!",
    ];
    for src in cases {
        assert_scans!(src);
    }
}

#[test]
fn reversing() {
    assert_scans!(
        "const const message = \"Hello\"!\nprint(message)!\nconst const message = \"world\"!\nreverse!"
    );
}

#[test]
fn dbx() {
    let cases = [
        "funct App() => {\n   return <div>Hello world!</div>\n}",
        "funct App() => {\n   return <div htmlClassName=\"greeting\">Hello world!</div>\n}",
        "funct App() => {\n   return (\n      <label for=\"name\">Name</label>\n      <input id=\"name\" />\n   )\n}",
    ];
    for src in cases {
        assert_scans!(src);
    }
}

#[test]
fn rich_text() {
    let cases = [
        "const const <b>name</b> = \"Lu\"!\nconst const <i>name</i> = \"Luke\"!",
        "print(<b>name</b>)! // Lu",
        "<p>Click <a href=\"https://dreamberd.computer\">here</a></p>",
    ];
    for src in cases {
        assert_scans!(src);
    }
}

#[test]
fn asynchronous_functions() {
    let cases = [
        "async funct count() => {\n   print(1)!\n   print(3)!\n}\n\ncount()!\nprint(2)!",
        "async func count() => {\n   print(1)!\n   noop!\n   print(4)!\n}\n\ncount()!\nprint(2)!\nprint(3)!",
    ];
    for src in cases {
        assert_scans!(src);
    }
}

#[test]
fn signals() {
    let cases = [
        "const var score = use(0)!",
        "const var score = use(0)!\n\nscore(9)! // Set the value\nscore()?  // Get the value (and print it)",
        "const var [getScore, setScore] = use(0)!\n\nsetScore(9)!\ngetScore()?",
        "const var [[[getScore, setScore], setScore], setScore] = use(0)!",
    ];
    for src in cases {
        assert_scans!(src);
    }
}

// SPEC: unclosed quotes/brackets are valid (AQMI / ABI / AI).
#[test]
fn unclosed_constructs_are_valid() {
    let cases = [
        "print(\"Hello world\") // This is fine",
        "print(\"Hello world\" // This is also fine",
        "print(\"Hello world // This is fine as well",
        "addEventListener(\"click\", (e) => {\n   requestAnimationFrame(() => {\n      print(\"You clicked on the page\n\n      // This is fine",
        "print( // This is probably fine",
        "(add (3, (add (5, 6)!",
    ];
    for src in cases {
        assert_scans!(src);
    }
}

#[test]
fn parentheses_do_nothing() {
    let cases = [
        "add(3, 2)!",
        "add 3, 2!",
        "(add (3, 2))!",
        "add)3, 2(!",
        "(add (3, (add (5, 6))))!",
    ];
    for src in cases {
        assert_scans!(src);
    }
}

#[test]
fn main_gom_statements() {
    let cases = [
        "unc le(left, right) => left - right!",
        "union station(choo) => choo--choo!",
        "      in deep() => {\n   delete 42!  // what's the meaning of life now? :3\n   }",
        "const const quote = \"Science, my lad, has been built upon many errors\"!!!",
        "const var password = [\nle(quote[station(4)], 0x20),\nquote[1],\nquote[le(55, 0x20)],\n   ]!",
        "password[0.5] = quote[station(56)]!",
        "(0..4).forEach((i) => {\npassword.push(exclamation[i])!\n   });",
        "if (lucky === 13) {\npassword.push(quote[-1] + 0x20)!\n   }",
        "const const const lucky<-Infinity> = 13!!",
    ];
    for src in cases {
        assert_scans!(src);
    }
}

#[test]
fn main_gom_scans_cleanly() {
    let src = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/main.gom"))
        .expect("main.gom should exist");
    assert_scans!(src.as_str());
}
