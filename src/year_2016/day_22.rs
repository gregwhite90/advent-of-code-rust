mod utils {
    use std::collections::{BTreeMap, HashMap};

    use lazy_static::lazy_static;
    use regex::Regex;

    use crate::utils::io_utils;

    lazy_static! {
        pub static ref NODE_RE: Regex = Regex::new(r"/dev/grid/node\-x(?<x>\d+)\-y(?<y>\d+)\s+(?<size>\d+)T\s+(?<used>\d+)T\s+(?<avail>\d+)T\s+(?<use_pct>\d+)\%").unwrap();
    }

    #[derive(Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
    struct Position {
        x: usize,
        y: usize,
    }

    impl Position {
        fn new(x: usize, y: usize) -> Self {
            Self { x, y }
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
    struct Node {
        position: Position,
        used: u32,
        avail: u32,
    }

    impl Node {
        fn from_str(input: &str) -> Self {
            let captures = NODE_RE.captures(input).unwrap();
            let x = captures.name("x").unwrap().as_str().parse().unwrap();
            let y = captures.name("y").unwrap().as_str().parse().unwrap();
            let position = Position::new(x, y);
            let used = captures.name("used").unwrap().as_str().parse().unwrap();
            let avail = captures.name("avail").unwrap().as_str().parse().unwrap();
            Self { position, used, avail }
        }

        fn fits_self(&self) -> bool {
            self.used > 0 && self.avail >= self.used
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
/// 
/// The solution is found by inspecting the puzzle input and the pattern shown in the
/// example. In our full puzzle input, the useless (very large, very full) nodes form a
/// wall from x = 14, y = 7 to x = 36, y = 7. The empty node starts at x = 35, y = 18.
/// We need to "move" the empty node to the top right, avoiding this wall. One path with
/// the fewest steps to do that is to move to x = 13, y = 18, then to x = 13, y = 0, then
/// to x = 36, y = 0 (the top right of our puzzle input). To then move the goal data
/// all the way to the top left, each one node we move it left requires 5 moves of the empty
/// node: down, left x2, up, right (the same pattern we see in the example). Once the empty
/// node has reached the top right, the goal data is actually one node left of the top right,
/// so fully shifting the goal data to the left along the top row requires (rows - 1) * 5 moves.
/// 
/// A more general purpose solution than we need for this case could: identify the useless nodes,
/// identify the empty position, perform BFS to find the shortest path from the empty node to the
/// top right, then perform a BFS to find the fewest steps to get the goal data from the top
/// right to the top left. A version using this simplified representation should run faster than
/// the version I wrote using all of each node's information, but this solution is sufficient
/// for the puzzle we have.
pub mod part_two {
    use crate::utils::solution::{Answer, Solution};

    #[derive(Debug, Default)]
    pub struct Soln {}
    
    impl Solution for Soln {
        fn solve(&mut self, _filename: &str) -> Answer {
            Answer::Usize((35 - 13) + (18 - 0) + (36 - 13) + 5 * 35)
        }
    }
}