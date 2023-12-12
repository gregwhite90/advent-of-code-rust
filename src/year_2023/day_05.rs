#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 5 };

mod utils {
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    pub enum Category {
        Seed,
        Soil,
        Fertilizer,
        Water,
        Light,
        Temperature,
        Humidity,
        Location,
    }

    impl Category {
        pub fn from_str(category: &str) -> Self {
            match category {
                "seed" => Self::Seed,
                "soil" => Self::Soil,
                "fertilizer" => Self::Fertilizer,
                "water" => Self::Water,
                "light" => Self::Light,
                "temperature" => Self::Temperature,
                "humidity" => Self::Humidity,
                "location" => Self::Location,
                _ => panic!("Unrecognized category"),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct MapEntry {
        dest_range_start: u64,
        source_range_start: u64,
        range_len: u64,
    }

    impl MapEntry {
        pub fn new(dest_range_start: u64, source_range_start: u64, range_len: u64) -> Self {
            Self {
                dest_range_start,
                source_range_start,
                range_len,
            }
        }

        pub fn dest(&self, source: u64) -> Option<u64> {
            if source >= self.source_range_start && source < self.source_range_start + self.range_len {
                Some(self.dest_range_start + source - self.source_range_start)
            } else {
                None
            }
        }
    }
}

/// This implementation assumes that map entries do not overlap.
pub mod part_one {
    use std::collections::HashMap;

    use regex::Regex;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    use super::utils::{Category, MapEntry};

    #[derive(Debug, PartialEq, Eq, Clone)]
    struct Map {
        source_category: Category,
        dest_category: Category,
        map_entries: Vec<MapEntry>,
    }

    impl Map {
        fn with_categories(source_category: Category, dest_category: Category) -> Self {
            Self {
                source_category,
                dest_category,
                map_entries: vec![],
            }
        }

        fn source_category(&self) -> Category {
            self.source_category
        }

        fn add_map_entry(&mut self, map_entry: MapEntry) {
            self.map_entries.push(map_entry);
        }

        fn dest(&self, source: u64) -> u64 {
            for map_entry in &self.map_entries {
                if let Some(dst) = map_entry.dest(source) {
                    return dst;
                }
            }
            source
        }
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        seeds: Vec<u64>,
        maps: HashMap<Category, Map>, // maps source category to its map
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U64(self.minimum_location_number())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let seeds_re = Regex::new(r"seeds: (?<seeds>[ \d]+)").unwrap();
            let map_start_re = Regex::new(r"(?<source>[a-z]+)\-to\-(?<dest>[a-z]+) map:").unwrap();
            let map_entry_re = Regex::new(r"(?<dest_range_start>\d+) (?<source_range_start>\d+) (?<range_len>\d+)").unwrap();
            let mut cur_map: Option<Map> = None;
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    if seeds_re.is_match(&line) {
                        self.seeds = seeds_re.captures(&line)
                            .unwrap()
                            .name("seeds")
                            .unwrap()
                            .as_str()
                            .split(" ")
                            .map(|part| part.parse().unwrap())
                            .collect();
                    } else if map_start_re.is_match(&line) {
                        let captures = map_start_re.captures(&line).unwrap();
                        let source_category = Category::from_str(
                            captures.name("source").unwrap().as_str()
                        );
                        let dest_category = Category::from_str(
                            captures.name("dest").unwrap().as_str()
                        );
                        cur_map = Some(Map::with_categories(source_category, dest_category));
                    } else if line.len() == 0 {
                        if cur_map != None {
                            let c_m = cur_map.clone().unwrap();
                            self.maps.insert(c_m.source_category(), c_m);
                            cur_map = None;
                        }
                    } else {
                        let captures = map_entry_re.captures(&line).unwrap();
                        let dest_range_start = captures.name("dest_range_start").unwrap().as_str().parse().unwrap();
                        let source_range_start = captures.name("source_range_start").unwrap().as_str().parse().unwrap();
                        let range_len = captures.name("range_len").unwrap().as_str().parse().unwrap();
                        cur_map.as_mut().unwrap().add_map_entry(
                            MapEntry::new(
                                dest_range_start,
                                source_range_start,
                                range_len,
                            )
                        )
                    }
                });
            let c_m = cur_map.unwrap();
            self.maps.insert(c_m.source_category(), c_m);
        }

        fn seed_to_location(&self, seed: u64) -> u64 {
            let mut next_source_category = Category::Seed;
            let mut next_source = seed;
            while next_source_category != Category::Location {
                let map = self.maps.get(&next_source_category).unwrap();
                next_source = map.dest(next_source);
                next_source_category = map.dest_category;
            }
            next_source
        }

