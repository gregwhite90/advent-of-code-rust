#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 6 };

pub mod part_one {
    use std::iter::zip;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    #[derive(Debug, PartialEq, Eq)]
    struct Record {
        time: u32,
        distance: u32,
    }

    impl Record {
        /// Let $h$ be the hold time, $t$ be the race time, and $d$ be the record distance.
        /// The record is beaten if the hold time $h$ (which becomes the speed) times
        /// $t - h$ is greater than $d$. The record is tied if the following quadratic
        /// formula is satisfied:
        /// $$ -h^2 + th - d = 0 $$
        /// This is true when:
        /// $$ h = \dfrac{-t \pm \sqrt{t^2 - 4d}}{-2} $$
        /// We then need the count of all integers within the range of the two roots.
        fn ways_to_beat(&self) -> u32 {
            let sqrt = ((self.time as f64).powi(2) - 4.0 * self.distance as f64).sqrt();
            let max_root = (-(self.time as f64) - sqrt) / - 2.0;
            let min_root = (-(self.time as f64) + sqrt) / - 2.0;
            let mut ways = max_root.floor() as u32 - min_root.ceil() as u32 + 1;
            // if the roots are ints, the need to be excluded as they'd result in a tie with the record.
            if max_root.round() == max_root { ways -= 1; }
            if min_root.round() == min_root { ways -= 1; }
            ways
        }
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        product_of_ways_to_beat: u32,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.product_of_ways_to_beat)
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
                    Record { time, distance }.ways_to_beat()
                })
                .product()
        }
    }

    fn race_figures(input: &str) -> impl Iterator<Item = u32> + '_ {
        input.split_whitespace()
            .skip(1)
            .map(|figure| figure.parse::<u32>().unwrap())
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(288); "example_1")]
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