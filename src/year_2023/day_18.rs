#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 18 };

pub mod part_one {

    use std::{cmp, collections::{HashSet, VecDeque}};

    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Direction {
        Up,
        Down,
        Right,
        Left,
    }

    impl Direction {
        fn from_str(input: &str) -> Self {
            match input {
                "U" => Self::Up,
                "D" => Self::Down,
                "L" => Self::Left,
                "R" => Self::Right,
                _ => panic!("Unrecognized direction."),
            }
        }
    }

    #[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        fn next(&self, direction: Direction) -> Self {
            let x = self.x + match direction {
                Direction::Left => -1,
                Direction::Right => 1,
                _ => 0,
            };
            let y = self.y + match direction {
                Direction::Up => -1,
                Direction::Down => 1,
                _ => 0,
            };
            Self { x, y }
        }

        fn adjacent_points(&self) -> Vec<Point> {
            vec![
                Point { x: self.x, y: self.y + 1 },
                Point { x: self.x, y: self.y - 1 },
                Point { x: self.x + 1, y: self.y },
                Point { x: self.x - 1, y: self.y },
            ]
        }
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    struct BoundingBox {
        min_x: i32,
        min_y: i32,
        max_x: i32,
        max_y: i32,
    }

    impl BoundingBox {
        fn new(pt: &Point) -> Self {
            Self {
                min_x: pt.x,
                min_y: pt.y,
                max_x: pt.x,
                max_y: pt.y,
            }
        }
        fn update(&mut self, pt: &Point) {
            self.min_x = cmp::min(self.min_x, pt.x);
            self.min_y = cmp::min(self.min_y, pt.y);
            self.max_x = cmp::max(self.max_x, pt.x);
            self.max_y = cmp::max(self.max_y, pt.y);
        }

        fn contains(&self, other: &Self) -> bool {
            self.min_x <= other.min_x
                && self.min_y <= other.min_y
                && self.max_x >= other.max_x
                && self.max_y >= other.max_y
        }

        fn extend(&mut self, other: &BoundingBox) {
            self.min_x = cmp::min(self.min_x, other.min_x);
            self.min_y = cmp::min(self.min_y, other.min_y);
            self.max_x = cmp::max(self.max_x, other.max_x);
            self.max_y = cmp::max(self.max_y, other.max_y);            
        }
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    struct ConnectedPoints {
        points: HashSet<Point>,
        bounding_box: BoundingBox,
    }

    impl ConnectedPoints {
        fn new(pt: Point) -> Self {
            Self {
                points: HashSet::from([pt]),
                bounding_box: BoundingBox::new(&pt),
            }
        }

        fn add_point(&mut self, pt: Point) {
            self.bounding_box.update(&pt);
            self.points.insert(pt);
        }

        fn len(&self) -> usize {
            self.points.len()
        }

        fn contains(&self, point: &Point) -> bool {
            self.points.contains(point)
        }

        fn absorb(&mut self, other: Self) {
            self.points.extend(other.points);
            self.bounding_box.extend(&other.bounding_box);
        }
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct Soln {
        trench: ConnectedPoints,
        position: Point,
        interiors: Vec<ConnectedPoints>,
        exterior: ConnectedPoints,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            self.find_all_interiors();
            Answer::Usize(self.total_volume())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let re = Regex::new(r"(?<direction>[UDRL]) (?<distance>\d+) (?<color>\(\#[0-9a-f]{6}\))").unwrap();
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    let captures = re.captures(&line).expect("Line should match known pattern.");
                    let direction = Direction::from_str(captures.name("direction").unwrap().as_str());
                    let distance: i32 = captures.name("distance").unwrap().as_str().parse().unwrap();
                    self.dig(direction, distance);
                });
        }

        fn dig(&mut self, direction: Direction, distance: i32) {
            for _ in 0..distance {
                self.position = self.position.next(direction);
                self.trench.add_point(self.position);
            }
        }

        fn trench_volume(&self) -> usize {
            self.trench.len()
        }

        fn interiors_volume(&self) -> usize {
            self.interiors.iter().map(|interior| interior.len()).sum()
        }

        fn total_volume(&self) -> usize {
            self.trench_volume() + self.interiors_volume()
        }

        fn check_potential_interior(&mut self, pt: Point) {
            if self.trench.contains(&pt) { return; }
            let mut connected_points = ConnectedPoints::new(pt);
            let mut points_to_explore = VecDeque::from([pt]);
            let mut explored = HashSet::new();
            while !points_to_explore.is_empty() {
                let point = points_to_explore.pop_front().expect("Should have a point to explore.");
                if !explored.contains(&point) && !self.trench.contains(&point) {
                    // If we've hit another known interior point, this is an interior point.
                    if self.interiors.iter().any(|interior| interior.contains(&point)) {
                        return;
                    }
                    connected_points.add_point(point);
                    explored.insert(point);
                    // If we've hit another exterior point or expanded outside the trench's bounding box, this is an exterior
                    // set of connected points.
                    if self.exterior.contains(&point) || !self.trench.bounding_box.contains(&connected_points.bounding_box) {
                        self.exterior.absorb(connected_points);
                        return;
                    }
                    for adj in point.adjacent_points() {
                        points_to_explore.push_back(adj);
                    }
                }
            }
            self.interiors.push(connected_points);
        }

        fn find_all_interiors(&mut self) {
            // Rather than cloning, could figure out how to tell compiler that the immutable borrow of self 
            // is read-only for `trench`, and the mutable borrow is also read-only for `trench` and only writes
            // to other variables.
            for pt in self.trench.points.clone() {
                for adj in pt.adjacent_points() {
                    self.check_potential_interior(adj)
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(62); "example_1")]
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

/// This solution uses the [shoelace formula](https://www.themathdoctors.org/polygon-coordinates-and-areas/),
/// inspired by [this discussion on Reddit](https://www.reddit.com/r/adventofcode/comments/18l2nk2/2023_day_18_easiest_way_to_solve_both_parts/).
/// The shoelace formula works by summing the areas of the triangles each edge forms with the origin.
pub mod part_two {

    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Direction {
        Up,
        Down,
        Right,
        Left,
    }

    impl Direction {
        fn from_str(input: &str) -> Self {
            match input {
                "0" => Self::Right,
                "1" => Self::Down,
                "2" => Self::Left,
                "3" => Self::Up,
                _ => panic!("Unrecognized direction."),
            }
        }
    }

    #[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
    struct Point {
        x: i64,
        y: i64,
    }

    impl Point {
        fn next_point(&self, direction: Direction, distance: i64) -> Point {
            match direction {
                Direction::Down => Point { 
                    x: self.x,
                    y: self.y - distance 
                },
                Direction::Up => Point {
                    x: self.x,
                    y: self.y + distance
                },
                Direction::Right => Point {
                    x: self.x + distance,
                    y: self.y
                },
                Direction::Left => Point {
                    x: self.x - distance,
                    y: self.y
                },
            }    
        }
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct Soln {
        area: u64,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U64(self.area)
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let mut s_1 = 0;
            let mut s_2 = 0;
            let mut perimeter: u64 = 0;
            let start = Point { x: 0, y: 0 };
            let mut prev = start;
            let re = Regex::new(r"[UDRL] \d+ \(\#(?<distance>[0-9a-f]{5})(?<direction>[0-3])\)").unwrap();
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    let captures = re.captures(&line).expect("Line should match known pattern.");
                    let direction = Direction::from_str(captures.name("direction").unwrap().as_str());
                    let distance = i64::from_str_radix(captures.name("distance").unwrap().as_str(), 16).unwrap();
                    perimeter += distance as u64;
                    let cur = prev.next_point(direction, distance);
                    s_1 += prev.x * cur.y;
                    s_2 += prev.y * cur.x;
                    prev = cur;
                });
            s_1 += prev.x * start.y;
            s_2 += prev.y * start.x;
            // Before any moves, we already have 1 block "dug out" (the start
            // at the origin). And we are incorporating half the perimeter
            // in the shoelace formula.
            self.area = s_1.abs_diff(s_2) / 2 + perimeter / 2 + 1;
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U64(952_408_144_115); "example_1")]
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