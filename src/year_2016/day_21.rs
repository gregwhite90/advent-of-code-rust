#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 21 };

mod utils {
    use lazy_static::lazy_static;
    use regex::Regex;

    use crate::utils::io_utils;

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
    enum Instruction {
        SwapPosition(usize, usize),
        SwapLetter(char, char),
        RotateLeft(usize),
        RotateRight(usize),
        RotateBasedOnPosition(char),
        ReversePositions(usize, usize),
        MovePosition(usize, usize),
    }

    impl Instruction {
        fn from_str(instruction: &str) -> Self {
            let operation_captures = OPERATION_RE.captures(instruction).unwrap();
            let operation = operation_captures.name("operation").unwrap().as_str();
            let args = operation_captures.name("args").unwrap().as_str();
            match operation {
                "swap position" => {
                    let args_captures = SWAP_POSITION_ARGS_RE.captures(args).unwrap();
                    let x = args_captures.name("x").unwrap().as_str().parse().unwrap();
                    let y = args_captures.name("y").unwrap().as_str().parse().unwrap();
                    Self::SwapPosition(x, y)
                },
                "swap letter" => {
                    let args_captures = SWAP_LETTER_ARGS_RE.captures(args).unwrap();
                    let x = args_captures.name("x").unwrap().as_str().chars().next().unwrap();
                    let y = args_captures.name("y").unwrap().as_str().chars().next().unwrap();
                    Self::SwapLetter(x, y)
                },
                "rotate left" => {
                    let args_captures = ROTATE_DIRECTION_ARGS_RE.captures(args).unwrap();
                    let steps = args_captures.name("steps").unwrap().as_str().parse().unwrap();
                    Self::RotateLeft(steps)
                },
                "rotate right" => {
                    let args_captures = ROTATE_DIRECTION_ARGS_RE.captures(args).unwrap();
                    let steps = args_captures.name("steps").unwrap().as_str().parse().unwrap();
                    Self::RotateRight(steps)
                },
                "rotate based on position of letter" => {
                    let args_captures = ROTATE_BASED_ON_POSITION_ARGS_RE.captures(args).unwrap();
                    let letter = args_captures.name("letter").unwrap().as_str().chars().next().unwrap();
                    Self::RotateBasedOnPosition(letter)
                },
                "reverse positions" => {
                    let args_captures = REVERSE_POSITIONS_ARGS_RE.captures(args).unwrap();
                    let start: usize = args_captures.name("start").unwrap().as_str().parse().unwrap();
                    let end: usize = args_captures.name("end").unwrap().as_str().parse().unwrap();
                    Self::ReversePositions(start, end)
                },
                "move position" => {
                    let args_captures = MOVE_POSITION_ARGS_RE.captures(args).unwrap();
                    let x: usize = args_captures.name("x").unwrap().as_str().parse().unwrap();
                    let y: usize = args_captures.name("y").unwrap().as_str().parse().unwrap();
                    Self::MovePosition(x, y)
                },
                _ => panic!("Unrecognized operation"),
            }
        }
    }

    #[derive(Debug)]
    pub struct PasswordScrambler {
        password: String,
        instructions: Vec<Instruction>,
    }

    impl PasswordScrambler {
        pub fn new(start: &str) -> Self {
            Self {
                password: start.to_string(),
                instructions: Vec::new(),
            }
        }

        pub fn parse_input_file(&mut self, filename: &str) {
            self.instructions = io_utils::file_to_lines(filename).map(|line|{
                Instruction::from_str(&line)
            }).collect();
        }

        pub fn scramble(&mut self) {
            let mut chars: Vec<char> = self.password.chars().collect();
            for instruction in self.instructions.iter() {
                perform_instruction(&mut chars, instruction, false);
            }
            self.password = chars.into_iter().collect();
        }

        pub fn unscramble(&mut self) {
            let mut chars: Vec<char> = self.password.chars().collect();
            for instruction in self.instructions.iter().rev() {
                perform_instruction(&mut chars, instruction, true);
            }
            self.password = chars.into_iter().collect();
        }

        pub fn password(&self) -> String {
            self.password.clone()
        }

        #[cfg(test)]
        pub fn scramble_to_vec(&mut self) -> Vec<String> {
            let mut steps = vec![self.password()];
            let mut chars: Vec<char> = self.password.chars().collect();
            for instruction in self.instructions.iter() {
                perform_instruction(&mut chars, instruction, false);
                steps.push(chars.iter().collect());
            }
            self.password = chars.into_iter().collect();
            steps   
        }

        #[cfg(test)]
        pub fn unscramble_to_vec(&mut self) -> Vec<String> {
            let mut steps = vec![self.password()];
            let mut chars: Vec<char> = self.password.chars().collect();
            for instruction in self.instructions.iter().rev() {
                perform_instruction(&mut chars, instruction, true);
                steps.push(chars.iter().collect());
            }
            self.password = chars.into_iter().collect();
            steps   
        }
    }

