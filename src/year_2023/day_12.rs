#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 12 };

mod utils {
    use std::{collections::{HashMap, VecDeque}, iter};

    use crate::utils::io_utils;

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    enum Condition {
        Operational,
        Damaged,
        Unknown,
    }

    impl Condition {
        fn from_char(input: char) -> Self {
            match input {
                '.' => Self::Operational,
                '#' => Self::Damaged,
                '?' => Self::Unknown,
                _ => panic!("Unrecognized spring condition."),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    struct Group {
        conditions: Vec<Condition>,
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    struct ConditionRecord {
        groups: VecDeque<Group>,
        needed: VecDeque<u64>,
    }

    fn repeat_pattern(pattern: &str, num_repeats: usize, sep: &str) -> String {
        itertools::join(
            iter::repeat(pattern).take(num_repeats),
            sep,
        )
    }

    impl ConditionRecord {
        fn from_str(line: &str, num_repeats: usize) -> Self {
            let mut parts_iter = line.split(' ');
            let groups = repeat_pattern(parts_iter.next().unwrap(), num_repeats, "?")
                .split('.')
                .filter(|s| !s.is_empty())
                .map(|s| {
                    Group {
                        conditions: s.chars()
                            .map(|ch| Condition::from_char(ch))
                            .collect()
                    }
                })
                .collect();
            let needed = repeat_pattern(parts_iter.next().unwrap(), num_repeats, ",")
                .split(',')
                .map(|group| group.parse().unwrap())
                .collect();
            Self {
                groups,
                needed,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct ArrangementsCounter {
        num_repeats: usize,
        total_arrangements: u64,
        cache: HashMap<ConditionRecord, u64>,
    }

    impl ArrangementsCounter {
        pub fn with_num_repeats(num_repeats: usize) -> Self {
            Self {
                num_repeats,
                total_arrangements: 0,
                cache: HashMap::new(),
            }
        }

        fn arrangements_from_str(&mut self, line: &str) -> u64 {
            let condition_record = ConditionRecord::from_str(line, self.num_repeats);
            self.arrangements(condition_record)
        }

        /// Recursively calculates the number of arrangements for the given condition record
        /// (set of not-known-to-be operational condition groups and needed contiguous damaged blocks).
        fn arrangements(&mut self, cr: ConditionRecord) -> u64 {
            // base case: if no more damaged are needed
            if cr.needed.is_empty() {
                if cr.groups.iter().any(|group| {
                    group.conditions.iter().any(|condition| *condition == Condition::Damaged)
                }) {
                    return 0;
                } else {
                    return 1;
                }
            }
            match self.cache.get(&cr) {
                Some(arrangements) => return *arrangements,
                None => {
                    // base case: if exactly one group
                    if cr.groups.len() == 1 {
                        let mut groups = cr.groups.clone();
                        let group = groups.pop_back().expect("Should be exactly one group.");
                        if group.conditions.len() < (cr.needed.iter().sum::<u64>() as usize + cr.needed.len() - 1) {
                            // not enough room in the group for the needed damaged groups
                            return 0;
                        } else {
                            let mut arr: u64 = 0;
                            // try making the entire first needed block at the start of the group
                            let mut new_needed = cr.needed.clone();
                            let first_needed: usize = new_needed.pop_front().expect("Should be at least one needed.").try_into().unwrap();
                            if group.conditions.len() == first_needed {
                                if !new_needed.is_empty() {
                                    return 0;
                                } else {
                                    return 1;
                                }
                            }
                            if *group.conditions.get(first_needed).expect("Conditions should be long enough.") == Condition::Unknown {
                                let mut new_conditions = group.conditions.clone();
                                let new_group = Group {
                                    conditions: new_conditions.split_off(first_needed + 1),
                                };
                                let new_cr = ConditionRecord {
                                    groups: VecDeque::from([new_group]),
                                    needed: new_needed,
                                };
                                arr += self.arrangements(new_cr);
                            }
                            // try making the first condition operational
                            if *group.conditions.get(0).expect("Should be at least one condition.") == Condition::Unknown {
                                let mut new_conditions = group.conditions.clone();
                                let new_group = Group {
                                    conditions: new_conditions.split_off(1),
                                };
                                let new_cr = ConditionRecord {
                                    groups: VecDeque::from([new_group]),
                                    needed: cr.needed.clone(),
                                };
                                arr += self.arrangements(new_cr);
                            }
                            self.cache.insert(cr.clone(), arr);                      
                            return arr;
                        }                
                    } else {
                        // recursive case: more than one group. Take the first group, try to fit all combinations of the needed into it. then recurse.
                        let mut arr = 0;
                        for num_needed in 0..=cr.needed.len() {
                            // first group, x needed. * rest of groups, other needed
                            let mut groups = cr.groups.clone();
                            let rem_groups = groups.split_off(1);
                            let mut needed = cr.needed.clone();
                            let rem_needed = needed.split_off(num_needed);
                            arr += self.arrangements(ConditionRecord { groups, needed }) * self.arrangements(ConditionRecord { groups: rem_groups, needed: rem_needed});
                        }
                        self.cache.insert(cr.clone(), arr);
                        return arr;
                    }
                }
            }
        }

        pub fn parse_input_file(&mut self, filename: &str) {
            self.total_arrangements = io_utils::file_to_lines(filename)
                .map(|line| self.arrangements_from_str(&line))
                .sum();
        }

        pub fn total_arrangements(&self) -> u64 {
            self.total_arrangements
        }
    }
}

/// Solution to [2023-12 part one](https://adventofcode.com/2023/day/12).
/// 
/// We need to find a way to break down the problem into smaller (memo-ized, recursive)
/// versions of the same problem, because a brute force solution checking all combinations
/// of `#` and `.` for each `?` will be $O(2^n)$ for each line, where $n$ is the number of `?`s.
/// 
/// Manually solved examples with notes on how this algorithm was developed:
/// 
/// | Condition record      | Contiguous groups of `#`  | Size blocks that can house contiguous groups of `#` |
/// | ----------------      | ------------------------  | --- |
/// | `???.###`             | `1,1,3`                   | `3 3`  `1,1 3`    |  Last block has to be 3.
/// | `.??..??...?##.`      | `1,1,3`                   |  Last block has to be 3. each of first 2 blocks has to be 1.     |
/// | `?#?#?#?#?#?#?#?`     | `1,3,1,6`                 |  All one block. First block has to be solo. Then the 3 needs to be completed. This is fully determined.    |
/// | `????.#...#...`       | `4,1,1`                   |  Also fully determined. could start from back and know that.     |
/// | `????.######..#####.` | `1,6,5`                   |  Same as above.     |
/// | `?###????????`        | `3,2,1`                   |  This becomes 2,1 in 7     |
/// 
pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::ArrangementsCounter;

    #[derive(Debug, PartialEq, Eq)]
    pub struct Soln {
        arrangements_counter: ArrangementsCounter,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self {
                arrangements_counter: ArrangementsCounter::with_num_repeats(1),
            }
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.arrangements_counter.parse_input_file(filename);
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U64(self.arrangements_counter.total_arrangements())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U64(21); "example_1")]
        #[test_case(11, Answer::U64(1); "example_11")]
        #[test_case(12, Answer::U64(4); "example_12")]
        #[test_case(13, Answer::U64(1); "example_13")]
        #[test_case(14, Answer::U64(1); "example_14")]
        #[test_case(15, Answer::U64(4); "example_15")]
        #[test_case(16, Answer::U64(10); "example_16")]
        #[test_case(21, Answer::U64(23); "input_0001")]
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
    use crate::utils::solution::{Answer, Solution};

    use super::utils::ArrangementsCounter;

    #[derive(Debug, PartialEq, Eq)]
    pub struct Soln {
        arrangements_counter: ArrangementsCounter,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self {
                arrangements_counter: ArrangementsCounter::with_num_repeats(5),
            }
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.arrangements_counter.parse_input_file(filename);
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U64(self.arrangements_counter.total_arrangements())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U64(525_152); "example_1")]
        #[test_case(11, Answer::U64(1); "example_11")]
        #[test_case(12, Answer::U64(16_384); "example_12")]
        #[test_case(13, Answer::U64(1); "example_13")]
        #[test_case(14, Answer::U64(16); "example_14")]
        #[test_case(15, Answer::U64(2_500); "example_15")]
        #[test_case(16, Answer::U64(506_250); "example_16")]
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