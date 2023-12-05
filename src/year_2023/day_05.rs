#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 5 };

/// This implementation assumes that map entries do not overlap.
pub mod part_one {
    use std::collections::HashMap;

    use regex::Regex;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    enum Category {
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
        fn from_str(category: &str) -> Self {
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

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct MapEntry {
        dest_range_start: u64,
        source_range_start: u64,
        range_len: u64,
    }

    impl MapEntry {
        fn dest(&self, source: u64) -> Option<u64> {
            if source >= self.source_range_start && source < self.source_range_start + self.range_len {
                Some(self.dest_range_start + source - self.source_range_start)
            } else {
                None
            }
        }
    }

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
                            MapEntry {
                                dest_range_start,
                                source_range_start,
                                range_len,
                            }
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