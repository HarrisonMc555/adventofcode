use array2d::Array2D;

use crate::days::{Day, Debug, Example, Part};
use crate::{debug_print, debug_println};

const DEBUG: bool = false;

pub struct Day22;

impl Day for Day22 {
    fn number(&self) -> u32 {
        22
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day22 {
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        let (board, commands) = parse(&self.read_file(example)).unwrap();
        let state = simulate(board, &commands).unwrap();
        get_password(state)
    }

    fn part2(&self, _example: Example, _debug: Debug) -> usize {
        todo!()
    }
}

const BASE: u32 = 10;
type Board = Array2D<Cell>;
type Commands = Vec<Command>;

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
enum Cell {
    Open,
    Solid,
    Nonexistent,
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
enum Command {
    Forward(u32),
    Turn(Direction),
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct State {
    board: Board,
    row: usize,
    col: usize,
    orientation: Orientation,
}

#[derive(Debug, Hash, Default, Copy, Clone, Eq, PartialEq)]
enum Orientation {
    Up,
    #[default]
    Right,
    Down,
    Left,
}

fn get_password(state: State) -> usize {
    let row = state.row + 1;
    let col = state.col + 1;
    let facing = match state.orientation {
        Orientation::Right => 0,
        Orientation::Down => 1,
        Orientation::Left => 2,
        Orientation::Up => 3,
    };
    1000 * row + 4 * col + facing
}

fn simulate(board: Board, commands: &Commands) -> Option<State> {
    let mut state = State::new(board)?;
    for command in commands {
        state.run_command(*command)
    }
    Some(state)
}

impl State {
    fn new(board: Board) -> Option<Self> {
        let (row, col) = board
            .enumerate_row_major()
            .find(|(_, cell)| **cell == Cell::Open)
            .map(|(i, _)| i)?;
        Some(State {
            board,
            row,
            col,
            orientation: Default::default(),
        })
    }

    fn run_command(&mut self, command: Command) {
        match command {
            Command::Forward(amount) => self.forward(amount),
            Command::Turn(direction) => self.turn(direction),
        }
    }

    fn forward(&mut self, amount: u32) {
        for _ in 0..amount {
            self.step();
        }
    }

    fn step(&mut self) {
        #[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
        enum Diff {
            Increment,
            Decrement,
            Unchanged,
        }
        let (diff_row, diff_col) = match self.orientation {
            Orientation::Up => (Diff::Decrement, Diff::Unchanged),
            Orientation::Right => (Diff::Unchanged, Diff::Increment),
            Orientation::Down => (Diff::Increment, Diff::Unchanged),
            Orientation::Left => (Diff::Unchanged, Diff::Decrement),
        };

        fn apply(num: usize, diff: Diff, max: usize) -> usize {
            match diff {
                Diff::Unchanged => num,
                Diff::Increment => {
                    let new = num + 1;
                    if new > max {
                        0
                    } else {
                        new
                    }
                }
                Diff::Decrement => num.checked_sub(1).unwrap_or(max),
            }
        }

        let max_row = self.board.num_rows() - 1;
        let max_col = self.board.num_columns() - 1;
        let mut row = self.row;
        let mut col = self.col;
        loop {
            let new_row = apply(row, diff_row, max_row);
            let new_col = apply(col, diff_col, max_col);
            match self.board[(new_row, new_col)] {
                Cell::Open => {
                    self.row = new_row;
                    self.col = new_col;
                    return;
                }
                Cell::Solid => return,
                Cell::Nonexistent => {
                    row = new_row;
                    col = new_col;
                    continue;
                }
            }
        }
    }

    fn turn(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.orientation = self.orientation.counter_clockwise(),
            Direction::Right => self.orientation = self.orientation.clockwise(),
        }
    }
}

impl Orientation {
    fn clockwise(self) -> Self {
        match self {
            Orientation::Up => Orientation::Right,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
        }
    }

