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

pub mod part_two {

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

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum SplitDestWorkflow {
        Current,
        New,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct Range {
        min: u64, // inclusive
        max: u64, // inclusive
    }

    impl Default for Range {
        fn default() -> Self {
            Self {
                min: 1,
                max: 4_000,
            }
        }
    }

    impl Range {
        fn range(&self) -> u64 {
            self.max - self.min + 1
        }

        /// Returns a vector of 1-2 `Range`s and whether they should continue on to the next rule in the
        /// current workflow or go to a new workflow.
        fn split(self, threshold: u64, operation: Operation) -> Vec<(Range, SplitDestWorkflow)> {
            let mut result = Vec::new();
            if operation == Operation::GreaterThan && threshold >= self.max || operation == Operation::LessThan && threshold <= self.min {
                result.push((self, SplitDestWorkflow::Current));
            } else if operation == Operation::GreaterThan && threshold < self.min || operation == Operation::LessThan && threshold > self.max {
                result.push((self, SplitDestWorkflow::New));
            } else {
                let (current, new) = match operation {
                    Operation::GreaterThan => {
                        (Range { min: self.min, max: threshold }, Range { min: threshold + 1, max: self.max })
                    },
                    Operation::LessThan => {
                        (Range { min: threshold, max: self.max }, Range { min: self.min, max: threshold - 1 })
                    },
                };
                result.push((current, SplitDestWorkflow::Current));
                result.push((new, SplitDestWorkflow::New));
            }
            return result;
        }
    }

    #[derive(Debug, Default, PartialEq, Eq, Clone)]
    struct PartCombination {
        x: Range,
        m: Range,
        a: Range,
        s: Range,
    }

    impl PartCombination {
        fn category_clone(&self, category: &Category) -> Range {
            match category {
                Category::X => self.x.clone(),
                Category::M => self.m.clone(),
                Category::A => self.a.clone(),
                Category::S => self.s.clone(),
            }
        }

        fn combinations(&self) -> u64 {
            self.x.range() * self.m.range() * self.a.range() * self.s.range()
        }

        fn clone_with_new_category_range(&self, category: &Category, range: Range) -> PartCombination {
            PartCombination {
                x: if *category == Category::X { range } else { self.x },
                m: if *category == Category::M { range } else { self.m },
                a: if *category == Category::A { range } else { self.a },
                s: if *category == Category::S { range } else { self.s },
            }
        }
    }
    
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
        threshold: u64,
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
        /// Applies the rule to a `PartCombination`. Returns 1-2 new `PartCombination`s, possibly one that should continue
        /// on the current workflow, and possibly one that should have a new workflow as a destination.
        fn split(&self, part_combination: &PartCombination) -> Vec<(PartCombination, SplitDestWorkflow)> {
            let split_ranges = part_combination.category_clone(&self.category).split(self.threshold, self.operation);
            split_ranges.into_iter()
                .map(|(range, split_dest_workflow)| {
                    (part_combination.clone_with_new_category_range(&self.category, range), split_dest_workflow)
                }).collect()
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
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct Soln {
        to_process: VecDeque<(PartCombination, Destination)>,
        sum_of_combinations: u64,
        workflows: HashMap<String, Workflow>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            self.find_all_combinations();
            Answer::U64(self.sum_of_combinations())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let workflow_re = Regex::new(r"(?<name>[a-z]+)\{(?<rules>[ARa-z><\:\,\d]+)\,(?<final_dest>A|R|[a-z]+)\}").unwrap();
            for line in io_utils::file_to_lines(filename) {
                if line.is_empty() { 
                    return;
                } else {
                    let captures = workflow_re.captures(&line).unwrap();
                    let name = String::from(captures.name("name").unwrap().as_str());
                    let rules = captures.name("rules").unwrap().as_str();
                    let final_dest = Destination::from_str(captures.name("final_dest").unwrap().as_str());
                    let workflow = Workflow::new(rules, final_dest);
                    self.workflows.insert(name, workflow);
                }
            }
        }

        fn find_all_combinations(&mut self) {
            self.to_process.push_back((PartCombination::default(), Destination::Workflow(String::from("in"))));
            while !self.to_process.is_empty() {
                let (part_combination, dest) = self.to_process.pop_front().unwrap();
                match dest {
                    Destination::A => self.sum_of_combinations += part_combination.combinations(),
                    Destination::R => {},
                    Destination::Workflow(workflow_name) => self.process_workflow(&workflow_name, part_combination),
                }
            }
        }

        fn process_workflow(&mut self, workflow_name: &str, mut part_combination: PartCombination) {
            let workflow = self.workflows.get(workflow_name).expect("Workflow with name should exist");
            for rule in &workflow.rules {
                for (pc, split_dest_workflow) in rule.split(&part_combination) {
                    match split_dest_workflow {
                        SplitDestWorkflow::Current => part_combination = pc,
                        SplitDestWorkflow::New => self.to_process.push_back((pc, rule.destination.clone())),
                    }
                }
            }
            self.to_process.push_back((part_combination, workflow.final_dest.clone()));
        }

        fn sum_of_combinations(&self) -> u64 {
            self.sum_of_combinations
        }

    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U64(167_409_079_868_000); "example_1")]
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