#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2024, day: 9 };

mod utils {
    use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}};

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
        empty_blocks_by_len: HashMap<usize, BinaryHeap<Reverse<usize>>>,
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
                        }));
                        self.empty_blocks_by_len.entry(len)
                            .and_modify(|e| e.push(Reverse(cumulative_len)))
                            .or_insert(BinaryHeap::from([Reverse(cumulative_len)]));
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

        pub fn compact_whole_files(&mut self) {
            let mut new_filled_blocks = BinaryHeap::new();
            while !self.filled_blocks.is_empty() {
                let filled_block = self.filled_blocks.pop().unwrap();
                let mut new_empty_len_and_start_index: Option<(usize, usize)> = None;
                if let Some((len, start_indices)) = self.empty_blocks_by_len.iter_mut()
                    .filter(|(len, start_indices)| {
                        let Reverse(start_index) = start_indices.peek().unwrap();
                        **len >= filled_block.len && *start_index < filled_block.start_index
                    })
                    .min_by_key(|(_len, start_indices)| {
                        if let Some(Reverse(start_index)) = start_indices.peek() {
                            *start_index
                        } else {
                            usize::MAX
                        }
                    }) {
                        let Reverse(start_index) = start_indices.pop().unwrap();
                        new_filled_blocks.push(Block {
                            start_index,
                            len: filled_block.len,
                            id: filled_block.id,
                        });
                        // update empty blocks accounting
                        new_empty_len_and_start_index = Some((*len - filled_block.len, start_index + filled_block.len));
                    } else {
                        new_filled_blocks.push(filled_block);
                    }
                if let Some((len, start_index)) = new_empty_len_and_start_index {
                    self.empty_blocks_by_len.entry(len)
                        .and_modify(|start_indices| start_indices.push(Reverse(start_index)))
                        .or_insert(BinaryHeap::from([Reverse(start_index)]));
                    self.empty_blocks_by_len.retain(|_len, start_indices| !start_indices.is_empty());
                }
            }
            self.filled_blocks = new_filled_blocks;
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

pub mod part_two {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::DiskMap;

    #[derive(Debug, Default)]
    pub struct Soln {
        disk_map: DiskMap,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.disk_map.parse_input_file(filename);
            self.disk_map.compact_whole_files();
            Answer::Usize(self.disk_map.checksum())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(2_858); "example_1")]
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