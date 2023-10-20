#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 12 };

mod utils {
    use regex::Regex;

    use crate::{year_2017::utils::map_of_groups::MapOfGroups, utils::io_utils};

    pub trait Year2017Day12Solution {
        fn map_of_groups_mut(&mut self) -> &mut MapOfGroups<u32>;
    }

    pub fn parse_input_file<T>(
        soln: &mut T,       
        filename: &str,
    )
    where
        T: Year2017Day12Solution
    {
        let re = Regex::new(r"(?<program>\d+) <\-> (?<pipes>[ \d,]+)").unwrap();
        io_utils::file_to_lines(filename).for_each(|line| {
            let captures = re.captures(&line)
                .expect("Line should match regex.");
            let program: u32 = captures.name("program").unwrap().as_str().parse().unwrap();
            soln.map_of_groups_mut().add_member(
                program,
                captures.name("pipes")
                    .unwrap()
                    .as_str()
                    .split(", ")
                    .map(|pipe| pipe.parse::<u32>().expect("Should be able to parse to `u32`."))
                    .collect()
            );
        });
    }
}

pub mod part_one {
    use crate::utils::solution::{Solution, Answer};
    use crate::year_2017::utils::map_of_groups::MapOfGroups;

    use super::utils::{Year2017Day12Solution, self};

    #[derive(PartialEq, Eq, Debug, Default)]
    pub struct Soln {
        map_of_groups: MapOfGroups<u32>,
    }

    impl Year2017Day12Solution for Soln {
        fn map_of_groups_mut(&mut self) -> &mut MapOfGroups<u32> {
            &mut self.map_of_groups
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            utils::parse_input_file(self, filename);
            Answer::U32(self.map_of_groups.group_len(0).try_into().expect("Should fit into `u32 datatype."))
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(6); "example_1")]
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
    use crate::utils::solution::{Solution, Answer};
    use crate::year_2017::utils::map_of_groups::MapOfGroups;
    use super::utils::{Year2017Day12Solution, self};

    #[derive(PartialEq, Eq, Debug, Default)]
    pub struct Soln {
        map_of_groups: MapOfGroups<u32>,
    }

    impl Year2017Day12Solution for Soln {
        fn map_of_groups_mut(&mut self) -> &mut MapOfGroups<u32> {
            &mut self.map_of_groups
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            utils::parse_input_file(self, filename);
            Answer::U32(self.map_of_groups.groups())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(2); "example_1")]
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