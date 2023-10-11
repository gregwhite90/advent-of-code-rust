pub mod utils {
    use itertools::Itertools;

    use crate::utils::io_utils;

    pub trait KnotHasher {
        const ROUNDS: u8 = 64;
        const DENSE_HASH_CHUNK_SIZE: usize = 16;

        fn set_nums(&mut self, nums: Vec<u8>);
        fn get_nums(&self) -> &Vec<u8>;
        fn get_mut_nums(&mut self) -> &mut Vec<u8>;
        fn set_lengths(&mut self, lengths: Vec<usize>);
        fn get_lengths(&self) -> &Vec<usize>;
        fn get_length(&self) -> usize;
        fn increment_length_idx(&mut self);
        fn get_position(&self) -> usize;
        fn set_position(&mut self, position: usize);
        fn get_skip_size(&self) -> usize;
        fn increment_skip_size(&mut self);

        fn range_vec_max(max: u8) -> Vec<u8> {
            (0..=max).collect()
        }
    
        fn parse_input_file(&mut self, filename: &str) {
            let mut lengths: Vec<usize> = io_utils::file_to_string(filename)
                .chars()
                .map(|ch| ch as usize)
                .collect();
            lengths.append(&mut vec![17usize, 31, 73, 47, 23]);
            self.set_lengths(lengths);
        }

        fn step(&mut self) {
            let position = self.get_position();
            let nums_len = self.get_nums().len();
            let length = self.get_length();
            if position + length < nums_len {
                self.get_mut_nums()[position..position + length].reverse();
            } else {
                // Circularity applies and requires a special case
                let mut full_reversal_region: Vec<u8> = self.get_nums()[position..nums_len]
                    .to_vec();
                full_reversal_region.append(
                        &mut self.get_nums()[0..(position + length - nums_len)].to_vec()
                    );
                full_reversal_region.reverse();
                self.get_mut_nums().splice(position..nums_len, full_reversal_region[..nums_len - position].to_vec());
                self.get_mut_nums().splice(0..position + length - nums_len, full_reversal_region[nums_len - position..].to_vec());
            }
            self.set_position((position + length + self.get_skip_size()) % nums_len);
            self.increment_skip_size();
            self.increment_length_idx();
        }   

        fn all_steps(&mut self) {
            for _round in 0..Self::ROUNDS {
                for _length_idx in 0..self.get_lengths().len() {
                    self.step();
                }
            }
        } 

        fn knot_hash(&self) -> String {
            let nums: [u8; 256] = self.get_nums().clone().try_into().expect("Should be exactly 256 numbers.");
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
}