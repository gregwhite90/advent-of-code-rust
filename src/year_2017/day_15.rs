#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 15 };

mod utils {
    use regex::Regex;

    use crate::utils::io_utils;

    const FACTORS: [u64; 2] = [16_807, 48_271];
    const DIVISOR: u64 = 2_147_483_647;    
    const MULTIPLES_OF: [u64; 2] = [4, 8];

    #[derive(Clone, Copy)]
    pub enum Part {
        One,
        Two,
    }

    #[derive(Debug, Default)]
    pub struct Generator {
        value: u64,
        factor: u64,
        divisor: u64,
        multiple_of: u64,
    }
    
    impl Generator {
        pub fn new(id: usize, value: u64, part: Part) -> Self {
            Self {
                value,
                factor: FACTORS[id],
                divisor: DIVISOR,
                multiple_of: match part {
                    Part::One => 1,
                    Part::Two => MULTIPLES_OF[id],
                },
            }
        }

        pub fn step(&mut self) {
            self.value = (self.value * self.factor) % self.divisor;    
        }
        
        pub fn value_lowest_16_bits(&self) -> u64 {
            self.value % 2u64.pow(16)
        }

        pub fn multiple_of(&self) -> u64 {
            self.multiple_of
        }
    }

    pub trait Year2017Day15Solution {
        fn push_generator(&mut self, generator: Generator);
    }

    pub fn parse_input_file<T>(soln: &mut T, filename: &str, part: Part)
    where
        T: Year2017Day15Solution
    {
        let re = Regex::new(r"Generator [AB] starts with (?<value>\d+)").unwrap();
        io_utils::file_to_lines(filename).enumerate().for_each(|(i, line)| {
            let captures = re.captures(&line)
                .expect("Line should match regex.");
            let value: u64 = captures.name("value").unwrap().as_str().parse().unwrap();
            soln.push_generator(Generator::new(i, value, part));
        });
    }
}

pub mod part_one {
    use crate::utils::solution::{Solution, Answer};
    use super::utils::{Year2017Day15Solution, Generator, Part, self};

    const ITERATIONS: usize = 40_000_000;

    #[derive(Debug, Default)]
    pub struct Soln {
        generators: Vec<Generator>,
    }

    impl Year2017Day15Solution for Soln {
        fn push_generator(&mut self, generator: Generator) {
            self.generators.push(generator);
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            utils::parse_input_file(self, filename, Part::One);
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

pub mod part_two {
    use std::{sync::mpsc, thread};

    use crate::utils::solution::{Solution, Answer};
    use super::utils::{Year2017Day15Solution, Generator, Part, self};

    const ITERATIONS: usize = 5_000_000;
    
    #[derive(Debug, Default)]
    pub struct Soln {
        generators: Vec<Generator>,
    }

    impl Year2017Day15Solution for Soln {
        fn push_generator(&mut self, generator: Generator) {
            self.generators.push(generator);
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            utils::parse_input_file(self, filename, Part::Two);
            let mut matches: u32 = 0;

            let (tx_a, rx_a) = mpsc::channel();
            let (tx_b, rx_b) = mpsc::channel();

            let mut gen_b = self.generators.pop().unwrap();
            let mut gen_a = self.generators.pop().unwrap();

            let a = thread::spawn(move || {
                for _ in 0..ITERATIONS {
                    gen_a.step();
                    while gen_a.value_lowest_16_bits() % gen_a.multiple_of() != 0 {
                        gen_a.step();
                    }
                    tx_a.send(gen_a.value_lowest_16_bits()).unwrap();
                }                
            });

            let b = thread::spawn(move || {
                for _ in 0..ITERATIONS {
                    gen_b.step();
                    while gen_b.value_lowest_16_bits() % gen_b.multiple_of() != 0 {
                        gen_b.step();
                    }
                    tx_b.send(gen_b.value_lowest_16_bits()).unwrap();
                }                
            });

            for _ in 0..ITERATIONS {
                if rx_a.recv().unwrap() == rx_b.recv().unwrap() {
                    matches += 1;
                }
            }     
            a.join().unwrap();
            b.join().unwrap();

            Answer::U32(matches)
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(309); "example_1")]
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