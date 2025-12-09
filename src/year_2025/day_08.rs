#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2025, day: 8 };

mod utils {
    use std::collections::{BTreeSet, HashMap, HashSet};

    use itertools::Itertools;

    use crate::utils::io_utils;

    #[derive(Debug, Default)]
    struct Circuits {
        point_to_circuit_id: HashMap<Point, usize>,
        circuits: HashMap<usize, HashSet<Point>>,
        next_circuit_id: usize,
    }

    impl Circuits {
        fn add_connection(&mut self, points: Vec<Point>) {
            match (
                self.point_to_circuit_id.get(&points[0]),
                self.point_to_circuit_id.get(&points[1]),
            ) {
                (None, None) => {
                    // Create the new circuit
                    self.circuits.insert(
                        self.next_circuit_id,
                        HashSet::from_iter(points.iter().cloned())
                    );
                    points.into_iter().for_each(|pt| {
                        self.point_to_circuit_id.insert(
                            pt,
                            self.next_circuit_id,
                        );
                    });
                    self.next_circuit_id += 1;
                },
                (Some(&id), None) => {
                    self.point_to_circuit_id.insert(points[1], id);
                    self.circuits.entry(id)
                        .and_modify(|circuit| circuit.extend(points.into_iter()));
                }, 
                (None, Some(&id)) => {
                    self.point_to_circuit_id.insert(points[0], id);
                    self.circuits.entry(id)
                        .and_modify(|circuit| circuit.extend(points.into_iter()));
                },
                (Some(&id_0), Some(&id_1)) => {
                    // Merge the circuits
                    if id_0 == id_1 {
                        return;
                    }
                    if self.circuits.get(&id_0).unwrap().len() <= self.circuits.get(&id_1).unwrap().len() {
                        let circuit_0 = self.circuits.remove(&id_0).unwrap();
                        self.circuits.entry(id_1)
                            .and_modify(|circuit| {
                                circuit.extend(circuit_0.iter());
                            });
                        for pt in circuit_0.iter() {
                            self.point_to_circuit_id.insert(
                                *pt,
                                id_1,
                            );
                        }
                    } else {
                        let circuit_1 = self.circuits.remove(&id_1).unwrap();
                        self.circuits.entry(id_0)
                            .and_modify(|circuit| {
                                circuit.extend(circuit_1.iter());
                            });
                        for pt in circuit_1.iter() {
                            self.point_to_circuit_id.insert(
                                *pt,
                                id_0,
                            );
                        }

                    }
                },
            }
        }

        fn product_of_largest_circuits(&self, num_circuits: usize) -> usize {
            let mut circuit_lengths = self.circuits.values()
                .map(|v| v.len())
                .collect::<Vec<usize>>();
            circuit_lengths.sort_by(|a, b| b.cmp(a));
            circuit_lengths.into_iter()
                .take(num_circuits)
                .product()
        }
    }

    /*
     * Idea: limit the search space based on the number of connections that are needed.
     * But with only 1K inputs, it is feasible to calculate the distances pairwise.
     */
    #[derive(Debug)]
    pub struct Playground {
        points: Vec<Point>, // TODO: figure out the right data structure
        circuits: Circuits,
        num_connections: usize,
        num_circuits: usize,
    }

    impl Playground {
        pub fn new(
            num_connections: usize,
            num_circuits: usize,
        ) -> Self {
            Self {
                points: Vec::default(),
                circuits: Circuits::default(),
                num_connections,
                num_circuits,
            }
        }

        pub fn parse_input_file(&mut self, filename: &str) {
            self.points = io_utils::file_to_lines(filename)
                .map(|line| {
                    let mut coords = line.split(',')
                        .map(|coord| coord.parse::<i64>().unwrap());
                    Point {
                        x: coords.next().unwrap(),
                        y: coords.next().unwrap(),
                        z: coords.next().unwrap(),
                    }
                })
                .collect();
        }

        pub fn make_shortest_connections(&mut self) {
            let mut pairwise_distances: HashMap<BTreeSet<Point>, f64> = HashMap::new();
            for i in 0..self.points.len() {
                for j in (i + 1)..self.points.len() {
                    pairwise_distances.insert(
                        BTreeSet::from([
                            self.points[i],
                            self.points[j],
                        ]),
                        self.points[i].distance(&self.points[j]),
                    );
                }
            }
            pairwise_distances.into_iter()
                .sorted_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .take(self.num_connections)
                .for_each(|(pts, _)| {
                    let points: Vec<Point> = pts.into_iter().collect();
                    self.circuits.add_connection(points);
                });
        }

        pub fn product_of_largest_circuits(&self) -> usize {
            self.circuits.product_of_largest_circuits(self.num_circuits)    
        }
    }

    #[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    struct Point {
        x: i64,
        y: i64,
        z: i64,
    }

    impl Point {
        fn distance(&self, other: &Self) -> f64 {
            (
                (
                    (self.x - other.x).pow(2) 
                    + (self.y - other.y).pow(2)
                    + (self.z - other.z).pow(2)
                )
                as f64
            ).sqrt()
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};
    use super::utils::Playground;

    #[derive(Debug)]
    pub struct Soln {
        playground: Playground,
    }

    impl Soln {
        pub fn new(
            num_connections: usize,
            num_circuits: usize,
        ) -> Self {
            Self {
                playground: Playground::new(num_connections, num_circuits),
            }
        }
    }

    impl Default for Soln {
        fn default() -> Self {
            Self::new(1000, 3)
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.playground.parse_input_file(filename);
            self.playground.make_shortest_connections();
            Answer::Usize(self.playground.product_of_largest_circuits())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(40); "example_1")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::new(10, 3),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}