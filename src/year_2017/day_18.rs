#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 18 };

mod utils {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum ArgType {
        Char(char),
        I64(i64),
    }
}

pub mod part_one {
    use std::collections::HashMap;

    use regex::Regex;

    use crate::utils::{solution::{Solution, Answer}, io_utils};
    use super::utils::ArgType;

    #[derive(Debug, PartialEq, Eq)]
    enum Instruction {
        Snd(ArgType),
        Set(char, ArgType),
        Add(char, ArgType),
        Mul(char, ArgType),
        Mod(char, ArgType),
        Rcv(ArgType),
        Jgz(ArgType, ArgType),
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        instructions: Vec<Instruction>,
        position: i64,
        last_sound: i64,
        registers: HashMap<char, i64>,
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct InstructionResult {
        finished: bool,
        recovered_value: Option<i64>,
    }

    impl Soln {
        pub fn parse_input_file(&mut self, filename: &str) {
            let re = Regex::new(r"(?<operation>[a-z]{3}) (?<args>[ a-z\-\d]+)").unwrap();
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    let captures = re.captures(&line)
                        .expect("Line should match regex.");
                    let operation = captures.name("operation").unwrap().as_str();
                    let args = captures.name("args").unwrap().as_str();
                    let instruction = match operation {
                        "snd" => {
                            if let Ok(val) = args.parse() {
                                Instruction::Snd(ArgType::I64(val))
                            } else {
                                Instruction::Snd(ArgType::Char(args.chars().next().unwrap()))
                            }                
                        },
                        "set" => {
                            let mut args = args.split(" ");
                            let arg_1 = args.next().unwrap().chars().next().unwrap();
                            let arg_2 = args.next().unwrap();
                            if let Ok(val) = arg_2.parse() {
                                Instruction::Set(
                                    arg_1,
                                    ArgType::I64(val),
                                )
                            } else {
                                Instruction::Set(
                                    arg_1,
                                    ArgType::Char(arg_2.chars().next().unwrap()),
                                )
                            }                
                        },
                        "add" => {
                            let mut args = args.split(" ");
                            let arg_1 = args.next().unwrap().chars().next().unwrap();
                            let arg_2 = args.next().unwrap();
                            if let Ok(val) = arg_2.parse() {
                                Instruction::Add(
                                    arg_1,
                                    ArgType::I64(val),
                                )
                            } else {
                                Instruction::Add(
                                    arg_1,
                                    ArgType::Char(arg_2.chars().next().unwrap()),
                                )
                            }                
                        },
                        "mul" => {
                            let mut args = args.split(" ");
                            let arg_1 = args.next().unwrap().chars().next().unwrap();
                            let arg_2 = args.next().unwrap();
                            if let Ok(val) = arg_2.parse() {
                                Instruction::Mul(
                                    arg_1,
                                    ArgType::I64(val),
                                )
                            } else {
                                Instruction::Mul(
                                    arg_1,
                                    ArgType::Char(arg_2.chars().next().unwrap()),
                                )
                            }                
                        },
                        "mod" => {
                            let mut args = args.split(" ");
                            let arg_1 = args.next().unwrap().chars().next().unwrap();
                            let arg_2 = args.next().unwrap();
                            if let Ok(val) = arg_2.parse() {
                                Instruction::Mod(
                                    arg_1,
                                    ArgType::I64(val),
                                )
                            } else {
                                Instruction::Mod(
                                    arg_1,
                                    ArgType::Char(arg_2.chars().next().unwrap()),
                                )
                            }                
                        },
                        "rcv" => {
                            if let Ok(val) = args.parse() {
                                Instruction::Rcv(
                                    ArgType::I64(val),
                                )
                            } else {
                                Instruction::Rcv(
                                    ArgType::Char(args.chars().next().unwrap()),
                                )
                            }                
                        },
                        "jgz" => {
                            let mut args = args.split(" ");
                            let arg_1 = args.next().unwrap();
                            let val_1 = if let Ok(val) = arg_1.parse() {
                                ArgType::I64(val)
                            } else {
                                ArgType::Char(arg_1.chars().next().unwrap())
                            };
                            let arg_2 = args.next().unwrap();
                            let val_2 = if let Ok(val) = arg_2.parse() {
                                ArgType::I64(val)
                            } else {
                                ArgType::Char(arg_2.chars().next().unwrap())
                            };
                            Instruction::Jgz(
                                val_1,
                                val_2,
                            )
                        },
                        _ => panic!("Unrecognized operation: {operation}"),
                    };
                    self.instructions.push(instruction);
                });
        }

