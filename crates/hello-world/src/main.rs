//! Say hello to the world!

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: PathBuf,
}

/// Try cache invalidation by editing this doc comment.
/// Edited.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let Cli { path } = Cli::parse();
    let text = std::fs::read_to_string(path)?;
    println!("{text}");
    Ok(())
}
