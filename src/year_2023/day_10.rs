#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 10 };

pub mod part_one {
    use std::collections::HashMap;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    struct Point {
        row: i32,
        col: i32,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Direction {
        N,
        S,
        E,
        W,
    }

    impl Direction {
        fn opposite(&self) -> Self {
            match *self {
                Self::N => Self::S,
                Self::S => Self::N,
                Self::E => Self::W,
                Self::W => Self::E,
            }           
        }
    }

    impl Point {
        fn step(&self, dir: &Direction) -> Self {
            match *dir {
                Direction::N => Self { row: self.row - 1, col: self.col },                
                Direction::S => Self { row: self.row + 1, col: self.col },                
                Direction::E => Self { row: self.row, col: self.col + 1 },                
                Direction::W => Self { row: self.row, col: self.col - 1 },                
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Pipe {
        NS,
        EW,
        NE,
        NW,
        SW,
        SE,
    }

    impl Pipe {
        fn from_char(ch: char) -> Self {
            match ch {
                '|' => Self::NS,
                '-' => Self::EW,
                'L' => Self::NE,
                'J' => Self::NW,
                '7' => Self::SW,
                'F' => Self::SE,
                _ => panic!("Unrecognized pipe character."),
            }
        }

        fn connects(&self, dir: &Direction) -> bool {
            match dir {
                Direction::N => *self == Self::NS || *self == Self::NE || *self == Self::NW,
                Direction::S => *self == Self::NS || *self == Self::SE || *self == Self::SW,
                Direction::E => *self == Self::EW || *self == Self::SE || *self == Self::NE,
                Direction::W => *self == Self::EW || *self == Self::SW || *self == Self::NW,
            }
        }

        fn exit_dir(&self, entry_dir: &Direction) -> Direction {
            match *self {
                Self::NS | Self::EW => entry_dir.clone(),
                Self::NW => {
                    match *entry_dir {
                        Direction::S => Direction::W,
                        Direction::E => Direction::N,
                        _ => panic!("Unrecognized entry direction."),
                    }
                },
                Self::NE => {
                    match *entry_dir {
                        Direction::S => Direction::E,
                        Direction::W => Direction::N,
                        _ => panic!("Unrecognized entry direction."),
                    }
                },
                Self::SW => {
                    match *entry_dir {
                        Direction::N => Direction::W,
                        Direction::E => Direction::S,
                        _ => panic!("Unrecognized entry direction."),
                    }
                },
                Self::SE => {
                    match *entry_dir {
                        Direction::N => Direction::E,
                        Direction::W => Direction::S,
                        _ => panic!("Unrecognized entry direction."),
                    }
                },
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Path {
        steps: u32,
        point: Point,
        dir: Direction,
    }

    impl Path {
        fn same_point_as(&self, other: &Self) -> bool {
            self.point == other.point
        }
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        start: Option<Point>,
        pipes: HashMap<Point, Pipe>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.steps_to_furthest())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let mut row: i32 = 0;
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    for (col, ch) in line.chars().enumerate() {
                        let col = col as i32;
                        match ch {
                            'S' => self.start = Some(Point { row, col }),
                            '|' | '-' | 'L' | 'J' | '7' | 'F' => {
                                self.pipes.insert(Point { row, col }, Pipe::from_char(ch));
                            },
                            '.' => continue,
                            _ => panic!("Unrecognized character.")
                        }
                    }
                    row += 1;
                });
        }

        fn starting_paths(&self) -> Vec<Path> {
            let mut paths = vec![];
            self.add_path(&mut paths, &Direction::N);
            self.add_path(&mut paths, &Direction::S);
            self.add_path(&mut paths, &Direction::E);
            self.add_path(&mut paths, &Direction::W);
            assert!(paths.len() == 2);
            paths
        }

        fn add_path(&self, paths: &mut Vec<Path>, dir: &Direction) {
            let start = self.start.unwrap();
            let (row_offset, col_offset) = match *dir {
                Direction::N => (-1,  0),
                Direction::S => ( 1,  0),
                Direction::W => ( 0, -1),
                Direction::E => ( 0,  1),
            };
            let pt = Point { row: start.row + row_offset, col: start.col + col_offset };
            if let Some(pipe) = self.pipes.get(&pt) {
                if pipe.connects(&dir.opposite()) {
                    paths.push(
                        Path {
                            steps: 1,
                            point: start.step(dir),
                            dir: *dir,
                        }
                    );
                }
            }
        }

        fn steps_to_furthest(&self) -> u32 {
            let mut paths = self.starting_paths();
            while !paths[0].same_point_as(&paths[1]) {
                paths[0] = self.step(&paths[0]);
                paths[1] = self.step(&paths[1]);
            }
            assert_eq!(paths[0].steps, paths[1].steps);
            paths[0].steps
        }

        fn step(&self, path: &Path) -> Path {
            let next_dir = self.pipes.get(&path.point)
                .unwrap()
                .exit_dir(&path.dir);
            Path {
                steps: path.steps + 1,
                point: path.point.step(&next_dir),
                dir: next_dir,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(4); "example_1")]
        #[test_case(2, Answer::U32(8); "example_2")]
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
    use std::{cmp, collections::{HashMap, HashSet, VecDeque}};

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    struct Point {
        row: usize,
        col: usize,
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    enum Direction {
        N,
        S,
        E,
        W,
    }

    impl Direction {
        fn opposite(&self) -> Self {
            match *self {
                Self::N => Self::S,
                Self::S => Self::N,
                Self::E => Self::W,
                Self::W => Self::E,
            }           
        }
    }

    impl Point {
        fn step(&self, dir: &Direction) -> Self {
            match *dir {
                Direction::N => Self { row: self.row - 1, col: self.col },                
                Direction::S => Self { row: self.row + 1, col: self.col },                
                Direction::E => Self { row: self.row, col: self.col + 1 },                
                Direction::W => Self { row: self.row, col: self.col - 1 },                
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Pipe {
        NS,
        EW,
        NE,
        NW,
        SW,
        SE,
    }

    impl Pipe {
        fn from_char(ch: char) -> Self {
            match ch {
                '|' => Self::NS,
                '-' => Self::EW,
                'L' => Self::NE,
                'J' => Self::NW,
                '7' => Self::SW,
                'F' => Self::SE,
                _ => panic!("Unrecognized pipe character."),
            }
        }

        fn connects(&self, dir: &Direction) -> bool {
            match dir {
                Direction::N => *self == Self::NS || *self == Self::NE || *self == Self::NW,
                Direction::S => *self == Self::NS || *self == Self::SE || *self == Self::SW,
                Direction::E => *self == Self::EW || *self == Self::SE || *self == Self::NE,
                Direction::W => *self == Self::EW || *self == Self::SW || *self == Self::NW,
            }
        }

        fn exit_dir(&self, entry_dir: &Direction) -> Direction {
            match *self {
                Self::NS | Self::EW => entry_dir.clone(),
                Self::NW => {
                    match *entry_dir {
                        Direction::S => Direction::W,
                        Direction::E => Direction::N,
                        _ => panic!("Unrecognized entry direction."),
                    }
                },
                Self::NE => {
                    match *entry_dir {
                        Direction::S => Direction::E,
                        Direction::W => Direction::N,
                        _ => panic!("Unrecognized entry direction."),
                    }
                },
                Self::SW => {
                    match *entry_dir {
                        Direction::N => Direction::W,
                        Direction::E => Direction::S,
                        _ => panic!("Unrecognized entry direction."),
                    }
                },
                Self::SE => {
                    match *entry_dir {
                        Direction::N => Direction::E,
                        Direction::W => Direction::S,
                        _ => panic!("Unrecognized entry direction."),
                    }
                },
            }
        }

        fn from(directions: &HashSet<Direction>) -> Self {
            match (
                directions.contains(&Direction::N),
                directions.contains(&Direction::E), 
                directions.contains(&Direction::S), 
                directions.contains(&Direction::W),
            ) {
                (true, true, false, false) => Self::NE,
                (true, false, true, false) => Self::NS,
                (true, false, false, true) => Self::NW,
                (false, true, true, false) => Self::SE,
                (false, true, false, true) => Self::EW,
                (false, false, true, true) => Self::SW,
                (_, _, _, _) => panic!("Undeterminable start pipe."),
            } 
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Path {
        point: Point,
        dir: Direction,
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    struct Range {
        min: usize,
        max: usize,
    }

    impl Range {
        fn new(val: usize) -> Self {
            Self {
                min: val,
                max: val,
            }
        }

        fn update(&mut self, val: usize) {
            self.min = cmp::min(self.min, val);
            self.max = cmp::max(self.max, val);
        }

        fn contains(&self, other: &Self) -> bool {
            self.min <= other.min && self.max >= other.max
        }

        fn expand(&mut self, other: &Range) {
            self.min = cmp::min(self.min, other.min);
            self.max = cmp::max(self.max, other.max);
        }
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    struct BoundingBox {
        rows: Range,
        cols: Range,
    }

    impl BoundingBox {
        fn new(point: Point) -> Self {
            Self {
                rows: Range::new(point.row),
                cols: Range::new(point.col),
            }
        }

        fn update(&mut self, point: Point) {
            self.rows.update(point.row);
            self.cols.update(point.col);
        }

        fn contains(&self, other: &Self) -> bool {
            self.rows.contains(&other.rows) && self.cols.contains(&other.cols)
        }

        fn expand(&mut self, other: &Self) {
            self.rows.expand(&other.rows);
            self.cols.expand(&other.cols);
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum EnclosureStatus {
        Enclosed,
        FreeRange,
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    enum Location {
        Tile(Point),
        Intersection(Point), // Intersection at the top left of the `Point`
    }

    #[derive(Debug, Default)]
    struct ConnectedTiles {
        enclosure_status: Option<EnclosureStatus>,       
        tiles: HashSet<Point>, // TODO: make Location?
        bounding_box: BoundingBox,
        frontier: VecDeque<Location>,
        explored: HashSet<Location>,        
    }

    impl ConnectedTiles {
        fn num_tiles(&self) -> usize {
            self.tiles.len()
        }
    }

    impl ConnectedTiles {
        fn new(location: Location) -> Self {
            let bounding_box = match location {
                Location::Tile(point) => BoundingBox::new(point),
                Location::Intersection(point) => {
                    let mut bb = BoundingBox::new(point);
                    bb.update(Point { 
                        row: point.row - if point.row != 0 { 1 } else { 0 }, 
                        col: point.col - if point.col != 0 { 1 } else { 0 },
                    });                    
                    bb.update(Point { 
                        row: point.row - if point.row != 0 { 1 } else { 0 }, 
                        col: point.col,
                    });                    
                    bb.update(Point { 
                        row: point.row,
                        col: point.col - if point.col != 0 { 1 } else { 0 },
                    });                    
                    bb
                },
            };
            Self {
                enclosure_status: None,
                tiles: match location {
                    Location::Tile(pt) => HashSet::from([pt]),
                    Location::Intersection(_) => HashSet::new(),
                },
                bounding_box,
                frontier: VecDeque::from([location]),
                explored: HashSet::new(),               
            }
        }

        fn absorb(&mut self, other: &mut Self) {
            self.enclosure_status = match (self.enclosure_status, other.enclosure_status) {
                (Some(EnclosureStatus::FreeRange), _) | (_, Some(EnclosureStatus::FreeRange)) => Some(EnclosureStatus::FreeRange),
                (None, o) => o,
                (s, None) => s,
                (Some(EnclosureStatus::Enclosed), Some(EnclosureStatus::Enclosed)) => Some(EnclosureStatus::Enclosed),
            };
            self.tiles.extend(other.tiles.iter());
            self.explored.extend(other.explored.iter());
            self.frontier.append(&mut other.frontier); // TODO: remove from frontier any in explored or tiles
            self.frontier.retain(|location| {
                if self.explored.contains(location) { return false; }
                if let Location::Tile(point) = location {
                    if self.tiles.contains(point) { return false; } 
                }
                true
            });
            self.bounding_box.expand(&other.bounding_box);
        }

        fn set_enclosure_status(&mut self, enclosure_status: EnclosureStatus) {
            self.enclosure_status = Some(enclosure_status);
        }
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    struct PipeLoop {
        pipes: HashSet<Point>,
        bounding_box: BoundingBox,
    }

    impl PipeLoop {
        fn new(point: Point) -> Self {
            Self {
                pipes: HashSet::from([point]),
                bounding_box: BoundingBox::new(point),
            }
        }

        fn add_point(&mut self, point: Point) {
            self.pipes.insert(point);
            self.bounding_box.update(point);
        }
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        start: Option<Point>,
        pipes: HashMap<Point, Pipe>,
        pipe_loop: PipeLoop,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            self.calculate_pipe_loop();
            Answer::Usize(self.enclosed_tiles())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let mut rows = 0;
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    for (col, ch) in line.chars().enumerate() {
                        match ch {
                            'S' => self.start = Some(Point { row: rows, col }),
                            '|' | '-' | 'L' | 'J' | '7' | 'F' => {
                                self.pipes.insert(Point { row: rows, col }, Pipe::from_char(ch));
                            },
                            '.' => continue,
                            _ => panic!("Unrecognized character.")
                        }
                    }
                    rows += 1;
                });
            self.calculate_start_pipe();
        }

        fn starting_path(&self) -> Path {
            for dir in [Direction::N, Direction::S, Direction::N, Direction::E] {
                if let Some(path) = self.valid_starting_path(&dir) { return path; }
            }
            panic!("Should have two valid starting paths.");
        }

        fn valid_starting_path(&self, dir: &Direction) -> Option<Path> {
            let start = self.start.unwrap();
            if *dir == Direction::N && start.row == 0 || *dir == Direction::W && start.col == 0 {
                return None;
            }
            let pt = match *dir {
                Direction::N => Point { row: start.row - 1, col: start.col },
                Direction::S => Point { row: start.row + 1, col: start.col },
                Direction::W => Point { row: start.row, col: start.col - 1 },
                Direction::E => Point { row: start.row, col: start.col + 1 },
            };
            if let Some(pipe) = self.pipes.get(&pt) {
                if pipe.connects(&dir.opposite()) {
                    return Some(
                        Path {
                            point: start.step(dir),
                            dir: *dir,
                        }
                    )
                }
            }
            None
        }

        fn calculate_start_pipe(&mut self) {
            let mut directions = HashSet::from([
                Direction::N,
                Direction::E,
                Direction::S,
                Direction::W,
            ]);
            directions.retain(|dir| self.valid_starting_path(dir) != None);
            self.pipes.insert(self.start.unwrap(), Pipe::from(&directions));
        }

        fn step(&self, path: &Path) -> Path {
            let next_dir = self.pipes.get(&path.point)
                .unwrap()
                .exit_dir(&path.dir);
            Path {
                point: path.point.step(&next_dir),
                dir: next_dir,
            }
        }

        fn calculate_pipe_loop(&mut self) {
            self.pipe_loop = PipeLoop::new(self.start.unwrap());
            let mut path = self.starting_path();
            while path.point != self.start.unwrap() {
                self.pipe_loop.add_point(path.point);
                path = self.step(&path);
            }
        }

        fn enclosed_tiles(&self) -> usize {
            let start = self.start.unwrap();
            let mut all_connected_tiles = VecDeque::from([
                ConnectedTiles::new(Location::Intersection(start)),
                ConnectedTiles::new(Location::Intersection(Point { row: start.row + 1, col: start.col })),
                ConnectedTiles::new(Location::Intersection(Point { row: start.row + 1, col: start.col + 1 })),
                ConnectedTiles::new(Location::Intersection(Point { row: start.row, col: start.col + 1 })),
            ]);
            loop {
                let mut connected_tiles = all_connected_tiles.pop_front().unwrap();
                self.update_enclosure_status(&mut connected_tiles);
                if connected_tiles.enclosure_status == Some(EnclosureStatus::Enclosed) {
                    return connected_tiles.num_tiles();
                }
                self.explore_frontier(&mut connected_tiles);
                let mut absorbed = false;
                for other_cts in all_connected_tiles.iter_mut() {
                    if !connected_tiles.explored.is_disjoint(&other_cts.explored) {
                        other_cts.absorb(&mut connected_tiles);
                        absorbed = true;
                        break;
                    }
                }
                if !absorbed {
                    all_connected_tiles.push_back(connected_tiles);
                }
                /* TODOs
                Expand frontier
                Update accounting
                Check if can be merged into any others, otherwise put back into queue.
                Merge into others can happen if there's overlap in tiles, or in explored. Could also add frontier?
                 */
            }
            // TODO: have to figue out how to merge them together. Look back at the other similar problem.
        }

        fn update_enclosure_status(&self, connected_tiles: &mut ConnectedTiles) {
            if connected_tiles.enclosure_status != None { return; }
            if !self.pipe_loop.bounding_box.contains(&connected_tiles.bounding_box) {
                connected_tiles.set_enclosure_status(EnclosureStatus::FreeRange);
                connected_tiles.frontier.drain(..);
            } else if connected_tiles.frontier.is_empty() {
                connected_tiles.set_enclosure_status(EnclosureStatus::Enclosed);
            }
        }

        /* If you're at an intersection you can go on to any of the 4 tiles (with standard tile checks)
        or to any of 4 points.
        If you're on a tile, you can go to any of your 4 intersections or to any of 4 tiles.
        
        When do we do the checks? */

        fn explore_frontier(&self, connected_tiles: &mut ConnectedTiles) {
            let mut new_frontier = VecDeque::new();
            while !connected_tiles.frontier.is_empty() {
                let location = connected_tiles.frontier.pop_front().unwrap();
                if connected_tiles.explored.contains(&location) { continue; }
                connected_tiles.explored.insert(location);
                match location {
                    Location::Tile(point) => {
                        connected_tiles.tiles.insert(point);
                        connected_tiles.bounding_box.update(point);
                        if point.row != 0 {
                            let new_pt = Point { row: point.row - 1, col: point.col };
                            if !self.pipe_loop.pipes.contains(&new_pt) && !connected_tiles.explored.contains(&Location::Tile(new_pt)) {
                                new_frontier.push_back(Location::Tile(new_pt));
                            }
                        }
                        if point.col != 0 {
                            let new_pt = Point { row: point.row, col: point.col - 1 };
                            if !self.pipe_loop.pipes.contains(&new_pt) && !connected_tiles.explored.contains(&Location::Tile(new_pt)) {
                                new_frontier.push_back(Location::Tile(new_pt));
                            }
                        }
                        for (row_offset, col_offset) in [(1, 0), (0, 1)] {
                            let new_pt = Point { row: point.row + row_offset, col: point.col + col_offset };
                            if !self.pipe_loop.pipes.contains(&new_pt) && !connected_tiles.explored.contains(&Location::Tile(new_pt)) {
                                new_frontier.push_back(Location::Tile(new_pt));
                            }
                        }
                        for (row_offset, col_offset) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                            let new_pt = Point { row: point.row + row_offset, col: point.col + col_offset };
                            if !connected_tiles.explored.contains(&Location::Intersection(new_pt)) {
                                new_frontier.push_back(Location::Intersection(new_pt));
                            }
                        }
                    },
                    Location::Intersection(point) => {
                        if point.row != 0 {
                            let new_pt = Point { row: point.row - 1, col: point.col };
                            if !self.pipe_loop.pipes.contains(&new_pt) && !connected_tiles.explored.contains(&Location::Tile(new_pt)) {
                                new_frontier.push_back(Location::Tile(new_pt));
                            }
                        }
                        if point.col != 0 {
                            let new_pt = Point { row: point.row, col: point.col - 1 };
                            if !self.pipe_loop.pipes.contains(&new_pt) && !connected_tiles.explored.contains(&Location::Tile(new_pt)) {
                                new_frontier.push_back(Location::Tile(new_pt));
                            }
                        }
                        if point.row != 0 && point.col != 0 {
                            let new_pt = Point { row: point.row - 1, col: point.col - 1 };
                            if !self.pipe_loop.pipes.contains(&new_pt) && !connected_tiles.explored.contains(&Location::Tile(new_pt)) {
                                new_frontier.push_back(Location::Tile(new_pt));
                            }
                        }
                        if !self.pipe_loop.pipes.contains(&point) && !connected_tiles.explored.contains(&Location::Tile(point)) {
                            new_frontier.push_back(Location::Tile(point));
                        }
                        // Intersections
                        // check North
                        if point.row != 0 {
                            let new_pt = Point { row: point.row - 1, col: point.col };
                            if !connected_tiles.explored.contains(&Location::Intersection(new_pt)) && (
                                point.col == 0
                                || !self.pipe_loop.pipes.contains(&new_pt)
                                || !self.pipes.get(&new_pt).unwrap().connects(&Direction::W)
                                || !self.pipe_loop.pipes.contains(&Point { row: point.row - 1, col: point.col - 1 })
                                || !self.pipes.get(&Point { row: point.row - 1, col: point.col - 1 }).unwrap().connects(&Direction::E)
                            ) {
                                new_frontier.push_back(Location::Intersection(new_pt));
                            }
                        }
                        // check West
                        if point.col != 0 {
                            let new_pt = Point { row: point.row, col: point.col - 1 };
                            if !connected_tiles.explored.contains(&Location::Intersection(new_pt)) && (
                                point.row == 0
                                || !self.pipe_loop.pipes.contains(&new_pt)
                                || !self.pipes.get(&new_pt).unwrap().connects(&Direction::N)
                                || !self.pipe_loop.pipes.contains(&Point { row: point.row - 1, col: point.col - 1 })
                                || !self.pipes.get(&Point { row: point.row - 1, col: point.col - 1 }).unwrap().connects(&Direction::S)
                            ) {
                                new_frontier.push_back(Location::Intersection(new_pt));
                            }
                        }
                        // check South
                        let new_pt = Point { row: point.row + 1, col: point.col };
                        if !connected_tiles.explored.contains(&Location::Intersection(new_pt)) && (
                            point.col == 0
                            || !self.pipe_loop.pipes.contains(&point)
                            || !self.pipes.get(&point).unwrap().connects(&Direction::W)
                            || !self.pipe_loop.pipes.contains(&Point { row: point.row, col: point.col - 1 })
                            || !self.pipes.get(&Point { row: point.row, col: point.col - 1 }).unwrap().connects(&Direction::E)
                        ) {
                            new_frontier.push_back(Location::Intersection(new_pt));
                        }
                        // check East
                        let new_pt = Point { row: point.row, col: point.col + 1 };
                        if !connected_tiles.explored.contains(&Location::Intersection(new_pt)) && (
                            point.row == 0
                            || !self.pipe_loop.pipes.contains(&point)
                            || !self.pipes.get(&point).unwrap().connects(&Direction::N)
                            || !self.pipe_loop.pipes.contains(&Point { row: point.row - 1, col: point.col })
                            || !self.pipes.get(&Point { row: point.row - 1, col: point.col }).unwrap().connects(&Direction::S)
                        ) {
                            new_frontier.push_back(Location::Intersection(new_pt));
                        }
                    },
                }
            }
            connected_tiles.frontier = new_frontier;
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(3, Answer::Usize(4); "example_3")]
        #[test_case(4, Answer::Usize(4); "example_4")]
        #[test_case(5, Answer::Usize(8); "example_5")]
        #[test_case(6, Answer::Usize(10); "example_6")]
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