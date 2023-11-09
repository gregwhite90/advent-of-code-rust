#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 21 };

const STARTING_PATTERN: [[u32; 3]; 3] = [
    [0, 1, 0],
    [0, 0, 1],
    [1, 1, 1],
];

mod utils {
    use std::collections::HashMap;
    use ndarray;

    use crate::utils::io_utils;

    use super::STARTING_PATTERN;

    fn ndarray2_from_str(input: &str) -> ndarray::Array2<u32> {
        let vec_of_vecs: Vec<Vec<u32>> = input.split("/")
            .map(|row| {
                row.chars()
                    .map(|c| {
                        match c {
                            '#' => 1,
                            '.' => 0,
                            _ => panic!("Unknown character {}", c),
                        }
                    })
                    .collect()
            })
            .collect();
        match input.len() {
            5  => {
                // 2x2
                let vec_of_arrays: Vec<[u32; 2]> = vec_of_vecs.into_iter().map(|row_vec| {
                    let array: [u32; 2] = row_vec.try_into().unwrap();
                    array    
                }).collect();
                ndarray::Array2::<u32>::from(vec_of_arrays)
            },
            11 => {
                // 3x3
                let vec_of_arrays: Vec<[u32; 3]> = vec_of_vecs.into_iter().map(|row_vec| {
                    let array: [u32; 3] = row_vec.try_into().unwrap();
                    array    
                }).collect();
                ndarray::Array2::<u32>::from(vec_of_arrays)
            },
            19 => {
                // 4x4
                let vec_of_arrays: Vec<[u32; 4]> = vec_of_vecs.into_iter().map(|row_vec| {
                    let array: [u32; 4] = row_vec.try_into().unwrap();
                    array    
                }).collect();
                ndarray::Array2::<u32>::from(vec_of_arrays)
            },
            _ => panic!("Unrecognized dimensions: should be 2x2 or 3x3 or 4x4.")
        }
    }

    /// Rotates in place
    fn rotate_90_deg_clockwise(input: &mut ndarray::Array2<u32>) {
        let (rows, cols) = input.dim();
        assert_eq!(rows, cols);
        match rows {
            2 => {
                let tmp = input[(0, 0)];
                input[(0, 0)] = input[(1, 0)];
                input[(1, 0)] = input[(1, 1)];
                input[(1, 1)] = input[(0, 1)];
                input[(0, 1)] = tmp;
            },
            3 => {
                let tmp = input[(0, 0)];
                input[(0, 0)] = input[(2, 0)];
                input[(2, 0)] = input[(2, 2)];
                input[(2, 2)] = input[(0, 2)];
                input[(0, 2)] = tmp;
                let tmp = input[(0, 1)];
                input[(0, 1)] = input[(1, 0)];
                input[(1, 0)] = input[(2, 1)];
                input[(2, 1)] = input[(1, 2)];
                input[(1, 2)] = tmp;
            }
            _ => panic!("Input pattern must be either 2x2 or 3x3"),
        }
    }

