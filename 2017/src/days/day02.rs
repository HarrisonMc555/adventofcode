use std::num::ParseIntError;

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
    fn part1(&self, example: Example, _debug: Debug) -> u32 {
        let rows = parse_lines(&self.read_file(example)).unwrap();
        calc_checksum(&rows)
    }

    fn part2(&self, example: Example, _debug: Debug) -> u32 {
        let rows = parse_lines(&self.read_file(example)).unwrap();
        get_even_division_sum(&rows).unwrap()
    }
}

fn calc_checksum(rows: &[Vec<u32>]) -> u32 {
    rows.iter().filter_map(|row| calc_min_max_difference(row)).sum()
}

fn calc_min_max_difference(row: &[u32]) -> Option<u32> {
    let (min, max) = row.iter().copied().min_max()?;
    Some(max - min)
}

trait MinMax: Iterator + Sized {
    fn min_max(mut self) -> Option<(Self::Item, Self::Item)>
    where
        Self::Item: Ord + Clone,
    {
        let first = self.next()?;
        let mut min = first.clone();
        let mut max = first;
        for value in self {
            if value < min {
                min = value;
            } else if value > max {
                max = value;
            }
        }
        Some((min, max))
    }
}

impl<T: Iterator> MinMax for T {}

fn get_even_division_sum(rows: &[Vec<u32>]) -> Option<u32> {
    rows.iter().map(|row| find_even_division(row)).sum()
}

fn find_even_division(row: &[u32]) -> Option<u32> {
    for (index, number1) in row.iter().copied().enumerate() {
        if index + 1 >= row.len() {
            break;
        }
        for number2 in &row[index + 1..] {
            if number1 % number2 == 0 {
                return Some(number1 / number2);
            }
            if number2 % number1 == 0 {
                return Some(number2 / number1);
            }
        }
    }
    None
}

fn parse_lines(text: &str) -> Result<Vec<Vec<u32>>, ParseIntError> {
    text.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Result<Vec<u32>, ParseIntError> {
    line.split_ascii_whitespace()
        .map(|word| word.parse())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_max() {
        let numbers = [7, 2, 3, 8, 4];
        let (min, max) = numbers.iter().copied().min_max().unwrap();
        assert_eq!(2, min);
        assert_eq!(8, max);

        let numbers: [u32; 0] = [];
        assert!(numbers.iter().min_max().is_none());
    }

    #[test]
    fn test_examples_part1() {
        let text = "5 1 9 5\n\
                    7 5 3\n\
                    2 4 6 8";
        let rows = parse_lines(text).unwrap();
        let checksum = calc_checksum(&rows);
        assert_eq!(18, checksum);
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(50376, Day02.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        let text = "5 9 2 8\n\
                    9 4 7 3\n\
                    3 8 6 5";
        let rows = parse_lines(text).unwrap();
        let sum = get_even_division_sum(&rows).unwrap();
        assert_eq!(9, sum);
    }

    #[test]
    fn test_real_part2() {
        assert_eq!(267, Day02.part2(Example::Real, Debug::NotDebug));
    }
}
