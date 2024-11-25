#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2015, day: 9 };

mod utils {
    use std::{cmp::Reverse, collections::{BTreeSet, BinaryHeap, HashMap}};

    use regex::Regex;

    use crate::utils::io_utils;

    #[derive(Debug, Default)]
    pub struct Atlas {
        distances: HashMap<BTreeSet<String>, usize>,
    }


    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
    struct Path {
        dist: usize,
        city: String,
        remaining: BTreeSet<String>,
    }

    impl Path {
        fn is_finished(&self) -> bool {
            self.remaining.is_empty()
        }
    }

    impl Atlas {
        pub fn parse_input_file(&mut self, filename: &str) {
            let line_re = Regex::new(r"(?<src>\w+) to (?<dst>\w+) = (?<distance>\d+)").unwrap();
            for line in io_utils::file_to_lines(filename) {
                let captures = line_re.captures(&line).unwrap();
                let src = captures.name("src").unwrap().as_str().to_string();
                let dst = captures.name("dst").unwrap().as_str().to_string();
                let distance = captures.name("distance").unwrap().as_str().parse().unwrap();
                self.distances.insert(BTreeSet::from([src, dst]), distance);
            }
        }

        pub fn shortest_path(&self) -> usize {
            let all_cities: BTreeSet<String> = self.distances.keys().cloned().flatten().collect();
            let mut pq = BinaryHeap::from_iter(all_cities.iter().map(|city| {                
                let mut remaining = all_cities.clone();
                remaining.remove(city);
                Reverse(Path {
                    dist: 0,
                    city: city.clone(),
                    remaining,
                })
            }));
            while !pq.is_empty() {
                let Reverse(path) = pq.pop().unwrap();
                if path.is_finished() {
                    return path.dist;
                }
                path.remaining.iter().for_each(|city| {
                    let mut remaining = path.remaining.clone();
                    remaining.remove(city);
                    pq.push(Reverse(Path {
                        dist: path.dist + self.distances.get(&BTreeSet::from([path.city.clone(), city.clone()])).unwrap(),
                        city: city.clone(),
                        remaining,
                    }));
                })
            }
            panic!("Did not find a solution");
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::Atlas;


    #[derive(Debug, Default)]
    pub struct Soln {
        atlas: Atlas,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.atlas.parse_input_file(filename);
            Answer::Usize(self.atlas.shortest_path())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(605); "example_1")]
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