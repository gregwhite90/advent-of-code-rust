use clap::Parser;

mod router;

/// Runs the specfied Advent of Code solution
#[derive(Parser, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Year of the Advent of Code solution
    #[arg(required = true)]
    year: u32,

    /// Day of the solution
    #[arg(required = true)]
    day: u8,

    /// Part of the solution
    #[clap(value_enum)]
    #[arg(short, long, default_value_t = Part::Both)]
    part: Part,
}

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