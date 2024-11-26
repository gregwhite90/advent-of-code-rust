#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2015, day: 14 };

mod utils {
    use std::cmp;

    use regex::Regex;

    use crate::utils::io_utils;

    #[derive(Debug)]
    struct Reindeer {
        velocity: usize,
        stamina: usize,
        required_rest: usize,
    }

    impl Reindeer {
        fn new(velocity: usize, stamina: usize, required_rest: usize) -> Self {
            Self {
                velocity,
                stamina,
                required_rest,
            }
        }
        fn distance(&self, time: usize) -> usize {
            self.velocity * (
                (time / (self.required_rest + self.stamina)) * self.stamina
                + cmp::min(self.stamina, time % (self.required_rest + self.stamina))
            )
        }
    }

    #[derive(Debug, Default)]
    pub struct ReindeerRace {
        reindeers: Vec<Reindeer>,
    }

    impl ReindeerRace {
        pub fn parse_input_file(&mut self, filename: &str) {
            let reindeer_re = Regex::new(r"(?<name>\w+) can fly (?<velocity>\d+) km/s for (?<stamina>\d+) seconds, but then must rest for (?<required_rest>\d+) seconds\.").unwrap();
            self.reindeers = io_utils::file_to_lines(filename).map(|line| {
                let caps = reindeer_re.captures(&line).unwrap();
                let velocity = caps.name("velocity").unwrap().as_str().parse().unwrap();
                let stamina = caps.name("stamina").unwrap().as_str().parse().unwrap();
                let required_rest = caps.name("required_rest").unwrap().as_str().parse().unwrap();
                Reindeer::new(velocity, stamina, required_rest)
            }).collect()
        }
        
        pub fn winner_distance(&self, time: usize) -> usize {
            self.reindeers.iter().map(|r| r.distance(time)).max().unwrap()
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::ReindeerRace;

    #[derive(Debug)]
    pub struct Soln {
        time: usize,
        reindeer_race: ReindeerRace,
    }

    impl Soln {
        fn with_time(time: usize) -> Self {
            Self {
                time,
                reindeer_race: ReindeerRace::default(),
            }
        }
    }

    impl Default for Soln {
        fn default() -> Self {
            Self::with_time(2_503)
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.reindeer_race.parse_input_file(filename);
            Answer::Usize(self.reindeer_race.winner_distance(self.time))
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(1_120); "example_1")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::with_time(1_000),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}