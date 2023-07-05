use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

use crate::days::{Day, Debug, Example, Part};

const DEBUG: bool = false;

pub struct Day20;

impl Day for Day20 {
    fn number(&self) -> u32 {
        20
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day20 {
    fn part1(&self, example: Example, _debug: Debug) -> u32 {
        let ranges = Range::parse_ranges(&self.read_file(example)).unwrap();
        first_uncovered_value(ranges)
    }

    fn part2(&self, example: Example, _debug: Debug) -> u32 {
        let ranges = Range::parse_ranges(&self.read_file(example)).unwrap();
        count_uncovered_values(ranges)
    }
}

fn first_uncovered_value(ranges: Vec<Range>) -> u32 {
    let sorted = sort_and_merge_ranges(ranges);
    let first_value = 0;
    let Some(first_range) = sorted.first() else {
        return first_value;
    };
    if !first_range.contains(first_value) {
        return first_value;
    }
    first_range.max + 1
}

fn count_uncovered_values(ranges: Vec<Range>) -> u32 {
    let sorted = sort_and_merge_ranges(ranges);
    let (Some(first), Some(last)) = (sorted.first(), sorted.last()) else {
        return 0; // overflow, theoretically should be u32::MAX + 1
    };
    let num_uncovered_values_between: u32 = sorted
        .iter()
        .tuple_windows()
        .map(|(range1, range2)| range2.min - range1.max - 1)
        .sum();
    first.min + (u32::MAX - last.max) + num_uncovered_values_between
}

fn sort_and_merge_ranges(ranges: Vec<Range>) -> Vec<Range> {
    let mut result = Vec::new();
    for range in ranges {
        result = insert_into_sorted(result, range);
    }
    result
}

fn insert_into_sorted(ranges: Vec<Range>, mut new_range: Range) -> Vec<Range> {
    let mut result = Vec::new();
    let mut range_iter = ranges.into_iter();
    while let Some(range) = range_iter.next() {
        match range.compare_or_merge(new_range) {
            RangeCmpResult::Less => result.push(range),
            RangeCmpResult::Greater => {
                result.push(new_range);
                result.push(range);
                result.extend(range_iter);
                return result;
            }
            RangeCmpResult::Merged(merged) => new_range = merged,
        }
    }
    result.push(new_range);
    result
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Range {
    min: u32,
    max: u32,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum RangeCmpResult {
    Less,
    Greater,
    Merged(Range),
}

impl Range {
    pub fn new(min: u32, max: u32) -> Option<Self> {
        if max < min {
            return None;
        }
        Some(Self {
            min: min.min(max),
            max: min.max(max),
        })
    }

    pub fn compare_or_merge(self, other: Self) -> RangeCmpResult {
        if self.max.saturating_add(1) < other.min {
            RangeCmpResult::Less
        } else if self.min > other.max.saturating_add(1) {
            RangeCmpResult::Greater
        } else {
            RangeCmpResult::Merged(Self {
                min: self.min.min(other.min),
                max: self.max.max(other.max),
            })
        }
    }

    pub fn contains(self, value: u32) -> bool {
        self.min <= value && value <= self.max
    }

    pub fn parse_ranges(string: &str) -> Option<Vec<Self>> {
        string.lines().map(|line| line.parse().ok()).collect()
    }
}

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)-(\d+)").unwrap();
        }
        let caps = RE
            .captures(s)
            .ok_or_else(|| format!("Invalid range \"{}\"", s))?;
        let parse = |group| {
            let num_str = caps.get(group).unwrap().as_str();
            num_str.parse::<u32>().map_err(|e| e.to_string())
        };
        let min = parse(1)?;
        let max = parse(2)?;
        Self::new(min, max).ok_or_else(|| s.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort_and_merge_ranges() {
        let ranges = create_ranges(&[(5, 8), (0, 2), (4, 7)]);
        let actual = sort_and_merge_ranges(ranges);
        let expected = create_ranges(&[(0, 2), (4, 8)]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_first_uncovered_value() {
        let ranges = create_ranges(&[(5, 8), (0, 2), (4, 7)]);
        let actual = first_uncovered_value(ranges);
        let expected = 3;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse() {
        let actual = Range::parse_ranges("5-8\n0-2\n4-7");
        let expected = create_ranges(&[(5, 8), (0, 2), (4, 7)]);
        assert_eq!(Some(expected), actual);
    }

    fn create_ranges(pairs: &[(u32, u32)]) -> Vec<Range> {
        pairs
            .into_iter()
            .copied()
            .map(|(min, max)| Range::new(min, max).unwrap())
            .collect()
    }
}
