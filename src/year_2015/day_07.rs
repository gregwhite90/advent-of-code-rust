#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2015, day: 7 };

mod utils {
    use std::collections::HashMap;

    use regex::Regex;

    use crate::utils::io_utils;

    #[derive(Debug, Clone)]
    struct Gate {
        gate_type: GateType,
        inputs: Vec<GateInput>,
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    enum GateType {
        Wire,
        And,
        Or,
        LShift,
        RShift,
        Not,
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    enum GateInput {
        Wire(String),
        Value(u16),
    }

    impl GateInput {
        fn from_str(input: &str) -> Self {
            if let Ok(val) = input.parse::<u16>() {
                Self::Value(val)
            } else {
                Self::Wire(input.to_string())
            }
        }
    }

    /**
     * Gate types:
     *  - WIRE (one input: u16 or wire)
     *  - AND (two inputs: u16 or wire)
     *  - OR (two inputs: u16 or wire)
     *  - LSHIFT (two inputs: u16 or wire and u16)
     *  - RSHIFT (two inputs: u16 or wire and u16)
     *  - NOT (one input: u16 or wire)
     */
    // Input is either the output of a gate or a u16 directly. We could just do a wire gate.
    // TODO: figure out the ownership complexities
    // TODO: figure out if we can memoize

    #[derive(Debug, Default)]
    pub struct Circuit {
        wires: HashMap<String, Gate>,
        wire_values: HashMap<String, u16>,
    }

    impl Circuit {
        pub fn parse_input_file(&mut self, filename: &str) {
            let wire_re = Regex::new(r"(?<input_0>\d+|[a-z]+) -> (?<output>[a-z]+)").unwrap();
            let not_re = Regex::new(r"NOT (?<input_0>\d+|[a-z]+) -> (?<output>[a-z]+)").unwrap();
            let two_operand_re = Regex::new(r"(?<input_0>\d+|[a-z]+) (?<op>(AND)|(OR)|(LSHIFT)|(RSHIFT)) (?<input_1>\d+|[a-z]+) -> (?<output>[a-z]+)").unwrap();
            for line in io_utils::file_to_lines(filename) {
                if let Some(captures) = two_operand_re.captures(&line) {
                    let input_0 = captures.name("input_0").unwrap().as_str();
                    let input_0 = GateInput::from_str(input_0);
                    let input_1 = captures.name("input_1").unwrap().as_str();
                    let input_1 = GateInput::from_str(input_1);
                    let op = captures.name("op").unwrap().as_str();
                    let output = captures.name("output").unwrap().as_str();
                    let gate_type = match op {
                        "AND" => GateType::And,
                        "OR" => GateType::Or,
                        "LSHIFT" => GateType::LShift,
                        "RSHIFT" => GateType::RShift,
                        _ => panic!("Unrecognized operation"),
                    };
                    self.wires.insert(
                        output.to_string(), 
                        Gate { 
                            gate_type,
                            inputs: vec![input_0, input_1],
                        },
                    );
                } else if let Some(captures) = not_re.captures(&line) {
                    let input = captures.name("input_0").unwrap().as_str();
                    let input = GateInput::from_str(input);
                    let output = captures.name("output").unwrap().as_str();
                    self.wires.insert(
                        output.to_string(), 
                        Gate { 
                            gate_type: GateType::Not,
                            inputs: vec![input],
                        },
                    );
                } else if let Some(captures) = wire_re.captures(&line) {
                    let input = captures.name("input_0").unwrap().as_str();
                    let input = GateInput::from_str(input);
                    let output = captures.name("output").unwrap().as_str();
                    self.wires.insert(
                        output.to_string(), 
                        Gate { 
                            gate_type: GateType::Wire,
                            inputs: vec![input],
                        },
                    );
                } else {
                    panic!("Unrecognized line");
                }
            }
        }

        pub fn wire_value(&mut self, wire: &str) -> u16 {
            if let Some(val) = self.wire_values.get(wire) {
                return *val;
            }
            let gate = self.wires.get(wire).unwrap().clone();
            let signal = match gate.gate_type {
                GateType::Wire => {
                    self.evaluate_gate_input(&gate, 0)
                },
                GateType::And => {
                    self.evaluate_gate_input(&gate, 0) & self.evaluate_gate_input(&gate, 1)
                },
                GateType::Or => {
                    self.evaluate_gate_input(&gate, 0) | self.evaluate_gate_input(&gate, 1)                    
                },
                GateType::LShift => {
                    self.evaluate_gate_input(&gate, 0) << self.evaluate_gate_input(&gate, 1)
                },
                GateType::RShift => {
                    self.evaluate_gate_input(&gate, 0) >> self.evaluate_gate_input(&gate, 1)                    
                },
                GateType::Not => {
                    !self.evaluate_gate_input(&gate, 0)
                },
            };
            self.wire_values.insert(wire.to_string(), signal);
            signal
        }

        fn evaluate_gate_input(&mut self, gate: &Gate, index: usize) -> u16 {
            match gate.inputs.get(index).unwrap() {
                GateInput::Wire(wire) => self.wire_value(wire),
                GateInput::Value(val) => {
                    *val
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use io_utils::{InputFileType, input_filename};
        use test_case::test_case;
        use super::*;
        use super::super::DAY;

        #[test_case("d", 72; "d")]
        #[test_case("e", 507; "e")]
        #[test_case("f", 492; "f")]
        #[test_case("g", 114; "g")]
        #[test_case("h", 65_412; "h")]
        #[test_case("i", 65_079; "i")]
        #[test_case("x", 123; "x")]
        #[test_case("y", 456; "y")]
        fn examples_are_correct(wire: &str, answer: u16) {
            let mut circuit = Circuit::default();
            circuit.parse_input_file(&input_filename(&DAY, InputFileType::Example(1)));
            assert_eq!(answer, circuit.wire_value(wire));
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::Circuit;

    #[derive(Debug, Default)]
    pub struct Soln {
        circuit: Circuit,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.circuit.parse_input_file(filename);
            Answer::U16(self.circuit.wire_value("a"))
        }
    }
}