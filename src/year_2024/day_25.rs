#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2024, day: 25 };

mod utils {
    use itertools::iproduct;

    use crate::utils::io_utils;

    const MAX_HEIGHT: usize = 5;

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    enum HeightsType {
        Key,
        Lock,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    struct Heights {
        heights_type: HeightsType,
        pin_heights: [Option<usize>; 5],
    }

    impl Heights {
        fn new(heights_type: HeightsType) -> Self {
            Self {
                heights_type,
                pin_heights: [None; 5],
            }
        }

        fn update_pin_height(&mut self, col: usize, height: usize) {
            if self.pin_heights[col] == None {
                self.pin_heights[col] = Some(height);
            }
        }

        fn update_all_nones(&mut self) {
            self.pin_heights.iter_mut()
                .filter(|height| **height == None)
                .for_each(|height| {
                    *height = Some(match self.heights_type {
                        HeightsType::Lock => MAX_HEIGHT,
                        HeightsType::Key => 0,
                    });
                });
        }

        fn fits(&self, other: &Self) -> bool {
            self.pin_heights.iter().zip(other.pin_heights.iter()).all(|(s, o)| {
                s.unwrap() + o.unwrap() <= MAX_HEIGHT
            })
        }
    }

    #[derive(Debug, Default)]
    pub struct Schematics {
        keys: Vec<Heights>,
        locks: Vec<Heights>,
    }

    impl Schematics {
        pub fn parse_input_file(&mut self, filename: &str) {
            let mut cur: Option<Heights> = None;
            let mut rows: usize = 0;
            io_utils::file_to_lines(filename).for_each(|line| {
                if line.is_empty() {
                    cur = None;
                    rows = 0;
                } else if cur == None {
                    if line == "#####" {
                        cur = Some(Heights::new(HeightsType::Lock));
                    } else if line == "....." {
                        cur = Some(Heights::new(HeightsType::Key));
                    } else {
                        panic!("Unrecognized starting row");
                    }
                } else {
                    if cur.as_ref().unwrap().heights_type == HeightsType::Lock {
                        if line == "....." {
                            cur.as_mut().unwrap().update_all_nones();
                            self.locks.push(cur.as_ref().unwrap().clone());
                        } else {
                            line.char_indices()
                                .filter(|(_idx, ch)| {
                                    *ch == '.'
                                })
                                .for_each(|(col, _ch)| {
                                    cur.as_mut().unwrap().update_pin_height(col, rows);
                                });
                        }
                    } else {
                        assert_eq!(cur.as_ref().unwrap().heights_type, HeightsType::Key);
                        if line == "#####" {
                            cur.as_mut().unwrap().update_all_nones();
                            self.keys.push(cur.as_ref().unwrap().clone());
                        } else {
                            line.char_indices()
                                .filter(|(_idx, ch)| {
                                    *ch == '#'
                                })
                                .for_each(|(col, _ch)| {
                                    cur.as_mut().unwrap().update_pin_height(col, MAX_HEIGHT - rows);
                                });
                        }
                    }
                    rows += 1;
                }
            })
        }

        pub fn num_pairs_fitting(&self) -> usize {
            iproduct!(self.keys.iter(), self.locks.iter())
                .filter(|(key, lock)| {
                    key.fits(&lock)
                })
                .count()
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::Schematics;

    #[derive(Debug, Default)]
    pub struct Soln {
        schematics: Schematics,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.schematics.parse_input_file(filename);
            Answer::Usize(self.schematics.num_pairs_fitting())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(3); "example_1")]
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