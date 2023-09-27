pub mod part_one {
    pub use unicode_segmentation::UnicodeSegmentation;
    pub use itertools::Itertools;
    
    pub fn sum_of_repeated_characters(text: &str) -> u32 {
        let graphemes: Vec<u32> = text
            .graphemes(true)
            .map(|digit| digit.parse::<u32>().unwrap())
            .collect();
        graphemes.iter()
            .circular_tuple_windows()
            .fold(0, |acc, (&elem, &next)| acc + if elem == next { elem } else { 0 })
    }
}

#[cfg(test)]
mod tests {
    use crate::year_2017::day_01::part_one::part_one::sum_of_repeated_characters;

    #[test]
    fn examples() {
        assert_eq!(3, sum_of_repeated_characters("1122"));
        assert_eq!(4, sum_of_repeated_characters("1111"));
        assert_eq!(0, sum_of_repeated_characters("1234"));
        assert_eq!(9, sum_of_repeated_characters("91212129"));
    }
}
