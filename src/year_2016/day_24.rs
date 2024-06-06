#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 24 };

mod utils {
    use std::{cmp::Reverse, collections::{BTreeSet, BinaryHeap, HashMap, HashSet}};

    use crate::utils::io_utils;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    struct Point {
        x: usize,
        y: usize,
    }

    #[derive(Debug, Default)]
    pub struct GraphBuilder {
        points_of_interest: HashMap<Point, u32>,
        walls: HashSet<Point>,
    }

    impl GraphBuilder {
        pub fn parse_input_file(&mut self, filename: &str) {
            let mut rows: usize = 0;
            io_utils::file_to_lines(filename).for_each(|line| {
                line.chars().enumerate().for_each(|(col, ch)| {
                    if ch == '#' {
                        self.walls.insert(Point { x: col, y: rows });
                    } else if ch.is_numeric() {
                        self.points_of_interest.insert(
                            Point { x: col, y: rows },
                            ch.to_digit(10).unwrap(),
                        );
                    }
                });
                rows += 1;
            });
        }

        fn point_of_interest(&self, point: &Point) -> Option<u32> {
            self.points_of_interest.get(point).copied()
        }

        fn next_points(&self, point: &Point) -> Vec<Point> {
            [
                Point { x: point.x, y: point.y + 1},
                Point { x: point.x + 1, y: point.y},
                Point { x: point.x, y: point.y - 1},
                Point { x: point.x - 1, y: point.y},
            ].into_iter().filter(|point| !self.walls.contains(point)).collect()
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
    struct GraphBuilderPath {
        length: usize,
        point: Point,
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
    struct GraphPath {
        length: usize,
        unvisited: BTreeSet<u32>,
        current: u32,
        return_home: bool,
    }

    impl GraphPath {
        fn to_graph_path_status(&self) -> GraphPathStatus {
            GraphPathStatus {
                unvisited: self.unvisited.clone(),
                current: self.current,
            }
        }

        fn is_finished(&self) -> bool {
            self.unvisited.is_empty() && (!self.return_home || self.current == 0)
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
    struct GraphPathStatus {
        unvisited: BTreeSet<u32>,
        current: u32,
    }

    #[derive(Debug, Default)]
    pub struct Graph {
        /// Maps the points of interest vertices 
        /// to the shortest length between them
        edges: HashMap<BTreeSet<u32>, usize>,
        return_home: bool,
    }

    impl Graph {
        pub fn from_graph_builder(graph_builder: GraphBuilder, return_home: bool) -> Self {
            // TODO: populate
            let mut edges = HashMap::new();
            for (point, val) in graph_builder.points_of_interest.iter() {
                let mut pq: BinaryHeap<Reverse<GraphBuilderPath>> = BinaryHeap::from([
                    Reverse(GraphBuilderPath { length: 0, point: *point }),
                ]);
                let mut visited = HashSet::new();
                while !pq.is_empty() {
                    let path = pq.pop().unwrap().0;
                    if visited.contains(&path.point) { continue; }
                    visited.insert(path.point);
                    if let Some(poi) = graph_builder.point_of_interest(&path.point) {
                        if poi != *val {
                            edges.insert(BTreeSet::from([poi, *val]), path.length);
                            continue;
                        }
                    }
                    for next_point in graph_builder.next_points(&path.point) {
                        pq.push(Reverse(GraphBuilderPath { length: path.length + 1, point: next_point }));
                    }
                }
            }
            Self {
                edges,
                return_home,
            }
        }

        pub fn shortest_path(&self) -> usize {
            let mut unvisited: BTreeSet<u32> = self.edges.clone().into_keys()
                .reduce(|acc, elem| {
                    acc.union(&elem).copied().collect()
                })                
                .unwrap();
            unvisited.remove(&0);
            let mut pq: BinaryHeap<Reverse<GraphPath>> = BinaryHeap::from([
                Reverse(GraphPath { 
                    length: 0,
                    unvisited,
                    current: 0, 
                    return_home: self.return_home,
                })
            ]);
            let mut visited = HashSet::new();
            while !pq.is_empty() {
                let path = pq.pop().unwrap().0;
                if path.is_finished() { return path.length; }
                let status = path.to_graph_path_status();
                if visited.contains(&status) { continue; }
                visited.insert(status);
                for next_path in self.next_paths(&path) {
                    pq.push(Reverse(next_path));
                }
            }
            panic!("Explored all paths without finding solution")
        }

        fn next_paths(&self, path: &GraphPath) -> Vec<GraphPath> {
            self.edges.iter()
                .filter(|(pois, _length)| {
                    pois.contains(&path.current)                    
                })
                .map(|(pois, length)| {
                    let mut pois = pois.clone();
                    assert!(pois.remove(&path.current));
                    let next = pois.into_iter().next().unwrap();
                    let mut unvisited = path.unvisited.clone();
                    unvisited.remove(&next);
                    GraphPath {
                        length: path.length + length,
                        unvisited,
                        current: next,
                        return_home: self.return_home,
                    }
                })
                .collect()
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::{Graph, GraphBuilder};

    #[derive(Debug, Default)]
    pub struct Soln {
        graph: Graph,
    }
    
    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            let mut graph_builder = GraphBuilder::default();
            graph_builder.parse_input_file(filename);
            self.graph = Graph::from_graph_builder(graph_builder, false);
            Answer::Usize(self.graph.shortest_path())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(14); "example_1")]
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

    use super::utils::{Graph, GraphBuilder};

    #[derive(Debug, Default)]
    pub struct Soln {
        graph: Graph,
    }
    
    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            let mut graph_builder = GraphBuilder::default();
            graph_builder.parse_input_file(filename);
            self.graph = Graph::from_graph_builder(graph_builder, true);
            Answer::Usize(self.graph.shortest_path())
        }
    }
}