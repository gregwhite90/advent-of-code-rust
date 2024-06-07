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
        static ref NODE_RE: Regex = Regex::new(r"/dev/grid/node\-x(?<x>\d+)\-y(?<y>\d+)\s+(?<size>\d+)T\s+(?<used>\d+)T\s+(?<avail>\d+)T\s+(?<use_pct>\d+)\%").unwrap();
    }

    #[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
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

        pub fn fewest_steps(&self) -> usize {
            // TODO: Avoid generating all the legal moves every time.
            let mut visited: HashSet<GridPathStatus> = HashSet::new();
            let goal_data = self.nodes.keys()
                .filter(|pos| pos.y == 0)
                .max_by_key(|pos| pos.x)
                .unwrap();
            let mut pq = BinaryHeap::from([Reverse(GridPath::new(
                0, 
                *goal_data,
                self.nodes.clone(), 
            ))]);
            while !pq.is_empty() {
                let path = pq.pop().unwrap().0;
                if path.is_finished() { return path.moves; }
                let status = path.to_status();
                if visited.contains(&status) { continue; }
                visited.insert(status);
                pq.append(&mut path.next_paths());
            }
            panic!("Explored all states without finishing.");
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
    struct GridPath {
        moves: usize,
        goal_data: Position,
        nodes: BTreeMap<Position, Node>,
    }

    impl GridPath {
        fn new(moves: usize, goal_data: Position, nodes: BTreeMap<Position, Node>) -> Self {
            Self {
                moves,
                goal_data,
                nodes,
            }
        }

        fn next_paths(&self) -> BinaryHeap<Reverse<Self>> {
            self.legal_moves().iter().map(|m| {
                let mut nodes = self.nodes.clone();
                let amt = nodes.get_mut(&m.src).unwrap().move_from();
                nodes.get_mut(&m.dst).unwrap().move_to(amt);
                let goal_data = if self.goal_data == m.src {
                    m.dst
                } else {
                    self.goal_data
                };
                Reverse(Self::new(
                    self.moves + 1,
                    goal_data,
                    nodes,
                ))
            }).collect()
        }

        fn is_finished(&self) -> bool {
            self.goal_data == Position::new(0, 0)
        }

        fn to_status(&self) -> GridPathStatus {
            GridPathStatus {
                nodes: self.nodes.clone(),
                goal_data: self.goal_data,
            }
        }

        fn legal_moves(&self) -> BTreeSet<Move> {
            let mut moves = BTreeSet::new();
            for (pos, node) in self.nodes.iter() {
                for adj in self.adjacent_positions(pos) {
                    if node.fits(self.nodes.get(&adj).unwrap()) {
                        moves.insert(Move { src: adj, dst: *pos});
                    }
                    if self.nodes.get(&adj).unwrap().fits(node) {
                        moves.insert(Move { src: *pos, dst: adj});
                    }                    
                }
            }
            moves
        }
    
        fn adjacent_positions(&self, pos: &Position) -> Vec<Position> {
            let mut adjacent = Vec::new();
            if pos.x != 0 { adjacent.push(Position::new(pos.x - 1, pos.y)); }
            if pos.y != 0 { adjacent.push(Position::new(pos.x, pos.y - 1)); }
            for new_pos in [Position::new(pos.x, pos.y + 1), Position::new(pos.x + 1, pos.y)] {
                if self.nodes.contains_key(&new_pos) {
                    adjacent.push(new_pos);
                }
            }
            adjacent
        }    
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct GridPathStatus {
        nodes: BTreeMap<Position, Node>,
        goal_data: Position,
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    struct Move {
        src: Position,
        dst: Position,
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

pub mod part_two {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::Grid;

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