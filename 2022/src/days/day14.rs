use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

use itertools::Itertools;

use crate::days::{Day, Debug, Example, Part};
use crate::{debug_print, debug_println};

const DEBUG: bool = false;

// This would probably be much faster if we used an array instead of a hash map.

pub struct Day14;

impl Day for Day14 {
    fn number(&self) -> u32 {
        14
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day14 {
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        let lines = parse_lines(&self.read_file(example)).unwrap();
        let mut grid = Grid::new(&lines, LowerExistence::Abyss).unwrap();
        grid.simulate_falling_sand(SAND_SOURCE);
        grid.count_sand()
    }

    fn part2(&self, example: Example, _debug: Debug) -> usize {
        let lines = parse_lines(&self.read_file(example)).unwrap();
        let mut grid = Grid::new(&lines, LowerExistence::Floor).unwrap();
        grid.simulate_falling_sand(SAND_SOURCE);
        grid.count_sand()
    }
}

const SAND_SOURCE: Point = Point { x: 500, y: 0 };
const FLOOR_DIST_FROM_MAX_Y: Index = 2;

#[derive(Debug, Eq, PartialEq)]
struct Line(Vec<Point>);

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct Point {
    x: Index,
    y: Index,
}

type Index = isize;

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Cell {
    Rock,
    Sand,
}

#[derive(Debug, Eq, PartialEq)]
struct Grid {
    cells: HashMap<Point, Cell>,
    bounds: Bounds,
    lower_existence: LowerExistence,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct Bounds {
    min_x: Index,
    max_x: Index,
    min_y: Index,
    max_y: Index,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
enum SimulationResult {
    Settled,
    FallIntoAbyss,
    BlockingSource,
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum LowerExistence {
    Abyss,
    Floor,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
enum SimulationResult2 {
    Settled { next_starting_point: Option<Point> },
    BlockingSource,
}

impl Grid {
    fn new(lines: &[Line], lower_existence: LowerExistence) -> Option<Self> {
        let Some(first) = lines.first().and_then(|line| line.0.first()) else {
            return None;
        };

        let mut cells = HashMap::new();
        for line in lines {
            for point in line.get_all_points()? {
                cells.insert(point, Cell::Rock);
            }
        }

        let edge_points = || lines.iter().flat_map(|line| line.0.iter());
        let min_x = edge_points().map(|point| point.x).min().unwrap_or(first.x);
        let max_x = edge_points().map(|point| point.x).max().unwrap_or(first.x);
        let min_y = edge_points().map(|point| point.y).min().unwrap_or(first.y);
        let max_y = edge_points().map(|point| point.y).max().unwrap_or(first.y);

        let bounds = Bounds {
            min_x,
            max_x,
            min_y,
            max_y,
        };

        Some(Grid {
            cells,
            bounds,
            lower_existence,
        })
    }

    fn count_sand(&self) -> usize {
        self.cells
            .values()
            .filter(|cell| **cell == Cell::Sand)
            .count()
    }

    fn simulate_falling_sand(&mut self, start: Point) {
        debug_println!("Simulating falling sand");
        self.print_cells(&start);
        debug_println!();
        let mut path = vec![start.clone()];
        loop {
            match self.simulate_one_sand(&mut path) {
                SimulationResult::Settled => {
                    self.print_cells(&start);
                    debug_println!();
                }
                SimulationResult::FallIntoAbyss => {
                    if self.lower_existence == LowerExistence::Floor {
                        panic!("Should not fall into abyss if there is an infinite floor");
                    } else {
                        debug_println!("Further sand is falling into the abyss.");
                        break;
                    }
                }
                SimulationResult::BlockingSource => {
                    debug_println!("Sand is now blocking the source");
                    break;
                }
            }
        }
        self.print_cells_with_path(&start, path.iter());
    }

    fn simulate_one_sand(&mut self, path: &mut Vec<Point>) -> SimulationResult {
        let mut seen = HashSet::new();
        debug_println!("= Simulating one sand, path has {} points =", path.len());
        let mut cur_point = loop {
            let Some(point) = path.pop() else {
                debug_println!("Path is now empty, sand must be blocking the source");
                return SimulationResult::BlockingSource;
            };
            if self.is_solid(&point) {
                debug_println!("Path point {} is now solid, skipping", point);
                continue;
            }
            debug_println!("Path point {} is empty, using it as starting point", point);
            break point;
        };
        debug_println!("Simulating one sand starting at point {:?}", cur_point);
        loop {
            if seen.contains(&cur_point) {
                panic!("Already seen {}, this is likely an infinte loop", cur_point);
            }
            seen.insert(cur_point.clone());

            if self.lower_existence == LowerExistence::Abyss && self.falling_into_abyss(&cur_point)
            {
                debug_println!("We are now falling into the abyss");
                return SimulationResult::FallIntoAbyss;
            }

            debug_println!("\tCurrent point: {}", cur_point);
            let below = Point {
                x: cur_point.x,
                y: cur_point.y + 1,
            };
            if !self.is_solid(&below) {
                debug_println!("\t\tBelow is empty");
                path.push(cur_point);
                cur_point = below;
                continue;
            }

            let below_left = Point {
                x: cur_point.x - 1,
                y: cur_point.y + 1,
            };
            if !self.is_solid(&below_left) {
                debug_println!("\t\tBelow left is empty");
                path.push(cur_point);
                cur_point = below_left;
                continue;
            }

            let below_right = Point {
                x: cur_point.x + 1,
                y: cur_point.y + 1,
            };
            if !self.is_solid(&below_right) {
                debug_println!("\t\tBelow right is empty");
                path.push(cur_point);
                cur_point = below_right;
                continue;
            }

            debug_println!("\t\tAll are solid, settling at {}", cur_point);
            self.cells.insert(cur_point, Cell::Sand);
            return SimulationResult::Settled;
        }
    }

    fn falling_into_abyss(&self, point: &Point) -> bool {
        !(self.bounds.min_x <= point.x
            && point.x <= self.bounds.max_x
            && point.y <= self.bounds.max_y)
    }

    fn is_solid(&self, point: &Point) -> bool {
        self.cells.contains_key(point)
            || (self.lower_existence == LowerExistence::Floor
                && point.y == self.bounds.max_y + FLOOR_DIST_FROM_MAX_Y)
    }

    fn print_cells(&self, start: &Point) {
        self.print_cells_with_path(start, std::iter::empty());
    }

    fn print_cells_with_path<'a, T: Iterator<Item = &'a Point>>(&self, start: &Point, path: T) {
        let path_points = path.collect::<HashSet<_>>();
        let min_x = cmp::min(self.bounds.min_x, start.x);
        let max_x = cmp::max(self.bounds.max_x, start.x);
        let min_y = cmp::min(self.bounds.min_y, start.y);
        let max_y = cmp::max(self.bounds.max_y, start.y);
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let point = Point { x, y };
                let c = match self.cells.get(&point) {
                    _ if point == *start => '+',
                    _ if path_points.contains(&point) => '~',
                    None => '.',
                    Some(Cell::Rock) => '#',
                    Some(Cell::Sand) => 'o',
                };
                debug_print!("{}", c);
            }
            debug_println!();
        }
    }
}

impl Bounds {
    fn contains(&self, point: &Point) -> bool {
        self.min_x <= point.x
            && point.x <= self.max_x
            && self.min_y <= point.y
            && point.y <= self.max_y
    }
}

impl Line {
    fn get_all_points(&self) -> Option<impl Iterator<Item = Point> + '_> {
        let iterators = self
            .0
            .iter()
            .tuple_windows()
            .map(|(p1, p2)| p1.points_to(p2))
            .collect::<Option<Vec<_>>>()?;
        Some(
            iterators
                .into_iter()
                .flatten()
                .chain(self.0.last().cloned().into_iter()),
        )
    }
}

impl Point {
    fn points_to<'a>(&self, point_to: &'a Point) -> Option<impl Iterator<Item = Point> + 'a> {
        let dx = (point_to.x - self.x).signum();
        let dy = (point_to.y - self.y).signum();
        match (dx, dy) {
            (0, 0) => return None,
            (0, _) | (_, 0) => {}
            (_, _) => return None,
        }
        Some(
            itertools::iterate(self.clone(), move |p| Point {
                x: p.x + dx,
                y: p.y + dy,
            })
            .take_while(move |p| p != point_to),
        )
    }
}

