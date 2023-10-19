#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 14 };

const ROWS: usize = 128;
const COLS: usize = 128;

pub mod part_one {
    use crate::{utils::{solution::{Solution, Answer}, io_utils}, year_2017::utils::knot_hasher::KnotHasher};

    use super::ROWS;

    #[derive(PartialEq, Eq, Debug, Default)]
    pub struct Soln {
        ones: u32,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.ones)
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let key = io_utils::file_to_string(filename);
            self.ones = (0..ROWS)
                .map(|idx| {
                    let mut hasher = KnotHasher::default();
                    hasher.parse_key(&format!("{key}-{idx}"));
                    hasher.all_steps();
                    u128::from_str_radix(&hasher.knot_hash(), 16)
                        .expect("Should be able to parse base 16 string to u128.")
                        .count_ones()
                })
                .sum()
        }
    }
 
    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(8108); "example_1")]
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
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::collections::{HashSet, HashMap};

    use crate::{utils::{solution::{Solution, Answer}, io_utils}, year_2017::utils::knot_hasher::KnotHasher};

    use super::{ROWS, COLS};

    #[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
    struct Point {
        col: i32,
        row: i32,
    }

    impl Point {
        fn up_and_right_neighbors(&self) -> [Point; 2] {
            [ 
                Point { col: self.col + 1, row: self.row },
                Point { col: self.col, row: self.row - 1 },
            ]
        }
    }

    #[derive(PartialEq, Eq, Debug)]
    struct Group {
        points: HashSet<Point>,
    }

    impl Group {
        fn add_point(&mut self, point: Point) {
            self.points.insert(point);
        }
    }

    #[derive(PartialEq, Eq, Debug, Default)]
    pub struct Soln {
        used: HashMap<Point, Rc<RefCell<Group>>>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.groups())
        }
    }

    impl Soln {
        fn handle_point(&mut self, pt: &Point) {
            let mut neighbor_groups: HashMap<Point, Rc<RefCell<Group>>> = HashMap::with_capacity(2);
            for neighbor in pt.up_and_right_neighbors() {
                if let Some(neighbor_group) = self.used.remove(&neighbor) {
                    neighbor_groups.insert(neighbor, neighbor_group);
                }                           
            }
            if neighbor_groups.len() == 2 {
                let mut groups = neighbor_groups.values();
                if groups.next() == groups.next() {
                    self.used.insert(
                        pt.up_and_right_neighbors()[1].clone(),
                        neighbor_groups
                            .remove(&pt.up_and_right_neighbors()[1])
                            .expect("Neighbor groups should include groups for both neighbors.")
                    );
                }
            }
            match neighbor_groups.len() {
                0 => { 
                    self.used.insert(
                        pt.clone(), 
                        Rc::new(RefCell::new(Group { points: HashSet::from([pt.clone()])}))
                    );
                },
                1 => { 
                    let (neighbor, neighbor_group) = neighbor_groups.drain()
                        .next()
                        .expect("Should be one neighbor group.");
                    neighbor_group
                        .borrow_mut()
                        .add_point(pt.clone());
                    self.used.insert(pt.clone(), Rc::clone(&neighbor_group));
                    self.used.insert(neighbor.clone(), neighbor_group);
                },
                2 => {
                    // TODO: decide which to merge into the other?
                    let mut neighbor_groups = neighbor_groups.iter_mut();
                    let (absorber_neighbor, absorber) = neighbor_groups.next().expect("Should be at least one neighbor group.");
                    let (_absorbed_neighbor, absorbed) = neighbor_groups.next().expect("Should be a second neighbor group.");
                    absorber.borrow_mut().add_point(pt.clone());
                    self.used.insert(pt.clone(), Rc::clone(&absorber));
                    absorber.borrow_mut().points.extend(absorbed.borrow().points.iter());
                    for pt in absorbed.borrow().points.iter() {
                        self.used.insert(pt.clone(), Rc::clone(&absorber));
                    }
                    self.used.insert(absorber_neighbor.clone(), Rc::clone(&absorber));
                },
                _ => panic!("Should not have more than 2 neighbor groups."),
            }
        }

        fn parse_input_file(&mut self, filename: &str) {
            let key = io_utils::file_to_string(filename);
            // TODO: actually process the input
            (0..ROWS)
                .for_each(|row| {
                    let mut hasher = KnotHasher::default();
                    hasher.parse_key(&format!("{key}-{row}"));
                    hasher.all_steps();
                    let mut num = u128::from_str_radix(&hasher.knot_hash(), 16)
                        .expect("Should be able to parse base 16 string to u128.");
                    self.handle_num(&mut num, row);
                })
        }

        fn handle_num(&mut self, num: &mut u128, row: usize) {
            let mut col = COLS as i32 - 1;
            while *num != 0 {
                if *num & 1 == 1 { 
                    self.handle_point(&Point { col: col as i32, row: row as i32 });
                }
                col -= 1;
                *num >>= 1;
            }
        }

        fn groups(&self) -> u32 {
            let mut counted: HashSet<Point> = HashSet::new();
            self.used.values()
                .map(|group| {
                    match counted.intersection(&group.borrow().points).count() {
                        0 => {
                            counted.extend(&group.borrow().points);
                            1
                        },
                        x => {
                            assert_eq!(group.borrow().points.len(), x);
                            0
                        },
                    }
                })
                .sum()
        }

    }
 
    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test]
        fn handle_num_is_correct() {
            let mut soln = Soln::default();
            let mut num: u128 = 0b101;
            soln.handle_num(&mut num, 0);
            assert_eq!(soln.groups(), 2);
            num = 0b111;
            soln.handle_num(&mut num, 1);
            assert_eq!(soln.groups(), 1);
        }

        #[test_case(1, Answer::U32(1242); "example_1")]
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