pub mod knot_hasher {
    //! A collection of utilities used by multiple 2017 days' solutions.
    use itertools::Itertools;
    use crate::utils::io_utils;

    // TODO: fix this example
    /// A trait for solutions or parts of solutions that require knot hashing
    /// (days 10 and 14).
    /// 
    /// # Examples
    /// ```
    /// use advent_of_code_rust::year_2017::utils::utils::KnotHasher;
    /// 
    /// #[derive(PartialEq, Eq, Debug)]
    /// struct KnotHasherSoln {
    ///     nums: Vec<u8>,
    ///     lengths: Vec<usize>,
    ///     length_idx: usize,
    ///     position: usize,
    ///     skip_size: usize,
    /// }
    /// 
    /// impl KnotHasher for KnotHasherSoln {
    ///     fn set_nums(&mut self, nums: Vec<u8>) {
    ///         self.nums = nums;
    ///     }
    /// 
    ///    fn get_nums(&self) -> &Vec<u8> {
    ///        &self.nums
    ///    }
    /// 
    ///    fn get_mut_nums(&mut self) -> &mut Vec<u8> {
    ///        &mut self.nums
    ///    }
    ///
    ///    fn set_lengths(&mut self, lengths: Vec<usize>) {
    ///        self.lengths = lengths;
    ///    }
    ///
    ///    fn get_lengths(&self) -> &Vec<usize> {
    ///        &self.lengths
    ///    }
    ///
    ///    fn get_length(&self) -> usize {
    ///        *self.lengths.get(self.length_idx).expect("Should be able to get the length at the current index.")
    ///    }
    ///
    ///    fn increment_length_idx(&mut self) {
    ///        self.length_idx = (self.length_idx + 1) % self.lengths.len();
    ///    }
    ///
    ///    fn get_position(&self) -> usize {
    ///        self.position
    ///    }
    ///
    ///    fn set_position(&mut self, position: usize) {
    ///        self.position = position;
    ///    }
    ///
    ///    fn get_skip_size(&self) -> usize {
    ///        self.skip_size
    ///    }
    ///
    ///    fn increment_skip_size(&mut self) {
    ///        self.skip_size += 1;
    ///    }
    /// }
    /// 
    /// let mut soln = KnotHasherSoln {
    ///     nums: vec![0, 1, 2, 3, 4],
    ///     lengths: vec![0, 1, 5, 4],
    ///     length_idx: 0,
    ///     position: 0,
    ///     skip_size: 0,
    /// };
    /// soln.step();
    /// assert_eq!(
    ///     soln,
    ///     KnotHasherSoln {
    ///         nums: vec![0, 1, 2, 3, 4],
    ///         lengths: vec![0, 1, 5, 4],
    ///         length_idx: 1,
    ///         position: 0,
    ///         skip_size: 1,
    ///     },
    /// );
    /// soln.step();
    /// assert_eq!(
    ///     soln,
    ///     KnotHasherSoln {
    ///         nums: vec![0, 1, 2, 3, 4],
    ///         lengths: vec![0, 1, 5, 4],
    ///         length_idx: 2,
    ///         position: 2,
    ///         skip_size: 2,
    ///     },
    /// );
    /// soln.step();
    /// assert_eq!(
    ///     soln,
    ///     KnotHasherSoln {
    ///         nums: vec![3, 2, 1, 0, 4],
    ///         lengths: vec![0, 1, 5, 4],
    ///         length_idx: 3,
    ///         position: 4,
    ///         skip_size: 3,
    ///     },
    /// );
    /// soln.step();
    /// assert_eq!(
    ///     soln,
    ///     KnotHasherSoln {
    ///         nums: vec![2, 3, 4, 0, 1],
    ///         lengths: vec![0, 1, 5, 4],
    ///         length_idx: 0,
    ///         position: 1,
    ///         skip_size: 4,
    ///     },
    /// );
    /// ```
    
    #[derive(PartialEq, Eq, Debug)]
    pub struct KnotHasher {
        nums: Vec<u8>,
        lengths: Vec<usize>,
        length_idx: usize,
        position: usize,
        skip_size: usize,
    }

    impl Default for KnotHasher {
        fn default() -> Self {
            Self::with_max(u8::MAX)
        }
    }
    
    impl KnotHasher {

        const ROUNDS: u8 = 64;
        const DENSE_HASH_CHUNK_SIZE: usize = 16;

        pub fn new(
            nums: Vec<u8>,
            lengths: Vec<usize>,
            length_idx: usize,
            position: usize,
            skip_size: usize,
        ) -> Self {
            KnotHasher {
                nums, 
                lengths,
                length_idx,
                position,
                skip_size
            }
        }

        /// Sets the private `nums` field to the specified vector.
        /// Takes ownership of the vector.
        //fn set_nums(&mut self, nums: Vec<u8>) {
        //   self.nums = nums;
        //}

        /// Gets a reference to the `nums` vector.
        pub fn nums(&self) -> &Vec<u8> {
            &self.nums
        }

        /// Gets a mutable reference to the `nums` vector.       
        fn nums_mut(&mut self) -> &mut Vec<u8> {
            &mut self.nums
        }

        /// Sets the private `lengths` field to the specified vector. 
        /// Takes ownership of the vector.
        pub fn set_lengths(&mut self, lengths: Vec<usize>) {
            self.lengths = lengths;
        }

