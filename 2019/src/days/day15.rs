#![allow(unused_labels, unused_imports, dead_code, unused_variables)]

use crate::util::intcode::{Error, IntCode, Result, Stopped, Value};
use crate::val;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../static/day15.txt");

lazy_static! {
    static ref VALUE_NORTH: Value = val!(1);
    static ref VALUE_SOUTH: Value = val!(2);
    static ref VALUE_EAST: Value = val!(3);
    static ref VALUE_WEST: Value = val!(4);
    static ref VALUE_HIT_WALL: Value = val!(0);
    static ref VALUE_MOVED: Value = val!(1);
    static ref VALUE_MOVED_FOUND_GOAL: Value = val!(2);
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
    let map = explore_everywhere(program);
    // println!("{:?}", map);
    debug_print_map(&map, Index::ORIGIN, Direction::FIRST);
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
            row: self.row - 1,
            ..self
        }
    }

    fn east(self) -> Self {
        Self {
            column: self.column + 1,
            ..self
        }
    }

    fn south(self) -> Self {
        Self {
            row: self.row + 1,
            ..self
        }
    }

    fn west(self) -> Self {
        Self {
            column: self.column - 1,
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

    fn to_command(self) -> Value {
        match self {
            Direction::North => VALUE_NORTH.clone(),
            Direction::East => VALUE_EAST.clone(),
            Direction::South => VALUE_SOUTH.clone(),
            Direction::West => VALUE_WEST.clone(),
        }
    }
}

// fn explore_everywhere(program: IntCode) -> HashMap<Index, Cell> {
//     let mut map = HashMap::new();
//     let init_index = Index { row: 0, column: 0 };
//     map.insert(init_index, Cell::Empty);
//     explore(&mut map, init_index);
// }

#[derive(Debug)]
struct Breadcrumb {
    index: Index,
    direction: Direction,
}

impl Breadcrumb {
    pub fn new(index: Index, direction: Direction) -> Self {
        Self { index, direction }
    }
}

impl Response {
    pub fn try_from_value(value: Value) -> Result<Self> {
        Ok(match () {
            _ if value == *VALUE_HIT_WALL => Response::HitWall,
            _ if value == *VALUE_MOVED => Response::Moved,
            _ if value == *VALUE_MOVED_FOUND_GOAL => Response::OxygenSystem,
            _ => return Err(format!("Invalid status code {}", value)),
        })
    }
}

fn explore_everywhere(mut program: IntCode) -> HashMap<Index, Cell> {
    let mut cur_index = Index::ORIGIN;
    let mut cur_direction = Direction::FIRST;

    let mut map = HashMap::new();
    map.insert(cur_index, Cell::Empty);

    let mut breadcrumbs = Vec::<Breadcrumb>::new();
    // let mut breadcrumbs = vec![Breadcrumb::new(init_index, Direction::FIRST)];

    let mut count = 0;
    const MAX_COUNT: usize = 1_000_000;
    // const MAX_COUNT: usize = 1_000;
    const EVERY: usize = 10_000;
    'outer: loop {
        if count > MAX_COUNT {
            break;
        } else if count % EVERY == 0 {
            // println!("Count: {}", count);
            debug_print_map(&map, cur_index, cur_direction);
            println!();
        }
        count += 1;

        // println!(
        //     "Index: ({}, {}), Direction: {:?}",
        //     cur_index.row, cur_index.column, cur_direction
        // );

        let next_index = cur_index.step(cur_direction);
        if map.contains_key(&next_index) {
            // Map contains key, so we've already been here. Better rotate or backtrack.
            match cur_direction.next() {
                // Successful rotate.
                Some(next_direction) => cur_direction = next_direction,
                // Unsuccessful rotate, backgrack.
                None => {
                    'inner: while let Some(breadcrumb) = breadcrumbs.pop() {
                        // println!("Popped breadcrumb: {:?}", breadcrumb);
                        send_move_command(&mut program, breadcrumb.direction.opposite());
                        cur_index = breadcrumb.index;
                        if let Some(direction) = breadcrumb.direction.next() {
                            // println!("\tBreadcrumb next direction: {:?}", direction);
                            cur_direction = direction;
                            continue 'outer;
                        } else {
                            // println!("\tBreadcrumb no next direction, keep popping");
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
            let response = send_move_command(&mut program, cur_direction);
            match response {
                Response::Moved => {
                    breadcrumbs.push(Breadcrumb::new(cur_index, cur_direction));
                    map.insert(next_index, Cell::Empty);
                    cur_index = next_index;
                    cur_direction = Direction::FIRST;
                }
                Response::HitWall => {
                    map.insert(next_index, Cell::Wall);
                    // Let the loop deal with this. Probably should refactor.
                }
                Response::OxygenSystem => {
                    breadcrumbs.push(Breadcrumb::new(cur_index, cur_direction));
                    map.insert(next_index, Cell::OxygenSystem);
                    cur_index = next_index;
                    cur_direction = Direction::FIRST;
                }
            };
        }
    }

    return map;
}

fn send_move_command(program: &mut IntCode, direction: Direction) -> Response {
    // println!("Pushing input: {:?}", direction.to_command());
    // println!("Pushing input: {:?}", direction);
    program.push_input(direction.to_command());
    // println!("Pushed input, running");
    let stopped = program.run_blocking_input().expect("Cannot run");
    assert!(stopped == Stopped::NeedInput);
    // println!("Ran, popping output");
    let value = program.pop_output().expect("No output");
    // println!("Output: {:?}", value);
    // Response::try_from_value(value).expect("Invalid output")
    let response = Response::try_from_value(value).expect("Invalid output");
    // println!(", response: {:?}", response);
    response
}

fn debug_print_map(map: &HashMap<Index, Cell>, cur_index: Index, cur_direction: Direction) {
    let drone_char = match cur_direction {
        Direction::North => '^',
        Direction::East => '>',
        Direction::South => 'v',
        Direction::West => '<',
    };

    let rows = map.keys().map(|index| index.row);
    let min_row = rows.clone().min().expect("Empty map");
    let max_row = rows.max().expect("Empty map");
    let columns = map.keys().map(|index| index.column);
    let min_column = columns.clone().min().expect("Empty map");
    let max_column = columns.max().expect("Empty map");

    for row in min_row..=max_row {
        for column in min_column..=max_column {
            let index = Index { row, column };
            let c = if index == cur_index {
                drone_char
            } else {
                match map.get(&index) {
                    Some(Cell::Empty) => '.',
                    Some(Cell::Wall) => '#',
                    Some(Cell::OxygenSystem) => 'O',
                    None => ' ',
                }
            };
            print!("{}", c);
        }
        println!();
    }
}
