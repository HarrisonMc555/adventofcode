use crate::days::{Day, Debug, Example, Part};
use array2d::Array2D;

pub struct Day03;

impl Day for Day03 {
    fn number(&self) -> u32 {
        3
    }

    fn part1(&self, example: Example, _debug: Debug) -> String {
        let lines = Day03.get_lines(Part::Part1, example);
        let grid = parse_grid(&lines).unwrap();
        get_part_numbers(&grid).into_iter().sum::<u32>().to_string()
    }

    fn part2(&self, _example: Example, _debug: Debug) -> String {
        todo!()
    }
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Cell {
    Blank,
    Digit(u32),
    Symbol(char),
}

const BASE: u32 = 10;

type Grid = Array2D<Cell>;

fn get_part_numbers(grid: &Grid) -> Vec<u32> {
    let mut part_numbers = Vec::new();
    for (row, cells) in grid.rows_iter().enumerate() {
        let mut iter = cells.enumerate();
        while let Some((column, cell)) = iter.next() {
            if let Some(digit) = cell.digit() {
                let mut is_next_to_symbol = has_neighboring_symbol(grid, (row, column));
                let mut number = digit;
                for (column, cell) in iter.by_ref() {
                    let Some(digit) = cell.digit() else { break };
                    number *= BASE;
                    number += digit;
                    is_next_to_symbol =
                        is_next_to_symbol || has_neighboring_symbol(grid, (row, column));
                }
                if is_next_to_symbol {
                    part_numbers.push(number);
                }
            }
        }
    }
    part_numbers
}

impl Cell {
    fn digit(self) -> Option<u32> {
        match self {
            Self::Digit(digit) => Some(digit),
            _ => None,
        }
    }

    fn is_symbol(self) -> bool {
        matches!(self, Self::Symbol(_))
    }
}

fn has_neighboring_symbol(grid: &Grid, (row, column): (usize, usize)) -> bool {
    let prev_row = row.checked_sub(1);
    let next_row = if row + 1 >= grid.num_rows() {
        None
    } else {
        Some(row + 1)
    };
    let prev_column = column.checked_sub(1);
    let next_column = if column + 1 >= grid.num_columns() {
        None
    } else {
        Some(column + 1)
    };
    let rows = [prev_row, Some(row), next_row]
        .iter()
        .filter_map(|r| *r)
        .collect::<Vec<_>>();
    let columns = [prev_column, Some(column), next_column]
        .iter()
        .filter_map(|r| *r)
        .collect::<Vec<_>>();
    for row in rows {
        for &column in columns.iter() {
            if let Some(cell) = grid.get(row, column) {
                if cell.is_symbol() {
                    return true;
                }
            }
        }
    }
    false
}

fn parse_grid<T: AsRef<str>>(lines: &[T]) -> Option<Grid> {
    let num_rows = lines.len();
    let num_columns = lines.first()?.as_ref().len();
    let iter = lines
        .iter()
        .flat_map(|line| line.as_ref().chars())
        .map(Cell::from);
    Array2D::from_iter_row_major(iter, num_rows, num_columns).ok()
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        if value == '.' {
            return Self::Blank;
        }
        if let Some(digit) = value.to_digit(BASE) {
            return Self::Digit(digit);
        }
        Self::Symbol(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Part;

    #[test]
    fn test_parse() {
        let lines = Day03.get_lines(Part::Part1, Example::Example);
        assert!(parse_grid(&lines).is_some());
    }

    #[test]
    fn test_examples_part1() {
        assert_eq!("4361", Day03.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!("553825", Day03.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        // assert_eq!("0", Day03.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        // assert_eq!("0", Day03.part2(Example::Real, Debug::NotDebug));
    }
}
