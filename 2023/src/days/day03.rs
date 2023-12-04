use crate::days::{Day, Debug, Example, Part};
use array2d::Array2D;
use std::collections::VecDeque;

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

    fn part2(&self, example: Example, _debug: Debug) -> String {
        let lines = Day03.get_lines(Part::Part1, example);
        let grid = parse_grid(&lines).unwrap();
        get_gears(&grid)
            .map(|(num1, num2)| num1 * num2)
            .sum::<u32>()
            .to_string()
    }
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Cell {
    Blank,
    Digit(u32),
    Symbol(char),
}

const BASE: u32 = 10;
const GEAR_CHAR: char = '*';
const PART_NUMBERS_PER_GEAR: usize = 2;

type Grid = Array2D<Cell>;

fn get_part_numbers(grid: &Grid) -> Vec<u32> {
    let mut part_numbers = Vec::new();
    for (row, cells) in grid.rows_iter().enumerate() {
        let mut iter = cells.enumerate();
        while let Some((column, cell)) = iter.next() {
            let Some(digit) = cell.digit() else {
                continue;
            };
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
    part_numbers
}

impl Cell {
    fn digit(self) -> Option<u32> {
        match self {
            Self::Digit(digit) => Some(digit),
            _ => None,
        }
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
            let Some(cell) = grid.get(row, column) else {
                continue;
            };
            if matches!(cell, Cell::Symbol(_)) {
                return true;
            }
        }
    }
    false
}

fn get_gears(grid: &Grid) -> impl Iterator<Item = (u32, u32)> + '_ {
    grid.enumerate_row_major()
        .filter_map(|(index, cell)| get_gear(grid, cell, index))
}

fn get_gear(grid: &Grid, cell: &Cell, (row, column): (usize, usize)) -> Option<(u32, u32)> {
    let Cell::Symbol(GEAR_CHAR) = cell else {
        return None;
    };
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
    let mut part_numbers = Vec::with_capacity(PART_NUMBERS_PER_GEAR);
    let mut part_number_addresses = Vec::with_capacity(PART_NUMBERS_PER_GEAR);
    for row in rows {
        for &column in columns.iter() {
            let Some((address, part_number)) = get_part_number(grid, (row, column)) else {
                continue;
            };
            if part_number_addresses.contains(&address) {
                continue;
            }
            if part_numbers.len() >= PART_NUMBERS_PER_GEAR {
                return None;
            }
            part_numbers.push(part_number);
            part_number_addresses.push(address);
        }
    }
    match part_numbers[..] {
        [num1, num2] => Some((num1, num2)),
        _ => None,
    }
}

fn get_part_number(grid: &Grid, (row, column): (usize, usize)) -> Option<((usize, usize), u32)> {
    let Some(Cell::Digit(digit)) = grid.get(row, column) else {
        return None;
    };
    let mut digits = VecDeque::new();
    digits.push_back(digit);
    let mut first_column = column;
    for next_column in (0..column).rev() {
        let Some(Cell::Digit(next_digit)) = grid.get(row, next_column) else {
            break;
        };
        digits.push_front(next_digit);
        first_column = next_column;
    }
    for next_column in (column + 1).. {
        let Some(Cell::Digit(next_digit)) = grid.get(row, next_column) else {
            break;
        };
        digits.push_back(next_digit);
    }
    let part_number = digits.iter().fold(0, |sum, &digit| sum * BASE + digit);
    Some(((row, first_column), part_number))
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
        assert_eq!("467835", Day03.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        // assert_eq!("0", Day03.part2(Example::Real, Debug::NotDebug));
    }
}
