#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 19 };

pub mod part_one {

    use std::collections::{HashMap, VecDeque};

    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, PartialEq, Eq)]
    enum Category {
        X,
        M,
        A,
        S,
    }

    impl Category {
        fn from_str(input: &str) -> Self {
            match input {
                "x" => Self::X,
                "m" => Self::M,
                "a" => Self::A,
                "s" => Self::S,
                _ => panic!("Unrecognized category"),
            }
        }
    }
    
    #[derive(Debug, PartialEq, Eq)]
    enum Operation {
        LessThan,
        GreaterThan,
    }

    impl Operation {
        fn from_str(input: &str) -> Self {
            match input {
                "<" => Self::LessThan,
                ">" => Self::GreaterThan,
                _ => panic!("Unrecognized operation"),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Part {
        x: u32,
        m: u32,
        a: u32,
        s: u32,
    }

    impl Part {
        fn category(&self, category: &Category) -> u32 {
            match category {
                Category::X => self.x,
                Category::M => self.m,
                Category::A => self.a,
                Category::S => self.s,
            }
        }

        fn sum_of_categories(&self) -> u32 {
            self.x + self.m + self.a + self.s
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    enum Destination {
        A,
        R,
        Workflow(String),
    }

    impl Destination {
        fn from_str(input: &str) -> Self {
            match input {
                "A" => Self::A,
                "R" => Self::R,
                workflow => Self::Workflow(String::from(workflow)),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Rule {
        category: Category,
        operation: Operation,
        threshold: u32,
        destination: Destination,
    }

    impl Rule {
        fn from_str(input: &str) -> Self {
            // TODO: only create this once?
            let re = Regex::new(r"(?<category>[xmas])(?<operation>[<>])(?<threshold>\d+)\:(?<dest>A|R|[a-z]+)").unwrap();
            let captures = re.captures(input).expect("Should match rule form.");
            let category = Category::from_str(captures.name("category").unwrap().as_str());
            let operation = Operation::from_str(captures.name("operation").unwrap().as_str());
            let threshold = captures.name("threshold").unwrap().as_str().parse().unwrap();
            let destination = Destination::from_str(captures.name("dest").unwrap().as_str());
            Self {
                category,
                operation,
                threshold,
                destination,
            }
        }
    }

    impl Rule {
        fn test(&self, part: &Part) -> Option<Destination> {
            match self.operation {
                Operation::LessThan => if part.category(&self.category) < self.threshold { Some(self.destination.clone()) } else { None },
                Operation::GreaterThan => if part.category(&self.category) > self.threshold { Some(self.destination.clone()) } else { None },
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Workflow {
        rules: Vec<Rule>,
        final_dest: Destination,
    }

    impl Workflow {
        fn new(rules: &str, final_dest: Destination) -> Self {
            Self {
                rules: rules.split(",").map(|rule| Rule::from_str(rule)).collect(),
                final_dest,
            }
        }

        fn process(&self, part: &Part) -> Destination {
            for rule in &self.rules {
                if let Some(dest) = rule.test(part) {
                    return dest;
                }
            }
            self.final_dest.clone()
        }
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct Soln {
        to_process: VecDeque<(Part, Destination)>,
        sum_of_accepted: u32,
        workflows: HashMap<String, Workflow>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            self.process_all_parts();
            Answer::U32(self.sum_of_accepted())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let workflow_re = Regex::new(r"(?<name>[a-z]+)\{(?<rules>[ARa-z><\:\,\d]+)\,(?<final_dest>A|R|[a-z]+)\}").unwrap();
            let part_re = Regex::new(r"\{x\=(?<x>\d+)\,m\=(?<m>\d+)\,a\=(?<a>\d+)\,s\=(?<s>\d+)\}").unwrap();
            let mut parsing_workflows = true;
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    if line.is_empty() { 
                        parsing_workflows = false;
                    } else if parsing_workflows {
                        let captures = workflow_re.captures(&line).unwrap();
                        let name = String::from(captures.name("name").unwrap().as_str());
                        let rules = captures.name("rules").unwrap().as_str();
                        let final_dest = Destination::from_str(captures.name("final_dest").unwrap().as_str());
                        let workflow = Workflow::new(rules, final_dest);
                        self.workflows.insert(name, workflow);
                    } else {
                        let captures = part_re.captures(&line).unwrap();
                        let x = captures.name("x").unwrap().as_str().parse().unwrap();
                        let m = captures.name("m").unwrap().as_str().parse().unwrap();
                        let a = captures.name("a").unwrap().as_str().parse().unwrap();
                        let s = captures.name("s").unwrap().as_str().parse().unwrap();
                        let part = Part {
                            x,
                            m,
                            a,
                            s,
                        };
                        self.to_process.push_back((part, Destination::Workflow(String::from("in"))));
                    }
                });
        }

        fn process_all_parts(&mut self) {
            while !self.to_process.is_empty() {
                let (part, destination) = self.to_process.pop_front().unwrap();
                match destination {
                    Destination::A => self.sum_of_accepted += part.sum_of_categories(),
                    Destination::R => {},
                    Destination::Workflow(name) => {
                        let workflow = self.workflows.get(&name).expect("Should have a workflow matching this name.");
                        let dest = workflow.process(&part);
                        self.to_process.push_back((part, dest));
                    },
                }
            }
        }

        fn sum_of_accepted(&self) -> u32 {
            self.sum_of_accepted
        }

    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(19_114); "example_1")]
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