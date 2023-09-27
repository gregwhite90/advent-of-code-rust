pub mod part_two {
    pub use unicode_segmentation::UnicodeSegmentation;
    pub use itertools::Itertools;
    
    pub fn sum_of_matching_halfway_around_digits(text: &str) -> u32 {
        let graphemes: Vec<u32> = text
            .graphemes(true)
            .map(|digit| digit.parse::<u32>().unwrap())
            .collect();
        let mut sum: u32 = 0;
        for (i, &digit) in graphemes.iter().enumerate() {
            if digit == graphemes[(i + (graphemes.len() / 2)) % graphemes.len()] {
                sum = sum + digit;
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use crate::year_2017::day_01::part_two::part_two::sum_of_matching_halfway_around_digits;

    #[test]
    fn examples() {
        assert_eq!(6, sum_of_matching_halfway_around_digits(include_str!("input/test_examples/example_5.txt")));
        assert_eq!(0, sum_of_matching_halfway_around_digits(include_str!("input/test_examples/example_6.txt")));
        assert_eq!(4, sum_of_matching_halfway_around_digits(include_str!("input/test_examples/example_7.txt")));
        assert_eq!(12, sum_of_matching_halfway_around_digits(include_str!("input/test_examples/example_8.txt")));
        assert_eq!(4, sum_of_matching_halfway_around_digits(include_str!("input/test_examples/example_9.txt")));
    }
}
