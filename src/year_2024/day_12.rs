#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2024, day: 12 };

mod utils {
    use std::collections::{HashMap, HashSet};

    use crate::utils::io_utils;

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    struct Point {
        row: isize,
        col: isize,
    }

    impl Point {
        fn neighbors(&self) -> impl Iterator<Item = Point> + '_ {
            [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().map(|(delta_row, delta_col)| {
                Point { row: self.row + delta_row, col: self.col + delta_col }
            })
        }
    }

    #[derive(Debug, Default)]
    struct Region {
        points: HashSet<Point>,
    }

    impl Region {
        fn add_point(&mut self, point: Point) {
            self.points.insert(point);
        }

        fn area(&self) -> usize {
            self.points.len()
        }
        
        fn perimeter(&self) -> usize {
            let mut neighbor_counts: HashMap<Point, usize> = HashMap::new();
            for neighbor_point in self.points.iter()
                .map(|pt| pt.neighbors())
                .flatten()
                .filter(|pt| { !self.points.contains(pt) }) {
                    neighbor_counts.entry(neighbor_point).and_modify(|count| *count += 1).or_insert(1);
                }
            neighbor_counts.values().sum()
        }

        fn price(&self) -> usize {
            self.area() * self.perimeter()
        }
    }

    #[derive(Debug, Default)]
    pub struct Garden {
        regions: HashMap<char, HashMap<Point, usize>>,
    }

    impl Garden {
        pub fn parse_input_file(&mut self, filename: &str) {
            let mut next_id = 0;
            io_utils::file_to_lines(filename).enumerate().for_each(|(row, line)| {
                line.char_indices().for_each(|(col, ch)| {
                    self.add_plot(ch, Point { row: row as isize, col: col as isize }, &mut next_id);
                })
            })
        }

        fn add_plot(&mut self, ch: char, point: Point, next_id: &mut usize) {
            let neighbors: HashSet<Point> = point.neighbors().collect();
            let neighboring_regions: HashSet<usize> = if let Some(regions) = self.regions.get(&ch) {
                regions.iter().filter_map(|(pt, region_id)| {
                    if neighbors.contains(pt) {
                        Some(*region_id)
                    } else {
                        None
                    }
                })
                .collect()
            } else {
                HashSet::new()
            };
            if neighboring_regions.is_empty() {
                self.regions.entry(ch)
                    .and_modify(|point_to_region_id| {
                        point_to_region_id.insert(point, *next_id);
                    })
                    .or_insert(HashMap::from([(point, *next_id)]));
                *next_id += 1;
            } else {
                // Merge regions
                let merged_id = *neighboring_regions.iter().min().unwrap();
                self.regions.get_mut(&ch).unwrap().iter_mut()
                    .for_each(|(_pt, region_id)| {
                        if neighboring_regions.contains(region_id) {
                            *region_id = merged_id;
                        }
                    });
                self.regions.get_mut(&ch).unwrap().insert(point, merged_id);
            }
        }

        pub fn sum_of_prices(&self) -> usize {
            let mut regions: HashMap<usize, Region> = HashMap::new();
            self.regions.iter().for_each(|(_ch, point_to_region_id)| {
                point_to_region_id.iter().for_each(|(point, region_id)| {
                    regions.entry(*region_id)
                        .or_default()
                        .add_point(*point);
                })
            });
            regions.values().map(|region| region.price()).sum()
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::Garden;

    #[derive(Debug, Default)]
    pub struct Soln {
        garden: Garden,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.garden.parse_input_file(filename);
            Answer::Usize(self.garden.sum_of_prices())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(140); "example_1")]
        #[test_case(2, Answer::Usize(772); "example_2")]
        #[test_case(3, Answer::Usize(1_930); "example_3")]
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