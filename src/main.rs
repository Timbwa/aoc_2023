use anyhow::Result;
use aoc_2023::{run, Cli};
use clap::Parser;

fn main() -> Result<()> {
    let mut args = Cli::parse();

    run(&mut args)?;

    Ok(())
}
