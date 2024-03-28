#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 5 };

mod utils {
    use std::{cmp::Reverse, collections::{BTreeSet, BinaryHeap, HashMap, VecDeque}};

    use regex::Regex;

    #[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
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

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum PointInRangeComparison {
        Below,
        Within(u64), // with the offset into the range
        Above,
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct Range {
        pub start: u64,
        pub len: u64,
    }

    impl Range {
        pub fn point_comparison(&self, point: u64) -> PointInRangeComparison {
            if point < self.start {
                PointInRangeComparison::Below
            } else if point >= self.start + self.len {
                PointInRangeComparison::Above
            } else {
                PointInRangeComparison::Within(point - self.start)
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct CategoryRange {
        pub category: Category,
        pub range: Range,
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct MapEntry {
        source: Range,
        dest: Range,
    }

    impl MapEntry {
        pub fn new(dest_range_start: u64, source_range_start: u64, range_len: u64) -> Self {
            Self {
                dest: Range { start: dest_range_start, len: range_len },
                source: Range { start: source_range_start, len: range_len },
            }
        }

        pub fn dest(&self, source: u64) -> Option<u64> {
            if source >= self.source.start && source < self.source.start + self.source.len {
                Some(self.dest.start + source - self.source.start)
            } else {
                None
            }
        }

        pub fn overlap(&self, source: CategoryRange, dest_category: Category) -> Vec<CategoryRange> {
            let start_in_range = self.source.point_comparison(source.range.start);
            let end_in_range = self.source.point_comparison(source.range.start + source.range.len - 1);
            match (start_in_range, end_in_range) {
                (PointInRangeComparison::Above, _) | (_, PointInRangeComparison::Below) => vec![source],
                (PointInRangeComparison::Below, PointInRangeComparison::Within(offset)) => {
                    // upper part of source overlaps this map entry
                    vec![
                        CategoryRange {
                            category: source.category,
                            range: Range {
                                start: source.range.start,
                                len: source.range.len - (offset + 1),
                            }
                        },
                        CategoryRange {
                            category: dest_category,
                            range: Range {
                                start: self.dest.start,
                                len: offset + 1,
                            }
                        },
                    ]
                },
                (PointInRangeComparison::Below, PointInRangeComparison::Above) => {
                    // source fully contains this map entry. Need to split into 3
                    vec![
                        CategoryRange {
                            category: source.category,
                            range: Range {
                                start: source.range.start,
                                len: self.source.start - source.range.start,
                            }
                        },
                        CategoryRange {
                            category: dest_category,
                            range: Range {
                                start: self.dest.start,
                                len: self.dest.len,
                            }
                        },
                        CategoryRange {
                            category: source.category,
                            range: Range {
                                start: self.source.start + self.source.len,
                                len: source.range.start + source.range.len - (self.source.start + self.source.len),
                            }
                        },
                    ]
                },
                (PointInRangeComparison::Within(_start_offset), PointInRangeComparison::Within(_end_offset)) => {
                    // this map entry fully contains source. just need the 1 going into dest
                    vec![
                        CategoryRange {
                            category: dest_category,
                            range: Range {
                                start: self.dest.start + source.range.start - self.source.start,
                                len: source.range.len,
                            }
                        }
                    ]
                },
                (PointInRangeComparison::Within(offset), PointInRangeComparison::Above) => {
                    // lower part of source overlaps this map entry
                    vec![
                        CategoryRange {
                            category: dest_category,
                            range: Range {
                                start: self.dest.start + offset,
                                len: self.dest.len - offset,
                            }
                        },
                        CategoryRange {
                            category: source.category,
                            range: Range {
                                start: source.range.start + self.dest.len - offset,
                                len: source.range.len - (self.dest.len - offset),
                            }
                        },
                    ]
                },
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Map {
        source_category: Category,
        dest_category: Category,
        map_entries: BTreeSet<MapEntry>,
    }

    impl Map {
        pub fn with_categories(source_category: Category, dest_category: Category) -> Self {
            Self {
                source_category,
                dest_category,
                map_entries: BTreeSet::new(),
            }
        }

        pub fn source_category(&self) -> Category {
            self.source_category
        }

        pub fn dest_category(&self) -> Category {
            self.dest_category
        }

        pub fn add_map_entry(&mut self, map_entry: MapEntry) {
            self.map_entries.insert(map_entry);
        }

        pub fn dest(&self, source: u64) -> u64 {
            for map_entry in &self.map_entries {
                if let Some(dst) = map_entry.dest(source) {
                    return dst;
                }
            }
            source
        }

        pub fn all_dest_ranges(&self, range: CategoryRange) -> BinaryHeap<Reverse<CategoryRange>> {
            let mut processed = BinaryHeap::new();
            let mut to_process = VecDeque::from([range]);
            for map_entry in self.map_entries.iter() {
                for _ in 0..to_process.len() {
                    let category_range = to_process.pop_front().unwrap();
                    let overlaps = map_entry.overlap(category_range, self.dest_category);
                    overlaps.into_iter().for_each(|cr| {
                        if cr.category == self.dest_category {
                            processed.push(Reverse(cr));
                        } else {
                            to_process.push_back(cr);
                        }
                    })
                }
            }
            for cr in to_process.into_iter() {
                processed.push(Reverse(CategoryRange {
                    category: self.dest_category,
                    range: cr.range,
                }));    
            }
            processed
        }
    }

    /// Returns a mapping of source category to the map
    pub fn parse_maps(lines: impl Iterator<Item = String>) -> HashMap<Category, Map> {
        let mut maps = HashMap::new();
        let seeds_re = Regex::new(r"seeds: (?<seeds>[ \d]+)").unwrap();
        let map_start_re = Regex::new(r"(?<source>[a-z]+)\-to\-(?<dest>[a-z]+) map:").unwrap();
        let map_entry_re = Regex::new(r"(?<dest_range_start>\d+) (?<source_range_start>\d+) (?<range_len>\d+)").unwrap();
        let mut cur_map: Option<Map> = None;
        lines.for_each(|line| {
            if seeds_re.is_match(&line) {
                panic!("Should not still have the seeds line in the iterator.");
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
                    maps.insert(c_m.source_category(), c_m);
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
        maps.insert(c_m.source_category(), c_m);
        maps
    }


    #[cfg(test)]
    mod tests {
        use test_case::test_case;

        use super::*;

        const SOURCE_CATEGORY: Category = Category::Seed;
        const DEST_CATEGORY: Category = Category::Soil;

        #[test_case(
            Range{start: 11, len: 2}, 
            vec![
                CategoryRange {
                    category: DEST_CATEGORY,
                    range: Range {
                        start: 21,
                        len: 2,
                    },
                },
            ]; "map_entry_contains_source")]
        #[test_case(
            Range{start: 9, len: 7}, 
            vec![
                CategoryRange {
                    category: SOURCE_CATEGORY,
                    range: Range {
                        start: 9,
                        len: 1,
                    },
                },
                CategoryRange {
                    category: DEST_CATEGORY,
                    range: Range {
                        start: 20,
                        len: 5,
                    },
                },
                CategoryRange {
                    category: SOURCE_CATEGORY,
                    range: Range {
                        start: 15,
                        len: 1,
                    },
                },
            ]; "source_contains_map_entry")]
        #[test_case(
            Range{start: 15, len: 7}, 
            vec![
                CategoryRange {
                    category: SOURCE_CATEGORY,
                    range: Range {
                        start: 15,
                        len: 7,
                    },
                },
            ]; "source_above_map_entry")]
        #[test_case(
            Range{start: 7, len: 3}, 
            vec![
                CategoryRange {
                    category: SOURCE_CATEGORY,
                    range: Range {
                        start: 7,
                        len: 3,
                    },
                },
            ]; "source_below_map_entry")]
        #[test_case(
            Range{start: 12, len: 5}, 
            vec![
                CategoryRange {
                    category: DEST_CATEGORY,
                    range: Range {
                        start: 22,
                        len: 3,
                    },
                },
                CategoryRange {
                    category: SOURCE_CATEGORY,
                    range: Range {
                        start: 15,
                        len: 2,
                    },
                },
            ]; "lower_part_of_source_overlaps")]
        #[test_case(
            Range{start: 8, len: 5}, 
            vec![
                CategoryRange {
                    category: SOURCE_CATEGORY,
                    range: Range {
                        start: 8,
                        len: 2,
                    },
                },
                CategoryRange {
                    category: DEST_CATEGORY,
                    range: Range {
                        start: 20,
                        len: 3,
                    },
                },
            ]; "upper_part_of_source_overlaps")]
        fn map_entry_overlap_works(input_range: Range, expected: Vec<CategoryRange>) {
            let map_entry = MapEntry::new(20, 10, 5);
            let category_range = CategoryRange {
                category: SOURCE_CATEGORY,
                range: input_range,
            };
            assert_eq!(
                map_entry.overlap(category_range, DEST_CATEGORY),
                expected,
            )
        }
    }
}

