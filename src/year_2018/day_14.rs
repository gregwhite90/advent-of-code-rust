#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 14 };

mod utils {
    use crate::utils::io_utils;

    #[derive(Debug)]
    pub struct RecipeBoard {
        recipes: Vec<u8>,
        test_recipes: usize,
    }

    impl Default for RecipeBoard {
        fn default() -> Self {
            Self {
                recipes: vec![3, 7],
                test_recipes: 0,
            }
        }
    }

    impl RecipeBoard {
        pub fn parse_input_file(&mut self, filename: &str) {
            self.test_recipes = io_utils::file_to_string(filename).parse().unwrap();
        }

        pub fn make_recipes_test_recipes(&mut self) {
            let mut elf_0_idx = 0;
            let mut elf_1_idx = 1;
            while self.recipes.len() < self.test_recipes + 10 {
                let new_recipe = self.recipes[elf_0_idx] + self.recipes[elf_1_idx];
                let repr = format!("{}", new_recipe);
                repr.chars().for_each(|ch| {
                    self.recipes.push(ch.to_digit(10).unwrap().try_into().unwrap());
                });
                elf_0_idx = (elf_0_idx + 1 + self.recipes[elf_0_idx] as usize) % self.recipes.len();
                elf_1_idx = (elf_1_idx + 1 + self.recipes[elf_1_idx] as usize) % self.recipes.len();
            }
        }

        pub fn score_next_10(&self) -> String {
            self.recipes[self.test_recipes..self.test_recipes + 10].iter().map(|score| {
                format!("{}", score)
            })
                .collect::<String>()
        }

        pub fn make_recipes(&mut self) -> usize {
            let test_sequence: Vec<u8> = format!("{:0>5}", self.test_recipes).chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();
            let mut cur_idx = 0;
            let mut cur_test_sequence_idx = 0;
            let mut elf_0_idx = 0;
            let mut elf_1_idx = 1;
            loop {
                let new_recipe = self.recipes[elf_0_idx] + self.recipes[elf_1_idx];
                let repr = format!("{}", new_recipe);
                repr.chars().for_each(|ch| {
                    self.recipes.push(ch.to_digit(10).unwrap().try_into().unwrap());
                });
                elf_0_idx = (elf_0_idx + 1 + self.recipes[elf_0_idx] as usize) % self.recipes.len();
                elf_1_idx = (elf_1_idx + 1 + self.recipes[elf_1_idx] as usize) % self.recipes.len();
                while cur_idx < self.recipes.len() {
                    if self.recipes[cur_idx] == test_sequence[cur_test_sequence_idx] {
                        cur_idx += 1;
                        cur_test_sequence_idx += 1;
                        if cur_test_sequence_idx == test_sequence.len() {
                            return cur_idx - cur_test_sequence_idx;
                        }
                    } else {
                        cur_test_sequence_idx = if self.recipes[cur_idx] == test_sequence[0] { 1 } else { 0 };
                        cur_idx += 1;
                    }
                }
            }
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::RecipeBoard;

    #[derive(Debug, Default)]
    pub struct Soln {
        recipe_board: RecipeBoard,    
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.recipe_board.parse_input_file(filename);
            self.recipe_board.make_recipes_test_recipes();
            Answer::String(self.recipe_board.score_next_10())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::String("5158916779".to_string()); "example_1")]
        #[test_case(2, Answer::String("0124515891".to_string()); "example_2")]
        #[test_case(3, Answer::String("9251071085".to_string()); "example_3")]
        #[test_case(4, Answer::String("5941429882".to_string()); "example_4")]
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
    use crate::utils::solution::{Answer, Solution};

    use super::utils::RecipeBoard;

    #[derive(Debug, Default)]
    pub struct Soln {
        recipe_board: RecipeBoard,    
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.recipe_board.parse_input_file(filename);
            Answer::Usize(self.recipe_board.make_recipes())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(5, Answer::Usize(9); "example_5")]
        #[test_case(6, Answer::Usize(5); "example_6")]
        #[test_case(7, Answer::Usize(18); "example_7")]
        #[test_case(8, Answer::Usize(2018); "example_8")]
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