    fn perform_instruction(chars: &mut Vec<char>, instruction: &Instruction, reverse: bool) {
        match *instruction {
            Instruction::SwapPosition(x, y) => {
                chars.swap(x, y);
            },
            Instruction::SwapLetter(x, y) => {
                for ch in chars.iter_mut() {
                    if *ch == x { *ch = y }
                    else if *ch == y { *ch = x }
                }
            },
            // TODO: reverse
            Instruction::RotateLeft(steps) => {
                match reverse {
                    false => rotate(chars, steps),
                    true => rotate(chars, chars.len() - steps),
                }
            },
            Instruction::RotateRight(steps) => {
                let steps = steps % chars.len();
                match reverse {
                    false => rotate(chars, chars.len() - steps),
                    true => rotate(chars, steps),
                }
            },
            Instruction::RotateBasedOnPosition(letter) => {
                let position = chars.iter().position(|l| *l == letter).unwrap();
                match reverse {
                    false => {
                        let steps = (1 + position + if position >= 4 { 1 } else { 0 }) % chars.len();
                        rotate(chars, chars.len() - steps);
                    },
                    true => {
                        // Reverse scrambling details are dependent on the string length. This
                        // solution works for the puzzle input length of 8.
                        assert_eq!(chars.len(), 8);
                        let end_position = match position % 2 {
                            0 => {
                                let mut end_position = position;
                                while end_position <= chars.len() {
                                    end_position += chars.len();
                                }
                                end_position -= 2;
                                end_position /= 2;
                                end_position 
                            },
                            1 => {
                                (position - 1) / 2
                            },
                            _ => unreachable!(),
                        };
                        if end_position >= position {
                            let steps = end_position - position;
                            rotate(chars, chars.len() - steps);
                        } else {
                            let steps = position - end_position;
                            rotate(chars, steps);
                        }
                    },
                }
            },
            Instruction::ReversePositions(start, end) => {
                chars[start..=end].reverse();
            },
            Instruction::MovePosition(x, y) => {
                match reverse {
                    false => {
                        let ch = chars.remove(x);
                        chars.insert(y, ch);        
                    },
                    true => {
                        let ch = chars.remove(y);
                        chars.insert(x, ch);        
                    }
                }
            },
        }            
    }

    fn rotate(chars: &mut Vec<char>, steps: usize) {
        *chars = [&chars[steps..], &chars[..steps]].concat();
    }

    #[cfg(test)]
    mod tests {
        use crate::utils::io_utils;
        use super::*;
        use super::super::DAY;

        #[test]
        fn example_1_is_correct() {
            let mut password_scrambler = PasswordScrambler::new("bdfhgeca");
            password_scrambler.parse_input_file(&io_utils::input_filename(&DAY, io_utils::InputFileType::Input));
            password_scrambler.unscramble();
            assert_eq!(password_scrambler.password(), "abcdefgh".to_string());
        }

        #[test]
        fn it_reverses_correctly() {
            let mut password_scrambler = PasswordScrambler::new("abcdefgh");
            password_scrambler.parse_input_file(&io_utils::input_filename(&DAY, io_utils::InputFileType::Input));
            password_scrambler.scramble();
            password_scrambler.unscramble();
            assert_eq!(password_scrambler.password(), "abcdefgh".to_string());
        }

        #[test]
        fn steps_are_correct() {
            let mut forward = PasswordScrambler::new("abcdefgh");
            forward.parse_input_file(&io_utils::input_filename(&DAY, io_utils::InputFileType::Input));
            let forward_steps = forward.scramble_to_vec();
            let mut backward = PasswordScrambler::new("bdfhgeca");
            backward.parse_input_file(&io_utils::input_filename(&DAY, io_utils::InputFileType::Input));
            let backward_steps = backward.unscramble_to_vec();
            for i in 0..backward_steps.len() {
                assert_eq!(backward_steps[i], forward_steps[forward_steps.len() - 1 - i]);
            }
        }
    }    
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

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
            self.password_scrambler.parse_input_file(filename);
            self.password_scrambler.scramble();
            Answer::String(self.password_scrambler.password())
        }
    }

    impl Soln {
        fn new(password: &str) -> Self {
            Self {
                password_scrambler: PasswordScrambler::new(password),
            }
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

pub mod part_two {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::PasswordScrambler;

    #[derive(Debug)]
    pub struct Soln {
        password_scrambler: PasswordScrambler,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self::new("fbgdceah")
        }
    }
    
    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.password_scrambler.parse_input_file(filename);
            self.password_scrambler.unscramble();
            Answer::String(self.password_scrambler.password())
        }
    }

    impl Soln {
        fn new(password: &str) -> Self {
            Self {
                password_scrambler: PasswordScrambler::new(password),
            }
        }
    }
}