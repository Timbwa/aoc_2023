pub mod days;

use anyhow::Result;
use clap::Parser;

use crate::days::*;

/// Rust Program for running AOC solutions
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Run the latest day solutions or run all solutions (Default = All Solutions)
    #[arg(short, long, default_value_t = false)]
    run_latest: bool,
    /// Run using the test input
    #[arg(short, long, default_value_t = false)]
    test: bool,
}

pub fn run(cli: &mut Cli) -> Result<()> {
    // Append new Days to this vector
    let days: Vec<Box<dyn Day>> = vec![
        Box::new(Day01 {}),
        Box::new(Day02 {}),
        Box::new(Day03 {}),
        Box::new(Day04 {}),
    ];

    if cli.run_latest {
        run_day(days.last().unwrap().as_ref(), cli.test)?;
    } else {
        for (i, day) in days.iter().enumerate() {
            println!("-------------------------Day {i:?}----------------------------");

            run_day(day.as_ref(), cli.test)?;
        }
    }

    Ok(())
}

fn run_day(day: &dyn Day, is_test: bool) -> Result<()> {
    let content_lines = day.read_input(is_test)?;

    let now = std::time::Instant::now();
    let result_1 = day.solution_1(&content_lines);
    let elapsed_1 = now.elapsed();

    let now = std::time::Instant::now();
    let result_2 = day.solution_2(&content_lines);
    let elapsed_2 = now.elapsed();

    println!(
        "Result 1 is {result_1:?} ({elapsed_1:?})\nResults 2 is {result_2:?} ({elapsed_2:?})"
    );

    Ok(())
}
