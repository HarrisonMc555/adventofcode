use lazy_static::lazy_static;
use regex::Regex;

use crate::days::{Day, Debug, Example, Part};

pub struct Day03;

impl Day for Day03 {
    fn number(&self) -> u32 {
        3
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day03 {
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        let triangles = parse_triangles(&self.read_file(example)).unwrap();
        triangles.into_iter().filter(Triangle::is_possible).count()
    }

    fn part2(&self, _example: Example, _debug: Debug) -> usize {
        unimplemented!()
    }
}

fn parse_triangles(text: &str) -> Option<Vec<Triangle>> {
    text.trim().lines().map(Triangle::parse).collect()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Triangle {
    a: usize,
    b: usize,
    c: usize,
}

impl Triangle {
    fn is_possible(&self) -> bool {
        let (a, b, c) = (self.a, self.b, self.c);
        a + b > c && a + c > b && b + c > a
    }

    fn parse(line: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\s*(\d+)\s+(\d+)\s+(\d+)\s*$").unwrap();
        }
        let caps = RE.captures(line)?;
        let parse = |group| {
            caps.get(group).unwrap().as_str().parse().ok()
        };
        let a = parse(1)?;
        let b = parse(2)?;
        let c = parse(3)?;
        Some(Triangle { a, b, c })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples_part1() {
        let text = include_str!("../../static/example03.txt");
        let triangle = parse_triangles(text).unwrap()[0];
        assert!(!triangle.is_possible());
    }

    #[test]
    fn test_examples_part2() {}
}
