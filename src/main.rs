use clap::Parser;

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
    let args = Args::parse();

    println!(
        "Year: {:?}, Day: {:?}, Part: {:?}", 
        args.year, args.day, args.part
    )
}