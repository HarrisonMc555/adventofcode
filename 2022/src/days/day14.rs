use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

use itertools::Itertools;

use crate::days::{Day, Debug, Example, Part};
use crate::{debug_print, debug_println};

const DEBUG: bool = false;

// This is somewhat slow because every time the "starting point" returned from "simulate_one_sand" is occupied (which
// happens frequently), we have to start all the way back at the original source. If we instead returned the path the
// sand took (i.e. a vector of points), then we could simply pop the last point off until we find an unoccupied one
// and start from there.

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
        let mut grid = Grid::new(&lines, SAND_SOURCE).unwrap();
        grid.simulate_falling_sand();
        grid.count_sand()
    }

    fn part2(&self, example: Example, _debug: Debug) -> usize {
        let lines = parse_lines(&self.read_file(example)).unwrap();
        let mut grid = Grid::new(&lines, SAND_SOURCE).unwrap();
        grid.simulate_falling_sand2();
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
    start: Point,
    bounds: Bounds,
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
    Settled { next_starting_point: Option<Point> },
    FallIntoAbyss,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
enum SimulationResult2 {
    Settled { next_starting_point: Option<Point> },
    BlockingSource,
}

impl Grid {
    fn new(lines: &[Line], start: Point) -> Option<Self> {
        let Some(first) = lines.first().and_then(|line| line.0.first()) else {
            return None;
        };

        let mut cells = HashMap::new();
        for line in lines {
            for point in line.get_all_points()? {
                cells.insert(point, Cell::Rock);
            }
        }

        let edge_points = || {
            lines
                .iter()
                .flat_map(|line| line.0.iter())
                .chain(std::iter::once(&start))
        };
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
            start,
            bounds,
        })
    }

    fn count_sand(&self) -> usize {
        self.cells
            .values()
            .filter(|cell| **cell == Cell::Sand)
            .count()
    }

    fn simulate_falling_sand(&mut self) {
        let mut starting_point = self.start.clone();
        debug_println!("Simulating falling sand");
        self.print_cells();
        debug_println!();
        while let SimulationResult::Settled {
            next_starting_point,
        } = self.simulate_one_sand(starting_point)
        {
            debug_println!("Next starting point is {:?}", next_starting_point);
            starting_point = next_starting_point.unwrap_or_else(|| self.start.clone());
            debug_println!("Next starting point is now {}", starting_point);
            self.print_cells();
            debug_println!();
        }
        debug_println!("Further sand is falling into the abyss.");
    }

    fn simulate_falling_sand2(&mut self) {
        let mut starting_point = self.start.clone();
        debug_println!("Simulating falling sand");
        self.print_cells();
        debug_println!();
        while let SimulationResult2::Settled {
            next_starting_point,
        } = self.simulate_one_sand2(starting_point)
        {
            debug_println!("Next starting point: {:?}", next_starting_point);
            starting_point = next_starting_point.unwrap_or_else(|| self.start.clone());
            debug_println!("Next starting point is {}", starting_point);
            self.print_cells();
            debug_println!();
        }
        debug_println!("Further sand is falling into the abyss.");
    }

    fn simulate_one_sand(&mut self, start: Point) -> SimulationResult {
        let mut previous_point = None;
        debug_println!("Simulating one sand starting at {}", start);
        let mut seen = HashSet::new();
        let mut cur_point = start;
        while !self.falling_into_abyss(&cur_point) {
            if seen.contains(&cur_point) {
                debug_println!("Already seen {}", cur_point);
                panic!();
            }
            seen.insert(cur_point.clone());

            debug_println!("\tCurrent point: {}", cur_point);
            let below = Point {
                x: cur_point.x,
                y: cur_point.y + 1,
            };
            if !self.cells.contains_key(&below) {
                debug_println!("\t\tBelow is empty");
                previous_point = Some(cur_point);
                cur_point = below;
                continue;
            }

            let below_left = Point {
                x: cur_point.x - 1,
                y: cur_point.y + 1,
            };
            if !self.cells.contains_key(&below_left) {
                debug_println!("\t\tBelow left is empty");
                previous_point = Some(cur_point);
                cur_point = below_left;
                continue;
            }

            let below_right = Point {
                x: cur_point.x + 1,
                y: cur_point.y + 1,
            };
            if !self.cells.contains_key(&below_right) {
                debug_println!("\t\tBelow right is empty");
                previous_point = Some(cur_point);
                cur_point = below_right;
                continue;
            }

            debug_println!("\t\tAll are solid, settling at {}", cur_point);
            self.cells.insert(cur_point, Cell::Sand);
            return SimulationResult::Settled {
                next_starting_point: previous_point,
            };
        }
        SimulationResult::FallIntoAbyss
    }

    fn simulate_one_sand2(&mut self, start: Point) -> SimulationResult2 {
        let mut previous_point = None;
        debug_println!("Simulating one sand starting at {}", start);
        let mut seen = HashSet::new();
        let mut cur_point = start;
        loop {
            if seen.contains(&cur_point) {
                debug_println!("Already seen {}", cur_point);
                panic!();
            }
            seen.insert(cur_point.clone());

            debug_println!("\tCurrent point: {}", cur_point);
            let below = Point {
                x: cur_point.x,
                y: cur_point.y + 1,
            };
            if !self.is_solid(&below) {
                debug_println!("\t\tBelow is empty");
                previous_point = Some(cur_point);
                cur_point = below;
                continue;
            }

            let below_left = Point {
                x: cur_point.x - 1,
                y: cur_point.y + 1,
            };
            if !self.is_solid(&below_left) {
                debug_println!("\t\tBelow left is empty");
                previous_point = Some(cur_point);
                cur_point = below_left;
                continue;
            }

            let below_right = Point {
                x: cur_point.x + 1,
                y: cur_point.y + 1,
            };
            if !self.is_solid(&below_right) {
                debug_println!("\t\tBelow right is empty");
                previous_point = Some(cur_point);
                cur_point = below_right;
                continue;
            }

            debug_println!("\t\tAll are solid, settling at {}", cur_point);
            let result = if cur_point == self.start {
                SimulationResult2::BlockingSource
            } else {
                SimulationResult2::Settled {
                    next_starting_point: previous_point,
                }
            };
            self.cells.insert(cur_point, Cell::Sand);
            return result;
        }
    }

    fn falling_into_abyss(&self, point: &Point) -> bool {
        !(self.bounds.min_x <= point.x
            && point.x <= self.bounds.max_x
            && point.y <= self.bounds.max_y)
    }

    fn is_solid(&self, point: &Point) -> bool {
        self.cells.contains_key(point) || point.y == self.bounds.max_y + FLOOR_DIST_FROM_MAX_Y
    }

    fn print_cells(&self) {
        for y in self.bounds.min_y..=self.bounds.max_y {
            for x in self.bounds.min_x..=self.bounds.max_x {
                let point = Point { x, y };
                let c = if point == self.start {
                    '+'
                } else {
                    match self.cells.get(&point) {
                        None => '.',
                        Some(Cell::Rock) => '#',
                        Some(Cell::Sand) => 'o',
                    }
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
