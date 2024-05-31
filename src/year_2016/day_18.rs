#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 18 };

pub mod part_one {
    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Tile {
        Safe,
        Trap,
    }

    impl Tile {
        fn from_char(input: char) -> Self {
            match input {
                '^' => Self::Trap,
                '.' => Self::Safe,
                _ => panic!("Unrecognized tile character."),
            }
        }
    }

    #[derive(Debug)]
    struct Room {
        tiles: Vec<Vec<Tile>>,
        rows: usize,
    }

    impl Default for Room {
        fn default() -> Self {
            Self::with_rows(40)
        }
    }

    impl Room {
        fn with_rows(rows: usize) -> Self {
            Self {
                tiles: Vec::new(),
                rows,
            }
        }

        fn parse_input_file(&mut self, filename: &str) {
            self.tiles.push(io_utils::file_to_string(filename).chars().map(|ch| Tile::from_char(ch)).collect());
        }

        fn add_row(&mut self) {
            let mut new_row = Vec::new();
            // Leftmost tile considers first two, assumes left wall is safe.
            new_row.push(
                match self.tiles[self.tiles.len() - 1][..2] {
                    [_, Tile::Trap] => Tile::Trap,
                    _ => Tile::Safe,
                }
            );
            // Middle section all considers windows of 3.
            new_row.append(&mut 
                self.tiles[self.tiles.len() - 1].windows(3).map(|window| {
                    match *window {
                        [Tile::Trap, _, Tile::Safe] | [Tile::Safe, _, Tile::Trap] => Tile::Trap,
                        _ => Tile::Safe,
                    }
                }).collect()                
            );
            // Rightmost tile considers last two, assumes right wall is safe.
            new_row.push(
                match self.tiles[self.tiles.len() - 1][self.tiles[self.tiles.len() - 1].len() - 2..] {
                    [Tile::Trap, _] => Tile::Trap,
                    _ => Tile::Safe,
                }
            );
            self.tiles.push(new_row);
        }

        fn complete_rows(&mut self) {
            while self.tiles.len() < self.rows {
                self.add_row();
            }
        }

        fn safe_tiles(&self) -> usize {
            self.tiles.iter().map(|row| {
                row.iter().filter(|tile| **tile == Tile::Safe).count()
            }).sum()
        }
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        room: Room,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.room.parse_input_file(filename);
            self.room.complete_rows();
            Answer::Usize(self.room.safe_tiles())
        }
    }

    impl Soln {
        #[cfg(test)]
        fn with_rows(rows: usize) -> Self {
            Self {
                room: Room::with_rows(rows),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, 3, Answer::Usize(6); "example_1")]
        #[test_case(2, 10, Answer::Usize(38); "example_2")]
        fn examples_are_correct(example_key: u8, rows: usize, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::with_rows(rows),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}