        /// Gets a reference to the `lengths` vector.
        pub fn lengths(&self) -> &Vec<usize> {
            &self.lengths
        }

        /// Gets the next length to be used in the knot hashing algorithm.
        fn length(&self) -> usize {
            *self.lengths.get(self.length_idx).expect("Should be able to get the length at the current index.")
        }

        /// Increases the index into `lengths` by one.
        fn increment_length_idx(&mut self) {
            self.length_idx = (self.length_idx + 1) % self.lengths.len();
        }

        /// Gets the current `position` into the `nums` vector.
        fn position(&self) -> usize {
            self.position
        }

        /// Sets the current `position` into the `nums` vector.
        fn set_position(&mut self, position: usize) {
            self.position = position;
        }

        /// Gets the current `skip_size` (used to increase the position after a round of
        /// the knot hashing algorithm).
        fn skip_size(&self) -> usize {
            self.skip_size
        }

        /// Increases skip size by one.
        fn increment_skip_size(&mut self) {
            self.skip_size += 1;
        }

        /// Initializes a `KnotHasher` with `nums` initially set to the range `0..=max`.
        pub fn with_max(max: u8) -> Self {
            KnotHasher {
                nums: Self::range_vec_max(max),
                lengths: vec![],
                length_idx:0,
                position: 0,
                skip_size: 0,
            }
        }    

        /// Returns a vector from 0 to the specified `max`, inclusive. Used for the numbers 
        /// that will be manipulated and then knot hashed.
        fn range_vec_max(max: u8) -> Vec<u8> {
            (0..=max).collect()
        }
        
        /// Parses an input file containing a single key.
        pub fn parse_input_file(&mut self, filename: &str) {
            self.parse_key(&io_utils::file_to_string(filename));
        }

        /// Parses a key string to the lengths needed for the knot
        /// hashing algorithm.
        pub fn parse_key(&mut self, key: &str) {
            let mut lengths: Vec<usize> = key
                .chars()
                .map(|ch| ch as usize)
                .collect();
            lengths.append(&mut vec![17usize, 31, 73, 47, 23]);
            self.set_lengths(lengths);
        }

        /// Executes one step of the knot hashing algorithm: (1) reverses the order of a
        /// length of elements, (2) moves the next starting position forward, and 
        /// (3) increments the skip size.
        pub fn step(&mut self) {
            let position = self.position();
            let nums_len = self.nums().len();
            let length = self.length();
            if position + length < nums_len {
                self.nums_mut()[position..position + length].reverse();
            } else {
                // Circularity applies and requires a special case
                let mut full_reversal_region: Vec<u8> = self.nums()[position..nums_len]
                    .to_vec();
                full_reversal_region.append(
                        &mut self.nums()[0..(position + length - nums_len)].to_vec()
                    );
                full_reversal_region.reverse();
                self.nums_mut().splice(position..nums_len, full_reversal_region[..nums_len - position].to_vec());
                self.nums_mut().splice(0..position + length - nums_len, full_reversal_region[nums_len - position..].to_vec());
            }
            self.set_position((position + length + self.skip_size()) % nums_len);
            self.increment_skip_size();
            self.increment_length_idx();
        }   

        /// Fully executes all rounds in the knot hashing algorithm
        pub fn all_steps(&mut self) {
            for _round in 0..Self::ROUNDS {
                for _length_idx in 0..self.lengths().len() {
                    self.step();
                }
            }
        } 

        /// Returns the knot hash. Call after all the required steps have been executed.
        pub fn knot_hash(&self) -> String {
            let nums: [u8; 256] = self.nums().clone().try_into().expect("Should be exactly 256 numbers.");
            nums
                .into_iter()
                .chunks(Self::DENSE_HASH_CHUNK_SIZE)
                .into_iter()
                .map(|chunk| -> String {
                    format!("{:02x}", chunk
                    .reduce(|acc, num| acc ^ num)
                    .expect("No chunk should be empty."))
                })                
                .join("")
        }
    }

    // TODO: fix tests. add other tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn step_is_correct() {
            let mut soln = KnotHasher::with_max(4);
            soln.set_lengths(vec![0, 1, 5, 4]);
            soln.step();
            assert_eq!(
                soln,
                KnotHasher {
                    nums: vec![0, 1, 2, 3, 4],
                    lengths: vec![0, 1, 5, 4],
                    length_idx: 1,
                    position: 0,
                    skip_size: 1,
                },
            );
            soln.step();
            assert_eq!(
                soln,
                KnotHasher {
                    nums: vec![0, 1, 2, 3, 4],
                    lengths: vec![0, 1, 5, 4],
                    length_idx: 2,
                    position: 2,
                    skip_size: 2,
                },
            );
            soln.step();
            assert_eq!(
                soln,
                KnotHasher {
                    nums: vec![3, 2, 1, 0, 4],
                    lengths: vec![0, 1, 5, 4],
                    length_idx: 3,
                    position: 4,
                    skip_size: 3,
                },
            );
            soln.step();
            assert_eq!(
                soln,
                KnotHasher {
                    nums: vec![2, 3, 4, 0, 1],
                    lengths: vec![0, 1, 5, 4],
                    length_idx: 0,
                    position: 1,
                    skip_size: 4,
                },
            );
        }
    }    
}