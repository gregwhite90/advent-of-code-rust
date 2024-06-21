#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 13 };

mod utils {
    use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}, fmt::Display};

    use crate::utils::io_utils;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    enum Direction {
        N,
        E,
        S,
        W,
    }

    impl Direction {
        fn from_char(input: char) -> Self {
            match input {
                '^' => Self::N,
                '>' => Self::E,
                'v' => Self::S,
                '<' => Self::W,
                _ => panic!("Unrecognized direction input"),
            }
        }

        fn turn(&mut self, turn_direction: TurnDirection) {
            *self = match (&self, turn_direction) {
                (Direction::N, TurnDirection::Left) | (Direction::W, TurnDirection::Straight) | (Direction::S, TurnDirection::Right) => Direction::W,
                (Direction::E, TurnDirection::Left) | (Direction::N, TurnDirection::Straight) | (Direction::W, TurnDirection::Right) => Direction::N,
                (Direction::S, TurnDirection::Left) | (Direction::E, TurnDirection::Straight) | (Direction::N, TurnDirection::Right) => Direction::E,
                (Direction::W, TurnDirection::Left) | (Direction::S, TurnDirection::Straight) | (Direction::E, TurnDirection::Right) => Direction::S,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    enum TurnDirection {
        Left,
        Straight,
        Right,
    }

    impl TurnDirection {
        fn advance(&mut self) {
            *self = match self {
                TurnDirection::Left => TurnDirection::Straight,
                TurnDirection::Straight => TurnDirection::Right,
                TurnDirection::Right => TurnDirection::Left,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    pub struct Point {
        x: usize,
        y: usize,
    }

    impl Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{},{}", self.x, self.y)
        }
    }

    impl Point {
        fn next_point(&self, direction: &Direction) -> Self {
            Self {
                x: match direction { 
                    Direction::E => self.x + 1,
                    Direction::W => self.x - 1,
                    _ => self.x,
                },
                y: match direction { 
                    Direction::N => self.y - 1,
                    Direction::S => self.y + 1,
                    _ => self.y,
                },
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    struct Cart {
        location: Point,
        direction: Direction,
        next_turn_direction: TurnDirection,
    }

    impl Cart {
        fn new(location: Point, direction: Direction) -> Self {
            Self {
                location,
                direction,
                next_turn_direction: TurnDirection::Left,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Track {
        NS,
        EW,
        NWSE, // If going North, now going West, and vice versa. If going South, now going East, and vice versa.
        NESW, // If going North, now going East, and vice versa. If going South, now going West, and vice versa.
        Intersection,
    }

    impl Track {
        fn from_char(input: char) -> Self {
            match input {
                '|' | '^' | 'v' => Self::NS,
                '-' | '>' | '<' => Self::EW,
                '/' => Self::NESW,
                '\\' => Self::NWSE,
                '+' => Self::Intersection,
                _ => panic!("Unrecognized track input"),
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct Tracks {
        tracks: HashMap<Point, Track>,
        carts: BinaryHeap<Reverse<Cart>>,
    }

    impl Tracks {
        pub fn parse_input_file(&mut self, filename: &str) {
            for (row, line) in io_utils::file_to_lines(filename).enumerate() {
                for (col, ch) in line.char_indices() {
                    if ch == ' ' { continue; }
                    let point = Point { x: col, y: row };
                    let track = Track::from_char(ch);
                    self.tracks.insert(point, track);
                    if ch == '^' || ch == '>' || ch == 'v' || ch == '<' {
                        let direction = Direction::from_char(ch);
                        self.carts.push(Reverse(Cart::new(point, direction)));
                    }
                }
            }
        }

        /// Returns a vector of points at which a collision occured
        fn tick(&mut self) -> HashSet<Point> {
            let mut new_carts = BinaryHeap::new();
            let mut new_positions = HashSet::new();
            let mut collisions = HashSet::new();
            while !self.carts.is_empty() {
                let mut cart = self.carts.pop().unwrap().0;
                self.advance(&mut cart);  
                // TODO: a collision happens when a cart moves into a space occupied by another
                // cart. This version only catches collisions when the carts start with one track
                // between them facing different directions. e.g.,
                //    -->-<--
                //    ---X---
                // We need to adjust to also catch collisions when the carts start next to each other. e.g.,
                //    --><--
                // This should result in
                //    ---X--
                // midway through the tick, but right now it results in
                //    --<>--
                // after the tick
                if !new_positions.insert(cart.location) {
                    collisions.insert(cart.location);
                    continue;
                }
                new_carts.push(Reverse(cart));
            }
            new_carts.retain(|cart| {
                !collisions.contains(&cart.0.location)
            });
            self.carts = new_carts;
            collisions
        }

        fn advance(&self, cart: &mut Cart) {
            let next_location = cart.location.next_point(&cart.direction);
            let mut next_direction = cart.direction;
            let mut next_next_turn_direction = cart.next_turn_direction;
            match self.tracks.get(&next_location).unwrap() {
                Track::NS | Track::EW => (),
                Track::NESW => {
                    next_direction = match cart.direction {
                        Direction::N => Direction::E,
                        Direction::E => Direction::N,
                        Direction::S => Direction::W,
                        Direction::W => Direction::S,
                    }
                },
                Track::NWSE => {
                    next_direction = match cart.direction {
                        Direction::N => Direction::W,
                        Direction::W => Direction::N,
                        Direction::S => Direction::E,
                        Direction::E => Direction::S,
                    }
                },
                Track::Intersection => {
                    next_direction.turn(cart.next_turn_direction);
                    next_next_turn_direction.advance();
                }
            }
            cart.location = next_location;
            cart.direction = next_direction;
            cart.next_turn_direction = next_next_turn_direction
        }

        pub fn first_collision_point(&mut self) -> Point {
            let mut collisions = HashSet::new();
            while collisions.is_empty() {
                collisions = self.tick();
            }
            assert_eq!(1, collisions.len());
            collisions.into_iter().next().unwrap()
        }

        pub fn last_cart_standing_location(&mut self) -> Point {
            while self.carts.len() > 1 {
                self.tick();
            }            
            self.carts.pop().unwrap().0.location
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::Tracks;

    #[derive(Debug, Default)]
    pub struct Soln {
        tracks: Tracks,    
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.tracks.parse_input_file(filename);
            Answer::String(format!("{}", self.tracks.first_collision_point()))
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::String("7,3".to_string()); "example_1")]
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

    use super::utils::Tracks;

    #[derive(Debug, Default)]
    pub struct Soln {
        tracks: Tracks,    
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.tracks.parse_input_file(filename);
            Answer::String(format!("{}", self.tracks.last_cart_standing_location()))
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(2, Answer::String("6,4".to_string()); "example_2")]
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