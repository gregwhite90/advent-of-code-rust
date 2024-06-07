#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 22 };

mod utils {
    use std::{cmp::Reverse, collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet}};

    use lazy_static::lazy_static;
    use regex::Regex;

    use crate::utils::io_utils;

    lazy_static! {
        pub static ref NODE_RE: Regex = Regex::new(r"/dev/grid/node\-x(?<x>\d+)\-y(?<y>\d+)\s+(?<size>\d+)T\s+(?<used>\d+)T\s+(?<avail>\d+)T\s+(?<use_pct>\d+)\%").unwrap();
    }

    #[derive(Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
    pub struct Position {
        pub x: usize,
        pub y: usize,
    }

    impl Position {
        pub fn new(x: usize, y: usize) -> Self {
            Self { x, y }
        }

        pub fn adjacent_positions(&self) -> Vec<Position> {
            let mut adjacent = Vec::new();
            if self.x != 0 { adjacent.push(Position::new(self.x - 1, self.y)); }
            if self.y != 0 { adjacent.push(Position::new(self.x, self.y - 1)); }
            for new_pos in [Position::new(self.x, self.y + 1), Position::new(self.x + 1, self.y)] {
                adjacent.push(new_pos);
            }
            adjacent
        }    
    
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
    pub struct Node {
        pub position: Position,
        pub used: u32,
        pub avail: u32,
    }

    impl Node {
        pub fn from_str(input: &str) -> Self {
            let captures = NODE_RE.captures(input).unwrap();
            let x = captures.name("x").unwrap().as_str().parse().unwrap();
            let y = captures.name("y").unwrap().as_str().parse().unwrap();
            let position = Position::new(x, y);
            let used = captures.name("used").unwrap().as_str().parse().unwrap();
            let avail = captures.name("avail").unwrap().as_str().parse().unwrap();
            Self { position, used, avail }
        }

        pub fn position(&self) -> Position {
            self.position
        }

        fn fits_self(&self) -> bool {
            self.used > 0 && self.avail >= self.used
        }

        pub fn size(&self) -> u32 {
            self.avail + self.used
        }

        fn fits(&self, other: &Self) -> bool {
            other.used > 0 && self.avail >= other.used
        }

        fn move_from(&mut self) -> u32 {
            let to_move = self.used;
            self.used = 0;
            self.avail += to_move;
            to_move
        }

        fn move_to(&mut self, amt: u32) {
            self.used += amt;
            self.avail -= amt;
        }
    }

    #[derive(Debug, Default)]
    pub struct Grid {
        nodes: BTreeMap<Position, Node>,
    }

    impl Grid {
        pub fn parse_input_file(&mut self, filename: &str) {
            self.nodes = io_utils::file_to_lines(filename)
                .skip(2)
                .map(|line| {
                    let node = Node::from_str(&line);
                    (node.position, node)
                })
                .collect();
        }

        fn nodes_fitting_self(&self) -> usize {
            self.nodes.values().filter(|node| node.fits_self()).count()
        }

        pub fn viable_pairs(&self) -> usize {
            let mut avail_count: HashMap<u32, usize> = HashMap::new();
            self.nodes.values().for_each(|node| {
                if node.avail == 0 { return; }
                *avail_count.entry(node.avail).or_default() += 1;
            });
            let mut used_count: HashMap<u32, usize> = HashMap::new();
            self.nodes.values().for_each(|node| {
                if node.used == 0 { return; }
                *used_count.entry(node.used).or_default() += 1;
            });
            used_count.into_iter().map(|(used, count)| {
                count * avail_count.iter()
                    .filter(|(avail, _)| **avail >= used)
                    .map(|(_, count)| *count)
                    .sum::<usize>()
            }).sum::<usize>() - self.nodes_fitting_self()
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::Grid;

    #[derive(Debug, Default)]
    pub struct Soln {
        grid: Grid,
    }
    
    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.grid.parse_input_file(filename);
            Answer::Usize(self.grid.viable_pairs())
        }
    }
}

/// This solution is not a general purpose solution (I wrote one, but it is too slow
/// for this specific problem). It instead relies on the pattern seen in the input data
/// (both the example in the puzzle statement and the full input data): the grid is made
/// up of one empty node, several useless nodes (very large and very full),
/// and the rest interchangeable nodes (full enough that no other node's data fits, 
/// and small enough that their data can be moved around).
pub mod part_two {
    use std::collections::{HashMap, HashSet};

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    use super::utils::{Position, Node};

    #[derive(Debug, Default)]
    struct Grid {
        rows: usize,
        cols: usize,
        empty: Position,
        useless: HashSet<Position>,
    }

    impl Grid {
        fn parse_input_file(&mut self, filename: &str) {
            let nodes: HashMap<Position, Node> = io_utils::file_to_lines(filename)
                .skip(2)
                .map(|line| {
                    let node = Node::from_str(&line);
                    (node.position(), node)
                })
                .collect();
            self.rows = nodes.keys()
                .map(|pos| pos.y)
                .max()
                .unwrap() + 1;
            self.cols = nodes.keys()
                .map(|pos| pos.x)
                .max()
                .unwrap() + 1;
            
            // Find the one empty node
            let empties: Vec<&Node> = nodes.values().filter(|node| node.used == 0).collect();
            assert_eq!(empties.len(), 1);
            self.empty = empties[0].position;
            
            // Find the useless nodes
            let mut nodes_less_empty = nodes.clone();
            nodes_less_empty.remove(&self.empty);
            self.useless = nodes_less_empty.iter().filter(|(pos, node)| {
                nodes_less_empty.values().all(|other| node.avail < other.used)
                    && pos.adjacent_positions().iter().all(|adj| {
                        !nodes.contains_key(adj) || node.used > nodes.get(adj).unwrap().size()
                    })
            })
                .map(|(pos, _node)| *pos)
                .collect();
        }

        fn fewest_steps(&self) -> usize {
            0
        }
    }


    #[derive(Debug, Default)]
    pub struct Soln {
        grid: Grid,
    }
    
    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.grid.parse_input_file(filename);
            Answer::Usize(self.grid.fewest_steps())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(7); "example_1")]
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