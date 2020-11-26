#![allow(unused_variables, dead_code, unused_imports)]
use crate::util::intcode::{IntCode, Result, Stopped, Value};
use crate::val;
use array2d::Array2D;
use num_bigint::BigInt;
use std::collections::HashMap;
use std::fmt;

const INPUT: &str = include_str!("../../static/day17.txt");

const CHAR_SCAFFOLD: char = '#';
const CHAR_OPEN_SPACE: char = '.';
const CHAR_ROBOT_UP: char = '^';
const CHAR_ROBOT_RIGHT: char = '>';
const CHAR_ROBOT_DOWN: char = 'v';
const CHAR_ROBOT_LEFT: char = '<';

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Cell {
    Scaffold,
    OpenSpace,
    Robot(Direction),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Index {
    row: isize,
    column: isize,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub fn main() {
    let answer1 = solve1(INPUT);
    println!("{}", answer1.unwrap());
    // let answer2 = solve2(INPUT);
    // println!("{:?}", answer2);
}

fn solve1(input: &str) -> Result<usize> {
    let program = IntCode::from_str(input)?;
    let map = get_map(program)?;
    // debug_print_map(&map);
    let intersections = intersection_indices(&map);
    Ok(intersections.map(|(row, column)| row * column).sum())
}

fn get_map(intcode: IntCode) -> Result<Array2D<Cell>> {
    let product = intcode.run()?;
    let string = to_chars(product.outputs()).iter().collect::<String>();
    let string = string.trim();
    let lines = string
        .lines()
        .map(|line| {
            line.chars()
                .map(Cell::try_from_char)
                .collect::<Option<Vec<_>>>()
        })
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| "Invalid character".to_string())?;
    Ok(Array2D::from_rows(&lines).expect("Invalid outputs"))
}

fn intersection_indices<'a>(map: &'a Array2D<Cell>) -> impl Iterator<Item = (usize, usize)> + 'a {
    map.indices_row_major()
        .filter(move |i| is_intersection_index(map, *i))
}

fn is_intersection_index(map: &Array2D<Cell>, index: (usize, usize)) -> bool {
    if !map[index].has_scaffold() {
        return false;
    }
    let num_neighboring_scaffolds = Direction::all()
        .iter()
        .filter_map(|&d| index_step(index, d))
        .filter_map(|(r, c)| map.get(r, c))
        .filter(|c| c.has_scaffold())
        .count();
    num_neighboring_scaffolds >= 3
}

fn index_step((row, column): (usize, usize), direction: Direction) -> Option<(usize, usize)> {
    let new_row = match direction {
        Direction::Up => {
            if row == 0 {
                return None;
            } else {
                row - 1
            }
        }
        Direction::Down => row + 1,
        _ => row,
    };
    let new_column = match direction {
        Direction::Left => {
            if column == 0 {
                return None;
            } else {
                column - 1
            }
        }
        Direction::Right => column + 1,
        _ => column,
    };
    Some((new_row, new_column))
}

fn has_scaffold(cell: Cell) -> bool {
    match cell {
        Cell::Scaffold => true,
        Cell::Robot(_) => true,
        _ => false,
    }
}

fn to_chars(outputs: &[Value]) -> Vec<char> {
    outputs.iter().map(bigint_to_char).collect()
}

fn bigint_to_char(num: &BigInt) -> char {
    let (_, digits) = num.to_u32_digits();
    let num = *digits.last().unwrap_or(&0);
    let num = num as u8;
    char::from(num)
}

impl Cell {
    fn try_from_char(c: char) -> Option<Cell> {
        Some(match c {
            CHAR_SCAFFOLD => Cell::Scaffold,
            CHAR_OPEN_SPACE => Cell::OpenSpace,
            CHAR_ROBOT_UP => Cell::Robot(Direction::Up),
            CHAR_ROBOT_RIGHT => Cell::Robot(Direction::Right),
            CHAR_ROBOT_DOWN => Cell::Robot(Direction::Down),
            CHAR_ROBOT_LEFT => Cell::Robot(Direction::Left),
            _ => return None,
        })
    }

    fn to_char(self) -> char {
        match self {
            Cell::Scaffold => CHAR_SCAFFOLD,
            Cell::OpenSpace => CHAR_OPEN_SPACE,
            Cell::Robot(Direction::Up) => CHAR_ROBOT_UP,
            Cell::Robot(Direction::Right) => CHAR_ROBOT_RIGHT,
            Cell::Robot(Direction::Down) => CHAR_ROBOT_DOWN,
            Cell::Robot(Direction::Left) => CHAR_ROBOT_LEFT,
        }
    }

    fn has_scaffold(self) -> bool {
        match self {
            Cell::Scaffold => true,
            Cell::Robot(_) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl Direction {
    const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];

    fn all() -> &'static [Direction] {
        &Direction::ALL
    }
}

fn debug_print_map(map: &Array2D<Cell>) {
    print!("   ");
    for i in 0..map.num_columns() {
        print!("{}", i % 10);
    }
    println!();
    for (i, row_iter) in map.rows_iter().enumerate() {
        print!("{:2} ", i);
        for elem in row_iter {
            print!("{}", elem);
        }
        println!();
    }
}
