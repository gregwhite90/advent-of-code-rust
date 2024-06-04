#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 21 };

mod utils {
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        static ref OPERATION_RE: Regex = Regex::new(r"(?<operation>(swap position)|(swap letter)|(rotate left)|(rotate right)|(rotate based on position of letter)|(reverse positions)|(move position)) (?<args>.*)").unwrap();
        static ref SWAP_POSITION_ARGS_RE: Regex = Regex::new(r"(?<x>\d+) with position (?<y>\d+)").unwrap();
        static ref SWAP_LETTER_ARGS_RE: Regex = Regex::new(r"(?<x>[a-z]) with letter (?<y>[a-z])").unwrap();
        static ref ROTATE_DIRECTION_ARGS_RE: Regex = Regex::new(r"(?<steps>\d+) steps?").unwrap();
        static ref ROTATE_BASED_ON_POSITION_ARGS_RE: Regex = Regex::new(r"(?<letter>[a-z])").unwrap();
        static ref REVERSE_POSITIONS_ARGS_RE: Regex = Regex::new(r"(?<start>\d+) through (?<end>\d+)").unwrap();
        static ref MOVE_POSITION_ARGS_RE: Regex = Regex::new(r"(?<x>\d+) to position (?<y>\d+)").unwrap();
    }

    #[derive(Debug)]
    pub struct PasswordScrambler {
        password: String,
    }

    impl PasswordScrambler {
        pub fn new(start: &str) -> Self {
            Self {
                password: start.to_string(),
            }
        }

        pub fn perform_instruction(&mut self, instruction: &str) {
            let operation_captures = OPERATION_RE.captures(instruction).unwrap();
            let operation = operation_captures.name("operation").unwrap().as_str();
            let args = operation_captures.name("args").unwrap().as_str();

            // TODO: fix up. Mutate the chars vec to be reassigned to password field
            match operation {
                "swap position" => {
                    let args_captures = SWAP_POSITION_ARGS_RE.captures(args).unwrap();
                    let x = args_captures.name("x").unwrap().as_str().parse().unwrap();
                    let y = args_captures.name("y").unwrap().as_str().parse().unwrap();
                    let mut chars: Vec<char> = self.password.chars().collect();
                    chars.swap(x, y);
                    self.password = chars.into_iter().collect();
                },
                "swap letter" => {
                    let args_captures = SWAP_LETTER_ARGS_RE.captures(args).unwrap();
                    let x = args_captures.name("x").unwrap().as_str().chars().next().unwrap();
                    let y = args_captures.name("y").unwrap().as_str().chars().next().unwrap();
                    let mut chars: Vec<char> = self.password.chars().collect();
                    chars = chars.into_iter().map(|ch| {
                        if ch == x { y }
                        else if ch == y { x }
                        else { ch }
                    }).collect();
                    self.password = chars.into_iter().collect();
                },
                "rotate left" => {
                    let args_captures = ROTATE_DIRECTION_ARGS_RE.captures(args).unwrap();
                    let steps: usize = args_captures.name("steps").unwrap().as_str().parse().unwrap();
                    // TODO: mod math
                    let mut chars: Vec<char> = self.password.chars().collect();
                    chars = [&chars[steps..], &chars[..steps]].concat();
                    self.password = chars.into_iter().collect();
                },
                "rotate right" => {
                    let args_captures = ROTATE_DIRECTION_ARGS_RE.captures(args).unwrap();
                    let mut steps: usize = args_captures.name("steps").unwrap().as_str().parse().unwrap();
                    // TODO: mod math
                    let mut chars: Vec<char> = self.password.chars().collect();
                    while steps > chars.len() {
                        steps -= chars.len();
                    }
                    chars = [&chars[chars.len() - steps..], &chars[..chars.len() - steps]].concat();
                    self.password = chars.into_iter().collect();
                },
                "rotate based on position of letter" => {
                    let args_captures = ROTATE_BASED_ON_POSITION_ARGS_RE.captures(args).unwrap();
                    let letter = args_captures.name("letter").unwrap().as_str().chars().next().unwrap();
                    let mut chars: Vec<char> = self.password.chars().collect();
                    let position = chars.iter().position(|l| *l == letter).unwrap();
                    let mut steps = 1 + position + if position >= 4 { 1 } else { 0 };
                    while steps > chars.len() {
                        steps -= chars.len();
                    }
                    // TODO: mod math
                    // TODO: DRY with a rotate_right function
                    chars = [&chars[chars.len() - steps..], &chars[..chars.len() - steps]].concat();
                    self.password = chars.into_iter().collect();
                },
                "reverse positions" => {
                    let args_captures = REVERSE_POSITIONS_ARGS_RE.captures(args).unwrap();
                    let start: usize = args_captures.name("start").unwrap().as_str().parse().unwrap();
                    let end: usize = args_captures.name("end").unwrap().as_str().parse().unwrap();
                    self.password = self.password[..start].chars()
                        .chain(self.password[start..=end].chars().rev())
                        .chain(self.password[end + 1..].chars())
                        .collect();
                },
                "move position" => {
                    let args_captures = MOVE_POSITION_ARGS_RE.captures(args).unwrap();
                    let x: usize = args_captures.name("x").unwrap().as_str().parse().unwrap();
                    let y: usize = args_captures.name("y").unwrap().as_str().parse().unwrap();
                    let mut chars: Vec<char> = self.password.chars().collect();
                    let ch = chars.remove(x);
                    chars.insert(y, ch);
                    self.password = chars.into_iter().collect();
                },
                _ => panic!("Unrecognized operation"),
            }
        }

        pub fn password(&self) -> String {
            self.password.clone()
        }
    }
}

pub mod part_one {
    use crate::utils::{io_utils, solution::{Answer, Solution}};

    use super::utils::PasswordScrambler;

    #[derive(Debug)]
    pub struct Soln {
        password_scrambler: PasswordScrambler,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self::new("abcdefgh")
        }
    }
    
    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::String(self.password_scrambler.password())
        }
    }

    impl Soln {
        fn new(password: &str) -> Self {
            Self {
                password_scrambler: PasswordScrambler::new(password),
            }
        }

        fn parse_input_file(&mut self, filename: &str) {
            io_utils::file_to_lines(filename).for_each(|line| {
                self.password_scrambler.perform_instruction(&line);
            });
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::String("decab".to_string()); "example_1")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::new("abcde"),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}