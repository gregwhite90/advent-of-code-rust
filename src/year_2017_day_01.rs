pub mod year_2017_day_01 {
    pub use unicode_segmentation::UnicodeSegmentation;
    pub use itertools::Itertools;
    
    pub fn sum_of_repeated_characters(text: &str) -> u32{
        let graphemes: Vec<u32> = text
            .graphemes(true)
            .map(|digit| digit.parse::<u32>().unwrap())
            .collect();
        graphemes.iter()
            .circular_tuple_windows()
            .fold(0, |acc, (&elem, &next)| acc + if elem == next { elem } else { 0 })
    }
}