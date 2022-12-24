use std::collections::HashMap;

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
    fn part1(&self, example: Example, _debug: Debug) -> isize {
        let (board, commands) = parse(&self.read_file(example)).unwrap();
        let state = simulate(board, &commands).unwrap();
        get_password(state)
    }

    fn part2(&self, _example: Example, _debug: Debug) -> isize {
        todo!()
    }
}

const BASE: u32 = 10;
type Commands = Vec<Command>;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Board {
    cells: HashMap<(isize, isize), Cell>,
    num_rows: isize,
    num_cols: isize,
}

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

#[derive(Debug, Eq, PartialEq)]
struct State {
    board: Board,
    row: isize,
    col: isize,
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

fn get_password(state: State) -> isize {
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
        let (row, col) = board.cells
            .iter()
            .filter(|(_, cell)| **cell == Cell::Open)
            .map(|(index, _)| index)
            .copied()
            .min()?;
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
        let (diff_row, diff_col) = match self.orientation {
            Orientation::Up => (-1, 0),
            Orientation::Right => (0, 1),
            Orientation::Down => (1, 0),
            Orientation::Left => (0, -1),
        };

        let mut row = self.row;
        let mut col = self.col;
        loop {
            let new_row = (row + diff_row).rem_euclid(self.board.num_rows);
            let new_col = (col + diff_col).rem_euclid(self.board.num_cols);
            match self.board.cells.get(&(new_row, new_col)) {
                Some(Cell::Open) => {
                    self.row = new_row;
                    self.col = new_col;
                    return;
                }
                Some(Cell::Solid) => return,
                Some(Cell::Nonexistent) | None => {
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

fn parse_board(text: &str) -> Option<Board> {
    let rows = text
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

    let num_rows = rows.len() as isize;
    let num_cols = rows.iter().map(|row| row.len()).max()? as isize;
    let mut cells = HashMap::new();
    for (row_index, row) in rows.into_iter().enumerate() {
        for (col_index, cell) in row.into_iter().enumerate() {
            if cell != Cell::Nonexistent {
                cells.insert((row_index as isize, col_index as isize), cell);
            }
        }
    }

    debug_println!("Result: {:?}", cells);
    Some(Board {
        cells,
        num_rows,
        num_cols,
    })
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
