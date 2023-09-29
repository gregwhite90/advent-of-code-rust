use clap::Parser;

mod year_2017;
mod utils;

/// Runs the specfied Advent of Code solution
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
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

#[derive(clap::ValueEnum, Clone, Debug)]
enum Part {
    One,
    Two,
    Both,
}

fn main() {
    use crate::utils::utils::Solution;

    let _args = Args::parse();
    // TODO: run the correct function based on parameters

    let soln = crate::year_2017::day_01::part_one::Soln::new("input/year_2017/day_01/input.txt");
    println!("Part one:");
    println!("{}", soln.solve().expect_left("Solution should be an integer."));
}