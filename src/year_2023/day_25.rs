#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 25 };

/// Each vertex must have at least 4 edges coming out of it (otherwise the
/// answer would be a trivial (this assumption can also be confirmed).
/// At least in the example input, the components that will remain connected
/// when we remove the edges are connected in many ways.
/// One way to prioritize which edges we test removing is to assess how the
/// shortest path between the two vertices the edge connects increases.
/// Any edge whose removal would help create two connected components must
/// increase the shortest path between the vertices the edge connects to at
/// least 3 (one step within the start vertex's connected component, one step 
/// across connected components, and one step within the other connected
/// component). The exception to this is if we must remove multiple edges
/// that connect to the same vertex.
/// 
/// The edges that are most likely to create two connected components
/// are those that see the biggest increase in the shortest path between 
/// the nodes the edge previously connected when the edge is removed.
pub mod part_one {
    use std::{cmp::{Ordering, Reverse}, collections::{BinaryHeap, HashMap, HashSet, VecDeque}};

    use itertools::Itertools;
    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    struct Edge {
        vertices: [String; 2],
    }

    impl Edge {
        fn contains_vertex(&self, vertex: &str) -> bool {
            self.vertices[0] == vertex || self.vertices[1] == vertex
        }

        fn other_vertex(&self, start_vertex: &str) -> String {
            if self.vertices[0] == start_vertex {
                self.vertices[1].clone()
            } else {
                self.vertices[0].clone()
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Path {
        cur: String,
        steps: usize,
        visited: HashSet<String>,
    }

    impl Path {
        fn new(cur: &str) -> Self {
            Self {
                cur: cur.to_string(),
                steps: 0,
                visited: HashSet::from([cur.to_string()]),
            }
        }

        fn next_paths(&self, edges: &HashSet<Edge>) -> BinaryHeap<Path> {
            edges.iter()
                .filter(|e| e.contains_vertex(&self.cur))
                .map(|e| {
                    let next_vertex = e.other_vertex(&self.cur);
                    let mut new_visited = self.visited.clone();
                    new_visited.insert(next_vertex.clone());
                    Path {
                        cur: next_vertex,
                        steps: self.steps + 1,
                        visited: new_visited,
                    }
                })
                .collect()
        }
        
        fn len(&self) -> usize {
            self.visited.len() - 1
        }
    }

    impl Ord for Path {
        fn cmp(&self, other: &Self) -> Ordering {
            other.len().cmp(&self.len())
        }
    }

    impl PartialOrd for Path {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    /// Bidirectional breadth-first search.
    /// TODO: track the edges used in the path? or the vertices visited
    fn bidirectional_shortest_path(edge: &Edge, edges: &HashSet<Edge>) -> usize {
        let mut start_explored: HashMap<String, usize> = HashMap::new();
        let mut start_queue = VecDeque::from([Path::new(&edge.vertices[0])]);
        let mut end_explored: HashMap<String, usize> = HashMap::new();
        let mut end_queue = VecDeque::from([Path::new(&edge.vertices[1])]);
        loop {
            let start_cur = start_queue.pop_front().unwrap();
            if let Some(end_steps) = end_explored.get(&start_cur.cur) {
                return start_cur.steps + end_steps;
            }
            if !start_explored.contains_key(&start_cur.cur) {
                start_explored.insert(start_cur.cur.clone(), start_cur.steps);
                start_queue.extend(start_cur.next_paths(edges));
            }
            let end_cur = end_queue.pop_front().unwrap();
            if let Some(start_steps) = start_explored.get(&end_cur.cur) {
                return end_cur.steps + start_steps;                
            }
            if !end_explored.contains_key(&end_cur.cur) {
                end_explored.insert(end_cur.cur.clone(), end_cur.steps);
                end_queue.extend(end_cur.next_paths(edges));
            }
        }
    }

    fn connected_component_len(vertex_id: &str, edges: &HashSet<Edge>) -> usize {
        let mut connected_component: HashSet<String> = HashSet::new();
        let mut queue = VecDeque::from([vertex_id.to_string()]);
        while !queue.is_empty() {
            let vertex_id = queue.pop_front().unwrap();
            if !connected_component.insert(vertex_id.clone()) { continue; }
            edges.iter()
                .filter(|e| e.contains_vertex(&vertex_id))
                .for_each(|e| {
                    let next_vertex = e.other_vertex(&vertex_id);
                    if !connected_component.contains(&next_vertex) {
                        queue.push_back(next_vertex);
                    }
                });
        }
        connected_component.len()
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        // Maps id to vertex
        num_vertices: usize,
        edges: HashSet<Edge>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            let mut edges_clone = self.edges.clone();
            let edges_by_priority = self.edges.iter()
                .map(|edge| {
                    edges_clone.remove(edge);
                    let shortest_path = bidirectional_shortest_path(edge, &edges_clone);
                    edges_clone.insert(edge.clone());
                    (edge, shortest_path)
                })
                .sorted_by_key(|val| Reverse(val.1));
            // TODO: could vastly cut down on the number of combinations if we track the
            // visited edges/nodes that got us the shortest path. Because the shortest path
            // once we cut an edge must go through one of the other cut edges.
            //             
            // Tradeoff is more space, finding the shortest path preprocessing will be slower.
            for combo in edges_by_priority.combinations(3) {
                for (edge, _sp) in combo.iter() {
                    edges_clone.remove(*edge);
                }
                // check if it's two disjoint connected components now
                // TODO: could probably speed this up by short-circuiting as soon as all of the
                // cut edge endpoints are in the connected component.
                let ccl = connected_component_len(
                    &self.edges.iter().next().unwrap().vertices[0], 
                    &edges_clone
                );
                if ccl < self.num_vertices {
                    return Answer::Usize(ccl * (self.num_vertices - ccl));
                }
                for (edge, _sp) in combo {
                    edges_clone.insert(edge.clone());
                }
            }
            panic!("Should have found an answer by now");
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let re = Regex::new(r"(?<component>[a-z]+): (?<connected>[ a-z]+)").unwrap();
            let mut vertices: HashSet<String> = HashSet::new();
            io_utils::file_to_lines(filename).for_each(|line| {
                let captures = re.captures(&line).unwrap();
                let component = captures.name("component").unwrap().as_str();
                let connected = captures.name("connected").unwrap().as_str().split(' ');
                vertices.insert(component.to_string());
                for cc in connected {
                    vertices.insert(cc.to_string());
                    self.edges.insert(Edge { vertices: [component.to_string(), cc.to_string()] });
                }
            });
            self.num_vertices = vertices.len();
        }   
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(54); "example_1")]
        #[test_case(2, Answer::Usize(543_256); "full_input")]
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