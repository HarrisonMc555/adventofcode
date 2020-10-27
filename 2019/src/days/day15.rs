use crate::util::intcode::{Error, IntCode, Result, Stopped, Value};
use std::collections::HashMap;

const INPUT: &str = include_str!("../../static/day15.txt");

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Cell {
    Empty,
    Wall,
    OxygenSystem,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Response {
    HitWall,
    Moved,
    OxygenSystem,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Index {
    row: isize,
    column: isize,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

pub fn main() {
    let answer1 = solve1(INPUT);
    println!("{:?}", answer1);
    // let answer2 = solve2(INPUT);
    // println!("{:?}", answer2);
}

fn solve1(input: &str) -> Result<()> {
    explore_everywhere(Index { row: 0, column: 0 });
    Err("unimplemented".to_string())
}

impl Index {
    fn step(self, direction: Direction) -> Self {
        match direction {
            Direction::North => self.north(),
            Direction::East => self.east(),
            Direction::South => self.south(),
            Direction::West => self.west(),
        }
    }

    fn north(self) -> Self {
        Self {
            column: self.column - 1,
            ..self
        }
    }

    fn east(self) -> Self {
        Self {
            row: self.row + 1,
            ..self
        }
    }

    fn south(self) -> Self {
        Self {
            column: self.column + 1,
            ..self
        }
    }

    fn west(self) -> Self {
        Self {
            row: self.row - 1,
            ..self
        }
    }
}

impl Direction {
    const ALL_DIRECTIONS: [Self; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    fn all() -> impl Iterator<Item = Self> {
        Self::ALL_DIRECTIONS.iter().copied()
    }
}

fn explore_everywhere(cur_index: Index) -> HashMap<Index, Cell> {
    // let mut cur_index = Index { row: 0, column: 0 };
    let mut map = HashMap::new();
    map.insert(cur_index, Cell::Empty);
    for direction in Direction::all() {
        let new_index = cur_index.step(direction);
        if map.contains_key(&new_index) {
            continue;
        }
        match send_move_command(direction) {
            Response::HitWall => continue,
            Response::Moved => {
                map.insert(new_index, Cell::Empty);
                explore_everywhere(new_index);
            }
            Response::OxygenSystem => {
                map.insert(new_index, Cell::OxygenSystem);
                explore_everywhere(new_index);
            }
        }
    }
    // if
    panic!("Unimplemented");
}

fn send_move_command(direction: Direction) -> Response {
    unimplemented!()
}
