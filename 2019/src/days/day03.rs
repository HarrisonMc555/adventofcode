use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;
use std::iter::Rev;
use std::ops::{Add, RangeInclusive};

const CENTER: Position = Position { x: 0, y: 0 };

type Result<T> = std::result::Result<T, ()>;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Segment {
    pub direction: Direction,
    pub length: isize,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Position {
    pub x: isize,
    pub y: isize,
}

type Segments = Vec<Segment>;
type Positions = HashSet<Position>;
type Distances = HashMap<Position, usize>;

// const INPUT: &str = "R8,U5,L5,D3\nU7,R6,D4,L4";
// const INPUT: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
// const INPUT: &str =
//     "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
const INPUT: &str = include_str!("../../static/day03.txt");

pub fn main() {
    let answer1 = solve1(INPUT).unwrap();
    println!("{}", answer1);
    let answer2 = solve2(INPUT).unwrap();
    println!("{}", answer2);
}

fn solve1(input: &str) -> Result<isize> {
    let (segments1, segments2) = parse_input(input)?;
    let positions1 = find_traversed_positions(segments1);
    let positions2 = find_traversed_positions(segments2);
    let common_positions = &positions1 & &positions2;
    let mut cp = common_positions.iter().collect::<Vec<_>>();
    cp.sort_by_key(|pos| pos.manhattan_distance(CENTER));
    find_closest_distance(&common_positions, CENTER).ok_or(())
}

fn solve2(input: &str) -> Result<usize> {
    let (segments1, segments2) = parse_input(input)?;
    let distances1 = find_distances(segments1);
    let distances2 = find_distances(segments2);
    find_fewest_steps(&distances1, &distances2).ok_or(())
}

fn find_closest_distance(positions: &Positions, target: Position) -> Option<isize> {
    positions
        .iter()
        .map(|pos| pos.manhattan_distance(target))
        .min()
}

fn find_fewest_steps(distances1: &Distances, distances2: &Distances) -> Option<usize> {
    let common_positions = distances1.keys().filter(|k| distances2.contains_key(k));
    common_positions
        .filter_map(|k| {
            let distance1 = distances1.get(k)?;
            let distance2 = distances2.get(k)?;
            Some(distance1 + distance2)
        })
        .min()
        // Distances don't include either starting position
        .map(|n| n + 2)
}

fn find_traversed_positions(segments: Segments) -> Positions {
    let mut pos = Position::new(0, 0);
    let mut positions = HashSet::new();
    for segment in segments {
        for intermediate_pos in pos.intermediate_positions(&segment) {
            positions.insert(intermediate_pos);
        }
        pos = pos + segment;
    }
    positions
}

fn find_distances(segments: Segments) -> Distances {
    let mut pos = Position::new(0, 0);
    let mut distances = HashMap::new();
    let mut index = 0;
    for segment in segments {
        for intermediate_pos in pos.intermediate_positions(&segment) {
            distances.insert(intermediate_pos, index);
            index += 1;
        }
        pos = pos + segment;
    }
    distances
}

fn parse_input(input: &str) -> Result<(Segments, Segments)> {
    let mut all_segments = input
        .lines()
        .map(|line| line.trim().split(',').map(str::parse).collect())
        .collect::<Result<Vec<_>>>()?;
    let second = all_segments.pop().ok_or(())?;
    let first = all_segments.pop().ok_or(())?;
    if !all_segments.is_empty() {
        return Err(());
    }
    Ok((first, second))
}

impl std::str::FromStr for Segment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self> {
        let mut iter = s.chars();
        let direction_char = iter.next().ok_or(())?;
        let direction = match direction_char {
            'U' => Direction::Up,
            'R' => Direction::Right,
            'D' => Direction::Down,
            'L' => Direction::Left,
            _ => return Err(()),
        };
        let length = iter.collect::<String>().parse().map_err(|_| ())?;
        Ok(Segment::new(direction, length))
    }
}

impl From<(isize, isize)> for Position {
    fn from((x, y): (isize, isize)) -> Self {
        Position::new(x, y)
    }
}

impl Add<Segment> for Position {
    type Output = Self;

    fn add(self, segment: Segment) -> Self {
        let (x, y) = self.tuple().clone();
        let len = segment.length;
        let (dx, dy) = match segment.direction {
            Direction::Up => (0, len),
            Direction::Right => (len, 0),
            Direction::Down => (0, -len),
            Direction::Left => (-len, 0),
        };
        Position::new(x + dx, y + dy)
    }
}

impl Segment {
    fn new(direction: Direction, length: isize) -> Self {
        Segment { direction, length }
    }
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Position { x, y }
    }

    fn tuple(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    fn manhattan_distance(&self, other: Self) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn intermediate_positions(self, segment: &Segment) -> impl Iterator<Item = Self> + '_ {
        let (x, y) = self.tuple().clone();
        let len = segment.length;
        let (beg, end) = match segment.direction {
            Direction::Up => (y + 1, y + len),
            Direction::Right => (x + 1, x + len),
            Direction::Down => (y - 1, y - len),
            Direction::Left => (x - 1, x - len),
        };
        let range = EitherRangeInclusive::new(beg, end);
        range.map(move |num| match segment.direction {
            Direction::Up | Direction::Down => Position::new(x, num),
            Direction::Left | Direction::Right => Position::new(num, y),
        })
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.direction, self.length)
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Direction::Up => "U",
            Direction::Right => "R",
            Direction::Down => "D",
            Direction::Left => "L",
        };
        write!(f, "{}", s)
    }
}

enum EitherRangeInclusive {
    Up(RangeInclusive<isize>),
    Down(Rev<RangeInclusive<isize>>),
}

impl Iterator for EitherRangeInclusive {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            EitherRangeInclusive::Up(iter) => iter.next(),
            EitherRangeInclusive::Down(iter) => iter.next(),
        }
    }
}

impl EitherRangeInclusive {
    fn new(start: isize, end: isize) -> Self {
        if start <= end {
            EitherRangeInclusive::Up(start..=end)
        } else {
            let new_start = end;
            let new_end = start;
            EitherRangeInclusive::Down((new_start..=new_end).rev())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn either_range_up() {
        let mut iter = EitherRangeInclusive::new(0, 3);
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn either_range_down() {
        let mut iter = EitherRangeInclusive::new(3, 0);
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn either_range_up_negative() {
        let mut iter = EitherRangeInclusive::new(-2, 2);
        assert_eq!(iter.next(), Some(-2));
        assert_eq!(iter.next(), Some(-1));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn either_range_down_negative() {
        let mut iter = EitherRangeInclusive::new(1, -3);
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(-1));
        assert_eq!(iter.next(), Some(-2));
        assert_eq!(iter.next(), Some(-3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn answer01a() {
        assert_eq!(solve1(INPUT), Ok(221));
    }

    #[test]
    fn answer01b() {
        assert_eq!(solve2(INPUT), Ok(18542));
    }
}
