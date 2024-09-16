use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use grrs::{search_sequential, search_parallel};

#[derive(Parser)]
pub struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
    /// Perform case-insensitive search
    #[clap(short, long)]
    ignore_case: bool,
    /// Use parallel processing for large files
    #[clap(short, long)]
    parallel: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let file = File::open(&args.path)
        .with_context(|| format!("could not open file `{}`", args.path.display()))?;
    let reader = BufReader::new(file);

    let pattern = if args.ignore_case {
        args.pattern.to_lowercase()
    } else {
        args.pattern.clone()
    };

    let result = if args.parallel {
        search_parallel(reader, &pattern, args.ignore_case, |line| println!("{}", line))
    } else {
        search_sequential(reader, &pattern, args.ignore_case, |line| println!("{}", line))
    };

    result.with_context(|| "search failed")?;

    Ok(())
}