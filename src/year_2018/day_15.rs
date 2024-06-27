#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 15 };

mod utils {
    use std::{cell::RefCell, cmp::{min, Reverse}, collections::{BinaryHeap, HashMap, HashSet}, fmt::Display};
    use derivative::Derivative;
    use itertools::Itertools;

    use crate::utils::io_utils;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    struct Point {
        row: usize,
        col: usize,
    }

    impl Point {
        fn is_adjacent(&self, other: &Self) -> bool {
            self.row.abs_diff(other.row) + self.col.abs_diff(other.col) == 1
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    enum CombatUnitType {
        Elf,
        Goblin,
    }

    impl CombatUnitType {
        fn from_char(input: char) -> Self {
            match input {
                'E' => Self::Elf,
                'G' => Self::Goblin,
                _ => panic!("Unrecognized unit type"),
            }
        }

        fn to_str(&self) -> &'static str {
            match self {
                Self::Elf => "E",
                Self::Goblin => "G",
            }
        }
    }

    #[derive(Derivative)]
    #[derivative(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct CombatUnit {
        round: usize,
        point: Point,
        unit_type: CombatUnitType,
        attack_power: usize,
        id: usize,

        // RefCell to allow interior mutability pattern, ignored for purposes
        // of PartialOrd so that changing this value does not change the
        // ordering in the units collection (so that we can decrement hit
        // points when a unit is attacked without creating a logic error)
        #[derivative(PartialOrd="ignore", PartialEq="ignore", Ord="ignore")]
        hit_points: RefCell<usize>,
    }

    impl CombatUnit {
        fn new(point: Point, unit_type_input: char, id: usize) -> Self {
            Self {
                round: 0,
                point,
                unit_type: CombatUnitType::from_char(unit_type_input),
                attack_power: 3,
                id,
                hit_points: RefCell::new(200),
            }
        }

        fn is_enemy(&self, other: &Self) -> bool {
            self.unit_type != other.unit_type
        }

        fn is_adjacent(&self, other: &Self) -> bool {
            self.point.is_adjacent(&other.point)
        }

