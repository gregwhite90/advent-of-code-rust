#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 20 };

pub mod part_one {
    use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet, VecDeque}};

    /** TODO: can i build it up from the start?
     * keep a priority queue of the doors tracked, location.
     * but also need to know the depth in the regex maybe
    */
    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    struct Point {
        x: isize,
        y: isize,
    }

    #[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
    struct ConstructionMapStatus {
        doors_crossed: usize,
        point: Point,
    }

    #[derive(Debug, Default)]
    pub struct Soln {}

    impl Soln {
        fn largest_number_of_doors(&mut self, filename: &str) -> usize {
            // stack of branch starting points?
            // TODO: what i really need is the set of current paths that need to be processed.
            let mut branch_origins: VecDeque<ConstructionMapStatus> = VecDeque::new();
            let mut branch_ends: VecDeque<ConstructionMapStatus> = VecDeque::new();
            let mut pq: BinaryHeap<Reverse<ConstructionMapStatus>> = BinaryHeap::from([
                Reverse(ConstructionMapStatus::default())
            ]);
            let mut visited: HashSet<Point> = HashSet::new(); // TODO: is this enough?
            // TODO: track the depth too?
            io_utils::file_to_string(filename)
                .chars()
                .for_each(|ch| {
                    pq.pop();
                    match ch {
                        '^' | '$' => (),
                        'N' | 'S' | 'E' | 'W' => {
                            // TODO: handle
                            // Move all the current paths along this direction.
                            // TODO: should I mark the doors? Then later can deal with the pq and visited?
                        }, 
                        '(' => {
                            // TODO: open a new branch
                            // Save all current paths as starting points for this branch
                        },
                        '|' => {
                            // TODO: branch.
                            // Add all current paths to the branch ends.
                            // Restart with all the top starting points as current.
                        },
                        ')' => {
                            // TODO: close the branches
                            // Pop off the top starting points and continue processing
                        },
                        _ => panic!("Unrecognized character."),
                    }
                });
            0
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            Answer::Usize(self.largest_number_of_doors(filename))
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(3); "example_1")]
        #[test_case(2, Answer::Usize(10); "example_2")]
        #[test_case(3, Answer::Usize(18); "example_3")]
        #[test_case(4, Answer::Usize(23); "example_4")]
        #[test_case(5, Answer::Usize(31); "example_5")]
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