        fn minimum_location_number(&self) -> u64 {
            self.seeds.iter()
                .map(|seed| self.seed_to_location(*seed))
                .min()
                .unwrap()
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U64(35); "example_1")]
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

/// This implementation assumes that map entries do not overlap.
pub mod part_two {
    use std::collections::{HashMap, BTreeSet};
    use itertools::Itertools;
    use regex::Regex;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    use super::utils::{Category, MapEntry};

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    struct Seed {
        start: u64,
        range_len: u64,
    }

    // TODO: pull into utils?
    #[derive(Debug, PartialEq, Eq, Clone)]
    struct Map {
        source_category: Category,
        dest_category: Category,
        map_entries: BTreeSet<MapEntry>,
    }

    impl Map {
        fn with_categories(source_category: Category, dest_category: Category) -> Self {
            Self {
                source_category,
                dest_category,
                map_entries: BTreeSet::new(),
            }
        }

        fn source_category(&self) -> Category {
            self.source_category
        }

        fn dest_category(&self) -> Category {
            self.dest_category
        }

        fn add_map_entry(&mut self, map_entry: MapEntry) {
            self.map_entries.insert(map_entry);
        }

        fn dest(&self, source: u64) -> u64 {
            for map_entry in &self.map_entries {
                if let Some(dst) = map_entry.dest(source) {
                    return dst;
                }
            }
            source
        }
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        seeds: BTreeSet<Seed>,
        maps: HashMap<Category, Map>, // maps dest category to its map
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U64(self.minimum_location_number())
        }
    }

    impl Soln {
        // TODO: share functionality
        fn parse_input_file(&mut self, filename: &str) {
            let seeds_re = Regex::new(r"seeds: (?<seeds>[ \d]+)").unwrap();
            let map_start_re = Regex::new(r"(?<source>[a-z]+)\-to\-(?<dest>[a-z]+) map:").unwrap();
            let map_entry_re = Regex::new(r"(?<dest_range_start>\d+) (?<source_range_start>\d+) (?<range_len>\d+)").unwrap();
            let mut cur_map: Option<Map> = None;
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    if seeds_re.is_match(&line) {
                        self.seeds = seeds_re.captures(&line)
                            .unwrap()
                            .name("seeds")
                            .unwrap()
                            .as_str()
                            .split(" ")
                            .map(|part| part.parse::<u64>().unwrap())
                            .chunks(2)
                            .into_iter()
                            .map(|mut chunk| {
                                Seed { 
                                    start: chunk.next().unwrap(),
                                    range_len: chunk.next().unwrap(),
                                }
                            })
                            .collect();
                    } else if map_start_re.is_match(&line) {
                        let captures = map_start_re.captures(&line).unwrap();
                        let source_category = Category::from_str(
                            captures.name("source").unwrap().as_str()
                        );
                        let dest_category = Category::from_str(
                            captures.name("dest").unwrap().as_str()
                        );
                        cur_map = Some(Map::with_categories(source_category, dest_category));
                    } else if line.len() == 0 {
                        if cur_map != None {
                            let c_m = cur_map.clone().unwrap();
                            self.maps.insert(c_m.dest_category(), c_m);
                            cur_map = None;
                        }
                    } else {
                        let captures = map_entry_re.captures(&line).unwrap();
                        let dest_range_start = captures.name("dest_range_start").unwrap().as_str().parse().unwrap();
                        let source_range_start = captures.name("source_range_start").unwrap().as_str().parse().unwrap();
                        let range_len = captures.name("range_len").unwrap().as_str().parse().unwrap();
                        cur_map.as_mut().unwrap().add_map_entry(
                            MapEntry::new(
                                dest_range_start,
                                source_range_start,
                                range_len,
                            )
                        )
                    }
                });
            let c_m = cur_map.unwrap();
            self.maps.insert(c_m.dest_category(), c_m);
        }

        fn seed_to_location(&self, seed: u64) -> u64 {
            // Check that there's no way to get off the maps?
            // Iterate backward from minimum mapped location number
            // We know the range, and the source starting point.
            // Go back to the previous map, find the maximum dest starting point
            // that is less than or equal to the source starting point. Keep going back.
            // If not that one, go to the next one up, until the range is exhausted.
            // 
            // Might have an offset into the previous map range that we need to incorporate.
            // 
            // Once get all the way back to seeds, 

            let mut next_source_category = Category::Seed;
            let mut next_source = seed;
            while next_source_category != Category::Location {
                let map = self.maps.get(&next_source_category).unwrap();
                next_source = map.dest(next_source);
                next_source_category = map.dest_category;
            }
            next_source
        }

        fn minimum_location_number(&self) -> u64 {
            self.seeds.iter()
                .map(|seed| self.seed_to_location(seed.start)) // TODO: fix
                .min()
                .unwrap()
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U64(46); "example_1")]
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