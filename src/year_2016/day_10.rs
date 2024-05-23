#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 10 };

pub mod part_one {
    use std::collections::{HashMap, HashSet};

    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, Clone, Copy)]
    enum Destination {
        Bot(u32),
        OutputBin(u32),
    }

    impl Destination {
        fn new(dest_type: &str, id: u32) -> Self {
            match dest_type {
                "bot" => Self::Bot(id),
                "output" => Self::OutputBin(id),
                _ => panic!("Unrecognized destination type"),
            }
        }
    }

    #[derive(Debug)]
    struct Bot {
        id: u32,
        values: Vec<u32>,
        low_dest: Option<Destination>,
        high_dest: Option<Destination>,
    }

    impl Bot {
        fn new(id: u32) -> Self {
            Self {
                id,
                values: Vec::new(),
                low_dest: None,
                high_dest: None,
            }
        }

        fn update_dests(&mut self, low_dest: Destination, high_dest: Destination) {
            self.low_dest = Some(low_dest);
            self.high_dest = Some(high_dest);
        }

        fn push_val(&mut self, val: u32) {
            self.values.push(val);
        }

        fn values(&self) -> usize {
            self.values.len()
        }

        fn clear_values(&mut self) {
            self.values.clear();
        }
    }

    #[derive(Debug)]
    struct OutputBin {
        id: u32,
        values: Vec<u32>,
    }

    #[derive(Debug)]
    pub struct Soln {
        designated_values: HashSet<u32>,
        id_comparing_designated_values: Option<u32>,
        bots: HashMap<u32, Bot>,
        output_bins: HashMap<u32, OutputBin>,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self::with_designated_values(HashSet::from([17, 61]))
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            self.process();
            Answer::U32(self.id_comparing_designated_values.unwrap())
        }
    }

    impl Soln {
        fn with_designated_values(designated_values: HashSet<u32>) -> Self {
            Self {
                designated_values,
                id_comparing_designated_values: None,
                bots: HashMap::new(),
                output_bins: HashMap::new(),
            }
        }

        fn parse_input_file(&mut self, filename: &str) {
            let input_re = Regex::new(r"value (?<val>\d+) goes to bot (?<bot_id>\d+)").unwrap();
            let instruction_re = Regex::new(
                r"bot (?<bot_id>\d+) gives low to (?<low_dest_type>(bot)|(output)) (?<low_dest_id>\d+) and high to (?<high_dest_type>(bot)|(output)) (?<high_dest_id>\d+)"
                ).unwrap();
            for line in io_utils::file_to_lines(filename) {
                if let Some(captures) = input_re.captures(&line) {
                    let bot_id = captures.name("bot_id").unwrap().as_str().parse().unwrap();
                    let val = captures.name("val").unwrap().as_str().parse().unwrap();
                    self.bots.entry(bot_id).or_insert(Bot::new(bot_id)).push_val(val);
                } else {
                    let captures = instruction_re.captures(&line).unwrap();
                    let bot_id = captures.name("bot_id").unwrap().as_str().parse().unwrap();
                    let low_dest = Destination::new(
                        captures.name("low_dest_type").unwrap().as_str(), 
                        captures.name("low_dest_id").unwrap().as_str().parse().unwrap(),
                    );
                    let high_dest = Destination::new(
                        captures.name("high_dest_type").unwrap().as_str(), 
                        captures.name("high_dest_id").unwrap().as_str().parse().unwrap(),
                    );
                    self.bots.entry(bot_id).or_insert(Bot::new(bot_id)).update_dests(low_dest, high_dest);
                }
            }
        }

        fn process(&mut self) {
            'outer: while self.bots.values().any(|bot| bot.values() == 2) {
                let mut destinations = Vec::<(Destination, u32)>::new();
                for bot in self.bots.values_mut().filter(|bot| bot.values() == 2) {
                    if HashSet::from_iter(bot.values.iter().cloned()) == self.designated_values {
                        self.id_comparing_designated_values = Some(bot.id);
                        break 'outer;
                    }
                    destinations.push((
                        bot.low_dest.unwrap(),
                        *bot.values.iter().min().unwrap(),
                    ));
                    destinations.push((
                        bot.high_dest.unwrap(),
                        *bot.values.iter().max().unwrap(),
                    ));
                    bot.clear_values();
                }
                for (dest, val) in destinations.into_iter() {
                    if let Destination::Bot(id) = dest {
                        self.bots.get_mut(&id).unwrap().push_val(val);
                    }
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

        #[test_case(1, Answer::U32(2); "example_1")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::with_designated_values(HashSet::from([2, 5])),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}

pub mod part_two {
    use std::collections::{HashMap, HashSet};

    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, Clone, Copy)]
    enum Destination {
        Bot(u32),
        OutputBin(u32),
    }

    impl Destination {
        fn new(dest_type: &str, id: u32) -> Self {
            match dest_type {
                "bot" => Self::Bot(id),
                "output" => Self::OutputBin(id),
                _ => panic!("Unrecognized destination type"),
            }
        }
    }

    #[derive(Debug)]
    struct Bot {
        id: u32,
        values: Vec<u32>,
        low_dest: Option<Destination>,
        high_dest: Option<Destination>,
    }

    impl Bot {
        fn new(id: u32) -> Self {
            Self {
                id,
                values: Vec::new(),
                low_dest: None,
                high_dest: None,
            }
        }

        fn update_dests(&mut self, low_dest: Destination, high_dest: Destination) {
            self.low_dest = Some(low_dest);
            self.high_dest = Some(high_dest);
        }

        fn push_val(&mut self, val: u32) {
            self.values.push(val);
        }

        fn values(&self) -> usize {
            self.values.len()
        }

        fn clear_values(&mut self) {
            self.values.clear();
        }
    }

    #[derive(Debug)]
    struct OutputBin {
        id: u32,
        values: Vec<u32>,
    }

    impl OutputBin {
        fn new(id: u32) -> Self {
            Self {
                id,
                values: Vec::new(),
            }
        }

        fn push_val(&mut self, val: u32) {
            self.values.push(val);
        }

        fn value(&self) -> u32 {
            *self.values.iter().next().unwrap()
        }
    }

    #[derive(Debug)]
    pub struct Soln {
        designated_values: HashSet<u32>,
        id_comparing_designated_values: Option<u32>,
        bots: HashMap<u32, Bot>,
        output_bins: HashMap<u32, OutputBin>,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self::with_designated_values(HashSet::from([17, 61]))
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            self.process();
            Answer::U32(self.output_value())
        }
    }

    impl Soln {
        fn with_designated_values(designated_values: HashSet<u32>) -> Self {
            Self {
                designated_values,
                id_comparing_designated_values: None,
                bots: HashMap::new(),
                output_bins: HashMap::new(),
            }
        }

        fn parse_input_file(&mut self, filename: &str) {
            let input_re = Regex::new(r"value (?<val>\d+) goes to bot (?<bot_id>\d+)").unwrap();
            let instruction_re = Regex::new(
                r"bot (?<bot_id>\d+) gives low to (?<low_dest_type>(bot)|(output)) (?<low_dest_id>\d+) and high to (?<high_dest_type>(bot)|(output)) (?<high_dest_id>\d+)"
                ).unwrap();
            for line in io_utils::file_to_lines(filename) {
                if let Some(captures) = input_re.captures(&line) {
                    let bot_id = captures.name("bot_id").unwrap().as_str().parse().unwrap();
                    let val = captures.name("val").unwrap().as_str().parse().unwrap();
                    self.bots.entry(bot_id).or_insert(Bot::new(bot_id)).push_val(val);
                } else {
                    let captures = instruction_re.captures(&line).unwrap();
                    let bot_id = captures.name("bot_id").unwrap().as_str().parse().unwrap();
                    let low_dest = Destination::new(
                        captures.name("low_dest_type").unwrap().as_str(), 
                        captures.name("low_dest_id").unwrap().as_str().parse().unwrap(),
                    );
                    let high_dest = Destination::new(
                        captures.name("high_dest_type").unwrap().as_str(), 
                        captures.name("high_dest_id").unwrap().as_str().parse().unwrap(),
                    );
                    self.bots.entry(bot_id).or_insert(Bot::new(bot_id)).update_dests(low_dest, high_dest);
                }
            }
        }

        fn process(&mut self) {
            while self.bots.values().any(|bot| bot.values() == 2) {
                let mut destinations = Vec::<(Destination, u32)>::new();
                for bot in self.bots.values_mut().filter(|bot| bot.values() == 2) {
                    if HashSet::from_iter(bot.values.iter().cloned()) == self.designated_values {
                        self.id_comparing_designated_values = Some(bot.id);
                    }
                    destinations.push((
                        bot.low_dest.unwrap(),
                        *bot.values.iter().min().unwrap(),
                    ));
                    destinations.push((
                        bot.high_dest.unwrap(),
                        *bot.values.iter().max().unwrap(),
                    ));
                    bot.clear_values();
                }
                for (dest, val) in destinations.into_iter() {
                    match dest {
                        Destination::Bot(id) => self.bots.get_mut(&id).unwrap().push_val(val),
                        Destination::OutputBin(id) => {
                            self.output_bins.entry(id).or_insert(OutputBin::new(id)).push_val(val);
                        }
                    }
                }
            }
        }

        fn output_value(&self) -> u32 {
            [0, 1, 2].iter().map(|id| self.output_bins.get(id).unwrap().value()).product()
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(30); "example_1")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::with_designated_values(HashSet::from([2, 5])),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}