fn parse_lines(text: &str) -> Option<Vec<Line>> {
    text.trim().split('\n').map(Line::parse).collect()
}

impl Line {
    fn parse(text: &str) -> Option<Self> {
        let points = text
            .split(" -> ")
            .map(Point::parse)
            .collect::<Option<_>>()?;
        Some(Line(points))
    }
}

impl Point {
    fn parse(text: &str) -> Option<Self> {
        let (x, y) = text.split(',').collect_tuple()?;
        let x = x.parse().ok()?;
        let y = y.parse().ok()?;
        Some(Point { x, y })
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_points_from_to() {
        let from = p(3, 4);
        let to = p(3, 7);
        let actual = from.points_to(&to).unwrap().collect::<Vec<_>>();
        let expected = vec![p(3, 4), p(3, 5), p(3, 6)];
        assert_eq!(expected, actual);

        let from = p(5, 4);
        let to = p(1, 4);
        let actual = from.points_to(&to).unwrap().collect::<Vec<_>>();
        let expected = vec![p(5, 4), p(4, 4), p(3, 4), p(2, 4)];
        assert_eq!(expected, actual);

        let from = p(5, 4);
        let to = p(1, 3);
        assert!(from.points_to(&to).is_none());
    }

    #[test]
    fn test_get_all_points() {
        let line = Line(vec![p(498, 4), p(498, 6), p(496, 6)]);
        let actual = line.get_all_points().unwrap().collect::<Vec<_>>();
        let expected = vec![p(498, 4), p(498, 5), p(498, 6), p(497, 6), p(496, 6)];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse() {
        let lines = parse_lines(include_str!("../../static/example14.txt")).unwrap();

        let line0 = Line(vec![p(498, 4), p(498, 6), p(496, 6)]);
        assert_eq!(line0, lines[0]);

        let line1 = Line(vec![p(503, 4), p(502, 4), p(502, 9), p(494, 9)]);
        assert_eq!(line1, lines[1]);
    }

    #[test]
    fn test_examples_part1() {
        assert_eq!(24, Day14.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(737, Day14.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        assert_eq!(93, Day14.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        // assert_eq!(0, Day14.part2(Example::Real, Debug::NotDebug));
    }

    fn p(x: Index, y: Index) -> Point {
        Point { x, y }
    }
}
