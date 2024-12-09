#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2024, day: 9 };

mod utils {
    use std::{cmp::Reverse, collections::BinaryHeap};

    use crate::utils::io_utils;

    #[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Block {
        start_index: usize,
        len: usize,
        id: Option<usize>,
    }

    #[derive(Debug, Default)]
    pub struct DiskMap {
        filled_blocks: BinaryHeap<Block>,
        empty_blocks: BinaryHeap<Reverse<Block>>,
    }

    impl DiskMap {
        pub fn parse_input_file(&mut self, filename: &str) {
            let mut id: usize = 0;
            let mut file = true;
            let mut cumulative_len: usize = 0;
            io_utils::file_to_string(filename)
                .chars()
                .for_each(|ch| {
                    let len = ch.to_string().parse().unwrap();
                    if file {
                        self.filled_blocks.push(Block {
                            start_index: cumulative_len,
                            len,
                            id: Some(id),
                        });
                        id += 1;
                    } else {
                        self.empty_blocks.push(Reverse(Block {
                            start_index: cumulative_len,
                            len,
                            id: None,
                        }))
                    }
                    file = !file;
                    cumulative_len += len;
                })
        }

        pub fn compact(&mut self) {
            while !self.is_compacted() {
                let max_filled = self.filled_blocks.pop().unwrap();
                let Reverse(min_empty) = self.empty_blocks.pop().unwrap();
                if max_filled.len == min_empty.len {
                    self.filled_blocks.push(Block {
                        start_index: min_empty.start_index,
                        len: max_filled.len,
                        id: max_filled.id,
                    });
                } else if max_filled.len < min_empty.len {
                    // entire filled block fits within empty block
                    self.filled_blocks.push(Block {
                        start_index: min_empty.start_index,
                        len: max_filled.len,
                        id: max_filled.id,
                    });
                    self.empty_blocks.push(Reverse(Block {
                        start_index: min_empty.start_index + max_filled.len,
                        len: min_empty.len - max_filled.len,
                        id: min_empty.id,
                    }));
                } else {
                    // filled block overflows this empty block
                    self.filled_blocks.push(Block {
                        start_index: min_empty.start_index,
                        len: min_empty.len,
                        id: max_filled.id,
                    });
                    self.filled_blocks.push(Block {
                        start_index: max_filled.start_index,
                        len: max_filled.len - min_empty.len,
                        id: max_filled.id,
                    });
                }
            }
        }

        fn is_compacted(&self) -> bool {
            let max_filled = self.filled_blocks.peek().unwrap();
            let Reverse(min_empty) = self.empty_blocks.peek().unwrap();
            max_filled.start_index < min_empty.start_index
        }

        pub fn checksum(&self) -> usize {
            self.filled_blocks.iter()
                .map(|block| {
                    (block.start_index..block.start_index + block.len)
                        .map(|i| i * block.id.unwrap())
                        .sum::<usize>()
                })
                .sum()
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::DiskMap;

    #[derive(Debug, Default)]
    pub struct Soln {
        disk_map: DiskMap,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.disk_map.parse_input_file(filename);
            self.disk_map.compact();
            Answer::Usize(self.disk_map.checksum())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(1_928); "example_1")]
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