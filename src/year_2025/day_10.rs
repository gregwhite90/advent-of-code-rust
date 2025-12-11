#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2025, day: 10 };

mod utils {
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        pub static ref MACHINE_RE: Regex = Regex::new(r"\[(?<indicator_lights>[\.\#]+)\] (?<buttons>[\( \)\,\d]+) \{(?<joltages>[\d\,]+)\}").unwrap();
        pub static ref BUTTONS_RE: Regex = Regex::new(r"(?:\()([\d\,]+)(?:\))").unwrap();
    }
}

pub mod part_one {
    use std::{cmp::Reverse, collections::{BinaryHeap, HashSet}};

    use crate::utils::{io_utils, solution::{Answer, Solution}};
    use super::utils::{MACHINE_RE, BUTTONS_RE};

    
    // TODO: this may be overkill
    #[derive(Debug, Default, PartialEq, Eq)]
    struct Button {
        mask: u16,
    }

    impl Button {
        fn new_indicator_lights(&self, indicator_lights: u16) -> u16 {
            self.mask ^ indicator_lights
        }
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    struct Machine {
        on_indicator_lights: u16,
        buttons: Vec<Button>, // TODO: decide if Vec is right?
        joltages: Vec<u64>,
    }

    impl Machine {
        pub fn from_str(input: &str) -> Self {
            let caps = MACHINE_RE.captures(input).unwrap();
            let on_indicator_lights = caps.name("indicator_lights").unwrap().as_str().to_string();
            let buttons = BUTTONS_RE.captures_iter(caps.name("buttons").unwrap().as_str())
                .map(|cap| {
                    let idxs: HashSet<usize> = cap.get(1)
                        .unwrap()
                        .as_str()
                        .split(',')
                        .map(|num| num.parse().unwrap())
                        .collect();
                    let mask = u16::from_str_radix(
                        (0..on_indicator_lights.len()).map(|idx| {
                            match idxs.contains(&idx) {
                                true => '1',
                                false => '0',
                            }
                        })
                        .collect::<String>()
                        .as_str(),
                        2,
                    ).unwrap();
                    Button { mask }
                })
                .collect();
            let on_indicator_lights = u16::from_str_radix(
                on_indicator_lights.chars()
                    .map(|ch| {
                        match ch {
                            '.' => '0',
                            '#' => '1',
                            _ => unreachable!(),
                        }
                    })
                    .collect::<String>()
                    .as_str(),
                    2,
                ).unwrap();
            let joltages = caps.name("joltages")
                .unwrap()
                .as_str()
                .split(',')
                .map(|num| num.parse().unwrap())
                .collect();
            Self {
                on_indicator_lights,
                buttons,
                joltages,
            }
        }

        fn next_states(&self, state: &State) -> Vec<State> {
            self.buttons.iter()
                .map(|btn| {
                    State {
                        cost: state.cost + 1,
                        indicator_lights: btn.new_indicator_lights(state.indicator_lights)
                    }
                })
                .collect()
        }

        fn is_end_state(&self, state: &State) -> bool {
            self.on_indicator_lights == state.indicator_lights
        }

        // Returns the minimal cost (button clicks) to confiugre the indicator lights
        pub fn configure_indicator_lights(&self) -> u64 {
            let mut pq: BinaryHeap<Reverse<State>> = BinaryHeap::from([Reverse(State::default())]);
            let mut visited_indicator_lights: HashSet<u16> = HashSet::new();
            while let Some(Reverse(state)) = pq.pop() {
                if self.is_end_state(&state) {
                    return state.cost;
                }
                if !visited_indicator_lights.insert(state.indicator_lights) {
                    continue;
                }
                pq.extend(
                    self.next_states(&state).into_iter().map(|state| Reverse(state))
                );
            }
            panic!("Emptied priority queue without reaching end state.");
        }
    }

    #[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    struct State {
        cost: u64,
        indicator_lights: u16,
    }


    #[derive(Debug, Default)]
    pub struct Soln {}

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            Answer::U64(
                io_utils::file_to_lines(filename)
                    .map(|line| {
                        let machine = Machine::from_str(&line);
                        machine.configure_indicator_lights()
                    })
                    .sum()
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
            Machine {
                on_indicator_lights: 6,
                buttons: vec![
                    Button { mask:  1 },
                    Button { mask:  5 },
                    Button { mask:  2 },
                    Button { mask:  3 },
                    Button { mask: 10 },
                    Button { mask: 12 },
                ],
                joltages: vec![3, 5, 4, 7],
            };
            "example_1"
        )]
        fn machine_from_str_is_correct(input: &str, expected: Machine) {
            assert_eq!(
                Machine::from_str(input),
                expected,
            );
        }

        #[test_case(1, Answer::U64(7); "example_1")]
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