    /// Flips in place
    fn flip_over_horizontal(input: &mut ndarray::Array2<u32>) {
        let (rows, cols) = input.dim();
        assert_eq!(rows, cols);
        for col in 0..cols {
            for row in 0..(rows / 2) {
                input.swap((row, col), (rows - row - 1, col));
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct PatternEnhancer {
        iterations: u32,
        rules: HashMap<ndarray::Array2<u32>, ndarray::Array2<u32>>, // TODO: make the value here a RC<RefCell<String>>? To share outputs.
        pattern: ndarray::Array2<u32>,
    }

    impl PatternEnhancer {
        pub fn with_iterations(iterations: u32) -> Self {
            Self {
                iterations,
                rules: HashMap::new(),
                pattern: ndarray::Array2::<u32>::from(STARTING_PATTERN.into_iter().collect::<Vec<[u32; 3]>>()),
            }
        }

        pub fn parse_input_file(&mut self, filename: &str) {
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    let mut split = line.split(" => ");
                    let mut input = ndarray2_from_str(
                        split.next().expect("Should have one part of split.")
                    );
                    let output = ndarray2_from_str(
                        split.next().expect("Should have two parts of split.")
                    );
                    assert_eq!(split.next(), None);
                    self.add_all_rotations(&input, &output);
                    if input.nrows() == 3 {
                        assert_eq!(input.ncols(), 3);
                        flip_over_horizontal(&mut input);
                        self.add_all_rotations(&input, &output);
                    }
                });
        }

        fn add_all_rotations(&mut self, input: &ndarray::Array2<u32>, output: &ndarray::Array2<u32>) {
            let mut input = input.clone();
            self.rules.insert(input.clone(), output.clone());
            for _ in 0..3 {
                rotate_90_deg_clockwise(&mut input);
                self.rules.insert(input.clone(), output.clone());
            }
        }

        pub fn iterate_all(&mut self) {
            for _ in 0..self.iterations {
                self.iterate();
            }
        }

        fn iterate(&mut self) {
            let (rows, cols) = self.pattern.dim();
            assert_eq!(rows, cols);
            let step: usize = if rows % 2 == 0 { 2 } else { 3 };
            let new_dim = (step + 1) * rows / step;
            let mut new_pattern = ndarray::Array2::<u32>::uninit((new_dim, new_dim));
            let new_pattern_chunks = new_pattern
                .exact_chunks_mut((step + 1, step + 1)).into_iter();
            let pattern_chunks = self.pattern
                .exact_chunks((step, step)).into_iter();
            for (pattern_chunk, new_pattern_chunk) in pattern_chunks.zip(new_pattern_chunks) {
                let output = self.rules.get(&pattern_chunk.to_owned()).unwrap();
                output.clone().assign_to(new_pattern_chunk);
            }
            // all elements of `new_pattern` have just been assigned, so it is safe to use.
            unsafe {
                self.pattern = new_pattern.assume_init();
            }
        }

        pub fn sum(&self) -> u32 {
            self.pattern.sum()
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use ndarray;
        use super::*;

        #[test_case([
            ndarray::array![[1, 0], [0, 0]],
            ndarray::array![[0, 1], [0, 0]],
            ndarray::array![[0, 0], [0, 1]],
            ndarray::array![[0, 0], [1, 0]]
            ]; 
            "2x2"
        )]
        #[test_case([
            ndarray::array![[0, 1, 0], [0, 0, 1], [1, 1, 1]],
            ndarray::array![[1, 0, 0], [1, 0, 1], [1, 1, 0]],
            ndarray::array![[1, 1, 1], [1, 0, 0], [0, 1, 0]],
            ndarray::array![[0, 1, 1], [1, 0, 1], [0, 0, 1]],
            ]; 
            "3x3"
        )]
        fn rotate_is_correct(seq: [ndarray::Array2<u32>; 4]) {
            let mut input = seq[0].clone();
            rotate_90_deg_clockwise(&mut input);
            assert_eq!(input, seq[1]);
            rotate_90_deg_clockwise(&mut input);
            assert_eq!(input, seq[2]);
            rotate_90_deg_clockwise(&mut input);
            assert_eq!(input, seq[3]);
            rotate_90_deg_clockwise(&mut input);
            assert_eq!(input, seq[0]);
        }

        #[test_case([
            ndarray::array![[1, 0], [0, 0]],
            ndarray::array![[0, 0], [1, 0]],
            ]; 
            "2x2"
        )]
        #[test_case([
            ndarray::array![[0, 1, 0], [0, 0, 1], [1, 1, 1]],
            ndarray::array![[1, 1, 1], [0, 0, 1], [0, 1, 0]],
            ]; 
            "3x3"
        )]
        fn flip_is_correct(seq: [ndarray::Array2<u32>; 2]) {
            let mut input = seq[0].clone();
            flip_over_horizontal(&mut input);
            assert_eq!(input, seq[1]);
            flip_over_horizontal(&mut input);
            assert_eq!(input, seq[0]);
        }
    }    
}


pub mod part_one {
    use crate::utils::solution::{Solution, Answer};

    use super::utils::PatternEnhancer;

    #[derive(Debug, PartialEq, Eq)]
    pub struct Soln {
        pattern_enhancer: PatternEnhancer,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self::with_iterations(5)
        }
    }

    impl Soln {
        pub fn with_iterations(iterations: u32) -> Self {
            Self {
                pattern_enhancer: PatternEnhancer::with_iterations(iterations),
            }
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.pattern_enhancer.parse_input_file(filename);
            self.pattern_enhancer.iterate_all();
            Answer::U32(self.pattern_enhancer.sum())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(12), 2; "example_1")]
        fn examples_are_correct(example_key: u8, answer: Answer, iterations: u32) {
            test_utils::check_example_case(
                &mut Soln::with_iterations(iterations),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}