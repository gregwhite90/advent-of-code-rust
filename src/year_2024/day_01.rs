#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2024, day: 1 };

pub mod part_one {
    use std::{collections::BinaryHeap, iter::zip};

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, Default)]
    pub struct Soln {}

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            let mut l: BinaryHeap<usize> = BinaryHeap::new();
            let mut r: BinaryHeap<usize> = BinaryHeap::new();
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    let components: Vec<&str> = line.split_whitespace().collect();
                    l.push(components[0].parse().unwrap());
                    r.push(components[1].parse().unwrap());
                });
            Answer::Usize(
                zip(l.into_sorted_vec().into_iter(), r.into_sorted_vec().into_iter())
                    .map(|(l_elem, r_elem)| l_elem.abs_diff(r_elem))
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

        #[test_case(1, Answer::Usize(11); "example_1")]
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

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, Default)]
    pub struct Soln {}

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            // Counters
            let mut l: HashMap<usize, usize> = HashMap::new();
            let mut r: HashMap<usize, usize> = HashMap::new();
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    let components: Vec<&str> = line.split_whitespace().collect();
                    l.entry(components[0].parse().unwrap()).and_modify(|count| *count += 1).or_insert(1);
                    r.entry(components[1].parse().unwrap()).and_modify(|count| *count += 1).or_insert(1);
                });
            Answer::Usize(
                l.iter().map(|(num, l_count)| {
                    if let Some(r_count) = r.get(num) {
                        num * l_count * r_count
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

        #[test_case(1, Answer::Usize(31); "example_1")]
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