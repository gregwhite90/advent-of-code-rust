pub mod part_one {
    pub fn checksum(_text: &str) -> i32 {
        0 // TODO: implement
    }
}

#[cfg(test)]
mod tests {
    mod part_one {
        use crate::year_2017::day_02::part_one::checksum;

        #[test]
        fn examples() {
            assert_eq!(18, checksum(include_str!("input/day_02/test_examples/example_1.txt")));
        }
    }
}