        fn absorb_attack(&self, attack_power: usize) {
            let hit_points = self.hit_points.borrow().to_owned();
            *self.hit_points.borrow_mut() = hit_points - min(hit_points, attack_power);
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
    struct Path {
        steps: usize,
        location: Point,
        first_step: Point,
    }

    struct PathsFinder {
        start: Point,
        obstacles: HashSet<Point>,
        destinations: HashSet<Point>,
    }

    impl PathsFinder {
        fn new(start: Point, obstacles: HashSet<Point>, destinations: HashSet<Point>) -> Self {
            Self {
                start,
                obstacles,
                destinations,
            }
        }

        fn take_step(&mut self) -> Option<Point> {
            let mut paths_to_destinations: BinaryHeap<Reverse<Path>> = BinaryHeap::new();
            let mut pq: BinaryHeap<Reverse<Path>> = adjacent_points(self.start, &self.obstacles)
                .iter()
                .map(|pt| {
                    Reverse(Path {
                        steps: 1,
                        location: *pt,
                        first_step: *pt,
                    })
                })
                .collect();
            let mut visited: HashMap<Point, usize> = HashMap::from([(self.start, 0)]);
            let mut explored_paths: HashSet<Path> = HashSet::new();
            while !pq.is_empty() {
                let path = pq.pop().unwrap().0;
                if !paths_to_destinations.is_empty() && paths_to_destinations.peek().unwrap().0.steps < path.steps {
                    return Some(paths_to_destinations.pop().unwrap().0.first_step);
                }
                if explored_paths.contains(&path) || visited.contains_key(&path.location) && *visited.get(&path.location).unwrap() < path.steps {
                    continue;
                }
                explored_paths.insert(path.clone());
                visited.insert(path.location, path.steps);
                // Generate the next paths
                pq.append(
                    &mut adjacent_points(path.location, &self.obstacles)
                        .iter()
                        .filter(|pt| {
                            !visited.contains_key(&pt)
                        })
                        .map(|pt| {
                            Reverse(Path {
                                steps: path.steps + 1,
                                location: *pt,
                                first_step: path.first_step,
                            })
                        })
                        .collect()
                );
                if self.destinations.contains(&path.location) {
                    paths_to_destinations.push(Reverse(path));
                }
            }
            if !paths_to_destinations.is_empty() {
                Some(paths_to_destinations.pop().unwrap().0.first_step)
            } else {
                None
            }
        }
    }

    fn adjacent_points(point: Point, obstacles: &HashSet<Point>) -> HashSet<Point> {
        let mut adj = HashSet::from([
            Point { row: point.row + 1, col: point.col },
            Point { row: point.row, col: point.col + 1 },
        ]);
        if point.row != 0 {
            adj.insert(Point { row: point.row - 1, col: point.col });
        }
        if point.col != 0 {
            adj.insert(Point { row: point.row, col: point.col - 1 });
        }
        adj.retain(|pt| !obstacles.contains(pt));
        adj
    }

    #[derive(Debug, Default)]
    pub struct CombatSimulator {
        walls: HashSet<Point>,
        units: BinaryHeap<Reverse<CombatUnit>>,
    }

    impl CombatSimulator {
        pub fn parse_input_file(&mut self, filename: &str) {
            let mut row = 0;
            let mut id = 0;
            io_utils::file_to_lines(filename).for_each(|line| {
                line.char_indices().for_each(|(col, ch)| {
                    match ch {
                        '#' => {
                            self.walls.insert(Point { row, col });
                        },
                        '.' => (),
                        'G' | 'E' => {
                            self.units.push(Reverse(CombatUnit::new(
                                Point { row, col },
                                ch,
                                id,
                            )));
                            id += 1;
                        },
                        _ => panic!("Unrecognized character"),
                    }
                });
                row += 1;
            });
        }

        pub fn simulate_combat(&mut self) {
            loop {
                let mut unit = self.units.pop().unwrap().0;
                // Remove from units collection if the unit is dead.
                if *unit.hit_points.borrow() == 0 {
                    continue;
                }
                let targets = self.units.iter().filter(|u| {
                    unit.is_enemy(&u.0) && *u.0.hit_points.borrow() > 0
                });
                // Combat ends if there are no targets
                if targets.clone().count() == 0 {
                    self.units.push(Reverse(unit));
                    break;
                }
                // If in attacking range, attack.
                if attack(&unit, targets.clone()) {
                    unit.round += 1;
                    self.units.push(Reverse(unit));
                    continue;
                }
                // If not in attacking range, move
                let mut obstacles = self.walls.clone();
                self.units.iter()
                    .filter(|u| *u.0.hit_points.borrow() > 0)
                    .map(|u| u.0.point)
                    .for_each(|pt| { obstacles.insert(pt); });
                let mut destinations = HashSet::new();
                targets.clone().for_each(|u| {
                    adjacent_points(u.0.point, &obstacles).iter().for_each(|adj| {
                        destinations.insert(*adj);
                    });
                });
                let mut paths_finder = PathsFinder::new(
                    unit.point,
                    obstacles,
                    destinations,
                );
                if let Some(pt) = paths_finder.take_step() {
                    // Attempt attack
                    unit.point = pt;
                    attack(&unit, self.units.iter().filter(|u| {
                        unit.is_enemy(&u.0) && *u.0.hit_points.borrow() > 0
                    }));
                }
                unit.round += 1;
                self.units.push(Reverse(unit));
            }
        }

        pub fn outcome(&self) -> usize {
            self.units.iter().map(|unit| *unit.0.hit_points.borrow()).sum::<usize>()
            * self.units.iter().filter(|unit| *unit.0.hit_points.borrow() > 0).map(|unit| unit.0.round).min().unwrap()
        }
    }

    impl Display for CombatSimulator {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let min_row = self.walls.iter().map(|pt| pt.row).min().unwrap();
            let max_row = self.walls.iter().map(|pt| pt.row).max().unwrap();
            let min_col = self.walls.iter().map(|pt| pt.col).min().unwrap();
            let max_col = self.walls.iter().map(|pt| pt.col).max().unwrap();
            let units: HashMap<Point, CombatUnitType> = self.units.iter().map(|unit| {
                (unit.0.point, unit.0.unit_type)
            }).collect();
            for row in min_row..=max_row {
                for col in min_col..=max_col {
                    let pt = Point { row, col };
                    if self.walls.contains(&pt) {
                        write!(f, "#")?;
                    } else if units.contains_key(&pt) {
                        write!(f, "{}", units.get(&pt).unwrap().to_str())?;
                    } else {
                        write!(f, ".")?;
                    }
                }
                write!(f, "\n")?;
            }
            write!(f, "\n")?;
            for unit in self.units.iter().sorted_by_key(|u| u.0.id) {
                write!(
                    f, 
                    "{:02} {} at {:?}: {} hit points\n",
                    unit.0.id,
                    unit.0.unit_type.to_str(),
                    unit.0.point,
                    *unit.0.hit_points.borrow(),
                )?;
            }
            Ok(())
        }
    }

    fn attack<'a>(attacker: &CombatUnit, targets: impl Iterator<Item = &'a Reverse<CombatUnit>>) -> bool {
        if let Some(target) = targets.filter(|u| {
            attacker.is_adjacent(&u.0)
        }).sorted_by(|l, r| {
            l.0.hit_points.borrow().cmp(&r.0.hit_points.borrow())
                .then_with(|| l.0.point.cmp(&r.0.point))
        }).next() {
            target.0.absorb_attack(attacker.attack_power);
            return true;
        }
        false
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::CombatSimulator;

    #[derive(Debug, Default)]
    pub struct Soln {
        combat_simulator: CombatSimulator,    
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.combat_simulator.parse_input_file(filename);
            self.combat_simulator.simulate_combat();
            Answer::Usize(self.combat_simulator.outcome())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(27_730); "example_1")]
        #[test_case(2, Answer::Usize(36_334); "example_2")]
        #[test_case(3, Answer::Usize(39_514); "example_3")]
        #[test_case(4, Answer::Usize(27_755); "example_4")]
        #[test_case(5, Answer::Usize(28_944); "example_5")]
        #[test_case(6, Answer::Usize(18_740); "example_6")]
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