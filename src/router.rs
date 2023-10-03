use either::*;
use crate::{Args, Part};
use crate::utils::utils::{Solution, InputFileType, input_filename};

pub fn run_solution(args: &Args) {

    let solns = get_solns(&args);
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
                for_both!(p_one.solve(), answer => println!("{answer}"));            
            },
            None => println!("No solution found for part one of this day."),
        }
    }
    if !matches!(args.part, Part::One) {
        match daily_solns.part_two {
            Some(mut p_two) => {
                println!("Part two:");
                for_both!(p_two.solve(), answer => println!("{answer}"));            
            },
            None => println!("No solution found for part two of this day."),
        }
    }
}

struct DailySolutions {
    part_one: Option<Box<dyn Solution>>,
    part_two: Option<Box<dyn Solution>>,
}

fn get_solns(args: &Args) -> Option<DailySolutions> {
    let mut daily_solutions: Option<DailySolutions> = match (args.year, args.day) {
        (2017, 1) => {
            let part_one = crate::year_2017::day_01::part_one::Soln::default();
            let part_two = crate::year_2017::day_01::part_two::Soln::default();
            Some(DailySolutions { 
                part_one: Some(Box::new(part_one)),
                part_two: Some(Box::new(part_two)),
            })
        },
        (2017, 2) => {
            let part_one = crate::year_2017::day_02::part_one::Soln::default();
            let part_two = crate::year_2017::day_02::part_two::Soln::default();
            Some(DailySolutions { 
                part_one: Some(Box::new(part_one)),
                part_two: Some(Box::new(part_two)),
            })
        },
        (2017, 3) => {
            let part_one = crate::year_2017::day_03::part_one::Soln::default();
            let part_two = crate::year_2017::day_03::part_two::Soln::default();
            Some(DailySolutions {
                part_one: Some(Box::new(part_one)),
                part_two: Some(Box::new(part_two)),
            })
        },
        (2017, 4) => {
            let part_one = crate::year_2017::day_04::part_one::Soln::default();
            let part_two = crate::year_2017::day_04::part_two::Soln::default();
            Some(DailySolutions {
                part_one: Some(Box::new(part_one)),
                part_two: Some(Box::new(part_two)),
            })
        },
        (2017, 5) => {
            let part_one = crate::year_2017::day_05::part_one::Soln::default();
            let part_two = crate::year_2017::day_05::part_two::Soln::default();
            Some(DailySolutions {
                part_one: Some(Box::new(part_one)),
                part_two: Some(Box::new(part_two)),
            })
        }
        _ => None,
    };
    if let Some(ref mut solutions) = daily_solutions {
        let input_filename = input_filename(args.year, args.day, InputFileType::Input);
        if let Some(ref mut p_one) = solutions.part_one {
            p_one.parse_input_file(&input_filename);
        }
        if let Some(ref mut p_two) = solutions.part_two {
            p_two.parse_input_file(&input_filename);
        }
    };
    daily_solutions
}