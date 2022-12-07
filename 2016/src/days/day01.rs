use crate::days::{Day, Debug, Example, Part};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

pub struct Day01;

impl Day for Day01 {
    fn number(&self) -> u32 {
        1
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day01 {
    fn part1(&self, example: Example, _debug: Debug) -> isize {
        run(self.read_file(example)).unwrap()
    }

    fn part2(&self, example: Example, _debug: Debug) -> isize {
        run2(self.read_file(example)).unwrap()
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct State {
    orientation: Orientation,
    position: Position,
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Position {
    east_west: isize,
    north_south: isize,
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Orientation {
    #[default]
    North,
    East,
    South,
    West,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Command {
    turn: Turn,
    distance: isize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Turn {
    Left,
    Right,
}

fn run<T: AsRef<str>>(commands_str: T) -> Option<isize> {
    let commands = parse_commands(commands_str.as_ref())?;
    let state = follow_commands(commands);
    Some(state.position.total_distance())
}

fn run2<T: AsRef<str>>(commands_str: T) -> Option<isize> {
    let commands = parse_commands(commands_str.as_ref())?;
    let position = first_visited_twice(commands)?;
    Some(position.total_distance())
}

fn follow_commands<T>(commands: T) -> State
where
    T: IntoIterator<Item = Command>,
{
    let mut state = State::default();
    for command in commands.into_iter() {
        state.follow_command(command);
    }
    state
}

fn first_visited_twice<T>(commands: T) -> Option<Position>
where
    T: IntoIterator<Item = Command>,
{
    let mut state = State::default();
    let mut seen = HashSet::new();
    seen.insert(state.position);
    for command in commands.into_iter() {
        let command: Command = command;
        for position in state.follow_command(command) {
            if seen.contains(&position) {
                return Some(position);
            }
            seen.insert(position);
        }
    }
    None
}

impl State {
    fn follow_command(&mut self, command: Command) -> Vec<Position> {
        self.orientation = self.orientation.turn(command.turn);
        let mut positions = Vec::new();
        for _ in 0..command.distance {
            self.position.advance(self.orientation);
            positions.push(self.position)
        }
        positions
    }
}

impl Position {
    fn advance(&mut self, orientation: Orientation) {
        match orientation {
            Orientation::North => self.north_south += 1,
            Orientation::East => self.east_west += 1,
            Orientation::South => self.north_south -= 1,
            Orientation::West => self.east_west -= 1,
        }
    }

    fn total_distance(&self) -> isize {
        self.north_south.abs() + self.east_west.abs()
    }
}

impl Orientation {
    fn turn(&self, turn: Turn) -> Self {
        match turn {
            Turn::Left => self.left(),
            Turn::Right => self.right(),
        }
    }

    fn left(&self) -> Self {
        match self {
            Orientation::North => Orientation::West,
            Orientation::East => Orientation::North,
            Orientation::South => Orientation::East,
            Orientation::West => Orientation::South,
        }
    }

    fn right(&self) -> Self {
        match self {
            Orientation::North => Orientation::East,
            Orientation::East => Orientation::South,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
        }
    }
}

fn parse_commands<T: AsRef<str>>(string: T) -> Option<Vec<Command>> {
    string.as_ref().split(", ").map(Command::parse).collect()
}

impl Command {
    fn parse<T: AsRef<str>>(string: T) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([LR])(\d+)").unwrap();
        }
        let caps = RE.captures(string.as_ref())?;
        let direction = Turn::parse(caps.get(1).unwrap().as_str())?;
        let distance = caps.get(2).unwrap().as_str().parse().ok()?;
        Some(Command {
            turn: direction,
            distance,
        })
    }
}

impl Turn {
    fn parse<T: AsRef<str>>(string: T) -> Option<Self> {
        Some(match string.as_ref() {
            "L" => Turn::Left,
            "R" => Turn::Right,
            _ => return None,
        })
    }
}

impl Position {
    fn new(east_west: isize, north_south: isize) -> Self {
        Position {
            east_west,
            north_south,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples_part1() {
        let commands_str = "R2, L3";
        let commands = parse_commands(commands_str).unwrap();
        let state = follow_commands(commands);
        assert_eq!(Position::new(2, 3), state.position);
        assert_eq!(5, state.position.total_distance());

        let commands_str = "R2, R2, R2";
        let commands = parse_commands(commands_str).unwrap();
        let state = follow_commands(commands);
        assert_eq!(Position::new(0, -2), state.position);
        assert_eq!(2, state.position.total_distance());

        assert_eq!(Some(12), run("R5, L5, R5, R3"));
    }

    #[test]
    fn test_examples_part2() {
        let commands_str = "R8, R4, R4, R8";
        let commands = parse_commands(commands_str).unwrap();
        let position = first_visited_twice(commands);
        assert_eq!(Some(Position::new(4, 0)), position);
    }
}