/// This implementation assumes that map entries do not overlap.
pub mod part_one {
    use std::collections::HashMap;

    use regex::Regex;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    use super::utils::{self, Category, Map};

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
            let mut lines = io_utils::file_to_lines(filename);
            let line = lines.next().unwrap();
            self.seeds = seeds_re.captures(&line)
                .unwrap()
                .name("seeds")
                .unwrap()
                .as_str()
                .split(" ")
                .map(|part| part.parse().unwrap())
                .collect();
            self.maps = utils::parse_maps(lines);
        }

        fn seed_to_location(&self, seed: u64) -> u64 {
            let mut next_source_category = Category::Seed;
            let mut next_source = seed;
            while next_source_category != Category::Location {
                let map = self.maps.get(&next_source_category).unwrap();
                next_source = map.dest(next_source);
                next_source_category = map.dest_category();
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
    use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}};
    use itertools::Itertools;
    use regex::Regex;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    use super::utils::{self, Category, CategoryRange, Map, Range};

    #[derive(Debug, Default)]
    pub struct Soln {
        /// Uses `Reverse` because both default implementation of `Ord` for 
        /// both `Category` and `Range` works the opposite way we want it to
        /// because `BinaryHeap` defaults to a max-heap.
        ranges: BinaryHeap<Reverse<CategoryRange>>,
        maps: HashMap<Category, Map>, // maps dest category to its map
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
            let mut lines = io_utils::file_to_lines(filename);
            let line = lines.next().unwrap();
            seeds_re.captures(&line)
                .unwrap()
                .name("seeds")
                .unwrap()
                .as_str()
                .split(" ")
                .map(|part| part.parse::<u64>().unwrap())
                .chunks(2)
                .into_iter()
                .for_each(|mut chunk| {
                    self.ranges.push(Reverse(CategoryRange {
                        category: Category::Seed, 
                        range: Range {
                            start: chunk.next().unwrap(),
                            len: chunk.next().unwrap(),
                        },
                    }));
                });
            self.maps = utils::parse_maps(lines);
        }

        fn minimum_location_number(&mut self) -> u64 {
            loop {
                let Reverse(range) = self.ranges.pop().expect("Should be a range left to analyze");
                if range.category == Category::Location { return range.range.start; }
                // generate all ranges in the dest category and push them onto the binary heap.
                self.ranges.append(&mut self.all_dest_ranges(range));
            }
        }

        fn all_dest_ranges(&self, range: CategoryRange) -> BinaryHeap<Reverse<CategoryRange>> {
            let map = self.maps.get(&range.category).unwrap();
            map.all_dest_ranges(range)
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