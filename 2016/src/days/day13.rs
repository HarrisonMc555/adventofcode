use std::collections::{HashMap, VecDeque};

use crate::days::{Day, Debug, Example, Part};

const DEBUG: bool = false;

const START: Index = Index::new(1, 1);
const GOAL: Index = Index::new(31, 39);
const MAX_STEPS: usize = 50;

pub struct Day13;

impl Day for Day13 {
    fn number(&self) -> u32 {
        13
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day13 {
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        let favorite_number = self.read_file(example).parse().unwrap();
        debug_println!("Favorite number: {}", favorite_number);
        let generator = CellGenerator::new(favorite_number);
        generator.debug_print_grid(9, 6);
        generator.shortest_path_len(START, GOAL)
    }

    fn part2(&self, example: Example, _debug: Debug) -> usize {
        let favorite_number = self.read_file(example).parse().unwrap();
        debug_println!("Favorite number: {}", favorite_number);
        let generator = CellGenerator::new(favorite_number);
        generator.debug_print_grid(9, 6);
        generator.reachable_in_steps(START, MAX_STEPS)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Cell {
    Wall,
    OpenSpace,
}

#[derive(Debug, Copy, Clone)]
struct CellGenerator {
    favorite_number: u32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Index {
    x: u32,
    y: u32,
}

impl CellGenerator {
    pub fn new(favorite_number: u32) -> Self {
        Self { favorite_number }
    }

    pub fn get_cell(self, Index { x, y }: Index) -> Cell {
        let sum = (x * x) + (3 * x) + (2 * x * y) + (y) + (y * y) + self.favorite_number;
        let num_one_bits = sum.count_ones();
        if is_even(num_one_bits) {
            Cell::OpenSpace
        } else {
            Cell::Wall
        }
    }

    pub fn debug_print_grid(self, max_x: u32, max_y: u32) {
        debug_print!("  ");
        for x in 0..=max_x {
            debug_print!("{}", x);
        }
        debug_println!();
        for y in 0..=max_y {
            debug_print!("{} ", y);
            for x in 0..=max_x {
                let index = Index::new(x, y);
                debug_print!("{}", self.get_cell(index));
            }
            debug_println!();
        }
    }

    fn shortest_path_len(self, start: Index, goal: Index) -> usize {
        let mut distances = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));
        loop {
            let (index, distance) = queue.pop_front().expect("Impossible board");
            if index == goal {
                return distance;
            }
            match self.get_cell(index) {
                Cell::OpenSpace => {}
                Cell::Wall => continue,
            }
            distances.insert(index, distance);
            let next_distance = distance + 1;
            for neighbor in index.neighbors() {
                match distances.get(&neighbor) {
                    Some(d) if *d <= next_distance => {}
                    _ => queue.push_back((neighbor, next_distance)),
                }
            }
        }
    }

    fn reachable_in_steps(self, start: Index, max_steps: usize) -> usize {
        let mut distances = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));
        loop {
            let (index, distance) = queue.pop_front().expect("Impossible board");
            if distance > max_steps {
                return distances.len();
            }
            match self.get_cell(index) {
                Cell::OpenSpace => {}
                Cell::Wall => continue,
            }
            distances.insert(index, distance);
            let next_distance = distance + 1;
            for neighbor in index.neighbors() {
                match distances.get(&neighbor) {
                    Some(d) if *d <= next_distance => {}
                    _ => queue.push_back((neighbor, next_distance)),
                }
            }
        }
    }
}

impl Index {
    pub const fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn neighbors(self) -> impl Iterator<Item = Index> {
        let Index { x, y } = self;
        [
            Some(Index::new(x, y + 1)),
            Some(Index::new(x + 1, y)),
            y.checked_sub(1).map(|y| Index::new(x, y)),
            x.checked_sub(1).map(|x| Index::new(x, y)),
        ]
        .into_iter()
        .flatten()
    }
}

fn is_even(x: u32) -> bool {
    x % 2 == 0
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Cell::Wall => '#',
            Cell::OpenSpace => '.',
        };
        write!(f, "{}", c)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let generator = CellGenerator::new(10);
        let start = Index::new(1, 1);
        let goal = Index::new(7, 4);
        let len = generator.shortest_path_len(start, goal);
        assert_eq!(len, 11);
    }
}
