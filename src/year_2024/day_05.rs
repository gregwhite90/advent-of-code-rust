#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2024, day: 5 };

mod utils {
    use std::collections::{HashMap, HashSet};

    use crate::utils::io_utils;

    #[derive(Debug, Default)]
    pub struct PrintingDepartment {
        page_ordering_rules: HashMap<usize, HashSet<usize>>,
        updates: Vec<Vec<usize>>,
    }

    impl PrintingDepartment {
        pub fn parse_input_file(&mut self, filename: &str) {
            let mut lines = io_utils::file_to_lines(filename);
            // parse page ordering rules
            while let Some(line) = lines.next() {
                if line.is_empty() { break; }
                let l: Vec<usize> = line.split('|')
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect();
                self.page_ordering_rules.entry(l[0])
                    .and_modify(|nums| { nums.insert(l[1]); })
                    .or_insert(HashSet::from([l[1]]));
            }
            // parse updates
            self.updates = lines.map(|line| {
                line.split(',')
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect()
            }).collect();
        }

        pub fn is_in_correct_order(&self, update: &Vec<usize>) -> bool {
            let mut seen = HashSet::new();
            for num in update.iter() {
                if let Some(subsequents) = self.page_ordering_rules.get(num) {
                    if seen.intersection(subsequents).count() != 0 {
                        return false;
                    }
                }
                seen.insert(*num);
            }
            true
        }

        pub fn sum_of_mid_of_correct_updates(&self) -> usize {
            self.updates.iter()
                .filter(|update| self.is_in_correct_order(*update))
                .map(|update| update[(update.len() - 1) / 2])
                .sum()
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::PrintingDepartment;

    #[derive(Debug, Default)]
    pub struct Soln {
        printing_department: PrintingDepartment,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.printing_department.parse_input_file(filename);
            Answer::Usize(self.printing_department.sum_of_mid_of_correct_updates())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(143); "example_1")]
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