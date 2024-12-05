//! Utilities to take command line arguments and run the specified solution. 
use std::collections::{HashMap, HashSet};
use crate::{Args, Part};
use advent_of_code_rust::utils::{solution::Solution, io_utils::{self, InputFileType}, Day};
use advent_of_code_rust::{year_2015, year_2016, year_2017, year_2018, year_2023, year_2024};

/// Runs the solution(s) specified by the command line arguments.
pub fn run_solution(args: &Args) {

    let day = Day { year: args.year, day: args.day };

    let solns = get_solns(&day);
    let input_filename = io_utils::input_filename(&day, InputFileType::Input);

    if !matches!(args.part, Part::Two) {
        match solns.part_one {
            Some(mut p_one) => {
                println!("Part one:");
                println!("{}", p_one.solve(&input_filename));            
            },
            None => println!("No solution found for part one of this day."),
        }
    }

    if !matches!(args.part, Part::One) {
        match solns.part_two {
            Some(mut p_two) => {
                println!("Part two:");
                println!("{}", p_two.solve(&input_filename));            
            },
            None => println!("No solution found for part two of this day."),
        }
    }
}

/// A day's solutions can include a solution to part one and/or a solution to part two,
/// or neither.
struct DailySolutions {
    part_one: Option<Box<dyn Solution>>,
    part_two: Option<Box<dyn Solution>>,
}

