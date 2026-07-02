//! CLI entry point.
//!
//! Mirrors the flags the OCaml `bin/main.ml` exposed via `cmdliner`
//! (`FILE`, `-v`/`--verbose`, `--tokens`) but parses them by hand so the crate
//! stays dependency-free and builds offline. Swapping in `clap` later is a
//! drop-in change if richer help/usage output is wanted.

use std::process::ExitCode;

use dreamberd::{files, scanner};

fn main() -> ExitCode {
    let mut file: Option<String> = None;
    let mut verbose = false;
    let mut dump_tokens = false;

    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "-v" | "--verbose" => verbose = true,
            "--tokens" => dump_tokens = true,
            _ if arg.starts_with('-') => {
                eprintln!("Unknown option: {arg}");
                return ExitCode::FAILURE;
            }
            _ if file.is_none() => file = Some(arg),
            _ => {} // ignore extra positional args
        }
    }

    let Some(path) = file else {
        println!("No file provided!");
        return ExitCode::SUCCESS;
    };

    println!("Interpreting {path} (verbose: {verbose})");

    let source = match files::read_file(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Could not read {path}: {e}");
            return ExitCode::FAILURE;
        }
    };
    print!("Contents: {source}");

    if dump_tokens {
        let result = scanner::scan_tokens(&source);
        for tok in &result.tokens {
            println!("{tok}");
        }
        if !result.errors.is_empty() {
            eprintln!(
                "\n{} scan error(s) at char position(s): {:?}",
                result.errors.len(),
                result.errors
            );
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
