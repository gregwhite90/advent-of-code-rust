#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 17 };

pub mod part_one {
    use std::collections::BinaryHeap;

    use md5::{Digest, Md5};

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, PartialEq, Eq)]
    struct Point {
        x: usize,
        y: usize,
    }

    #[derive(Debug, Default, PartialEq, Eq, Clone)]
    struct Path {
        steps: Vec<char>,
    }

    impl Path {
        fn len(&self) -> usize {
            self.steps.len()
        }

        fn position(&self) -> Point {
            Point {
                x: self.steps.iter().filter(|step| **step == 'R').count() - self.steps.iter().filter(|step| **step == 'L').count(),
                y: self.steps.iter().filter(|step| **step == 'D').count() - self.steps.iter().filter(|step| **step == 'U').count(),
            }
        }

        fn is_finished(&self) -> bool {
            self.position() == Point { x: 3, y: 3 }
        }

        fn to_string(&self) -> String {
            self.steps.iter().collect::<String>()
        }

        fn push_step(&mut self, step: char) {
            self.steps.push(step);
        }
    }

    impl Ord for Path {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.len().cmp(&self.len()).then_with(|| self.steps.cmp(&other.steps))
        }
    }

    impl PartialOrd for Path {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        passcode: String,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::String(self.shortest_path())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.passcode = io_utils::file_to_string(filename);
        }

        fn shortest_path(&self) -> String {
            let mut pq = BinaryHeap::from([Path::default()]);
            while !pq.is_empty() {
                let path = pq.pop().unwrap();
                if path.is_finished() { return path.to_string(); }
                // TODO: could optimize to only calculate position once.
                let position = path.position();
                // TODO: hash it
                let hash = format!(
                    "{:x}",
                    Md5::digest(format!("{}{}", self.passcode, path.to_string()).as_bytes()),
                );
                let mut chars = hash.chars();
                let u = char_is_open(chars.next().unwrap());
                let d = char_is_open(chars.next().unwrap());
                let l = char_is_open(chars.next().unwrap());
                let r = char_is_open(chars.next().unwrap());
                if u && position.y != 0 {
                    let mut new_path = path.clone();
                    new_path.push_step('U');
                    pq.push(new_path);
                }
                if l && position.x != 0 {
                    let mut new_path = path.clone();
                    new_path.push_step('L');
                    pq.push(new_path);
                }
                if d && position.y != 3 {
                    let mut new_path = path.clone();
                    new_path.push_step('D');
                    pq.push(new_path);
                }
                if r && position.x != 3 {
                    let mut new_path = path.clone();
                    new_path.push_step('R');
                    pq.push(new_path);
                }
            }
            panic!("Explored all paths without reaching destination.");
        }

    }

    fn char_is_open(ch: char) -> bool {
        match ch {
            'b'..='f' => true,
            _ => false,
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::String("DDRRRD".to_string()); "example_1")]
        #[test_case(2, Answer::String("DDUDRLRRUDRD".to_string()); "example_2")]
        #[test_case(3, Answer::String("DRURDRUDDLLDLUURRDULRLDUUDDDRR".to_string()); "example_3")]
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
    use std::{cmp::max, collections::BinaryHeap};

    use md5::{Digest, Md5};

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, PartialEq, Eq)]
    struct Point {
        x: usize,
        y: usize,
    }

    #[derive(Debug, Default, PartialEq, Eq, Clone)]
    struct Path {
        steps: Vec<char>,
    }

    impl Path {
        fn len(&self) -> usize {
            self.steps.len()
        }

        fn position(&self) -> Point {
            Point {
                x: self.steps.iter().filter(|step| **step == 'R').count() - self.steps.iter().filter(|step| **step == 'L').count(),
                y: self.steps.iter().filter(|step| **step == 'D').count() - self.steps.iter().filter(|step| **step == 'U').count(),
            }
        }

        fn is_finished(&self) -> bool {
            self.position() == Point { x: 3, y: 3 }
        }

        fn to_string(&self) -> String {
            self.steps.iter().collect::<String>()
        }

        fn push_step(&mut self, step: char) {
            self.steps.push(step);
        }
    }

    impl Ord for Path {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.len().cmp(&self.len()).then_with(|| self.steps.cmp(&other.steps))
        }
    }

    impl PartialOrd for Path {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        passcode: String,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::Usize(self.longest_path())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.passcode = io_utils::file_to_string(filename);
        }

        fn shortest_path(&self) -> String {
            let mut pq = BinaryHeap::from([Path::default()]);
            while !pq.is_empty() {
                let path = pq.pop().unwrap();
                if path.is_finished() { return path.to_string(); }
                // TODO: could optimize to only calculate position once.
                let position = path.position();
                let hash = format!(
                    "{:x}",
                    Md5::digest(format!("{}{}", self.passcode, path.to_string()).as_bytes()),
                );
                let mut chars = hash.chars();
                let u = char_is_open(chars.next().unwrap());
                let d = char_is_open(chars.next().unwrap());
                let l = char_is_open(chars.next().unwrap());
                let r = char_is_open(chars.next().unwrap());
                if u && position.y != 0 {
                    let mut new_path = path.clone();
                    new_path.push_step('U');
                    pq.push(new_path);
                }
                if l && position.x != 0 {
                    let mut new_path = path.clone();
                    new_path.push_step('L');
                    pq.push(new_path);
                }
                if d && position.y != 3 {
                    let mut new_path = path.clone();
                    new_path.push_step('D');
                    pq.push(new_path);
                }
                if r && position.x != 3 {
                    let mut new_path = path.clone();
                    new_path.push_step('R');
                    pq.push(new_path);
                }
            }
            panic!("Explored all paths without reaching destination.");
        }

        fn longest_path(&self) -> usize {
            let mut pq = BinaryHeap::from([Path::default()]);
            let mut longest_path: Option<usize> = None;
            while !pq.is_empty() {
                let path = pq.pop().unwrap();
                if path.is_finished() { 
                    longest_path = match longest_path {
                        None => Some(path.len()),
                        Some(lp) => Some(max(lp, path.len())),
                    };
                    continue;
                }
                // TODO: could optimize to only calculate position once.
                let position = path.position();
                let hash = format!(
                    "{:x}",
                    Md5::digest(format!("{}{}", self.passcode, path.to_string()).as_bytes()),
                );
                let mut chars = hash.chars();
                let u = char_is_open(chars.next().unwrap());
                let d = char_is_open(chars.next().unwrap());
                let l = char_is_open(chars.next().unwrap());
                let r = char_is_open(chars.next().unwrap());
                if u && position.y != 0 {
                    let mut new_path = path.clone();
                    new_path.push_step('U');
                    pq.push(new_path);
                }
                if l && position.x != 0 {
                    let mut new_path = path.clone();
                    new_path.push_step('L');
                    pq.push(new_path);
                }
                if d && position.y != 3 {
                    let mut new_path = path.clone();
                    new_path.push_step('D');
                    pq.push(new_path);
                }
                if r && position.x != 3 {
                    let mut new_path = path.clone();
                    new_path.push_step('R');
                    pq.push(new_path);
                }
            }
            longest_path.unwrap()
        }
    }

    fn char_is_open(ch: char) -> bool {
        match ch {
            'b'..='f' => true,
            _ => false,
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(370); "example_1")]
        #[test_case(2, Answer::Usize(492); "example_2")]
        #[test_case(3, Answer::Usize(830); "example_3")]
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