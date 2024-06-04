#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 12 };

pub mod part_one {
    use crate::{utils::solution::{Answer, Solution}, year_2016::utils::assembunny_computer::AssembunnyComputer};

    #[derive(Debug, Default)]
    pub struct Soln {
        assembunny_computer: AssembunnyComputer,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer{
            self.assembunny_computer.parse_input_file(filename);
            self.assembunny_computer.execute_all();
            Answer::I64(self.assembunny_computer.register_value('a'))
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::I64(42); "example_1")]
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
    use std::collections::HashMap;

    use crate::{utils::solution::{Answer, Solution}, year_2016::utils::assembunny_computer::AssembunnyComputer};

    #[derive(Debug)]
    pub struct Soln {
        assembunny_computer: AssembunnyComputer,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer{
            self.assembunny_computer.parse_input_file(filename);
            self.assembunny_computer.execute_all();
            Answer::I64(self.assembunny_computer.register_value('a'))
        }
    }

    impl Default for Soln {
        fn default() -> Self {
            Self {
                assembunny_computer: AssembunnyComputer::with_registers(HashMap::from([('c', 1)])),
            }
        }
    }
}