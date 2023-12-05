#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 4 };

pub mod part_one {
    use std::collections::HashSet;

    use regex::{Regex, Captures};

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    /// This implementation assumes that numbers on the card are unique
    /// and winning numbers are unique.
    struct Card {
        numbers: HashSet<u32>,
        winning_numbers: HashSet<u32>,
    }

    impl Card {
        fn points(&self) -> u32 {
            let matches = self.numbers.intersection(&self.winning_numbers).count() as u32;
            if matches == 0 { return 0; }
            2u32.pow(matches - 1)            
        }
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        sum_of_card_points: u32,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.sum_of_card_points())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let re = Regex::new(r"Card[\s\d]+:(?<numbers>[\s\d]+)\|(?<winning_numbers>[\s\d]+)").unwrap();
            self.sum_of_card_points = io_utils::file_to_lines(filename)
                .map(|line| {
                    let captures = re.captures(&line).unwrap();
                    let numbers = to_set(&captures, "numbers");
                    let winning_numbers = to_set(&captures, "winning_numbers");
                    let card = Card { numbers, winning_numbers };
                    card.points()
                })
                .sum()
        }

        fn sum_of_card_points(&self) -> u32 {
            self.sum_of_card_points
        }
    }

    fn to_set(captures: &Captures, name: &str) -> HashSet<u32> {
        captures.name(name)
            .unwrap()
            .as_str()
            .split(" ")
            .filter(|part| part.parse::<u32>().is_ok())
            .map(|part| part.parse().unwrap())
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(13); "example_1")]
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
    use std::collections::{HashSet, HashMap};

    use regex::{Regex, Captures};

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    /// This implementation assumes that numbers on the card are unique
    /// and winning numbers are unique. It also assumes that the card IDs
    /// are contiguous.
    struct Card {
        numbers: HashSet<u32>,
        winning_numbers: HashSet<u32>,
    }

    // TODO: implement in a shared way?
    impl Card {
        fn matches(&self) -> usize {
            self.numbers.intersection(&self.winning_numbers).count()
        }
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        max_id: u32,
        instances: HashMap<u32, u32>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.sum_of_instances())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let re = Regex::new(r"Card (?<id>[\s\d]+):(?<numbers>[\s\d]+)\|(?<winning_numbers>[\s\d]+)").unwrap();
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    let captures = re.captures(&line).unwrap();
                    let id: u32 = captures.name("id").unwrap().as_str().trim().parse().unwrap();
                    self.max_id = id;
                    let instances = self.instances.entry(id)
                        .and_modify(|instances| *instances += 1)
                        .or_insert(1)
                        .clone();
                    let numbers = to_set(&captures, "numbers");
                    let winning_numbers = to_set(&captures, "winning_numbers");
                    let card = Card { numbers, winning_numbers };
                    let matches = card.matches() as u32;
                    for i in id + 1..=id + matches {
                        self.instances.entry(i)
                            .and_modify(|insts| *insts += instances)
                            .or_insert(instances);
                    }
                });
        }

        fn sum_of_instances(&self) -> u32 {
            self.instances.iter()
                .filter(|(id, _instances)| **id <= self.max_id)
                .map(|(_id, instances)| instances)
                .sum()
        }
    }

    fn to_set(captures: &Captures, name: &str) -> HashSet<u32> {
        captures.name(name)
            .unwrap()
            .as_str()
            .split(" ")
            .filter(|part| part.parse::<u32>().is_ok())
            .map(|part| part.parse().unwrap())
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(30); "example_1")]
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