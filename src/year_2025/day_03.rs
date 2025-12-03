#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2025, day: 3 };

mod utils {
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    struct Battery {
        joltage: u32,
        position_within_bank: usize,
    }

    impl Ord for Battery {
        // Sorting Batteries in descending order should sort by highest joltage then by lowest position within bank
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.joltage
                .cmp(&other.joltage)
                .then(other.position_within_bank.cmp(&self.position_within_bank))
        }
    }

    impl PartialOrd for Battery {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct BatteryBank {
        batteries: Vec<Battery>
    }

    impl BatteryBank {
        pub fn from_str(input: &str) -> Self {
            let mut batteries: Vec<Battery> = input
                .char_indices()
                .map(|(idx, ch)| {
                    Battery {
                        joltage: ch.to_digit(10).unwrap(),
                        position_within_bank: idx,
                    }
                })
                .collect();
            // Sort descending
            batteries.sort_by(|a, b| b.cmp(a));
            Self {
                batteries
            }
        }

        pub fn max_joltage(&self, num_batteries: usize) -> u32 {
            // Intentionally does not modify the batteries vec in place
            let mut batteries = self.batteries.clone();
            let mut batteries_remaining = num_batteries;
            let mut max_joltage = 0;
            while batteries_remaining > 0 {
                let battery = batteries.iter()
                    .filter(|battery| {
                        battery.position_within_bank <= (self.batteries.len() - batteries_remaining)
                    })
                    .next()
                    .unwrap()
                    .clone();
                max_joltage += battery.joltage * 10u32.pow(batteries_remaining as u32 - 1);
                batteries_remaining -= 1;
                batteries.retain(|&b| b.position_within_bank > battery.position_within_bank);
            }
            max_joltage
        }
    }
}

pub mod part_one {
    use crate::utils::{io_utils, solution::{Answer, Solution}};
    use super::utils::BatteryBank;

    #[derive(Debug, Default)]
    pub struct Soln {}

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            Answer::U32(
                io_utils::file_to_lines(filename)
                    .map(|line| {
                        BatteryBank::from_str(&line).max_joltage(2)
                    })
                    .sum()
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case("987654321111111", 98; "puzzle_987654321111111")]
        #[test_case("811111111111119", 89; "puzzle_811111111111119")]
        #[test_case("234234234234278", 78; "puzzle_234234234234278")]
        #[test_case("818181911112111", 92; "puzzle_818181911112111")]
        fn individual_examples_are_correct(input: &str, max_joltage: u32) {
            assert_eq!(
                BatteryBank::from_str(input).max_joltage(2),
                max_joltage,
            )
        }

        #[test_case(1, Answer::U32(357); "example_1")]
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