use either::*;
use crate::{Args, Part};

pub fn run_solution(args: Args) {
    use crate::utils::utils::Solution;

    // TODO: make these variables more generic (an enum?)
    let (part_one_soln, part_two_soln) = match (args.year, args.day) {
        (2017, 1) => {
            // TODO: error handling
            let mut part_one_soln = crate::year_2017::day_01::part_one::Soln::default();
            part_one_soln.parse_input_file("input/year_2017/day_01/input.txt");
            let mut part_two_soln = crate::year_2017::day_01::part_two::Soln::default();
            part_two_soln.parse_input_file("input/year_2017/day_01/input.txt");
            (part_one_soln, part_two_soln)
        },
        _ => panic!(), // TODO: handle the error case
    };
    if !matches!(args.part, Part::Two) {
        println!("Part one:");
        for_both!(part_one_soln.solve(), answer => println!("{answer}"));    
    }
    if !matches!(args.part, Part::One) {
        println!("Part two:");
        for_both!(part_two_soln.solve(), answer => println!("{answer}"));    
    }
}