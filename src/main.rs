//! CLI entry point.

use std::fs;

use clap::Parser;
use dreamberd::scanner;
use miette::IntoDiagnostic;

#[derive(Parser, Clone, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = false)]
    debug: bool,
}

fn main() -> miette::Result<()> {
    let args = Args::parse();
    let _ = miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(true)
                .unicode(true)
                .context_lines(3)
                .tab_width(4)
                .break_words(true)
                .build(),
        )
    }));

    let source = fs::read_to_string("main.gom").into_diagnostic()?;

    if args.debug {
        let result = scanner::scan_tokens(&source, "main.gom");
        for tok in &result.tokens {
            println!("{tok}");
        }
        if !result.errors.is_empty() {
            return Err(scanner::ScanErrors::new(result.errors).into());
        }
    }

    Ok(())
}
