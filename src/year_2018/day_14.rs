#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 14 };

mod utils {
    #[derive(Debug)]
    pub struct RecipeMaker {
        recipes: Vec<u8>,
        elves: [usize; 2],
    }

    impl Default for RecipeMaker {
        fn default() -> Self {
            Self {
                recipes: vec![3, 7],
                elves: [0, 1],
            }
        }
    }

    impl RecipeMaker {
        pub fn make_recipes(&mut self) {
            let new_recipe: u8 = self.elves.iter().map(|elf| self.recipes[*elf]).sum();
            let repr = format!("{}", new_recipe);
            repr.chars().for_each(|ch| {
                self.recipes.push(ch.to_digit(10).unwrap().try_into().unwrap());
            });
            self.elves.iter_mut().for_each(|elf| {
                *elf = (*elf + 1 + self.recipes[*elf] as usize) % self.recipes.len()
            });
        }

        pub fn num_recipes(&self) -> usize {
            self.recipes.len()
        }

        pub fn score(&self, start: usize, len: usize) -> String {
            self.recipes[start..start + len].iter().map(|score| {
                format!("{}", score)
            })
                .collect::<String>()
        }

        pub fn test_sequence_matches(&self, test_sequence: &Vec<u8>, start: usize) -> bool {
            self.recipes[start..start + test_sequence.len()] == *test_sequence
        }
    }
}

pub mod part_one {
    use crate::utils::{io_utils, solution::{Answer, Solution}};

    use super::utils::RecipeMaker;

    #[derive(Debug, Default)]
    pub struct RecipeBoard {
        recipe_maker: RecipeMaker,
        test_recipes: usize,
    }

    impl RecipeBoard {
        fn parse_input_file(&mut self, filename: &str) {
            self.test_recipes = io_utils::file_to_string(filename).parse().unwrap();
        }

        fn score(&mut self) -> String {
            while self.recipe_maker.num_recipes() < self.test_recipes + 10 {
                self.recipe_maker.make_recipes();
            }
            self.recipe_maker.score(self.test_recipes, 10)
        }
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        recipe_board: RecipeBoard,    
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.recipe_board.parse_input_file(filename);
            Answer::String(self.recipe_board.score())
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
    use crate::utils::{io_utils, solution::{Answer, Solution}};

    use super::utils::RecipeMaker;


    #[derive(Debug, Default)]
    pub struct RecipeBoard {
        recipe_maker: RecipeMaker,
        test_sequence: Vec<u8>,
    }

    impl RecipeBoard {
        fn parse_input_file(&mut self, filename: &str) {
            self.test_sequence = io_utils::file_to_string(filename)
                .chars()
                .map(|ch| ch.to_digit(10).unwrap().try_into().unwrap())
                .collect();
        }

        fn before_first_test_sequence(&mut self) -> usize {
            let mut cur_idx = 0;
            loop {
                while cur_idx + self.test_sequence.len() >= self.recipe_maker.num_recipes() { 
                    self.recipe_maker.make_recipes();
                }
                while cur_idx + self.test_sequence.len() < self.recipe_maker.num_recipes() {
                    if self.recipe_maker.test_sequence_matches(&self.test_sequence, cur_idx) {
                        return cur_idx;
                    }
                    cur_idx += 1;
                }
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        recipe_board: RecipeBoard,    
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.recipe_board.parse_input_file(filename);
            Answer::Usize(self.recipe_board.before_first_test_sequence())
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