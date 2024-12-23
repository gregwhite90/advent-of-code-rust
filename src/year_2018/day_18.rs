#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 18 };

mod utils {
    use std::collections::{BTreeMap, HashMap};
    
    use itertools::iproduct;

    use crate::utils::io_utils;

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    pub enum Acre {
        OPEN,
        TREES,
        LUMBERYARD,
    }

    impl Acre {
        pub fn from_char(ch: char) -> Self {
            match ch {
                '.' => Self::OPEN,
                '|' => Self::TREES,
                '#' => Self::LUMBERYARD,
                _ => panic!("Unrecognized character."),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    struct Point {
        x: isize,
        y: isize,
    }

    #[derive(Debug, Default)]
    pub struct LumberCollectionArea {
        acres: BTreeMap<Point, Acre>,
        minutes: usize,
        rows: usize,
        cols: usize,
        history: HashMap<BTreeMap<Point, Acre>, usize>,
    }

    impl LumberCollectionArea {
        pub fn parse_input_file(&mut self, filename: &str) {
            let mut row = 0;
            io_utils::file_to_lines(filename).for_each(|line| {
                self.cols = line.len();
                line.char_indices().for_each(|(col, ch)| {
                    self.acres.insert(Point{ x: col as isize, y: row}, Acre::from_char(ch));
                });
                row += 1;
            });
            self.rows = row as usize;
            self.history.insert(self.acres.clone(), self.minutes);
        }

        /// Returns the count of types of adjacent acres
        fn adjacent_acre_count(&self, point: &Point) -> HashMap<Acre, usize> {
            let mut res = HashMap::new();
            for (x, y) in iproduct!(
                point.x - 1 ..= point.x + 1,
                point.y - 1 ..= point.y + 1
            ) {
                let adj_pt = Point { x, y };
                if *point == adj_pt { continue; }
                if let Some(acre) = self.acres.get(&adj_pt) {
                    *res.entry(*acre).or_insert(0) += 1;
                }
            }
            res
        }

        /// Returns the period of repeating, if any
        fn progress_one_minute(&mut self) -> Option<usize> {
            let mut new_acres = BTreeMap::new();
            for (x, y) in iproduct!(0..self.cols as isize, 0..self.rows as isize) {
                let pt = Point { x, y };
                let adj_acre_count = self.adjacent_acre_count(&pt);
                match self.acres.get(&pt).unwrap() {
                    Acre::OPEN => {
                        new_acres.insert(pt, Acre::OPEN);
                        if let Some(adj_trees) = adj_acre_count.get(&Acre::TREES) {
                            if *adj_trees >= 3 {
                                new_acres.insert(pt, Acre::TREES);
                            }
                        }
                    },
                    Acre::TREES => {
                        new_acres.insert(pt, Acre::TREES);
                        if let Some(adj_lumberyards) = adj_acre_count.get(&Acre::LUMBERYARD) {
                            if *adj_lumberyards >= 3 {
                                new_acres.insert(pt, Acre::LUMBERYARD);
                            }
                        }
                    },
                    Acre::LUMBERYARD => {
                        new_acres.insert(pt, Acre::OPEN);
                        if let Some(adj_lumberyards) = adj_acre_count.get(&Acre::LUMBERYARD) {
                            if *adj_lumberyards >= 1 {
                                if let Some(adj_trees) = adj_acre_count.get(&Acre::TREES) {
                                    if *adj_trees >= 1 {
                                        new_acres.insert(pt, Acre::LUMBERYARD);
                                    }
                                }
                            }
                        }
                    },
                }
            }
            self.acres = new_acres;
            self.minutes += 1;
            if let Some(first_instance) = self.history.get(&self.acres) {
                return Some(self.minutes - *first_instance);
            }
            self.history.insert(self.acres.clone(), self.minutes);            
            None
        }

        pub fn progress_to(&mut self, minutes: usize) {
            while self.minutes < minutes {
                if let Some(period) = self.progress_one_minute() {
                    let remaining = (minutes - self.minutes) % period;
                    for _ in 0..remaining {
                        self.progress_one_minute();
                    }
                    return;
                }
            }
        }

        pub fn resource_value(&self) -> usize {
            self.acre_value(Acre::TREES) * self.acre_value(Acre::LUMBERYARD)
        }

        fn acre_value(&self, acre_type: Acre) -> usize {
            self.acres.values()
                .filter(|acre| **acre == acre_type)
                .count()
        }
    }
}


pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::LumberCollectionArea;

    #[derive(Debug, Default)]
    pub struct Soln {
        lumber_collection_area: LumberCollectionArea,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.lumber_collection_area.parse_input_file(filename);
            self.lumber_collection_area.progress_to(10);
            Answer::Usize(self.lumber_collection_area.resource_value())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(1_147); "example_1")]
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
    // TODO: need to find a circularity. so the lumber collection area needs to be hashable, it needs to use a BTreeMap instead
    // of a HashMap. Need to track the state of the LCA with the minutes, find the period of the repeating, and calculate how many more
    // minutes need to be progressed.
    use crate::utils::solution::{Answer, Solution};

    use super::utils::LumberCollectionArea;

    #[derive(Debug, Default)]
    pub struct Soln {
        lumber_collection_area: LumberCollectionArea,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.lumber_collection_area.parse_input_file(filename);
            self.lumber_collection_area.progress_to(1_000_000_000);
            Answer::Usize(self.lumber_collection_area.resource_value())
        }
    }
}