use std::fmt::Display;

use crate::days::{Day, Debug, Example, Part};

const DEBUG: bool = false;

const NUM_ROWS_PART1: usize = 40;
const NUM_ROWS_PART2: usize = 400000;

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
        let input = self.read_file(example);
        let input = input.trim();
        let line = parse_line(input).unwrap();
        itertools::iterate(line, |line| step(line))
            .take(NUM_ROWS_PART1)
            .map(|line| count_safe_tiles(&line))
            .sum()
    }

    fn part2(&self, example: Example, _debug: Debug) -> usize {
        let input = self.read_file(example);
        let input = input.trim();
        let line = parse_line(input).unwrap();
        itertools::iterate(line, |line| step(line))
            .take(NUM_ROWS_PART2)
            .map(|line| count_safe_tiles(&line))
            .sum()
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum Tile {
    Safe,
    Trap,
}

fn step(tiles: &[Tile]) -> Vec<Tile> {
    (0..tiles.len())
        .map(|index| next_tile(tiles, index))
        .collect()
}

fn next_tile(tiles: &[Tile], index: usize) -> Tile {
    let left = index
        .checked_sub(1)
        .and_then(|i| tiles.get(i).copied())
        .unwrap_or(Tile::Safe);
    let center = tiles.get(index).copied().unwrap_or(Tile::Safe);
    let right = tiles.get(index + 1).copied().unwrap_or(Tile::Safe);
    use Tile::*;
    match (left, center, right) {
        (Trap, Trap, Safe) => Trap,
        (Safe, Trap, Trap) => Trap,
        (Trap, Safe, Safe) => Trap,
        (Safe, Safe, Trap) => Trap,
        _ => Safe,
    }
}

impl Tile {
    pub fn from_char(c: char) -> Option<Tile> {
        Some(match c {
            '.' => Tile::Safe,
            '^' => Tile::Trap,
            _ => return None,
        })
    }
}

fn count_safe_tiles(line: &[Tile]) -> usize {
    line.iter().filter(|tile| **tile == Tile::Safe).count()
}

fn parse_line(line: &str) -> Option<Vec<Tile>> {
    line.chars().map(Tile::from_char).collect()
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Safe => '.',
            Tile::Trap => '^',
        };
        write!(f, "{}", c)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        use itertools::Itertools;

        let grid = "..^^.\n\
                    .^^^^\n\
                    ^^..^\n\
                    ";
        for (before, after) in parse_grid(grid).into_iter().tuples() {
            assert_eq!(after, step(&before));
        }

        let grid = ".^^.^.^^^^\n\
                    ^^^...^..^\n\
                    ^.^^.^.^^.\n\
                    ..^^...^^^\n\
                    .^^^^.^^.^\n\
                    ^^..^.^^..\n\
                    ^^^^..^^^.\n\
                    ^..^^^^.^^\n\
                    .^^^..^.^^\n\
                    ^^.^^^..^^\n\
                    ";

        for (before, after) in parse_grid(grid).into_iter().tuples() {
            let actual = step(&before);
            assert_eq!(
                after, actual,
                "Before {:?} should become {:?} but became {:?}",
                before, after, actual
            );
        }
    }

    fn parse_grid(grid: &str) -> Vec<Vec<Tile>> {
        grid.lines().map(|line| parse_line(line).unwrap()).collect()
    }
}
