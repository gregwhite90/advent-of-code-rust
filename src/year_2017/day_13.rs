#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 13 };

mod utils {
    use regex::Regex;

    use crate::utils::io_utils;

    #[derive(PartialEq, Eq, Debug)]
    pub struct Scanner {
        depth: u32,
        range: u32,
        period: u32,
    }

    impl Scanner {
        pub fn new(depth: u32, range: u32) -> Self {
            Self {
                depth,
                range,
                period: (range - 1) * 2,
            }
        }

        pub fn period(&self) -> u32 {
            self.period
        }

        pub fn severity(&self) -> u32 {
            self.depth * self.range
        }
    }

    pub trait Year2017Day13Solution {
        fn add_scanner(&mut self, depth: u32, range: u32);
    }
    
    pub fn parse_input_file<T>(soln: &mut T, filename: &str)
    where
        T: Year2017Day13Solution
    {
        let re = Regex:: new(r"(?<depth>\d+): (?<range>\d+)").unwrap();
        io_utils::file_to_lines(filename)
            .for_each(|line| {
                let captures = re.captures(&line)
                    .expect("Line should match regex.");
                let depth: u32 = captures.name("depth").unwrap().as_str().parse().unwrap();
                let range: u32 = captures.name("range").unwrap().as_str().parse().unwrap();
                soln.add_scanner(depth, range);
            });
    }
}

pub mod part_two {
    use std::collections::HashMap;

    use crate::utils::solution::{Solution, Answer};
    use super::utils::{self, Scanner, Year2017Day13Solution};

    #[derive(PartialEq, Eq, Debug, Default)]
    pub struct Soln {
        depths_to_scanners: HashMap<u32, Scanner>,
    }

    impl Year2017Day13Solution for Soln {
        fn add_scanner(&mut self, depth: u32, range: u32) {
            self.depths_to_scanners.insert(depth, Scanner::new(depth, range));
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            utils::parse_input_file(self, filename);
            let mut delay = 0u32;
            'delay: loop {
                for (depth, scanner) in self.depths_to_scanners.iter() {
                    if scanner.period() == 0 {
                        panic!("Impassible scanner with period 0.")
                    } else if (delay + depth) % scanner.period() == 0 {
                        delay += 1;
                        continue 'delay;
                    }  
                }
                break;
            }
            Answer::U32(delay)
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(10); "example_1")]
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

pub mod part_one {
    use std::collections::HashMap;

    use crate::utils::solution::{Solution, Answer};
    use super::utils::{self, Scanner, Year2017Day13Solution};

    #[derive(PartialEq, Eq, Debug, Default)]
    pub struct Soln {
        depths_to_scanners: HashMap<u32, Scanner>,
    }

    impl Year2017Day13Solution for Soln {
        fn add_scanner(&mut self, depth: u32, range: u32) {
            self.depths_to_scanners.insert(depth, Scanner::new(depth, range));
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            utils::parse_input_file(self, filename);
            Answer::U32(
                self.depths_to_scanners.iter()
                    .map(|(depth, scanner)| {
                        if scanner.period() == 0 || *depth % scanner.period() == 0 {
                            scanner.severity()
                        } else {
                            0
                        }
                    })
                    .sum()
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(24); "example_1")]
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