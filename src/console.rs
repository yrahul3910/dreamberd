//! Terminal diagnostics.

const YELLOW: &str = "\x1b[33;1m";
const RESET: &str = "\x1b[0m";

/// Print the offending source `line`, a caret under column `col`, and a yellow
/// `error:` message. This is a cleaned-up version of the OCaml `console.ml`,
/// whose `Printf` width arithmetic was fragile; the intent (line + caret +
/// message) is preserved.
pub fn error(line: &str, col: usize, msg: &str) {
    let pad = " ".repeat(col);
    eprintln!("{line}");
    eprintln!("{pad}^");
    eprintln!("{pad}{YELLOW}error:{RESET} {msg}");
}