    fn counter_clockwise(self) -> Self {
        match self {
            Orientation::Up => Orientation::Left,
            Orientation::Left => Orientation::Down,
            Orientation::Down => Orientation::Right,
            Orientation::Right => Orientation::Up,
        }
    }
}

fn parse(text: &str) -> Option<(Board, Commands)> {
    let (board_text, commands_text) = text.split_once("\n\n")?;
    debug_println!("Board text:\n{}\n", board_text);
    let board = parse_board(board_text)?;
    debug_println!("Board: {:?}\n", board);
    debug_println!("Commands text:\n{}\n", board_text);
    let commands = parse_commands(commands_text)?;
    debug_println!("Commands: {:?}\n", commands);
    Some((board, commands))
}

fn parse_board(text: &str) -> Option<Array2D<Cell>> {
    let mut rows = text
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Cell::parse(c))
                .collect::<Option<Vec<_>>>()
        })
        .collect::<Option<Vec<_>>>()?;
    debug_println!("Rows: {:?}", rows);
    for row in rows.iter() {
        for cell in row.iter() {
            debug_print!("{}", cell.to_char());
        }
        debug_println!();
    }

    let max_num_columns = rows.iter().map(|row| row.len()).max()?;
    for row in rows.iter_mut() {
        while row.len() < max_num_columns {
            row.push(Cell::Nonexistent);
        }
    }

    let result = Array2D::from_rows(&rows);
    debug_println!("Result: {:?}", result);
    result.ok()
}

fn parse_commands(text: &str) -> Option<Commands> {
    let mut chars = text.trim().chars();
    let mut digits = Vec::new();
    let mut commands = Vec::new();
    loop {
        let Some(c) = chars.next() else {
            break;
        };
        match c.to_digit(BASE) {
            Some(digit) => digits.push(digit),
            None => {
                let amount = from_digits(&digits);
                digits.clear();
                let direction = Direction::parse(c)?;
                commands.push(Command::Forward(amount));
                commands.push(Command::Turn(direction));
            }
        }
    }
    if digits.is_empty() {
        debug_println!("Should end with forward command");
        return None;
    }
    let amount = from_digits(&digits);
    commands.push(Command::Forward(amount));
    Some(commands)
}

impl Cell {
    fn parse(c: char) -> Option<Self> {
        Some(match c {
            '.' => Cell::Open,
            '#' => Cell::Solid,
            ' ' => Cell::Nonexistent,
            _ => {
                debug_println!("Unexpected cell character: {:?}", c);
                return None;
            }
        })
    }

    fn to_char(self) -> char {
        match self {
            Cell::Open => '.',
            Cell::Solid => '#',
            Cell::Nonexistent => ' ',
        }
    }
}

impl Direction {
    fn parse(c: char) -> Option<Self> {
        Some(match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => {
                debug_println!("Unexpected direction character: {:?}", c);
                return None;
            }
        })
    }
}

fn from_digits(digits: &[u32]) -> u32 {
    digits.iter().fold(0, |acc, digit| acc * BASE + *digit)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let commands_text = "10R5L5R10L4R5L5";
        let actual = parse_commands(commands_text).unwrap();
        let expected = vec![
            Command::Forward(10),
            Command::Turn(Direction::Right),
            Command::Forward(5),
            Command::Turn(Direction::Left),
            Command::Forward(5),
            Command::Turn(Direction::Right),
            Command::Forward(10),
            Command::Turn(Direction::Left),
            Command::Forward(4),
            Command::Turn(Direction::Right),
            Command::Forward(5),
            Command::Turn(Direction::Left),
            Command::Forward(5),
        ];
        assert_eq!(expected, actual);

        assert!(parse(include_str!("../../static/example22.txt")).is_some());
    }

    #[test]
    fn test_examples_part1() {
        assert_eq!(6032, Day22.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(146092, Day22.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        // assert_eq!(0, Day22.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        // assert_eq!(0, Day22.part2(Example::Real, Debug::NotDebug));
    }
}
