use array2d::Array2D;
use lazy_static::lazy_static;
use regex::Regex;

use crate::days::{Day, Debug, Example, Part};

pub struct Day08;

impl Day for Day08 {
    fn number(&self) -> u32 {
        8
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day08 {
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        let mut screen = Screen::default();
        let commands = parse(&self.read_file(example)).unwrap();
        for command in commands {
            screen.apply_command(&command);
        }
        screen
            .0
            .elements_row_major_iter()
            .filter(|cell| **cell == Cell::On)
            .count()
    }

    fn part2(&self, _example: Example, _debug: Debug) -> usize {
        todo!()
    }
}

#[derive(Debug)]
struct Screen(Array2D<Cell>);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Cell {
    On,
    Off,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Command {
    Rect(Rect),
    RotateRow(RotateRow),
    RotateColumn(RotateColumn),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Rect {
    width: usize,
    height: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct RotateRow {
    row: usize,
    amount: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct RotateColumn {
    column: usize,
    amount: usize,
}

impl Screen {
    const DEFAULT_WIDTH: usize = 50;
    const DEFAULT_HEIGHT: usize = 6;

    fn new(num_rows: usize, num_columns: usize) -> Self {
        Screen(Array2D::filled_with(Cell::Off, num_rows, num_columns))
    }

    fn apply_command(&mut self, command: &Command) {
        match command {
            Command::Rect(rect) => self.rect(rect),
            Command::RotateRow(rotate_row) => self.rotate_row(rotate_row),
            Command::RotateColumn(rotate_column) => self.rotate_column(rotate_column),
        }
    }

    fn rect(&mut self, command: &Rect) {
        for row in 0..command.height {
            for column in 0..command.width {
                if let Some(cell) = self.0.get_mut(row, column) {
                    *cell = Cell::On;
                }
            }
        }
    }

    fn rotate_row(&mut self, command: &RotateRow) {
        let Ok(row) = self.0.row_iter(command.row) else {
            return;
        };
        // This is a false positive.
        #[allow(clippy::needless_collect)]
        let row = row.cloned().collect::<Vec<_>>();
        for (original_column_index, value) in row.into_iter().enumerate() {
            let new_column_index = (original_column_index + command.amount) % self.0.num_columns();
            if let Some(cell) = self.0.get_mut(command.row, new_column_index) {
                *cell = value;
            }
        }
    }

    fn rotate_column(&mut self, command: &RotateColumn) {
        let Ok(column) = self.0.column_iter(command.column) else {
            return;
        };
        // This is a false positive.
        #[allow(clippy::needless_collect)]
        let column = column.cloned().collect::<Vec<_>>();
        for (original_row_index, value) in column.into_iter().enumerate() {
            let new_row_index = (original_row_index + command.amount) % self.0.num_rows();
            if let Some(cell) = self.0.get_mut(new_row_index, command.column) {
                *cell = value;
            }
        }
    }
}

impl Default for Screen {
    fn default() -> Self {
        Screen::new(Screen::DEFAULT_HEIGHT, Screen::DEFAULT_WIDTH)
    }
}

fn parse(text: &str) -> Option<Vec<Command>> {
    text.trim().split('\n').map(Command::parse).collect()
}

impl Command {
    fn parse(line: &str) -> Option<Self> {
        lazy_static! {
            static ref RECT: Regex = Regex::new(r"^rect (\d+)x(\d+)$").unwrap();
            static ref ROTATE_ROW: Regex = Regex::new(r"^rotate row y=(\d+) by (\d+)$").unwrap();
            static ref ROTATE_COLUMN: Regex =
                Regex::new(r"^rotate column x=(\d+) by (\d+)$").unwrap();
        }

        if let Some(caps) = RECT.captures(line) {
            let width = caps.get(1).unwrap().as_str().parse().ok()?;
            let height = caps.get(2).unwrap().as_str().parse().ok()?;
            Some(Rect { width, height }.into())
        } else if let Some(caps) = ROTATE_ROW.captures(line) {
            let row = caps.get(1).unwrap().as_str().parse().ok()?;
            let amount = caps.get(2).unwrap().as_str().parse().ok()?;
            Some(RotateRow { row, amount }.into())
        } else if let Some(caps) = ROTATE_COLUMN.captures(line) {
            let column = caps.get(1).unwrap().as_str().parse().ok()?;
            let amount = caps.get(2).unwrap().as_str().parse().ok()?;
            Some(RotateColumn { column, amount }.into())
        } else {
            None
        }
    }
}

impl From<Rect> for Command {
    fn from(rect: Rect) -> Self {
        Command::Rect(rect)
    }
}

impl From<RotateRow> for Command {
    fn from(rotate_row: RotateRow) -> Self {
        Command::RotateRow(rotate_row)
    }
}

impl From<RotateColumn> for Command {
    fn from(rotate_column: RotateColumn) -> Self {
        Command::RotateColumn(rotate_column)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let text = include_str!("../../static/example08.txt");
        let actual = parse(text).unwrap();
        let expected: Vec<Command> = vec![
            Rect {
                width: 3,
                height: 2,
            }
            .into(),
            RotateColumn {
                column: 1,
                amount: 1,
            }
            .into(),
            RotateRow { row: 0, amount: 4 }.into(),
            RotateColumn {
                column: 1,
                amount: 1,
            }
            .into(),
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_examples_part1() {
        let mut screen = Screen::new(3, 7);
        let actual = to_string(&screen);
        let expected = ".......\n\
                        .......\n\
                        .......\n";
        assert_eq!(expected, actual);

        let command = Command::parse("rect 3x2").unwrap();
        screen.apply_command(&command);
        let actual = to_string(&screen);
        let expected = "###....\n\
                        ###....\n\
                        .......\n";
        assert_eq!(expected, actual);

        let command = Command::parse("rotate column x=1 by 1").unwrap();
        screen.apply_command(&command);
        let actual = to_string(&screen);
        let expected = "#.#....\n\
                        ###....\n\
                        .#.....\n";
        assert_eq!(expected, actual);

        let command = Command::parse("rotate row y=0 by 4").unwrap();
        screen.apply_command(&command);
        let actual = to_string(&screen);
        let expected = "....#.#\n\
                        ###....\n\
                        .#.....\n";
        assert_eq!(expected, actual);

        let command = Command::parse("rotate column x=1 by 1").unwrap();
        screen.apply_command(&command);
        let actual = to_string(&screen);
        let expected = ".#..#.#\n\
                        #.#....\n\
                        .#.....\n";
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(119, Day08.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {}

    #[test]
    fn test_real_part2() {
        // assert_eq!(242, Day08.part2(Example::Real, Debug::NotDebug));
    }

    fn to_string(screen: &Screen) -> String {
        screen
            .0
            .rows_iter()
            .flat_map(|row| {
                row.map(|cell| if *cell == Cell::On { '#' } else { '.' })
                    .chain(Some('\n'))
            })
            .collect()
    }
}
