use lazy_static::lazy_static;
use regex::Regex;

use crate::days::{Day, Debug, Example, Part};

pub struct Day04;

impl Day for Day04 {
    fn number(&self) -> u32 {
        4
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day04 {
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        self.get_lines(example)
            .into_iter()
            .map(parse_line)
            .map(Option::unwrap)
            .filter(|pair| one_fully_contains_other(*pair))
            .count()
    }

    fn part2(&self, example: Example, _debug: Debug) -> usize {
        self.get_lines(example)
            .into_iter()
            .map(parse_line)
            .map(Option::unwrap)
            .filter(|pair| overlaps(*pair))
            .count()
    }
}

type AssignmentPair = (Assignment, Assignment);
type Assignment = (u32, u32);

fn parse_line<T: AsRef<str>>(line: T) -> Option<AssignmentPair> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    }
    let caps = RE.captures(line.as_ref()).unwrap();
    Some((
        (
            caps.get(1).unwrap().as_str().parse().ok()?,
            caps.get(2).unwrap().as_str().parse().ok()?,
        ),
        (
            caps.get(3).unwrap().as_str().parse().ok()?,
            caps.get(4).unwrap().as_str().parse().ok()?,
        ),
    ))
}

fn one_fully_contains_other((a1, a2): AssignmentPair) -> bool {
    fully_contain(a1, a2) || fully_contain(a2, a1)
}

fn fully_contain(a1: Assignment, a2: Assignment) -> bool {
    let (a1_start, a1_end) = a1;
    let (a2_start, a2_end) = a2;
    a1_start <= a2_start && a1_end >= a2_end
}

fn overlaps((a1, a2): AssignmentPair) -> bool {
    let (a1_start, a1_end) = a1;
    let (a2_start, a2_end) = a2;
    contains(a1, a2_start) || contains(a1, a2_end) || contains(a2, a1_start) || contains(a2, a1_end)
}

fn contains((x1, x2): Assignment, y: u32) -> bool {
    x1 <= y && y <= x2
}
