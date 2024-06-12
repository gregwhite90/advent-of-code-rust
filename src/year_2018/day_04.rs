#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 4 };

pub mod part_one {
    use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}};

    use regex::Regex;   
    use chrono::{Duration, NaiveDateTime, NaiveTime, TimeDelta};

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug)]
    struct SleepSession {
        start: NaiveDateTime,
        end: NaiveDateTime, // exclusive
    }

    impl SleepSession {
        fn from_strs(start: &str, end: &str) -> Self {
            let fmt = "%Y-%m-%d %H:%M";
            Self {
                start: NaiveDateTime::parse_from_str(start, fmt).unwrap(),
                end: NaiveDateTime::parse_from_str(end, fmt).unwrap(),
            }
        }

        fn duration(&self) -> TimeDelta {
            self.end - self.start
        }
    }

    #[derive(Debug)]
    struct Guard {
        id: usize,
        sleep_sessions: Vec<SleepSession>,
    }

    impl Guard {
        fn new(id: usize) -> Self {
            Self {
                id,
                sleep_sessions: Vec::new(),
            }
        }

        fn add_sleep_session(&mut self, sleep_session: SleepSession) {
            self.sleep_sessions.push(sleep_session);
        }

        fn total_sleep_minutes(&self) -> i64 {
            self.sleep_sessions.iter()
                .map(|ss| ss.duration().num_minutes())
                .sum()
        }

        fn most_overlapped_minute(&self) -> i64 {
            let basetime = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
            let mut minute_count: HashMap<i64, u32> = HashMap::new();
            self.sleep_sessions.iter().for_each(|ss| {
                let mut time = ss.start.time().clone();
                while time < ss.end.time() {
                    *minute_count.entry((time - basetime).num_minutes()).or_default() += 1;
                    time += Duration::minutes(1);
                }
            });
            *minute_count.iter().max_by_key(|(_minute, count)| **count).unwrap().0
        }
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        guards: HashMap<usize, Guard>,    
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::Usize(self.strategy_one())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let mut lines: BinaryHeap<Reverse<String>> = io_utils::file_to_lines(filename)
                .map(|line| Reverse(line))
                .collect();
            let guard_re = Regex::new(r"\[(?<dt>.*)\] Guard #(?<id>\d+) begins shift").unwrap();
            let action_re = Regex::new(r"\[(?<dt>.*)\] (?<action>(falls asleep)|(wakes up))").unwrap();
            let mut cur_guard_id: Option<usize> = None;
            let mut cur_sleep_sesion_start: Option<String> = None;
            while !lines.is_empty() {
                let line = lines.pop().unwrap().0;
                if let Some(caps) = guard_re.captures(&line) {
                    let id = caps.name("id").unwrap().as_str().parse().unwrap();
                    if !self.guards.contains_key(&id) {
                        self.guards.insert(id, Guard::new(id));    
                    }
                    cur_guard_id = Some(id);
                } else {
                    assert!(cur_guard_id.is_some());
                    let caps = action_re.captures(&line).unwrap();
                    let dt = caps.name("dt").unwrap().as_str();
                    let action = caps.name("action").unwrap().as_str();
                    match action {
                        "falls asleep" => {
                            assert!(cur_sleep_sesion_start.is_none());
                            cur_sleep_sesion_start = Some(dt.to_string());
                        },
                        "wakes up" => {
                            let start = cur_sleep_sesion_start.unwrap();
                            let sleep_session = SleepSession::from_strs(&start, dt);
                            self.guards.entry(cur_guard_id.unwrap()).and_modify(|guard| {
                                guard.add_sleep_session(sleep_session);
                            });
                            cur_sleep_sesion_start = None;
                        },
                        _ => panic!("Unrecognized action."),
                    }
                }
            }
        }   

        fn strategy_one(&self) -> usize {
            let (id, guard) = self.guards.iter()
                .max_by_key(|(_id, guard)| guard.total_sleep_minutes())
                .unwrap();
            let most_overlapped_minute: usize = guard.most_overlapped_minute().try_into().unwrap();
            id * most_overlapped_minute
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(240); "example_1")]
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