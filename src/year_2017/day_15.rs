#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 15 };

const FACTORS: [u64; 2] = [16_807, 48_271];
const DIVISOR: u64 = 2_147_483_647;
const ITERATIONS: usize = 40_000_000;

pub mod part_one {
    use regex::Regex;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    use super::FACTORS;
    use super::DIVISOR;
    use super::ITERATIONS;

    #[derive(Debug, Default)]
    struct Generator {
        value: u64,
        factor: u64,
        divisor: u64,
    }
    
    impl Generator {
        pub fn new(id: usize, value: u64) -> Self {
            Self {
                value,
                factor: FACTORS[id],
                divisor: DIVISOR,
            }
        }

        pub fn step(&mut self) {
            self.value = (self.value * self.factor) % self.divisor;    
        }
        
        pub fn value_lowest_16_bits(&self) -> u64 {
            self.value % 2u64.pow(16)
        }
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        generators: [Generator; 2],
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            let mut matches: u32 = 0;
            for _ in 0..ITERATIONS {
                self.generators[0].step();
                self.generators[1].step();
                if self.generators[0].value_lowest_16_bits() == self.generators[1].value_lowest_16_bits() {
                    matches += 1;
                }
            }        
            Answer::U32(matches)
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let re = Regex::new(r"Generator [AB] starts with (?<value>\d+)").unwrap();
            io_utils::file_to_lines(filename).enumerate().for_each(|(i, line)| {
                let captures = re.captures(&line)
                    .expect("Line should match regex.");
                let value: u64 = captures.name("value").unwrap().as_str().parse().unwrap();
                self.generators[i] = Generator::new(i, value);
            });
        }
    }
 
    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(588); "example_1")]
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