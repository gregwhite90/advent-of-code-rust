use std::collections::{HashMap, HashSet};
use crate::{Args, Part};
use advent_of_code_rust::utils::{solution::Solution, io_utils::{self, InputFileType}, Day};
use advent_of_code_rust::year_2017;

const TAB_WIDTH: usize = 2;

pub fn run_solution(args: &Args) {

    let day = Day { year: args.year, day: args.day };

    let solns = get_solns(&day);
    let input_filename = io_utils::input_filename(&day, InputFileType::Input);

    if !matches!(args.part, Part::Two) {
        match solns.part_one {
            Some(mut p_one) => {
                p_one.parse_input_file(&input_filename);
                println!("Part one:");
                println!("{}", p_one.solve());            
            },
            None => println!("No solution found for part one of this day."),
        }
    }

    if !matches!(args.part, Part::One) {
        match solns.part_two {
            Some(mut p_two) => {
                println!("Part two:");
                p_two.parse_input_file(&input_filename);
                println!("{}", p_two.solve());            
            },
            None => println!("No solution found for part two of this day."),
        }
    }
}

struct DailySolutions {
    part_one: Option<Box<dyn Solution>>,
    part_two: Option<Box<dyn Solution>>,
}

fn get_solns(day: &Day) -> DailySolutions {
    // Mutable because we will later move out the daily solutions to be able to return them.
    let mut daily_solutions: HashMap<Day, DailySolutions> = HashMap::from([
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
    ]);
    let daily_solns = match daily_solutions.remove(day) {
        Some(daily_solns) => daily_solns,
        None => {
            let mut days: Vec<Day> = daily_solutions.into_keys() // Consumes daily_solutions
                .collect();
            days.sort();
            let mut output: String = String::from("\nNo solutions found for this day. Days with solutions:\n");
            let mut years_seen: HashSet<u32> = HashSet::new();
            days.iter()
                .for_each(|day| {
                    if !years_seen.contains(&day.year) {
                        years_seen.insert(day.year);
                        output.push_str(&format!("{:>tab_width$}Year: {}\n"," ", day.year, tab_width = TAB_WIDTH));
                        output.push_str(&format!("{:>tab_width$}Days:\n", "", tab_width = 2 * TAB_WIDTH));
                    }
                    output.push_str(&format!("{:>tab_width$}\n", day.day, tab_width = 3 * TAB_WIDTH + 1));
                });
            println!("{output}");
            panic!();
        },
    };
    daily_solns
}