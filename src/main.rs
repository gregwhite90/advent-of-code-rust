use clap::{self, Parser};

mod router;

/// Runs the specfied Advent of Code solution
#[derive(Parser, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Year of the solution [possible values: 2015-2023 inclusive]
    #[arg(required = true)]
    #[arg(value_parser = clap::value_parser!(u32).range(2015..=2024))]
    year: u32,

    /// Day of the solution  [possible values: 1-25 inclusive]
    #[arg(required = true)]
    #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,

    /// Part of the solution
    #[clap(value_enum)]
    #[arg(short, long, default_value_t = Part::Both)]
    part: Part,
}

/// Represents a part of the day's solution to run (or both parts)
#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Part {
    One,
    Two,
    Both,
}

fn main() {
    let args = Args::parse();
    router::run_solution(&args);
}