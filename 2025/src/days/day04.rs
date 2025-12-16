use std::collections::HashSet;

use array2d::Array2D;

use crate::days::{Day, Debug, Example, Part};

pub struct Day04;

impl Day for Day04 {
    fn number(&self) -> u32 {
        4
    }

    fn part1(&self, example: Example, _debug: Debug) -> String {
        let lines = self.get_lines(Part::Part1, example);
        let lines = lines
            .into_iter()
            .map(|line| parse_line(&line).unwrap())
            .collect::<Vec<_>>();
        let grid = Array2D::from_rows(&lines).unwrap();
        let num_accessible = num_accessible(&grid);
        num_accessible.to_string()
    }

    fn part2(&self, example: Example, _debug: Debug) -> String {
        let lines = self.get_lines(Part::Part1, example);
        let lines = lines
            .into_iter()
            .map(|line| parse_line(&line).unwrap())
            .collect::<Vec<_>>();
        let mut grid = Array2D::from_rows(&lines).unwrap();
        let num_filled_before = grid
            .elements_row_major_iter()
            .filter(|cell| matches!(cell, Cell::Filled))
            .count();
        remove_accessible_repeatedly(&mut grid);
        let num_filled_after = grid
            .elements_row_major_iter()
            .filter(|cell| matches!(cell, Cell::Filled))
            .count();
        let num_filled_removed = num_filled_before - num_filled_after;
        num_filled_removed.to_string()
    }
}

const MAX_NUM_ADJACENT: usize = 4;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Cell {
    Empty,
    Filled,
}

const OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn num_accessible(grid: &Array2D<Cell>) -> usize {
    let mut num_accessible = 0;
    for row in 0..grid.num_rows() {
        for column in 0..grid.num_columns() {
            if grid.get(row, column) != Some(&Cell::Filled) {
                continue;
            }
            let num_adjacent_filled = num_adjacent_filled(grid, row, column);
            if num_adjacent_filled < MAX_NUM_ADJACENT {
                num_accessible += 1;
            }
        }
    }
    num_accessible
}

fn remove_accessible_repeatedly(grid: &mut Array2D<Cell>) {
    let mut dirty = (0..grid.num_rows())
        .flat_map(|row| (0..grid.num_columns()).map(move |column| (row, column)))
        .collect::<HashSet<_>>();
    loop {
        let Some(&index) = dirty.iter().next() else {
            return;
        };
        dirty.take(&index);
        let (row, column) = index;
        let cell = grid
            .get(row, column)
            .unwrap_or_else(|| panic!("Invalid dirty index {index:?}"));
        if !matches!(cell, Cell::Filled) {
            continue;
        }
        if num_adjacent_filled(grid, row, column) >= MAX_NUM_ADJACENT {
            continue;
        }
        grid.set(row, column, Cell::Empty)
            .unwrap_or_else(|e| panic!("Invalid dirty index {index:?}: {e:?}"));
        for (row_offset, column_offset) in OFFSETS {
            let Some(index) = apply_offset(grid, row, column, row_offset, column_offset) else {
                continue;
            };
            dirty.insert(index);
        }
    }
}

fn num_adjacent_filled(grid: &Array2D<Cell>, row: usize, column: usize) -> usize {
    let adjacent_cells = OFFSETS
        .into_iter()
        .filter_map(|(row_offset, column_offset)| {
            apply_offset(grid, row, column, row_offset, column_offset)
        })
        .filter_map(|(new_row, new_column)| grid.get(new_row, new_column));
    adjacent_cells
        .filter(|cell| matches!(cell, Cell::Filled))
        .count()
}

fn apply_offset<T>(
    grid: &Array2D<T>,
    row: usize,
    column: usize,
    row_offset: isize,
    column_offset: isize,
) -> Option<(usize, usize)> {
    let new_row = row as isize + row_offset;
    if new_row < 0 {
        return None;
    }
    let new_row = new_row as usize;
    if new_row >= grid.num_rows() {
        return None;
    }

    let new_column = column as isize + column_offset;
    if new_column < 0 {
        return None;
    }
    let new_column = new_column as usize;
    if new_column >= grid.num_columns() {
        return None;
    }

    Some((new_row as usize, new_column as usize))
}

fn parse_line(line: &str) -> Option<Vec<Cell>> {
    line.chars().map(Cell::from_char).collect::<Option<_>>()
}

impl Cell {
    pub fn from_char(c: char) -> Option<Self> {
        Some(match c {
            '.' => Self::Empty,
            '@' => Self::Filled,
            _ => return None,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples_part1() {
        assert_eq!("13", Day04.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!("1384", Day04.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        assert_eq!("43", Day04.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        assert_eq!("8013", Day04.part2(Example::Real, Debug::NotDebug));
    }
}
