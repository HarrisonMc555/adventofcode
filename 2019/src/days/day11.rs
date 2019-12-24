use crate::util::intcode::{Error, IntCode, Result, Stopped, Value};
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::fmt;

const INPUT: &str = include_str!("../../static/day11.txt");

type Location = (isize, isize);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Color {
    Black,
    White,
}

lazy_static! {
    static ref BLACK_VALUE: Value = Value::from(0);
    static ref WHITE_VALUE: Value = Value::from(1);
    static ref LEFT_VALUE: Value = Value::from(0);
    static ref RIGHT_VALUE: Value = Value::from(1);
}

struct Robot {
    program: IntCode,
    loc: Location,
    dir: Direction,
    grid: HashMap<Location, Color>,
}

pub fn main() {
    let answer1 = solve1(INPUT).unwrap();
    println!("{}", answer1);
    let grid = solve2(INPUT).unwrap();
    let radius = 25;
    print_grid(&grid, radius);
}

fn solve1(input: &str) -> Result<usize> {
    let program = IntCode::from_str(input)?;
    let robot = Robot::new(program);
    robot.num_painted()
}

fn solve2(input: &str) -> Result<HashMap<Location, Color>> {
    let program = IntCode::from_str(input)?;
    let mut robot = Robot::new(program);
    robot.paint_tile((0, 0), Color::White);
    robot.run_to_grid()
}

impl Robot {
    pub fn new(program: IntCode) -> Self {
        Robot {
            program,
            loc: (0, 0),
            dir: Direction::North,
            grid: HashMap::new(),
        }
    }

    pub fn run_to_grid(mut self) -> Result<HashMap<Location, Color>> {
        loop {
            let color = self.grid.get(&self.loc).unwrap_or(&Color::Black).clone();
            self.program.push_input(&color.value());
            match self.program.run_blocking_input()? {
                Stopped::NeedInput(p) => {
                    self.program = p;
                }
                Stopped::Complete(_) => {
                    return Ok(self.grid);
                }
            }
            let paint_command = self.program.pop_output()?;
            let turn_command = self.program.pop_output()?;
            self.run_paint_command(&paint_command)?;
            self.run_turn_command(&turn_command)?;
        }
    }

    pub fn num_painted(mut self) -> Result<usize> {
        let mut seen = HashSet::new();
        loop {
            let color = self.grid.get(&self.loc).unwrap_or(&Color::Black).clone();
            self.program.push_input(&color.value());
            match self.program.run_blocking_input()? {
                Stopped::NeedInput(p) => {
                    self.program = p;
                }
                Stopped::Complete(_) => break,
            }
            let paint_command = self.program.pop_output()?;
            let turn_command = self.program.pop_output()?;
            self.run_paint_command(&paint_command)?;
            seen.insert(self.loc);
            self.run_turn_command(&turn_command)?;
        }
        Ok(seen.len())
    }

    pub fn paint_tile(&mut self, location: Location, color: Color) {
        self.grid.insert(location, color);
    }

    fn run_paint_command(&mut self, paint_command: &Value) -> Result<()> {
        let color = Color::try_from(paint_command)?;
        self.grid.insert(self.loc, color);
        Ok(())
    }

    fn run_turn_command(&mut self, turn_command: &Value) -> Result<()> {
        let turn = Turn::try_from(turn_command)?;
        self.dir.turn(turn);
        self.forward();
        Ok(())
    }

    fn forward(&mut self) {
        let (x, y) = self.loc;
        self.loc = match self.dir {
            Direction::North => (x, y + 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y - 1),
            Direction::West => (x - 1, y),
        };
    }
}

impl TryFrom<&Value> for Color {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let color = match value {
            _ if value == &*BLACK_VALUE => Color::Black,
            _ if value == &*WHITE_VALUE => Color::White,
            _ => return Err(format!("Invalid color value {}", &value)),
        };
        Ok(color)
    }
}

impl TryFrom<&Value> for Turn {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let turn = match value {
            _ if value == &*LEFT_VALUE => Turn::Left,
            _ if value == &*RIGHT_VALUE => Turn::Right,
            _ => return Err(format!("Invalid turn value {}", &value)),
        };
        Ok(turn)
    }
}

// impl

impl Direction {
    fn turn(&mut self, turn: Turn) {
        match turn {
            Turn::Left => self.left(),
            Turn::Right => self.right(),
        }
    }

    fn left(&mut self) {
        let new = match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        *self = new;
    }

    fn right(&mut self) {
        let new = match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        *self = new;
    }
}

impl Color {
    fn value(&self) -> Value {
        match self {
            Color::Black => BLACK_VALUE.clone(),
            Color::White => WHITE_VALUE.clone(),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Direction::North => "^",
            Direction::East => ">",
            Direction::South => "v",
            Direction::West => "<",
        };
        write!(f, "{}", c)
    }
}

fn print_grid(grid: &HashMap<Location, Color>, radius: isize) {
    let max = radius.abs();
    let min = -max;
    for y in (min..=max).rev() {
        for x in min..=max {
            let loc = (x, y);
            let color = grid.get(&loc).unwrap_or(&Color::Black);
            let c = match color {
                Color::Black => ".",
                Color::White => "#",
            };
            print!("{}", c);
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn answer1() {
        assert_eq!(solve1(INPUT), Ok(1686));
    }
}
