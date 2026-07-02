//! Source file loading.

use std::fs;
use std::io;

/// Read the entire contents of `path` into a string.
///
/// The OCaml `read_file` hand-rolled a line-by-line loop that returned the
/// wrong accumulator on `End_of_file`, dropping the final line;
/// `fs::read_to_string` sidesteps both the bug and the fiddliness.
pub fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}
