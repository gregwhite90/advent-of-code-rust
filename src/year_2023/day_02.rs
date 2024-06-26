#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 2 };

pub mod part_one {
    use regex::Regex;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        sum_of_possible_ids: u32,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.sum_of_possible_ids)
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let id_re = Regex::new(r"Game (?<id>\d+)").unwrap();
            let reveal_re = Regex::new(r"(?<val>\d+) (?<color>red|green|blue)").unwrap();
            'lines: for line in io_utils::file_to_lines(filename) {
                let mut split = line.split(":");
                let game = split.next().unwrap();
                let reveals = split.next().unwrap();
                for reveal in reveals.split([';', ',']) {
                    let captures = reveal_re.captures(reveal).unwrap();
                    let val: u32 = captures.name("val").unwrap().as_str().parse().unwrap(); 
                    let color = captures.name("color").unwrap().as_str();               
                    if val > match color {
                        "red" => MAX_RED,
                        "green" => MAX_GREEN,
                        "blue" => MAX_BLUE,
                        _ => panic!("Unrecognized color."),
                    } {
                        continue 'lines;
                    }
                }
                let id_captures = id_re.captures(game).unwrap();
                let id: u32 = id_captures.name("id").unwrap().as_str().parse().unwrap();
                self.sum_of_possible_ids += id;
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(8); "example_1")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::default(),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}

pub mod part_two {
    use std::cmp;

    use regex::Regex;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    #[derive(Debug, PartialEq, Eq, Default)]
    struct MinimumGame {
        red: u32,
        green: u32,
        blue: u32,
    }

    impl MinimumGame {
        fn handle_reveal(&mut self, reveal_re: &Regex, reveal: &str) {
            let captures = reveal_re.captures(reveal).unwrap();
            let val: u32 = captures.name("val").unwrap().as_str().parse().unwrap(); 
            let color = captures.name("color").unwrap().as_str();               
            match color {
                "red" => self.red = cmp::max(self.red, val),
                "green" => self.green = cmp::max(self.green, val),
                "blue" => self.blue = cmp::max(self.blue, val),
                _ => panic!("Unrecognized color."),
            }                        
        }

        fn power(&self) -> u32 {
            self.red * self.green * self.blue
        }
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        sum_of_powers: u32,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.sum_of_powers)
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let reveal_re = Regex::new(r"(?<val>\d+) (?<color>red|green|blue)").unwrap();
            self.sum_of_powers = io_utils::file_to_lines(filename)
                .map(|line| {
                    let mut min_game = MinimumGame::default();
                    let mut split = line.split(":");
                    let _ = split.next().unwrap();
                    let reveals = split.next().unwrap();    
                    reveals.split([';', ',']).for_each(|reveal| min_game.handle_reveal(&reveal_re, reveal));
                    min_game.power()
                })
                .sum();
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(2286); "example_1")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::default(),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}
