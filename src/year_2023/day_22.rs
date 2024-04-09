#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 22 };

pub mod part_one {

    use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}};

    use itertools::Itertools;
    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct Range {
        min: u32,
        max: u32,
    }

    impl Range {
        fn overlaps(&self, other: &Self) -> bool {
            self.min <= other.max && self.min >= other.min
                || self.max <= other.max && self.max >= other.min
                || self.min <= other.min && self.max >= other.max
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Brick {
        id: usize,
        x: Range,
        y: Range,
        z: Range,
        supporting_ids: Vec<usize>,
        supported_by_ids: Vec<usize>,
    }

    impl Ord for Brick {
        // Orders by max z first (for use in binary max heap of settled bricks to check)
        fn cmp(&self, other: &Self) -> Ordering {
            (self.z.max, self.z.min, self.x, self.y, self.id, self.supported_by_ids.len(), self.supported_by_ids.len())
                .cmp(&(other.z.max, other.z.min, other.x, other.y, other.id, other.supported_by_ids.len(), other.supported_by_ids.len()))            
        }
    }

    impl PartialOrd for Brick {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Brick {
        fn from_str(input: &str, id: usize, re: &Regex) -> Self {
            let captures = re.captures(input).unwrap();
            Self {
                id,
                x: Range { 
                    min: captures.name("min_x").unwrap().as_str().parse().unwrap(), 
                    max: captures.name("max_x").unwrap().as_str().parse().unwrap(),
                },
                y: Range { 
                    min: captures.name("min_y").unwrap().as_str().parse().unwrap(), 
                    max: captures.name("max_y").unwrap().as_str().parse().unwrap(),
                },
                z: Range { 
                    min: captures.name("min_z").unwrap().as_str().parse().unwrap(), 
                    max: captures.name("max_z").unwrap().as_str().parse().unwrap(),
                },
                supporting_ids: vec![],
                supported_by_ids: vec![],
            }
        }

        fn lands_on(&self, other: &Self) -> bool {
            self.x.overlaps(&other.x) && self.y.overlaps(&other.y)
        }
    }

    #[derive(Default)]
    pub struct Soln {
        bricks: HashMap<usize, Brick>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            self.settle_bricks();
            Answer::Usize(self.num_safe_to_disintegrate())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let re = Regex::new(r"(?<min_x>\d+),(?<min_y>\d+),(?<min_z>\d+)\~(?<max_x>\d+),(?<max_y>\d+),(?<max_z>\d+)").unwrap();
            self.bricks = io_utils::file_to_lines(filename)
                .enumerate()
                .map(|(id, line)| (id, Brick::from_str(&line, id, &re)))
                .collect();
        }

        fn settle_bricks(&mut self) {
            /* TODO:
            Go in order of the lowest starting minimum z.
            Then go through the settled bricks in order of the highest maximum z.
            Check for collision. Once a collision is found, we know the settled
            minimum z for the brick. Keep going until we find a brick with a 
            maximum settled z lower than this brick's settled z (impossible
            for it to rest on any more of the bricks).

            If not implemented carefully, could result in lots of searching in O(n)
            or lots of copying of data uneccessarily.
             */
            // TODO: need to figure out ownership. Either need a data structure to look up
            // bricks by id like I am using now, or need to save references to other bricks
            // directly. Or could drain self.bricks then settle the bricks, then insert them
            // again.
            let mut settled_bricks: BinaryHeap<Brick> = BinaryHeap::new();
            let bricks_by_min_z_asc = self.bricks.clone()
                .into_iter()
                .sorted_by_key(|(_id, brick)| {
                    brick.z.min
                });
            for (_id, mut brick) in bricks_by_min_z_asc {
                let mut checked_bricks: BinaryHeap<Brick> = BinaryHeap::new();
                let mut settled_min_z: Option<u32> = None;
                while let Some(mut b) = settled_bricks.pop() {
                    if let Some(smz) = settled_min_z {
                        if b.z.max < smz - 1 {
                            checked_bricks.push(b); 
                            break;
                        }
                    }
                    if brick.lands_on(&b) {
                        brick.supporting_ids.push(b.id);
                        b.supported_by_ids.push(brick.id);
                        settled_min_z = Some(b.z.max + 1);
                    }
                    checked_bricks.push(b);
                }
                let smz = settled_min_z.unwrap_or(1);
                let z_range = brick.z.max - brick.z.min;
                brick.z.min = smz;
                brick.z.max = smz + z_range;
                settled_bricks.push(brick);
                settled_bricks.append(&mut checked_bricks);
            }
            self.bricks = HashMap::from_iter(settled_bricks.into_iter().map(|brick| (brick.id, brick)));
        }

        fn num_safe_to_disintegrate(&self) -> usize {
            self.bricks.values()
                .filter(|brick| {
                    // all bricks it supports have at least one other supporting brick
                    brick.supporting_ids.iter().all(|id| {
                        self.bricks.get(id).unwrap().supported_by_ids.len() > 1
                    })
                })
                .count()
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(5); "example_1")]
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