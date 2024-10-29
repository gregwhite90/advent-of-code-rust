#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 9 };

mod utils {
    use std::collections::HashMap;

    use regex::Regex;

    use crate::utils::io_utils;

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    struct Node {
        clockwise: usize,
        counterclockwise: usize
    }

    impl Node {
        fn clockwise(&self) -> usize {
            self.clockwise
        }

        fn counterclockwise(&self) -> usize {
            self.counterclockwise
        }

        fn set_clockwise(&mut self, clockwise: usize) {
            self.clockwise = clockwise;
        }

        fn set_counterclockwise(&mut self, counterclockwise: usize) {
            self.counterclockwise = counterclockwise;
        }
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct MarbleGame {
        players: usize,
        last_marble: usize,
        marbles: HashMap<usize, Node>,
        scores: HashMap<usize, usize>,
    }

    impl MarbleGame {
        pub fn parse_input_file(&mut self, filename: &str) {
            let input_re = Regex::new(r"(?<players>\d+) players; last marble is worth (?<points>\d+) points").unwrap();
            let input = io_utils::file_to_string(filename);
            let captures = input_re.captures(&input).unwrap();
            self.players = captures.name("players").unwrap().as_str().parse().unwrap();
            self.last_marble = captures.name("points").unwrap().as_str().parse().unwrap();
            self.marbles = HashMap::new();
            self.marbles.insert(0, Node { clockwise: 0, counterclockwise: 0 });
        }

        pub fn play(&mut self) {
            let mut cur_player = 0;
            let mut current_marble = 0;
            let mut next_marble = 1;
            while next_marble <= self.last_marble {
                if next_marble % 23 == 0 {
                    let mut score = next_marble;
                    for _ in 0..7 {
                        current_marble = self.marbles.get(&current_marble).unwrap().counterclockwise();
                    }
                    score += current_marble;
                    let counterclockwise = self.marbles.get(&current_marble).unwrap().counterclockwise();
                    let clockwise = self.marbles.get(&current_marble).unwrap().clockwise();
                    self.marbles.get_mut(&counterclockwise).unwrap().set_clockwise(clockwise);
                    self.marbles.get_mut(&clockwise).unwrap().set_counterclockwise(counterclockwise);
                    self.marbles.remove(&current_marble); // TODO: decide if need
                    current_marble = clockwise;
                    self.scores.entry(cur_player).and_modify(|s| *s += score).or_insert(score);
                } else {
                    let left = self.marbles.get(&current_marble).unwrap().clockwise();
                    let right = self.marbles.get(&left).unwrap().clockwise();
                    current_marble = next_marble;
                    self.marbles.insert(current_marble, Node { counterclockwise: left, clockwise: right });
                    self.marbles.get_mut(&left).unwrap().set_clockwise(current_marble);
                    self.marbles.get_mut(&right).unwrap().set_counterclockwise(current_marble);
                }
                cur_player = (cur_player + 1) % self.players;
                next_marble += 1;
            }
        }

        pub fn high_score(&self) -> usize {
            *self.scores.values().max().unwrap()
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::MarbleGame;

    #[derive(Debug, Default)]
    pub struct Soln {
        marble_game: MarbleGame,    
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.marble_game.parse_input_file(filename);
            self.marble_game.play();
            Answer::Usize(self.marble_game.high_score())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(32); "example_1")]
        #[test_case(2, Answer::Usize(8_317); "example_2")]
        #[test_case(3, Answer::Usize(146_373); "example_3")]
        #[test_case(4, Answer::Usize(2_764); "example_4")]
        #[test_case(5, Answer::Usize(54_718); "example_5")]
        #[test_case(6, Answer::Usize(37_305); "example_6")]
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