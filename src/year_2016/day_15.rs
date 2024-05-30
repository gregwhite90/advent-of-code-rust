#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 15 };

pub mod part_one {
    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Disc {
        index: u64,
        positions: u64,
        starting_position: u64,
    }

    impl Disc {
        fn position(&self, button_push_time: u64) -> u64 {
            (self.starting_position + button_push_time + self.index) % self.positions
        }
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        discs: Vec<Disc>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U64(self.min_button_push())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let re = Regex::new(r"Disc \#(?<index>\d+) has (?<positions>\d+) positions\; at time\=0, it is at position (?<starting_position>\d+)\.").unwrap();
            self.discs = io_utils::file_to_lines(filename).map(|line| {
                let captures = re.captures(&line).unwrap();
                let index = captures.name("index").unwrap().as_str().parse().unwrap();
                let positions = captures.name("positions").unwrap().as_str().parse().unwrap();
                let starting_position = captures.name("starting_position").unwrap().as_str().parse().unwrap();
                Disc { index, positions, starting_position }
            }).collect()
        }

        fn min_button_push(&self) -> u64 {
            // Note: this calculation is not guaranteed to work if the disc indices get higher than the number of maximum
            // positions. A more general solution would use signed integers and rem_euclid (Rust's modulo) function.
            let max_positions_disc = self.discs.iter().max_by_key(|disc| disc.positions).unwrap();
            let time_to_pos_0 = max_positions_disc.positions - max_positions_disc.starting_position;
            let mut t = if time_to_pos_0 >= max_positions_disc.index { 
                time_to_pos_0 - max_positions_disc.index 
            } else {
                time_to_pos_0 + max_positions_disc.positions - max_positions_disc.index
            };
            loop {
                if self.discs.iter().all(|disc| disc.position(t) == 0) { return t; }
                t += max_positions_disc.positions;
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U64(5); "example_1")]
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
    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Disc {
        index: u64,
        positions: u64,
        starting_position: u64,
    }

    impl Disc {
        fn position(&self, button_push_time: u64) -> u64 {
            (self.starting_position + button_push_time + self.index) % self.positions
        }
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        discs: Vec<Disc>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U64(self.min_button_push())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let re = Regex::new(r"Disc \#(?<index>\d+) has (?<positions>\d+) positions\; at time\=0, it is at position (?<starting_position>\d+)\.").unwrap();
            self.discs = io_utils::file_to_lines(filename).map(|line| {
                let captures = re.captures(&line).unwrap();
                let index = captures.name("index").unwrap().as_str().parse().unwrap();
                let positions = captures.name("positions").unwrap().as_str().parse().unwrap();
                let starting_position = captures.name("starting_position").unwrap().as_str().parse().unwrap();
                Disc { index, positions, starting_position }
            }).collect();
            self.discs.push(Disc { index: (self.discs.len() + 1).try_into().unwrap(), positions: 11, starting_position: 0 });
        }

        fn min_button_push(&self) -> u64 {
            // Note: this calculation is not guaranteed to work if the disc indices get higher than the number of maximum
            // positions. A more general solution would use signed integers and rem_euclid (Rust's modulo) function.
            let max_positions_disc = self.discs.iter().max_by_key(|disc| disc.positions).unwrap();
            let time_to_pos_0 = max_positions_disc.positions - max_positions_disc.starting_position;
            let mut t = if time_to_pos_0 >= max_positions_disc.index { 
                time_to_pos_0 - max_positions_disc.index 
            } else {
                time_to_pos_0 + max_positions_disc.positions - max_positions_disc.index
            };
            loop {
                if self.discs.iter().all(|disc| disc.position(t) == 0) { return t; }
                t += max_positions_disc.positions;
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U64(5); "example_1")]
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