use std::str::FromStr;

use crate::days::{Day, Debug, Example, Part};

pub struct Day06;

impl Day for Day06 {
    fn number(&self) -> u32 {
        6
    }

    fn part1(&self, example: Example, _debug: Debug) -> String {
        let text = self.read_file(Part::Part1, example);
        let worksheet = Worksheet::from_str(&text).unwrap();
        let values = worksheet.compute_columns().unwrap();
        let sum = values.into_iter().sum::<i64>();
        sum.to_string()
    }

    fn part2(&self, example: Example, _debug: Debug) -> String {
        let text = self.read_file(Part::Part2, example);
        let worksheet = Worksheet2::from_str(&text).unwrap();
        let values = worksheet.compute_columns().unwrap();
        let sum = values.into_iter().sum::<i64>();
        sum.to_string()
    }
}

const BASE: u32 = 10;

#[derive(Debug, Clone)]
struct Worksheet {
    number_grid: Vec<Vec<i64>>,
    operations: Vec<Operation>,
}

#[derive(Debug, Clone)]
struct Worksheet2 {
    number_columns: Vec<Vec<i64>>,
    operations: Vec<Operation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Operation {
    Add,
    Multiply,
}

impl Worksheet {
    pub fn compute_columns(&self) -> Result<Vec<i64>, String> {
        (0..self.operations.len())
            .map(|index| self.compute_column(index))
            .collect()
    }

    pub fn compute_column(&self, column_index: usize) -> Result<i64, String> {
        let column_numbers = self
            .number_grid
            .iter()
            .enumerate()
            .map(|(row_index, row)| {
                row.get(column_index)
                    .copied()
                    .ok_or_else(|| format!("Row {row_index} only has {} numbers", row.len()))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let operation = self
            .operations
            .get(column_index)
            .ok_or_else(|| format!("Only {} operations", self.operations.len()))?;
        let value = operation.compute(column_numbers.into_iter());
        Ok(value)
    }
}

impl Worksheet2 {
    pub fn compute_columns(&self) -> Result<Vec<i64>, String> {
        (0..self.operations.len())
            .map(|index| self.compute_column(index))
            .collect()
    }

    pub fn compute_column(&self, column_index: usize) -> Result<i64, String> {
        let Some(column) = self.number_columns.get(column_index) else {
            return Err(format!("Column index out of range: {column_index}"));
        };
        let Some(operation) = self.operations.get(column_index) else {
            return Err(format!(
                "Column index out of operations range: {column_index}"
            ));
        };
        let mut value = operation.identity();
        let accumulate = operation.accumulate();
        for number in column.iter().copied() {
            value = accumulate(value, number)
        }
        Ok(value)
    }
}

impl Operation {
    pub fn from_char(c: char) -> Result<Self, char> {
        Ok(match c {
            '+' => Self::Add,
            '*' => Self::Multiply,
            _ => return Err(c),
        })
    }

    pub fn compute<I>(self, numbers: I) -> i64
    where
        I: IntoIterator<Item = i64>,
    {
        let mut value = self.identity();
        let accumulate = self.accumulate();
        for number in numbers {
            value = accumulate(value, number);
        }
        value
    }

    pub fn identity(self) -> i64 {
        match self {
            Self::Add => 0,
            Self::Multiply => 1,
        }
    }

    pub fn accumulate(self) -> fn(i64, i64) -> i64 {
        match self {
            Self::Add => std::ops::Add::add,
            Self::Multiply => std::ops::Mul::mul,
        }
    }
}

impl FromStr for Worksheet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        let Some((operations_line, number_lines)) = lines.split_last() else {
            return Err("Not enough lines".to_string());
        };
        let number_grid = parse_all_numbers_lines(number_lines.into_iter().copied())?;
        let operations = parse_operations_line(&operations_line)?;
        Ok(Self {
            number_grid,
            operations,
        })
    }
}

impl FromStr for Worksheet2 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let Some((operations_line, number_lines)) = lines.split_last() else {
            return Err("Not enough lines".to_string());
        };
        let operations_and_indices = operations_line
            .iter()
            .enumerate()
            .filter_map(|(i, c)| Operation::from_char(*c).ok().map(|op| (i, op)))
            .collect::<Vec<_>>();
        let operations = operations_and_indices
            .iter()
            .map(|(_, op)| *op)
            .collect::<Vec<_>>();
        let operation_indices = operations_and_indices
            .iter()
            .map(|(i, _)| *i)
            .collect::<Vec<_>>();
        let column_start_and_end_indices = operation_indices
            .iter()
            .enumerate()
            .map(|(i, ci)| {
                let next_ci = operation_indices
                    .get(i + 1)
                    .copied()
                    .map(|next_ci| next_ci - 1)
                    .unwrap_or(operations_line.len());
                (*ci, next_ci)
            })
            .collect::<Vec<_>>();
        let number_columns =
            parse_all_numbers_columns(&number_lines, column_start_and_end_indices)?;
        Ok(Self {
            number_columns,
            operations,
        })
    }
}

fn parse_all_numbers_lines<'a, I>(lines: I) -> Result<Vec<Vec<i64>>, String>
where
    I: IntoIterator<Item = &'a str>,
{
    lines
        .into_iter()
        .map(|line| parse_numbers_line(line))
        .collect::<Result<Vec<_>, _>>()
}

fn parse_all_numbers_columns(
    lines: &[Vec<char>],
    column_start_and_end_indices: Vec<(usize, usize)>,
) -> Result<Vec<Vec<i64>>, String> {
    let mut columns = Vec::new();
    for (start_index, end_index) in column_start_and_end_indices.iter().copied() {
        let column = parse_column(lines, start_index, end_index)?;
        columns.push(column);
    }
    Ok(columns)
}

fn parse_column(
    lines: &[Vec<char>],
    start_index: usize,
    end_index: usize,
) -> Result<Vec<i64>, String> {
    (start_index..end_index)
        .map(|char_index| parse_number(lines, char_index))
        .collect()
}

fn parse_number(lines: &[Vec<char>], char_index: usize) -> Result<i64, String> {
    let mut value = 0i64;
    for line in lines {
        let Some(digit) = line.get(char_index) else {
            return Err(format!("Invalid index {char_index}"));
        };
        if let Some(digit) = digit.to_digit(BASE) {
            value *= i64::from(BASE);
            value += i64::from(digit);
        }
    }
    Ok(value)
}

fn parse_numbers_line(line: &str) -> Result<Vec<i64>, String> {
    line.split_ascii_whitespace()
        .map(|word| word.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())
}

fn parse_operations_line(line: &str) -> Result<Vec<Operation>, String> {
    line.split_ascii_whitespace()
        .map(|word| word.parse::<Operation>())
        .collect::<Result<Vec<_>, _>>()
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Self::Add,
            "*" => Self::Multiply,
            _ => return Err(format!("Invalid operation {s}")),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples_part1() {
        assert_eq!("4277556", Day06.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!("4387670995909", Day06.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        assert_eq!("3263827", Day06.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        assert_eq!("9625320374409", Day06.part2(Example::Real, Debug::NotDebug));
    }
}
