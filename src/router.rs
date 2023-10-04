use crate::{Args, Part};
use advent_of_code_rust::utils::{solution::Solution, io_utils::{self, InputFileType}, Day};
use advent_of_code_rust::year_2017;

pub fn run_solution(args: &Args) {

    let day = Day { year: args.year, day: args.day };

    let solns = get_solns(&day);
    let daily_solns = match solns {
        Some(daily_solns) => {
            daily_solns
        },
        None => {
            panic!("No solutions found for this day."); // TODO: possibly enumerate the valid days
        }
    };
    if !matches!(args.part, Part::Two) {
        match daily_solns.part_one {
            Some(mut p_one) => {
                println!("Part one:");
                println!("{}", p_one.solve());            
            },
            None => println!("No solution found for part one of this day."),
        }
    }
    if !matches!(args.part, Part::One) {
        match daily_solns.part_two {
            Some(mut p_two) => {
                println!("Part two:");
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

fn get_solns(day: &Day) -> Option<DailySolutions> {
    let mut daily_solutions: Option<DailySolutions> = match day {
        Day { year: 2017, day: 1 } => {
            let part_one = year_2017::day_01::part_one::Soln::default();
            let part_two = year_2017::day_01::part_two::Soln::default();
            Some(DailySolutions { 
                part_one: Some(Box::new(part_one)),
                part_two: Some(Box::new(part_two)),
            })
        },
        Day { year: 2017, day: 2 } => {
            let part_one = year_2017::day_02::part_one::Soln::default();
            let part_two = year_2017::day_02::part_two::Soln::default();
            Some(DailySolutions { 
                part_one: Some(Box::new(part_one)),
                part_two: Some(Box::new(part_two)),
            })
        },
        Day { year: 2017, day: 3 } => {
            let part_one = year_2017::day_03::part_one::Soln::default();
            let part_two = year_2017::day_03::part_two::Soln::default();
            Some(DailySolutions {
                part_one: Some(Box::new(part_one)),
                part_two: Some(Box::new(part_two)),
            })
        },
        Day { year: 2017, day: 4 } => {
            let part_one = year_2017::day_04::part_one::Soln::default();
            let part_two = year_2017::day_04::part_two::Soln::default();
            Some(DailySolutions {
                part_one: Some(Box::new(part_one)),
                part_two: Some(Box::new(part_two)),
            })
        },
        Day { year: 2017, day: 5 } => {
            let part_one = year_2017::day_05::part_one::Soln::default();
            let part_two = year_2017::day_05::part_two::Soln::default();
            Some(DailySolutions {
                part_one: Some(Box::new(part_one)),
                part_two: Some(Box::new(part_two)),
            })
        },
        Day { year: 2017, day: 6 } => {
            let part_one = year_2017::day_06::part_one::Soln::default();
            let part_two = year_2017::day_06::part_two::Soln::default();
            Some(DailySolutions {
                part_one: Some(Box::new(part_one)),
                part_two: Some(Box::new(part_two)),
            })
        },
        Day { year: 2017, day: 7 } => {
            let part_one = year_2017::day_07::part_one::Soln::default();
            let part_two = year_2017::day_07::part_two::Soln::default();
            Some(DailySolutions {
                part_one: Some(Box::new(part_one)),
                part_two: Some(Box::new(part_two)),
            })
        },
        Day { year: 2017, day: 8 } => {
            let part_one = year_2017::day_08::part_one::Soln::default();
            let part_two = year_2017::day_08::part_two::Soln::default();
            Some(DailySolutions {
                part_one: Some(Box::new(part_one)),
                part_two: Some(Box::new(part_two)),
            })
        },
        _ => None,
    };
    if let Some(ref mut solutions) = daily_solutions {
        let input_filename = io_utils::input_filename(&day, InputFileType::Input);
        if let Some(ref mut p_one) = solutions.part_one {
            p_one.parse_input_file(&input_filename);
        }
        if let Some(ref mut p_two) = solutions.part_two {
            p_two.parse_input_file(&input_filename);
        }
    };
    daily_solutions
}