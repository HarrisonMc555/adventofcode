use crate::util::intcode::{Error, IntCode, Result, Stopped, Value};
use crate::val;
use std::collections::HashMap;

const INPUT: &str = include_str!("../../static/day15.txt");

lazy_static! {
    static ref VALUE_NORTH: Value = val!(1);
    static ref VALUE_SOUTH: Value = val!(2);
    static ref VALUE_EAST: Value = val!(3);
    static ref VALUE_WEST: Value = val!(4);
    static ref VALUE_HIT_WALL: Value = val!(0);
    static ref VALUE_MOVED: Value = val!(1);
    static ref VALUE_MOVED_FOUND_GOAL: Value = val!(2);
    static ref VALUE_ONE: Value = val!(1);
}

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
    let program = IntCode::from_str(input)?;
    explore_everywhere(program);
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

    const ORIGIN: Self = Self { row: 0, column: 0 };
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

    const FIRST: Self = Direction::North;

    fn next(self) -> Option<Self> {
        Some(match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => return None,
        })
    }

    fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

// fn explore_everywhere(program: IntCode) -> HashMap<Index, Cell> {
//     let mut map = HashMap::new();
//     let init_index = Index { row: 0, column: 0 };
//     map.insert(init_index, Cell::Empty);
//     explore(&mut map, init_index);
// }

struct Breadcrumb {
    index: Index,
    direction: Direction,
}

impl Breadcrumb {
    pub fn new(index: Index, direction: Direction) -> Self {
        Self { index, direction }
    }
}

fn explore_everywhere(program: IntCode) -> HashMap<Index, Cell> {
    let mut cur_index = Index::ORIGIN;
    let mut cur_direction = Direction::FIRST;

    let mut map = HashMap::new();
    map.insert(cur_index, Cell::Empty);

    let mut breadcrumbs = Vec::<Breadcrumb>::new();
    // let mut breadcrumbs = vec![Breadcrumb::new(init_index, Direction::FIRST)];

    'outer: loop {
        let next_index = cur_index.step(cur_direction);
        if map.contains_key(&next_index) {
            // Map contains key, so we've already been here. Better rotate or backtrack.
            match cur_direction.next() {
                // Successful rotate.
                Some(next_direction) => cur_direction = next_direction,
                // Unsuccessful rotate, backgrack.
                None => {
                    'inner: while let Some(breadcrumb) = breadcrumbs.pop() {
                        send_move_command(breadcrumb.direction.opposite());
                        cur_index = breadcrumb.index;
                        if let Some(direction) = breadcrumb.direction.next() {
                            cur_direction = direction;
                            continue 'outer;
                        }
                    }
                    // Only get here if we ran out of breadcrumbs. That means we're back to the
                    // beginning.
                    assert_eq!(cur_index, Index::ORIGIN);
                    return map;
                }
            }
        } else {
            // Map does NOT contain key. Move forward.
            send_move_command(cur_direction);
            cur_index = cur_index.step(cur_direction);
            cur_direction = Direction::FIRST;
        }
    }
}

fn send_move_command(direction: Direction) -> Response {
    unimplemented!()
}
