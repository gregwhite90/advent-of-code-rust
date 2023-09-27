use clap::Parser;

mod year_2017_day_01;

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
    #[arg(short, long, default_value_t = Part::Two)]
    part: Part,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Part {
    One,
    Two,
}

fn main() {
    let _args = Args::parse();
    // TODO: run the correct function based on parameters

    println!("{}", year_2017_day_01::year_2017_day_01::sum_of_repeated_characters("1111"));
}