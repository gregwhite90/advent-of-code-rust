/// Written 2023-10-19. Uses same logic as my solution to 
/// [2020 day 24](https://github.com/gregwhite90/Advent-of-Code-Python/blob/4742ecf363f52b2137123b14c21bcc06afe03bb9/solutions/2020/24/solution.py)

#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 11 };

pub mod part_one {
    use crate::utils::{io_utils, solution::{Solution, Answer}};

    enum Step {
        N,
        NE,
        SE,
        S,
        SW,
        NW,
    }

    impl Step {
        fn from_str(step: &str) -> Self {
            match step {
                "n"  => Self::N,
                "ne" => Self::NE,
                "se" => Self::SE,
                "s"  => Self::S,
                "sw" => Self::SW,
                "nw" => Self::NW,
                _ => panic!("Invalid input for step."),
            }
        }
    }

    /// A position reached by a sequence of steps is uniquely determined by two coordinates. Moves directly N/S move 2 steps
    #[derive(PartialEq, Eq, Debug, Default)]
    pub struct Soln {
        net_n: i32,
        net_e: i32,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::I32(self.steps())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            io_utils::file_to_string(filename)
                .split(",")
                .for_each(|step| self.step(step));
        }

        fn step(&mut self, step: &str) {
            let step = Step::from_str(step);
            match step {
                Step::N => self.net_n += 2,
                Step::NE => { self.net_n += 1; self.net_e += 1; },
                Step::SE => { self.net_n -= 1; self.net_e += 1; },
                Step::S => self.net_n -= 2,
                Step::SW => { self.net_n -= 1; self.net_e -= 1; },
                Step::NW => { self.net_n += 1; self.net_e -= 1; },
            }
        }

        fn steps(&self) -> i32 {
            self.net_e.abs() + (self.net_n.abs() - self.net_e.abs()) / 2
        }
    }
 
    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::I32(3); "example_1")]
        #[test_case(2, Answer::I32(0); "example_2")]
        #[test_case(3, Answer::I32(2); "example_3")]
        #[test_case(4, Answer::I32(3); "example_4")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
            let mut soln = Soln::default();
            test_utils::check_example_case(
                &mut soln,
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}