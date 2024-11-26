#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2015, day: 13 };

mod utils {
    use std::collections::{BTreeSet, HashMap, HashSet};

    use itertools::Itertools;
    use regex::Regex;

    use crate::utils::io_utils;

    #[derive(Debug, Default)]
    pub struct Table {
        happiness_changes: HashMap<BTreeSet<String>, i64>,
    }

    impl Table {
        pub fn parse_input_file(&mut self, filename: &str) {
            let line_re = Regex::new(r"(?<name_0>\w+) would (?<op>(gain)|(lose)) (?<amount>\d+) happiness units by sitting next to (?<name_1>\w+).").unwrap();
            for line in io_utils::file_to_lines(filename) {
                let captures = line_re.captures(&line).unwrap();
                let name_0 = captures.name("name_0").unwrap().as_str().to_string();
                let name_1 = captures.name("name_1").unwrap().as_str().to_string();
                let mut amount: i64 = captures.name("amount").unwrap().as_str().parse().unwrap();
                let op = captures.name("op").unwrap().as_str();
                if op == "lose" {
                    amount *= -1;
                }
                let key = BTreeSet::from([name_0, name_1]);
                self.happiness_changes.entry(key).and_modify(|e| *e += amount).or_insert(amount);
            }
        }

        fn total_change_in_happiness(&self, seating: Vec<&String>) -> i64 {
            let edge_key = BTreeSet::from([seating[0].clone(), seating[seating.len() - 1].clone()]);
            self.happiness_changes.get(&edge_key).unwrap()
                + seating.windows(2).map(|w| {
                    let key = BTreeSet::from([w[0].clone(), w[1].clone()]);
                    self.happiness_changes.get(&key).unwrap()
                }).sum::<i64>()
        }

        pub fn max_change_in_happiness(&self) -> i64 {
            let all_names: HashSet<String> = HashSet::from_iter(self.happiness_changes.keys().cloned().flatten());
            all_names.iter().permutations(all_names.len()).map(|c| self.total_change_in_happiness(c)).max().unwrap()
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::Table;

    #[derive(Debug, Default)]
    pub struct Soln {
        table: Table,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.table.parse_input_file(filename);
            Answer::I64(self.table.max_change_in_happiness())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::I64(330); "example_1")]
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