        fn handle_next_instruction(&mut self) -> InstructionResult {
            if self.position < 0 || self.position as usize >= self.instructions.len() {
                return InstructionResult {
                    finished: true,
                    recovered_value: None,
                }
            }
            let finished = false;
            let mut recovered_value: Option<i64> = None;
            let instruction = &self.instructions[self.position as usize];
            let result = match instruction {
                Instruction::Snd(arg) => {
                    self.last_sound = self.get_value(*arg);
                    InstructionResult {
                        finished,
                        recovered_value,
                    }
                },
                Instruction::Set(register, arg) => {
                    let val = self.get_value(*arg);
                    self.registers.insert(*register, val);
                    InstructionResult {
                        finished,
                        recovered_value,
                    }
                },
                Instruction::Add(register, arg) => {
                    let val = self.get_value(*arg);
                    self.registers.entry(*register)
                        .and_modify(|e| *e += val)
                        .or_insert(val);
                    InstructionResult {
                        finished,
                        recovered_value,
                    }
                },
                Instruction::Mul(register, arg) => {
                    let val = self.get_value(*arg);
                    self.registers.entry(*register)
                        .and_modify(|e| *e *= val)
                        .or_insert(0);
                    InstructionResult {
                        finished,
                        recovered_value,
                    }
                },
                Instruction::Mod(register, arg) => {
                    let val = self.get_value(*arg);
                    self.registers.entry(*register)
                        .and_modify(|e| *e %= val)
                        .or_insert(0);
                    InstructionResult {
                        finished,
                        recovered_value,
                    }
                },
                Instruction::Rcv(arg) => {
                    let val = self.get_value(*arg);
                    if val != 0 { recovered_value = Some(self.last_sound); }
                    InstructionResult {
                        finished,
                        recovered_value,
                    }
                },
                Instruction::Jgz(arg_1, arg_2) => {
                    let val_1 = self.get_value(*arg_1);
                    let val_2 = self.get_value(*arg_2);
                    if val_1 > 0 { 
                        self.position += val_2 - 1;
                    }
                    InstructionResult {
                        finished,
                        recovered_value,
                    }
                },
            };
            self.position += 1;
            result
        }

