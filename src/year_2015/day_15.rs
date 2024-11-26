#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2015, day: 15 };

mod utils {
    use std::cmp;

    use ndarray::{array, Array, Array1, Array2};
    use regex::Regex;

    use crate::utils::io_utils;

    #[derive(Debug, Default)]
    struct Ingredients {
        features: Array2<i64>,
        calories: Array1<i64>,
    }

    impl Ingredients {
        pub fn parse_input_file(&mut self, filename: &str) {
            let ingredient_re = Regex::new(r"(?<name>\w+): capacity (?<capacity>\-?\d+), durability (?<durability>\-?\d+), flavor (?<flavor>\-?\d+), texture (?<texture>\-?\d+), calories (?<calories>\d+)").unwrap();
            let mut num_ingredients = 0;
            let mut cals: Vec<i64> = Vec::new();
            let ingredients = io_utils::file_to_lines(filename)
                .map(|line| {
                    num_ingredients += 1;
                    let caps = ingredient_re.captures(&line).unwrap();
                    let capacity: i64 = caps.name("capacity").unwrap().as_str().parse().unwrap();
                    let durability: i64 = caps.name("durability").unwrap().as_str().parse().unwrap();
                    let flavor: i64 = caps.name("flavor").unwrap().as_str().parse().unwrap();
                    let texture: i64 = caps.name("texture").unwrap().as_str().parse().unwrap();
                    let calories = caps.name("calories").unwrap().as_str().parse().unwrap();
                    cals.push(calories);
                    [capacity, durability, flavor, texture]
                });
            self.features = Array::from_iter(ingredients.flatten()).into_shape((num_ingredients, 4)).unwrap().reversed_axes();
            self.calories = Array::from_vec(cals);
        }

        fn calories(&self, recipe: &Array1<i64>) -> i64 {
            self.calories.dot(recipe)
        }

        fn score(&self, recipe: &Array1<i64>) -> i64 {
            self.features.dot(recipe).iter().map(|s| cmp::max(*s, 0)).product()
        }
    }

    #[derive(Debug, Default)]
    pub struct RecipeOptimizer {
        ingredients: Ingredients,
    }

    impl RecipeOptimizer {
        pub fn parse_input_file(&mut self, filename: &str) {
            self.ingredients.parse_input_file(filename);
        }

        pub fn max_total_score(&self, calorie_count: Option<i64>) -> i64 {
            // TODO: figure out how to do this dynamically
            let mut res = i64::MIN;
            for i in 0..=100 {
                  for j in 0..=(100 - i) {
                    for k in 0..=(100 - i - j) {
                        let l = 100 - i - j - k;
                        let recipe = array![i, j, k, l];
                        if let Some(calories) = calorie_count {
                            let cals = self.ingredients.calories(&recipe);
                            if cals != calories { continue; }
                        }
                        let score = self.ingredients.score(&recipe);
                        if score > res { res = score; }
                    }
                }
            }
            res
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::RecipeOptimizer;

    #[derive(Debug, Default)]
    pub struct Soln {
        recipe_optimizer: RecipeOptimizer,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.recipe_optimizer.parse_input_file(filename);
            Answer::I64(self.recipe_optimizer.max_total_score(None))
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::I64(62_842_880); "example_1")]
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

    use super::utils::RecipeOptimizer;

    #[derive(Debug, Default)]
    pub struct Soln {
        recipe_optimizer: RecipeOptimizer,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.recipe_optimizer.parse_input_file(filename);
            Answer::I64(self.recipe_optimizer.max_total_score(Some(500)))
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::I64(57_600_000); "example_1")]
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