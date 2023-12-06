#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 6 };

mod utils {
    #[derive(Debug, PartialEq, Eq)]
    pub struct Record {
        time: u64,
        distance: u64,
    }

    impl Record {

        pub fn new(time: u64, distance: u64) -> Self {
            Self { time, distance }
        }

        /// Let $h$ be the hold time, $t$ be the race time, and $d$ be the record distance.
        /// The record is beaten if the hold time $h$ (which becomes the speed) times
        /// $t - h$ is greater than $d$. The record is tied if the following quadratic
        /// formula is satisfied:
        /// $$ -h^2 + th - d = 0 $$
        /// This is true when:
        /// $$ h = \dfrac{-t \pm \sqrt{t^2 - 4d}}{-2} $$
        /// We then need the count of all integers within the range of the two roots.
        pub fn ways_to_beat(&self) -> u64 {
            let sqrt = ((self.time as f64).powi(2) - 4.0 * self.distance as f64).sqrt();
            let max_root = (-(self.time as f64) - sqrt) / - 2.0;
            let min_root = (-(self.time as f64) + sqrt) / - 2.0;
            let mut ways = max_root.floor() as u64 - min_root.ceil() as u64 + 1;
            // if the roots are ints, the need to be excluded as they'd result in a tie with the record.
            if max_root.round() == max_root { ways -= 1; }
            if min_root.round() == min_root { ways -= 1; }
            ways
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use super::*;

        #[test_case(7, 9, 4; "example_1")]
        #[test_case(15, 40, 8; "example_2")]
        #[test_case(30, 200, 9; "example_3")]
        #[test_case(71_530, 940_200, 71_503; "example_4")]
        fn ways_to_beat_is_correct(time: u64, distance: u64, answer: u64) {
            let record = Record { time, distance };
            assert_eq!(record.ways_to_beat(), answer);
        }
    }    
}

pub mod part_one {
    use std::iter::zip;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    use super::utils::Record;

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        product_of_ways_to_beat: u64,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U64(self.product_of_ways_to_beat)
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let mut lines = io_utils::file_to_lines(filename);
            let times = lines.next()
                .unwrap();
            let times = race_figures(
                &times
            );
            let distances = lines.next()
                .unwrap();
            let distances = race_figures(
                &distances               
            );
            self.product_of_ways_to_beat = zip(times, distances)
                .map(|(time, distance)| {
                    Record::new(time, distance).ways_to_beat()
                })
                .product()
        }
    }

    fn race_figures(input: &str) -> impl Iterator<Item = u64> + '_ {
        input.split_whitespace()
            .skip(1)
            .map(|figure| figure.parse::<u64>().unwrap())
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U64(288); "example_1")]
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
    use crate::utils::{solution::{Solution, Answer}, io_utils};

    use super::utils::Record;

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        ways_to_beat: u64,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U64(self.ways_to_beat)
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let mut lines = io_utils::file_to_lines(filename);
            let mut times = lines.next().unwrap();
            let time = race_figure(&mut times);
            let mut distances = lines.next().unwrap();
            let distance = race_figure(&mut distances);
            self.ways_to_beat = Record::new(time, distance).ways_to_beat()
        }
    }

    fn race_figure(input: &mut String) -> u64 {
        input.retain(|c| c.is_ascii_digit());
        input.parse().unwrap()        
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U64(71503); "example_1")]
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