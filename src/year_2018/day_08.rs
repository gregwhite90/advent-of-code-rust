#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 8 };

mod utils {
    #[derive(Debug, Default)]
    pub struct Node {
        children: Vec<Node>,
        metadata: Vec<usize>,
        value: Option<usize>,
    }

    impl Node {
        pub fn from_iter(iter: &mut impl Iterator<Item = usize>) -> Self {
            let num_children = iter.next().unwrap();
            let num_metadata = iter.next().unwrap();
            let children = (0..num_children).map(|_| {
                Node::from_iter(iter)
            }).collect();
            let metadata = iter.take(num_metadata).collect();
            Self {
                children,
                metadata,
                value: None,
            }
        }            

        pub fn metadata_sum_recursive(&self) -> usize {
            self.metadata.iter().sum::<usize>() + self.children.iter().map(|node| node.metadata_sum_recursive()).sum::<usize>()
        }

        pub fn value(&mut self) -> usize {
            if let Some(val) = self.value { val }
            else if self.children.is_empty() { 
                let val = self.metadata.iter().sum::<usize>();
                self.value = Some(val);
                val
            } else {
                let val = self.metadata.iter().map(|idx| {
                    if *idx >= 1 && *idx <= self.children.len() {
                        self.children[*idx - 1].value()
                    } else {
                        0
                    }
                }).sum::<usize>();
                self.value = Some(val);
                val
            }
        }
    }
}

pub mod part_one {
    use crate::utils::{io_utils, solution::{Answer, Solution}};

    use super::utils::Node;

    #[derive(Debug, Default)]
    pub struct Soln {
        root: Node,    
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::Usize(self.root.metadata_sum_recursive())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.root = Node::from_iter(
                &mut io_utils::file_to_string(filename)
                    .split_whitespace()
                    .map(|num| num.parse::<usize>().unwrap())
            );
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(138); "example_1")]
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
    use crate::utils::{io_utils, solution::{Answer, Solution}};

    use super::utils::Node;

    #[derive(Debug, Default)]
    pub struct Soln {
        root: Node,    
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::Usize(self.root.value())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.root = Node::from_iter(
                &mut io_utils::file_to_string(filename)
                    .split_whitespace()
                    .map(|num| num.parse::<usize>().unwrap())
            );
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(66); "example_1")]
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