mod shared {
    pub use unicode_segmentation::UnicodeSegmentation;

    pub fn graphemes(text: &str) -> Vec<u32> {
        text
            .graphemes(true)
            .map(|digit| digit.parse::<u32>().unwrap())
            .collect()
    }
}

pub mod part_one {
    pub use itertools::Itertools;
    use crate::year_2017::day_01::shared;
    
    pub fn sum_of_repeated_digits(text: &str) -> u32 {
        let graphemes = shared::graphemes(text);
        graphemes.iter()
            .circular_tuple_windows()
            .fold(0, |acc, (&elem, &next)| acc + if elem == next { elem } else { 0 })
    }
}

pub mod part_two {
    use crate::year_2017::day_01::shared;
    
    pub fn sum_of_matching_halfway_around_digits(text: &str) -> u32 {
        let graphemes = shared::graphemes(text);
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
    mod part_one {
        use crate::year_2017::day_01::part_one::sum_of_repeated_digits;

        #[test]
        fn examples() {
            assert_eq!(3, sum_of_repeated_digits(include_str!("input/day_01/test_examples/example_1.txt")));
            assert_eq!(4, sum_of_repeated_digits(include_str!("input/day_01/test_examples/example_2.txt")));
            assert_eq!(0, sum_of_repeated_digits(include_str!("input/day_01/test_examples/example_3.txt")));
            assert_eq!(9, sum_of_repeated_digits(include_str!("input/day_01/test_examples/example_4.txt")));
        }
    }

    mod part_two {
        use crate::year_2017::day_01::part_two::sum_of_matching_halfway_around_digits;

        #[test]
        fn examples() {
            assert_eq!(6, sum_of_matching_halfway_around_digits(include_str!("input/day_01/test_examples/example_5.txt")));
            assert_eq!(0, sum_of_matching_halfway_around_digits(include_str!("input/day_01/test_examples/example_6.txt")));
            assert_eq!(4, sum_of_matching_halfway_around_digits(include_str!("input/day_01/test_examples/example_7.txt")));
            assert_eq!(12, sum_of_matching_halfway_around_digits(include_str!("input/day_01/test_examples/example_8.txt")));
            assert_eq!(4, sum_of_matching_halfway_around_digits(include_str!("input/day_01/test_examples/example_9.txt")));
        }    
    }
}