/// Gets the daily solutions for the specified day.
fn get_solns(day: &Day) -> DailySolutions {
    // Mutable because we will later move out the daily solutions to be able to return them.
    let mut daily_solutions: HashMap<Day, DailySolutions> = HashMap::from([
        (
            Day { year: 2015, day: 6 },
            DailySolutions { 
                part_one: Some(Box::new(year_2015::day_06::part_one::Soln::default())),
                part_two: Some(Box::new(year_2015::day_06::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2015, day: 7 },
            DailySolutions { 
                part_one: Some(Box::new(year_2015::day_07::part_one::Soln::default())),
                part_two: Some(Box::new(year_2015::day_07::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2015, day: 8 },
            DailySolutions { 
                part_one: Some(Box::new(year_2015::day_08::part_one::Soln::default())),
                part_two: Some(Box::new(year_2015::day_08::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2015, day: 9 },
            DailySolutions { 
                part_one: Some(Box::new(year_2015::day_09::part_one::Soln::default())),
                part_two: Some(Box::new(year_2015::day_09::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2015, day: 10 },
            DailySolutions { 
                part_one: Some(Box::new(year_2015::day_10::part_one::Soln::default())),
                part_two: None,
            },
        ),
        (
            Day { year: 2015, day: 11 },
            DailySolutions { 
                part_one: Some(Box::new(year_2015::day_11::part_one::Soln::default())),
                part_two: Some(Box::new(year_2015::day_11::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2015, day: 12 },
            DailySolutions { 
                part_one: Some(Box::new(year_2015::day_12::part_one::Soln::default())),
                part_two: Some(Box::new(year_2015::day_12::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2015, day: 13 },
            DailySolutions { 
                part_one: Some(Box::new(year_2015::day_13::part_one::Soln::default())),
                part_two: Some(Box::new(year_2015::day_13::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2015, day: 14 },
            DailySolutions { 
                part_one: Some(Box::new(year_2015::day_14::part_one::Soln::default())),
                part_two: Some(Box::new(year_2015::day_14::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2015, day: 15 },
            DailySolutions { 
                part_one: Some(Box::new(year_2015::day_15::part_one::Soln::default())),
                part_two: Some(Box::new(year_2015::day_15::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2015, day: 16 },
            DailySolutions { 
                part_one: Some(Box::new(year_2015::day_16::part_one::Soln::default())),
                part_two: Some(Box::new(year_2015::day_16::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2015, day: 17 },
            DailySolutions { 
                part_one: Some(Box::new(year_2015::day_17::part_one::Soln::default())),
                part_two: Some(Box::new(year_2015::day_17::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2015, day: 18 },
            DailySolutions { 
                part_one: Some(Box::new(year_2015::day_18::part_one::Soln::default())),
                part_two: Some(Box::new(year_2015::day_18::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2016, day: 1 },
            DailySolutions { 
                part_one: Some(Box::new(year_2016::day_01::part_one::Soln::default())),
                part_two: Some(Box::new(year_2016::day_01::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2016, day: 2 },
            DailySolutions { 
                part_one: Some(Box::new(year_2016::day_02::part_one::Soln::default())),
                part_two: Some(Box::new(year_2016::day_02::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2016, day: 3 },
            DailySolutions { 
                part_one: Some(Box::new(year_2016::day_03::part_one::Soln::default())),
                part_two: Some(Box::new(year_2016::day_03::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2016, day: 4 },
            DailySolutions { 
                part_one: Some(Box::new(year_2016::day_04::part_one::Soln::default())),
                part_two: Some(Box::new(year_2016::day_04::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2016, day: 5 },
            DailySolutions { 
                part_one: Some(Box::new(year_2016::day_05::part_one::Soln::default())),
                part_two: Some(Box::new(year_2016::day_05::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2016, day: 6 },
            DailySolutions { 
                part_one: Some(Box::new(year_2016::day_06::part_one::Soln::default())),
                part_two: Some(Box::new(year_2016::day_06::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2016, day: 7 },
            DailySolutions { 
                part_one: Some(Box::new(year_2016::day_07::part_one::Soln::default())),
                part_two: Some(Box::new(year_2016::day_07::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2016, day: 8 },
            DailySolutions { 
                part_one: Some(Box::new(year_2016::day_08::part_one::Soln::default())),
                part_two: Some(Box::new(year_2016::day_08::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2016, day: 9 },
            DailySolutions { 
                part_one: Some(Box::new(year_2016::day_09::part_one::Soln::default())),
                part_two: Some(Box::new(year_2016::day_09::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2016, day: 10 },
            DailySolutions { 
                part_one: Some(Box::new(year_2016::day_10::part_one::Soln::default())),
                part_two: Some(Box::new(year_2016::day_10::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2016, day: 11 },
            DailySolutions { 
                part_one: Some(Box::new(year_2016::day_11::part_one::Soln::default())),
                part_two: None,
            },
        ),
        (
            Day { year: 2016, day: 12 },
            DailySolutions { 
                part_one: Some(Box::new(year_2016::day_12::part_one::Soln::default())),
                part_two: Some(Box::new(year_2016::day_12::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2016, day: 13 },
            DailySolutions { 
                part_one: Some(Box::new(year_2016::day_13::part_one::Soln::default())),
                part_two: Some(Box::new(year_2016::day_13::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2016, day: 14 },
            DailySolutions { 
                part_one: Some(Box::new(year_2016::day_14::part_one::Soln::default())),
                part_two: Some(Box::new(year_2016::day_14::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2016, day: 15 },
            DailySolutions { 
                part_one: Some(Box::new(year_2016::day_15::part_one::Soln::default())),
                part_two: Some(Box::new(year_2016::day_15::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2016, day: 16 },
            DailySolutions { 
                part_one: Some(Box::new(year_2016::day_16::part_one::Soln::default())),
                part_two: Some(Box::new(year_2016::day_16::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2016, day: 17 },
            DailySolutions { 
                part_one: Some(Box::new(year_2016::day_17::part_one::Soln::default())),
                part_two: Some(Box::new(year_2016::day_17::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2016, day: 18 },
            DailySolutions { 
                part_one: Some(Box::new(year_2016::day_18::part_one::Soln::default())),
                part_two: Some(Box::new(year_2016::day_18::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2016, day: 19 },
            DailySolutions { 
                part_one: Some(Box::new(year_2016::day_19::part_one::Soln::default())),
                part_two: Some(Box::new(year_2016::day_19::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2016, day: 20 },
            DailySolutions { 
                part_one: Some(Box::new(year_2016::day_20::part_one::Soln::default())),
                part_two: Some(Box::new(year_2016::day_20::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2016, day: 21 },
            DailySolutions { 
                part_one: Some(Box::new(year_2016::day_21::part_one::Soln::default())),
                part_two: Some(Box::new(year_2016::day_21::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2016, day: 22 },
            DailySolutions { 
                part_one: Some(Box::new(year_2016::day_22::part_one::Soln::default())),
                part_two: Some(Box::new(year_2016::day_22::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2016, day: 23 },
            DailySolutions { 
                part_one: Some(Box::new(year_2016::day_23::part_one::Soln::default())),
                part_two: Some(Box::new(year_2016::day_23::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2016, day: 24 },
            DailySolutions { 
                part_one: Some(Box::new(year_2016::day_24::part_one::Soln::default())),
                part_two: Some(Box::new(year_2016::day_24::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2016, day: 25 },
            DailySolutions { 
                part_one: Some(Box::new(year_2016::day_25::part_one::Soln::default())),
                part_two: None,
            },
        ),
        (
            Day { year: 2017, day: 1 },
            DailySolutions { 
                part_one: Some(Box::new(year_2017::day_01::part_one::Soln::default())),
                part_two: Some(Box::new(year_2017::day_01::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2017, day: 2 },
            DailySolutions { 
                part_one: Some(Box::new(year_2017::day_02::part_one::Soln::default())),
                part_two: Some(Box::new(year_2017::day_02::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2017, day: 3 },
            DailySolutions { 
                part_one: Some(Box::new(year_2017::day_03::part_one::Soln::default())),
                part_two: Some(Box::new(year_2017::day_03::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2017, day: 4 },
            DailySolutions { 
                part_one: Some(Box::new(year_2017::day_04::part_one::Soln::default())),
                part_two: Some(Box::new(year_2017::day_04::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2017, day: 5 },
            DailySolutions { 
                part_one: Some(Box::new(year_2017::day_05::part_one::Soln::default())),
                part_two: Some(Box::new(year_2017::day_05::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2017, day: 6 },
            DailySolutions { 
                part_one: Some(Box::new(year_2017::day_06::part_one::Soln::default())),
                part_two: Some(Box::new(year_2017::day_06::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2017, day: 7 },
            DailySolutions { 
                part_one: Some(Box::new(year_2017::day_07::part_one::Soln::default())),
                part_two: Some(Box::new(year_2017::day_07::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2017, day: 8 },
            DailySolutions { 
                part_one: Some(Box::new(year_2017::day_08::part_one::Soln::default())),
                part_two: Some(Box::new(year_2017::day_08::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2017, day: 9 },
            DailySolutions { 
                part_one: Some(Box::new(year_2017::day_09::part_one::Soln::default())),
                part_two: Some(Box::new(year_2017::day_09::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2017, day: 10 },
            DailySolutions { 
                part_one: Some(Box::new(year_2017::day_10::part_one::Soln::default())),
                part_two: Some(Box::new(year_2017::day_10::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2017, day: 11 },
            DailySolutions { 
                part_one: Some(Box::new(year_2017::day_11::part_one::Soln::default())),
                part_two: Some(Box::new(year_2017::day_11::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2017, day: 12 },
            DailySolutions { 
                part_one: Some(Box::new(year_2017::day_12::part_one::Soln::default())),
                part_two: Some(Box::new(year_2017::day_12::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2017, day: 13 },
            DailySolutions { 
                part_one: Some(Box::new(year_2017::day_13::part_one::Soln::default())),
                part_two: Some(Box::new(year_2017::day_13::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2017, day: 14 },
            DailySolutions { 
                part_one: Some(Box::new(year_2017::day_14::part_one::Soln::default())),
                part_two: Some(Box::new(year_2017::day_14::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2017, day: 15 },
            DailySolutions { 
                part_one: Some(Box::new(year_2017::day_15::part_one::Soln::default())),
                part_two: Some(Box::new(year_2017::day_15::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2017, day: 16 },
            DailySolutions { 
                part_one: Some(Box::new(year_2017::day_16::part_one::Soln::default())),
                part_two: Some(Box::new(year_2017::day_16::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2017, day: 17 },
            DailySolutions { 
                part_one: Some(Box::new(year_2017::day_17::part_one::Soln::default())),
                part_two: Some(Box::new(year_2017::day_17::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2017, day: 18 },
            DailySolutions { 
                part_one: Some(Box::new(year_2017::day_18::part_one::Soln::default())),
                part_two: Some(Box::new(year_2017::day_18::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2017, day: 19 },
            DailySolutions { 
                part_one: Some(Box::new(year_2017::day_19::part_one::Soln::default())),
                part_two: Some(Box::new(year_2017::day_19::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2017, day: 20 },
            DailySolutions { 
                part_one: Some(Box::new(year_2017::day_20::part_one::Soln::default())),
                part_two: Some(Box::new(year_2017::day_20::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2017, day: 21 },
            DailySolutions { 
                part_one: Some(Box::new(year_2017::day_21::part_one::Soln::default())),
                part_two: Some(Box::new(year_2017::day_21::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2017, day: 22 },
            DailySolutions { 
                part_one: Some(Box::new(year_2017::day_22::part_one::Soln::default())),
                part_two: Some(Box::new(year_2017::day_22::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2017, day: 23 },
            DailySolutions { 
                part_one: Some(Box::new(year_2017::day_23::part_one::Soln::default())),
                part_two: Some(Box::new(year_2017::day_23::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2017, day: 24 },
            DailySolutions { 
                part_one: Some(Box::new(year_2017::day_24::part_one::Soln::default())),
                part_two: Some(Box::new(year_2017::day_24::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2017, day: 25 },
            DailySolutions { 
                part_one: Some(Box::new(year_2017::day_25::part_one::Soln::default())),
                part_two: None,
            },
        ),
        (
            Day { year: 2018, day: 2 },
            DailySolutions { 
                part_one: Some(Box::new(year_2018::day_02::part_one::Soln::default())),
                part_two: Some(Box::new(year_2018::day_02::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2018, day: 3 },
            DailySolutions { 
                part_one: Some(Box::new(year_2018::day_03::part_one::Soln::default())),
                part_two: Some(Box::new(year_2018::day_03::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2018, day: 4 },
            DailySolutions { 
                part_one: Some(Box::new(year_2018::day_04::part_one::Soln::default())),
                part_two: Some(Box::new(year_2018::day_04::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2018, day: 5 },
            DailySolutions { 
                part_one: Some(Box::new(year_2018::day_05::part_one::Soln::default())),
                part_two: Some(Box::new(year_2018::day_05::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2018, day: 6 },
            DailySolutions { 
                part_one: Some(Box::new(year_2018::day_06::part_one::Soln::default())),
                part_two: Some(Box::new(year_2018::day_06::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2018, day: 7 },
            DailySolutions { 
                part_one: Some(Box::new(year_2018::day_07::part_one::Soln::default())),
                part_two: Some(Box::new(year_2018::day_07::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2018, day: 8 },
            DailySolutions { 
                part_one: Some(Box::new(year_2018::day_08::part_one::Soln::default())),
                part_two: Some(Box::new(year_2018::day_08::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2018, day: 9 },
            DailySolutions { 
                part_one: Some(Box::new(year_2018::day_09::part_one::Soln::default())),
                part_two: Some(Box::new(year_2018::day_09::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2018, day: 10 },
            DailySolutions { 
                part_one: Some(Box::new(year_2018::day_10::part_one::Soln::default())),
                part_two: Some(Box::new(year_2018::day_10::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2018, day: 11 },
            DailySolutions { 
                part_one: Some(Box::new(year_2018::day_11::part_one::Soln::default())),
                part_two: Some(Box::new(year_2018::day_11::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2018, day: 12 },
            DailySolutions { 
                part_one: Some(Box::new(year_2018::day_12::part_one::Soln::default())),
                part_two: Some(Box::new(year_2018::day_12::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2018, day: 13 },
            DailySolutions { 
                part_one: Some(Box::new(year_2018::day_13::part_one::Soln::default())),
                part_two: Some(Box::new(year_2018::day_13::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2018, day: 14 },
            DailySolutions { 
                part_one: Some(Box::new(year_2018::day_14::part_one::Soln::default())),
                part_two: Some(Box::new(year_2018::day_14::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2018, day: 15 },
            DailySolutions { 
                part_one: Some(Box::new(year_2018::day_15::part_one::Soln::default())),
                part_two: Some(Box::new(year_2018::day_15::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2018, day: 16 },
            DailySolutions { 
                part_one: Some(Box::new(year_2018::day_16::part_one::Soln::default())),
                part_two: Some(Box::new(year_2018::day_16::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2018, day: 17 },
            DailySolutions { 
                part_one: Some(Box::new(year_2018::day_17::part_one::Soln::default())),
                part_two: Some(Box::new(year_2018::day_17::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2018, day: 18 },
            DailySolutions { 
                part_one: Some(Box::new(year_2018::day_18::part_one::Soln::default())),
                part_two: Some(Box::new(year_2018::day_18::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2018, day: 19 },
            DailySolutions { 
                part_one: Some(Box::new(year_2018::day_19::part_one::Soln::default())),
                part_two: Some(Box::new(year_2018::day_19::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2018, day: 20 },
            DailySolutions { 
                part_one: Some(Box::new(year_2018::day_20::part_one::Soln::default())),
                part_two: Some(Box::new(year_2018::day_20::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2018, day: 21 },
            DailySolutions { 
                part_one: Some(Box::new(year_2018::day_21::part_one::Soln::default())),
                part_two: Some(Box::new(year_2018::day_21::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2018, day: 22 },
            DailySolutions { 
                part_one: Some(Box::new(year_2018::day_22::part_one::Soln::default())),
                part_two: Some(Box::new(year_2018::day_22::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2018, day: 23 },
            DailySolutions { 
                part_one: Some(Box::new(year_2018::day_23::part_one::Soln::default())),
                part_two: Some(Box::new(year_2018::day_23::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2018, day: 24 },
            DailySolutions { 
                part_one: Some(Box::new(year_2018::day_24::part_one::Soln::default())),
                part_two: Some(Box::new(year_2018::day_24::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2018, day: 25 },
            DailySolutions { 
                part_one: Some(Box::new(year_2018::day_25::part_one::Soln::default())),
                part_two: None,
            },
        ),
        (
            Day { year: 2023, day: 1 },
            DailySolutions { 
                part_one: Some(Box::new(year_2023::day_01::part_one::Soln::default())),
                part_two: Some(Box::new(year_2023::day_01::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2023, day: 2 },
            DailySolutions { 
                part_one: Some(Box::new(year_2023::day_02::part_one::Soln::default())),
                part_two: Some(Box::new(year_2023::day_02::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2023, day: 3 },
            DailySolutions { 
                part_one: Some(Box::new(year_2023::day_03::part_one::Soln::default())),
                part_two: Some(Box::new(year_2023::day_03::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2023, day: 4 },
            DailySolutions { 
                part_one: Some(Box::new(year_2023::day_04::part_one::Soln::default())),
                part_two: Some(Box::new(year_2023::day_04::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2023, day: 5 },
            DailySolutions { 
                part_one: Some(Box::new(year_2023::day_05::part_one::Soln::default())),
                part_two: Some(Box::new(year_2023::day_05::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2023, day: 6 },
            DailySolutions { 
                part_one: Some(Box::new(year_2023::day_06::part_one::Soln::default())),
                part_two: Some(Box::new(year_2023::day_06::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2023, day: 7 },
            DailySolutions { 
                part_one: Some(Box::new(year_2023::day_07::part_one::Soln::default())),
                part_two: Some(Box::new(year_2023::day_07::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2023, day: 8 },
            DailySolutions { 
                part_one: Some(Box::new(year_2023::day_08::part_one::Soln::default())),
                part_two: Some(Box::new(year_2023::day_08::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2023, day: 9 },
            DailySolutions { 
                part_one: Some(Box::new(year_2023::day_09::part_one::Soln::default())),
                part_two: Some(Box::new(year_2023::day_09::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2023, day: 10 },
            DailySolutions { 
                part_one: Some(Box::new(year_2023::day_10::part_one::Soln::default())),
                part_two: Some(Box::new(year_2023::day_10::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2023, day: 11 },
            DailySolutions { 
                part_one: Some(Box::new(year_2023::day_11::part_one::Soln::default())),
                part_two: Some(Box::new(year_2023::day_11::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2023, day: 12 },
            DailySolutions { 
                part_one: Some(Box::new(year_2023::day_12::part_one::Soln::default())),
                part_two: Some(Box::new(year_2023::day_12::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2023, day: 13 },
            DailySolutions { 
                part_one: Some(Box::new(year_2023::day_13::part_one::Soln::default())),
                part_two: Some(Box::new(year_2023::day_13::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2023, day: 14 },
            DailySolutions { 
                part_one: Some(Box::new(year_2023::day_14::part_one::Soln::default())),
                part_two: Some(Box::new(year_2023::day_14::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2023, day: 15 },
            DailySolutions { 
                part_one: Some(Box::new(year_2023::day_15::part_one::Soln::default())),
                part_two: Some(Box::new(year_2023::day_15::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2023, day: 16 },
            DailySolutions { 
                part_one: Some(Box::new(year_2023::day_16::part_one::Soln::default())),
                part_two: Some(Box::new(year_2023::day_16::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2023, day: 17 },
            DailySolutions { 
                part_one: Some(Box::new(year_2023::day_17::part_one::Soln::default())),
                part_two: Some(Box::new(year_2023::day_17::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2023, day: 18 },
            DailySolutions { 
                part_one: Some(Box::new(year_2023::day_18::part_one::Soln::default())),
                part_two: Some(Box::new(year_2023::day_18::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2023, day: 19 },
            DailySolutions { 
                part_one: Some(Box::new(year_2023::day_19::part_one::Soln::default())),
                part_two: Some(Box::new(year_2023::day_19::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2023, day: 20 },
            DailySolutions { 
                part_one: Some(Box::new(year_2023::day_20::part_one::Soln::default())),
                part_two: Some(Box::new(year_2023::day_20::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2023, day: 21 },
            DailySolutions { 
                part_one: Some(Box::new(year_2023::day_21::part_one::Soln::default())),
                part_two: Some(Box::new(year_2023::day_21::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2023, day: 22 },
            DailySolutions { 
                part_one: Some(Box::new(year_2023::day_22::part_one::Soln::default())),
                part_two: Some(Box::new(year_2023::day_22::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2023, day: 23 },
            DailySolutions { 
                part_one: Some(Box::new(year_2023::day_23::part_one::Soln::default())),
                part_two: Some(Box::new(year_2023::day_23::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2023, day: 24 },
            DailySolutions { 
                part_one: Some(Box::new(year_2023::day_24::part_one::Soln::default())),
                part_two: Some(Box::new(year_2023::day_24::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2023, day: 25 },
            DailySolutions { 
                part_one: Some(Box::new(year_2023::day_25::part_one::Soln::default())),
                part_two: None,
            },
        ),
        (
            Day { year: 2024, day: 1 },
            DailySolutions { 
                part_one: Some(Box::new(year_2024::day_01::part_one::Soln::default())),
                part_two: Some(Box::new(year_2024::day_01::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2024, day: 2 },
            DailySolutions { 
                part_one: Some(Box::new(year_2024::day_02::part_one::Soln::default())),
                part_two: Some(Box::new(year_2024::day_02::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2024, day: 3 },
            DailySolutions { 
                part_one: Some(Box::new(year_2024::day_03::part_one::Soln::default())),
                part_two: Some(Box::new(year_2024::day_03::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2024, day: 4 },
            DailySolutions { 
                part_one: Some(Box::new(year_2024::day_04::part_one::Soln::default())),
                part_two: Some(Box::new(year_2024::day_04::part_two::Soln::default())),
            },
        ),
        (
            Day { year: 2024, day: 5 },
            DailySolutions { 
                part_one: Some(Box::new(year_2024::day_05::part_one::Soln::default())),
                part_two: None,
            },
        ),
    ]);
    let daily_solns = match daily_solutions.remove(day) {
        Some(daily_solns) => daily_solns,
        None => {
            let mut days: Vec<Day> = daily_solutions.into_keys() // Consumes daily_solutions
                .collect();
            days.sort();
            let mut output: String = String::from("\nNo solutions found for this day. Days with solutions:\n");
            let mut years_seen: HashSet<u32> = HashSet::new();
            let tab_width: usize = 2;
            days.iter()
                .for_each(|day| {
                    if !years_seen.contains(&day.year) {
                        years_seen.insert(day.year);
                        output.push_str(&format!("{:>tab_width$}Year: {}\n"," ", day.year, tab_width = tab_width));
                        output.push_str(&format!("{:>tab_width$}Days:\n", "", tab_width = 2 * tab_width));
                    }
                    output.push_str(&format!("{:>tab_width$}\n", day.day, tab_width = 3 * tab_width + 1));
                });
            println!("{output}");
            panic!();
        },
    };
    daily_solns
}