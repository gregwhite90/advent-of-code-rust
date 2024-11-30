#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2015, day: 17 };

mod utils {
    use std::collections::{BTreeMap, HashMap};

    use crate::utils::io_utils;

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    struct CacheKey {
        containers: BTreeMap<usize, usize>,
        remaining: usize,
    }

    impl CacheKey {
        fn new(containers: &[usize], remaining: usize) -> Self {
            let mut containers_map: BTreeMap<usize, usize> = BTreeMap::new();
            containers.iter()
                .filter(|size| **size <= remaining)
                .for_each(|size| {
                    containers_map.entry(*size).and_modify(|count| *count += 1).or_insert(1);
                });
            Self {
                containers: containers_map,
                remaining,
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct Distributor {
        containers: Vec<usize>, // Sorted, descending
        cache: HashMap<CacheKey, usize>,
    }

    impl Distributor {
        pub fn parse_input_file(&mut self, filename: &str) {
            self.containers = io_utils::file_to_lines(filename).map(|line| line.parse().unwrap()).collect();
            self.containers.sort();
            self.containers.reverse();
        }

        pub fn num_combos(&mut self, amount: usize) -> usize {
            let containers = self.containers.clone();
            self.num_combos_recursive(&containers, amount)
        }

        // TODO: I think I need to track the length of the solutions. This is counting every single
        // combination as if the ordering matters.
        /// Returns a map of length to number of combos of that length 
        fn num_combos_recursive(&mut self, mut containers: &[usize], remaining: usize) -> usize {
            // Base cases
            if remaining == 0 { return 1; }
            if let Some(idx) = containers.iter().enumerate()
                .filter_map(|(idx, size)| {
                    if *size <= remaining {
                        Some(idx)
                    } else {
                        None
                    }
                })
                .min() {
                    containers = &containers[idx..];
                } else {
                    return 0;   
                }
            if containers.iter().sum::<usize>() < remaining { return 0; }
            if containers.len() == 1 && containers[0] == remaining { return 1; }
            // TODO: decide if this is overcomplicating
            // Check memoization
            let cache_key = CacheKey::new(&containers, remaining);
            if let Some(cached) = self.cache.get(&cache_key) { 
                return *cached;
            }
            let res = self.num_combos_recursive(&containers[1..], remaining - containers[0])
                + self.num_combos_recursive(&containers[1..], remaining);
            self.cache.insert(cache_key, res);
            res
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::Distributor;

    #[derive(Debug)]
    pub struct Soln {
        amount: usize,
        distributor: Distributor,
    }

    impl Soln {
        fn with_amount(amount: usize) -> Self {
            Self {
                amount,
                distributor: Distributor::default(),
            }
        }
    }

    impl Default for Soln {
        fn default() -> Self {
            Self::with_amount(150)
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.distributor.parse_input_file(filename);
            Answer::Usize(self.distributor.num_combos(self.amount))
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(4); "example_1")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::with_amount(25),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}