        fn get_value(&self, arg: ArgType) -> i64 {
            match arg {
                ArgType::Char(register) => {
                    *self.registers.get(&register).unwrap()
                },
                ArgType::I64(value) => {
                    value
                }
            }
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            loop {
                let result = self.handle_next_instruction();
                if let Some(val) = result.recovered_value {
                    return Answer::I64(val);
                }
                if result.finished { 
                    panic!("Finished without finding a recovered value.") 
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

        #[test_case(1, Answer::I64(4); "example_1")]
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
    use std::{collections::HashMap, sync::{mpsc::{self, Sender, Receiver}, Arc, Mutex}, thread};

    use regex::Regex;

    use crate::utils::{solution::{Solution, Answer}, io_utils};
    use super::utils::ArgType;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Instruction {
        Snd(ArgType),
        Set(char, ArgType),
        Add(char, ArgType),
        Mul(char, ArgType),
        Mod(char, ArgType),
        Rcv(char),
        Jgz(ArgType, ArgType),
    }

    #[derive(Debug)]
    struct CPU {
        instructions: Vec<Instruction>,
        position: i64,
        registers: HashMap<char, i64>,
        program: i64,
        tx: Sender<i64>,
        rx: Receiver<i64>,
        unreceived: Arc<Mutex<HashMap<i64, i32>>>,
        sends: u32,
    }

    impl CPU {
        pub fn new(
            instructions: Vec<Instruction>, 
            program: i64, 
            tx: Sender<i64>, 
            rx: Receiver<i64>, 
            unreceived: Arc<Mutex<HashMap<i64, i32>>>,
        ) -> Self {
            Self {
                instructions,
                position: 0,
                registers: HashMap::new(),
                program,
                tx,
                rx,
                unreceived,
                sends: 0,
            }
        }

        /// Returns whether the program is finished running
        fn handle_next_instruction(&mut self) -> bool {
            if self.position < 0 || self.position as usize >= self.instructions.len() {
                return true
            }
            let instruction = &self.instructions[self.position as usize];
            match instruction {
                Instruction::Snd(arg) => {
                    {
                        let unreceived = &mut *self.unreceived.lock().unwrap();
                        match self.tx.send(self.get_value(*arg)) {
                            Err(_) => (),
                            Ok(()) => (),
                        }
                        unreceived.entry(1 - self.program).and_modify(|e| *e += 1);
                    }
                    self.sends += 1;
                },
                Instruction::Set(register, arg) => {
                    let val = self.get_value(*arg);
                    self.registers.insert(*register, val);
                },
                Instruction::Add(register, arg) => {
                    let val = self.get_value(*arg);
                    self.registers.entry(*register)
                        .and_modify(|e| *e += val)
                        .or_insert(val);
                },
                Instruction::Mul(register, arg) => {
                    let val = self.get_value(*arg);
                    self.registers.entry(*register)
                        .and_modify(|e| *e *= val)
                        .or_insert(0);
                },
                Instruction::Mod(register, arg) => {
                    let val = self.get_value(*arg);
                    self.registers.entry(*register)
                        .and_modify(|e| *e %= val)
                        .or_insert(0);
                },
                Instruction::Rcv(arg) => {
                    {
                        let unreceived = &*self.unreceived.lock().unwrap();
                        if *unreceived.get(&(1 - self.program)).unwrap() == -1 && *unreceived.get(&self.program).unwrap() == 0 { 
                            // Deadlock. Other thread is already receiving and has not sent any unreceived
                            // messages to this thread.
                            return true; 
                        }
                    }
                    {
                        let unreceived = &mut *self.unreceived.lock().unwrap();
                        unreceived.entry(self.program).and_modify(|e| *e -= 1);
                    }
                    match self.rx.recv() {
                        Ok(val) => {
                            self.registers.insert(*arg, val); 
                        },
                        Err(_) => {
                            // CPU is trying to receive, but sending CPU is out of scope.
                            return true 
                        },
                    }
                },
                Instruction::Jgz(arg_1, arg_2) => {
                    let val_1 = self.get_value(*arg_1);
                    let val_2 = self.get_value(*arg_2);
                    if val_1 > 0 { 
                        self.position += val_2 - 1;
                    }
                },
            }
            self.position += 1;
            false
        }

        fn get_value(&self, arg: ArgType) -> i64 {
            match arg {
                ArgType::Char(register) => {
                    *self.registers.get(&register).unwrap()
                },
                ArgType::I64(value) => {
                    value
                }
            }
        }

        fn run(&mut self) {
            self.registers.insert('p', self.program);
            while !self.handle_next_instruction() {}
        }

        fn sends(&self) -> u32 {
            self.sends
        }
    }

    #[derive(Debug, Default)]
    pub struct Soln {}

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) -> Vec<Instruction> {
            let re = Regex::new(r"(?<operation>[a-z]{3}) (?<args>[ a-z\-\d]+)").unwrap();
            io_utils::file_to_lines(filename)
                .map(|line| {
                    let captures = re.captures(&line)
                        .expect("Line should match regex.");
                    let operation = captures.name("operation").unwrap().as_str();
                    let args = captures.name("args").unwrap().as_str();
                    match operation {
                        "snd" => {
                            if let Ok(val) = args.parse() {
                                Instruction::Snd(ArgType::I64(val))
                            } else {
                                Instruction::Snd(ArgType::Char(args.chars().next().unwrap()))
                            }                
                        },
                        "set" => {
                            let mut args = args.split(" ");
                            let arg_1 = args.next().unwrap().chars().next().unwrap();
                            let arg_2 = args.next().unwrap();
                            if let Ok(val) = arg_2.parse() {
                                Instruction::Set(
                                    arg_1,
                                    ArgType::I64(val),
                                )
                            } else {
                                Instruction::Set(
                                    arg_1,
                                    ArgType::Char(arg_2.chars().next().unwrap()),
                                )
                            }                
                        },
                        "add" => {
                            let mut args = args.split(" ");
                            let arg_1 = args.next().unwrap().chars().next().unwrap();
                            let arg_2 = args.next().unwrap();
                            if let Ok(val) = arg_2.parse() {
                                Instruction::Add(
                                    arg_1,
                                    ArgType::I64(val),
                                )
                            } else {
                                Instruction::Add(
                                    arg_1,
                                    ArgType::Char(arg_2.chars().next().unwrap()),
                                )
                            }                
                        },
                        "mul" => {
                            let mut args = args.split(" ");
                            let arg_1 = args.next().unwrap().chars().next().unwrap();
                            let arg_2 = args.next().unwrap();
                            if let Ok(val) = arg_2.parse() {
                                Instruction::Mul(
                                    arg_1,
                                    ArgType::I64(val),
                                )
                            } else {
                                Instruction::Mul(
                                    arg_1,
                                    ArgType::Char(arg_2.chars().next().unwrap()),
                                )
                            }                
                        },
                        "mod" => {
                            let mut args = args.split(" ");
                            let arg_1 = args.next().unwrap().chars().next().unwrap();
                            let arg_2 = args.next().unwrap();
                            if let Ok(val) = arg_2.parse() {
                                Instruction::Mod(
                                    arg_1,
                                    ArgType::I64(val),
                                )
                            } else {
                                Instruction::Mod(
                                    arg_1,
                                    ArgType::Char(arg_2.chars().next().unwrap()),
                                )
                            }                
                        },
                        "rcv" => {
                            Instruction::Rcv(
                                args.chars().next().unwrap(),
                            )
                        },
                        "jgz" => {
                            let mut args = args.split(" ");
                            let arg_1 = args.next().unwrap();
                            let val_1 = if let Ok(val) = arg_1.parse() {
                                ArgType::I64(val)
                            } else {
                                ArgType::Char(arg_1.chars().next().unwrap())
                            };
                            let arg_2 = args.next().unwrap();
                            let val_2 = if let Ok(val) = arg_2.parse() {
                                ArgType::I64(val)
                            } else {
                                ArgType::Char(arg_2.chars().next().unwrap())
                            };
                            Instruction::Jgz(
                                val_1,
                                val_2,
                            )
                        },
                        _ => panic!("Unrecognized operation: {operation}"),
                    }
                })
                .collect()
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            let instructions = self.parse_input_file(filename);
            let (tx_0, rx_1) = mpsc::channel();
            let (tx_1, rx_0) = mpsc::channel();
            let unreceived_0 = Arc::new(Mutex::new(HashMap::from([(0i64, 0i32), (1, 0)])));
            let unreceived_1 = Arc::clone(&unreceived_0);
            let mut cpu_0 = CPU::new(instructions.clone(), 0, tx_0, rx_0, unreceived_0);
            let mut cpu_1 = CPU::new(instructions, 1, tx_1, rx_1, unreceived_1);

            let thread_0 = thread::spawn(move || {
                cpu_0.run();
            });

            let thread_1 = thread::spawn(move || {
                cpu_1.run();
                cpu_1.sends()
            });

            thread_0.join().unwrap();
            let cpu_1_sends = thread_1.join().unwrap();
            
            Answer::U32(cpu_1_sends)
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(2, Answer::U32(3); "example_2")]
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