use std::cmp::min;
use crate::days::{Day, Debug, Example, Part};

pub struct Day02;

impl Day for Day02 {
    fn number(&self) -> u32 {
        2
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day02 {
    fn part1(&self, example: Example, _debug: Debug) -> String {
        get_code(&self.read_file(example)).unwrap()
    }

    fn part2(&self, _example: Example, _debug: Debug) -> String {
        unimplemented!()
    }
}

const MAX_ROW: usize = 2;
const MAX_COLUMN: usize = 2;

fn get_code(string: &str) -> Option<String> {
    let lines = string.trim().split("\n");
    let commands: Vec<Vec<Command>> = lines
        .map(|line| line.chars().map(Command::parse).collect())
        .collect::<Option<_>>()?;
    Some(run_commands(&commands))
}

fn run_commands(commands: &[Vec<Command>]) -> String {
    let mut position = Position::default();
    let mut code = Vec::new();
    for command_row in commands {
        for command in command_row {
            position.follow_command(*command);
        }
        code.push(position.to_keypad_digit());
    }
    code.into_iter().map(|d| d.to_string()).collect()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Command {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Position {
    row: usize,
    column: usize,
}

impl Position {
    fn follow_command(&mut self, command: Command) {
        match command {
            Command::Up => self.row = self.row.saturating_sub(1),
            Command::Right => self.column = min(self.column + 1, MAX_COLUMN),
            Command::Down => self.row = min(self.row + 1, MAX_ROW),
            Command::Left => self.column = self.column.saturating_sub(1),
        }
    }

    fn to_keypad_digit(&self) -> usize {
        match (self.row, self.column) {
            (0, 0) => 1,
            (0, 1) => 2,
            (0, 2) => 3,
            (1, 0) => 4,
            (1, 1) => 5,
            (1, 2) => 6,
            (2, 0) => 7,
            (2, 1) => 8,
            (2, 2) => 9,
            _ => panic!("Invalid position: {:?}", self),
        }
    }
}

impl Command {
    fn parse(c: char) -> Option<Self> {
        Some(match c {
            'U' => Command::Up,
            'R' => Command::Right,
            'D' => Command::Down,
            'L' => Command::Left,
            _ => return None,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples_part1() {
        let text = "ULL\n\
                    RRDDD\n\
                    LURDL\n\
                    UUUUD";
        let code = get_code(text);
        assert_eq!(Some("1985".to_string()), code);
    }

    #[test]
    fn test_examples_part2() {}
}
