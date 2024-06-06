#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 22 };

mod utils {
    use std::collections::HashMap;

    use lazy_static::lazy_static;
    use regex::Regex;

    use crate::utils::io_utils;

    lazy_static! {
        static ref NODE_RE: Regex = Regex::new(r"/dev/grid/node\-x(?<x>\d+)\-y(?<y>\d+)\s+(?<size>\d+)T\s+(?<used>\d+)T\s+(?<avail>\d+)T\s+(?<use_pct>\d+)\%").unwrap();
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    struct Position {
        x: usize,
        y: usize,
    }

    impl Position {
        fn new(x: usize, y: usize) -> Self {
            Self { x, y }
        }
    }

    #[derive(Debug, Clone)]
    struct Node {
        position: Position,
        size: u32,
        used: u32,
        avail: u32,
        use_pct: u32,
    }

    impl Node {
        fn from_str(input: &str) -> Self {
            let captures = NODE_RE.captures(input).unwrap();
            let x = captures.name("x").unwrap().as_str().parse().unwrap();
            let y = captures.name("y").unwrap().as_str().parse().unwrap();
            let position = Position::new(x, y);
            let size = captures.name("size").unwrap().as_str().parse().unwrap();
            let used = captures.name("used").unwrap().as_str().parse().unwrap();
            let avail = captures.name("avail").unwrap().as_str().parse().unwrap();
            let use_pct = captures.name("use_pct").unwrap().as_str().parse().unwrap();
            Self { position, size, used, avail, use_pct }
        }

        fn fits_self(&self) -> bool {
            self.used > 0 && self.avail >= self.used
        }
    }

    #[derive(Debug, Default)]
    pub struct Grid {
        nodes: HashMap<Position, Node>,
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