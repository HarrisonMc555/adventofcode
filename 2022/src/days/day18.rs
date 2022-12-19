use std::collections::HashSet;

use crate::days::{Day, Debug, Example, Part};

pub struct Day18;

impl Day for Day18 {
    fn number(&self) -> u32 {
        18
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day18 {
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        let cubes = parse_cubes(&self.read_file(example)).unwrap();
        let cubes = cubes.into_iter().collect();
        total_exposed_surface_area(&cubes)
    }

    fn part2(&self, _example: Example, _debug: Debug) -> usize {
        todo!()
    }
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

fn total_exposed_surface_area(cubes: &HashSet<Cube>) -> usize {
    cubes
        .iter()
        .map(|cube| cube.exposed_surface_area(&cubes))
        .sum()
}

impl Cube {
    fn exposed_surface_area(&self, cubes: &HashSet<Cube>) -> usize {
        self.neighbor_cubes()
            .iter()
            .filter(|neighbor| !cubes.contains(&neighbor))
            .count()
    }

    fn neighbor_cubes(&self) -> [Cube; 6] {
        let Cube { x, y, z } = *self;
        [
            Cube { x: x + 1, y, z },
            Cube { x: x - 1, y, z },
            Cube { x, y: y + 1, z },
            Cube { x, y: y - 1, z },
            Cube { x, y, z: z + 1 },
            Cube { x, y, z: z - 1 },
        ]
    }
}

fn parse_cubes(text: &str) -> Option<Vec<Cube>> {
    text.trim().split('\n').map(Cube::parse).collect()
}

impl Cube {
    fn parse(text: &str) -> Option<Self> {
        let mut nums = text.split(',').map(|w| w.parse().ok());
        let x = nums.next()??;
        let y = nums.next()??;
        let z = nums.next()??;
        if nums.next().is_some() {
            return None;
        }
        Some(Cube { x, y, z })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let text = "2,2,2\n1,2,2\n3,2,2\n2,1,2";
        let cubes = parse_cubes(text).unwrap();
        assert_eq!(4, cubes.len());
        assert_eq!(Cube { x: 2, y: 2, z: 2 }, cubes[0]);
        assert_eq!(Cube { x: 1, y: 2, z: 2 }, cubes[1]);
        assert_eq!(Cube { x: 3, y: 2, z: 2 }, cubes[2]);
        assert_eq!(Cube { x: 2, y: 1, z: 2 }, cubes[3]);
    }

    #[test]
    fn test_examples_part1() {
        assert_eq!(64, Day18.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(3576, Day18.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        // assert_eq!(0, Day18.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        // assert_eq!(0, Day18.part2(Example::Real, Debug::NotDebug));
    }
}
