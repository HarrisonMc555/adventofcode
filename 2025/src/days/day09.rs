use array2d::Array2D;
use std::{
    collections::VecDeque,
    ops::{Index, RangeInclusive},
    sync::Mutex,
};

use itertools::Itertools;

use crate::days::{Day, Debug, Example, Part};
use crate::{debug_print, debug_println};

static DEBUG: Mutex<Debug> = Mutex::new(Debug::NotDebug);

fn debug() -> Debug {
    *DEBUG.lock().unwrap()
}

pub struct Day09;

impl Day for Day09 {
    fn number(&self) -> u32 {
        9
    }

    fn part1(&self, example: Example, debug: Debug) -> String {
        *DEBUG.lock().unwrap() = debug;
        let lines = self.get_lines(Part::Part1, example);
        let points: Vec<Point> = parse_points(lines).unwrap();
        let answer = largest_area(&points);
        answer.to_string()
    }

    fn part2(&self, example: Example, debug: Debug) -> String {
        *DEBUG.lock().unwrap() = debug;
        let lines = self.get_lines(Part::Part2, example);
        let points: Vec<Point> = parse_points(lines).unwrap();
        let answer = largest_covered_area(&points).unwrap();
        answer.to_string()
    }
}

const NUM_DIMENSIONS: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dimension {
    X,
    Y,
}

fn parse_points<I, S>(lines: I) -> Result<Vec<Point>, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    lines
        .into_iter()
        .map(|line| parse_point(line.as_ref()))
        .collect()
}

fn parse_point(line: &str) -> Result<Point, String> {
    let numbers = line
        .split(",")
        .map(|word| word.parse())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Invalid number: {e}"))?;
    let array: [usize; Dimension::NUM] = numbers
        .try_into()
        .map_err(|_| format!("Invalid point format: {line}"))?;
    Ok(array.into())
}

fn calc_area(point1: Point, point2: Point) -> usize {
    Dimension::all()
        .map(|dimension| calc_distance(point1[dimension], point2[dimension]))
        .product()
}

fn calc_distance(position1: usize, position2: usize) -> usize {
    // Add one so a rectangle that starts and ends in the same row/column can still have a height/width of 1, etc.
    usize::abs_diff(position1, position2) + 1
}

fn largest_area(points: &[Point]) -> usize {
    let num_points = points.len();
    (0..num_points)
        .flat_map(|index1| (0..num_points).map(move |index2| (points[index1], points[index2])))
        .map(|(point1, point2)| calc_area(point1, point2))
        .max()
        .unwrap_or(0)
}

fn largest_covered_area(points: &[Point]) -> Option<usize> {
    let grid = FilledGrid::create(points)?;
    let debug = debug();
    if debug.into() {
        debug_println!(debug);
        debug_print!(debug, "  ");
        for x_index in grid.x_indices.iter() {
            debug_print!(debug, "{x_index:>2}");
        }
        debug_println!(debug);
        for (row_block_index, row) in grid.grid.rows_iter().enumerate() {
            debug_print!(
                debug,
                "{:>2}",
                grid.y_indices
                    .get(row_block_index)
                    .map(|i| i.to_string())
                    .unwrap_or_default()
            );
            for cell in row {
                let c = match cell {
                    Cell::Empty => ".",
                    Cell::Filled => "X",
                };
                debug_print!(debug, " {}", c);
            }
            debug_println!(debug);
        }
        debug_println!(debug);
    }

    let num_points = points.len();
    let point_pairs_with_area = {
        let mut point_pairs = (0..num_points)
            .flat_map(|index1| (0..num_points).map(move |index2| (points[index1], points[index2])))
            .filter(|(point1, point2)| point1 != point2)
            .map(|(point1, point2)| ((point1, point2), calc_area(point1, point2)))
            .collect::<Vec<_>>();
        point_pairs.sort_unstable_by_key(move |(_, area)| *area);
        point_pairs
    };
    if debug.into() {
        for ((point1, point2), area) in point_pairs_with_area.iter() {
            debug_print!(debug, "{point1} -> {point2} = {area}");
            if is_rect_covered(&grid, *point1, *point2) {
                debug_println!(debug, " (covered)");
            } else {
                debug_println!(debug, " (UNcovered)");
            }
        }
    }
    point_pairs_with_area
        .into_iter()
        .filter(|((point1, point2), _)| is_rect_covered(&grid, *point1, *point2))
        .map(|(_, area)| area)
        .max()
}

fn is_rect_covered(grid: &FilledGrid, point1: Point, point2: Point) -> bool {
    let Point { x: x1, y: y1 } = point1;
    let Point { x: x2, y: y2 } = point2;
    let x_indices = &grid.x_indices;
    let y_indices = &grid.y_indices;
    let x1_block_index = find_index(x_indices, x1);
    let x2_block_index = find_index(x_indices, x2);
    let y1_block_index = find_index(y_indices, y1);
    let y2_block_index = find_index(y_indices, y2);
    let min_x_block_index = usize::min(x1_block_index, x2_block_index);
    let max_x_block_index = usize::max(x1_block_index, x2_block_index);
    let min_y_block_index = usize::min(y1_block_index, y2_block_index);
    let max_y_block_index = usize::max(y1_block_index, y2_block_index);
    let mut block_indices = (min_x_block_index..=max_x_block_index).flat_map(|x_block_index| {
        (min_y_block_index..=max_y_block_index)
            .map(move |y_block_index| (x_block_index, y_block_index))
    });
    block_indices.all(|(x, y)| {
        let row = y;
        let column = x;
        grid.grid[(row, column)] == Cell::Filled
    })
}

fn find_index(indices: &[usize], value: usize) -> usize {
    match indices.binary_search(&value) {
        Ok(index) => index,
        Err(index) => index,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FilledGrid {
    grid: Array2D<Cell>,
    x_indices: Vec<usize>,
    y_indices: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Cell {
    Empty,
    Filled,
}

impl FilledGrid {
    pub fn create(points: &[Point]) -> Option<Self> {
        let debug = debug();
        if points.is_empty() {
            return None;
        }
        let all_x_indices = get_sorted_unique_indices(points, Dimension::X);
        let all_y_indices = get_sorted_unique_indices(points, Dimension::Y);
        if debug.into() {
            debug_println!(debug, "All x: {all_x_indices:?}");
            debug_println!(debug, "All y: {all_y_indices:?}");
            debug_print!(debug, "  ");
            for column_i in 0..=*all_x_indices.iter().max().unwrap() {
                debug_print!(debug, "{column_i:>2}");
            }
            debug_println!(debug);
            for row_i in 0..=*all_y_indices.iter().max().unwrap() {
                let y = row_i;
                debug_print!(debug, "{row_i:>2}");
                for column_i in 0..=*all_x_indices.iter().max().unwrap() {
                    let x = column_i;
                    if points.iter().contains(&Point { x, y }) {
                        debug_print!(debug, " X");
                    } else {
                        debug_print!(debug, " .");
                    }
                }
                debug_println!(debug);
            }
            debug_println!(debug);
        }
        let connections = create_connections(points).collect::<Vec<_>>();
        let rotations = create_rotations(&connections)
            .collect::<Option<Vec<_>>>()
            .unwrap();
        if debug.into() {
            for (connection, rotation) in connections.iter().zip(rotations.iter()) {
                debug_println!(
                    debug,
                    "{connection} ({:?}) to next = {rotation:?}",
                    connection.direction().unwrap()
                );
            }
            debug_println!(debug);
        }
        let mut block_states = Array2D::filled_with(
            BlockState::NotEdge,
            all_y_indices.len() + 1,
            all_x_indices.len() + 1,
        );
        for connection in connections {
            let Connection { from, to } = connection;
            let (x_range, y_range) = (range_inclusive(from.x, to.x), range_inclusive(from.y, to.y));
            debug_println!(debug, "{connection}: x {x_range:?}, y {y_range:?}");
            for x in x_range {
                for y in y_range.clone() {
                    let block_x_index = find_index(&all_x_indices, x);
                    let block_y_index = find_index(&all_y_indices, y);
                    debug_println!(
                        debug,
                        "\t({x}, {y}) to block index ({block_x_index}, {block_y_index})"
                    );
                    let row = block_y_index;
                    let column = block_x_index;
                    block_states.set(row, column, BlockState::Edge).ok()?;
                }
            }
        }

        if debug.into() {
            debug_println!(debug);
            debug_print!(debug, "  ");
            for column_i in 0..block_states.num_columns() {
                debug_print!(debug, "{column_i:>2}");
            }
            debug_println!(debug);
            for (row_i, row) in block_states.rows_iter().enumerate() {
                debug_print!(debug, "{row_i:>2}");
                for cell in row {
                    let c = match cell {
                        BlockState::NotEdge => ".",
                        BlockState::Edge => "X",
                    };
                    debug_print!(debug, " {c}");
                }
                debug_println!(debug);
            }
            debug_println!(debug);
        }

        let mut grid = Array2D::filled_with(
            Cell::Filled,
            block_states.num_rows(),
            block_states.num_columns(),
        );
        let mut explore = VecDeque::new();
        explore.push_back(Point { x: 0, y: 0 });
        debug_println!(debug, "==== Exploring ====");
        while let Some(point) = explore.pop_front() {
            debug_println!(debug, "\nExploring {point}");
            for neighbor in neighbors(point) {
                debug_println!(debug, "\tNeighbor {neighbor}");
                let Point { x: column, y: row } = neighbor;
                let Some(block_state) = block_states.get(row, column) else {
                    debug_println!(debug, "\t\tNo neighbor at {neighbor}");
                    continue;
                };
                if matches!(block_state, BlockState::Edge) {
                    debug_println!(debug, "\t\t{neighbor} is on edge, do not explore");
                    continue;
                }
                let Some(cell) = grid.get_mut(row, column) else {
                    debug_println!(debug, "\t\tNo neighbor at {neighbor}");
                    continue;
                };
                match cell {
                    Cell::Empty => {
                        debug_println!(debug, "\t\t{neighbor} was already empty");
                    }
                    Cell::Filled => {
                        debug_println!(
                            debug,
                            "\t\t{neighbor} was filled, marking empty and exploring its neighbors"
                        );
                        *cell = Cell::Empty;
                        explore.push_back(neighbor);
                    }
                }
            }
        }
        let grid_iter = grid
            .elements_row_major_iter()
            .copied()
            .map(|state| state.into());
        let grid = Array2D::from_iter_row_major(
            grid_iter,
            block_states.num_rows(),
            block_states.num_columns(),
        );
        let grid = match grid {
            Ok(grid) => grid,
            Err(e) => {
                debug_println!(debug, "Error: {e}");
                return None;
            }
        };
        Some(Self {
            grid,
            x_indices: all_x_indices,
            y_indices: all_y_indices,
        })
    }
}

fn neighbors(point: Point) -> impl Iterator<Item = Point> {
    let Point { x, y } = point;
    [
        (x.checked_sub(1), Some(y)),
        (Some(x + 1), Some(y)),
        (Some(x), y.checked_sub(1)),
        (Some(x), Some(y + 1)),
    ]
    .into_iter()
    .filter_map(|(x, y)| Some(Point { x: x?, y: y? }))
}

fn get_sorted_unique_indices(points: &[Point], dimension: Dimension) -> Vec<usize> {
    let indices = points.iter().flat_map(|point| {
        let index = point[dimension];
        [index.checked_sub(1), Some(index)].into_iter().flatten()
    });
    let mut indices = std::iter::once(0).chain(indices).collect::<Vec<_>>();
    indices.sort_unstable();
    indices.dedup();
    indices
}

fn create_connections(points: &[Point]) -> impl Iterator<Item = Connection> {
    circular_pair_windows(points).map(|(point1, point2)| Connection::new(*point1, *point2))
}

fn create_rotations(connections: &[Connection]) -> impl Iterator<Item = Option<Rotation>> {
    circular_pair_windows(connections)
        .map(|(connection1, connection2)| Rotation::from_connections(*connection1, *connection2))
}

fn range_inclusive(value1: usize, value2: usize) -> RangeInclusive<usize> {
    usize::min(value1, value2)..=usize::max(value1, value2)
}

fn circular_pair_windows<T>(array: &[T]) -> impl Iterator<Item = (&T, &T)> {
    let last_and_first = match (array.last(), array.first()) {
        (Some(last), Some(first)) => Some((last, first)),
        _ => None,
    };
    array.iter().tuple_windows().chain(last_and_first)
}

impl Dimension {
    pub const NUM: usize = 2;

    pub fn all() -> impl Iterator<Item = Self> {
        [Self::X, Self::Y].into_iter()
    }
}

impl Index<Dimension> for Point {
    type Output = usize;

    fn index(&self, index: Dimension) -> &Self::Output {
        match index {
            Dimension::X => &self.x,
            Dimension::Y => &self.y,
        }
    }
}

impl From<[usize; Dimension::NUM]> for Point {
    fn from(value: [usize; Dimension::NUM]) -> Self {
        Self {
            x: value[0],
            y: value[1],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum BlockState {
    NotEdge,
    Edge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct BlockIndices {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Connection {
    from: Point,
    to: Point,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Rotation {
    Clockwise,
    CounterClockwise,
}

impl Connection {
    pub fn new(from: Point, to: Point) -> Self {
        Self { from, to }
    }

    pub fn direction(self) -> Option<Direction> {
        let x_cmp = self.to.x.cmp(&self.from.x);
        let y_cmp = self.to.y.cmp(&self.from.y);
        use Direction::*;
        use std::cmp::Ordering::*;
        #[deny(non_snake_case)]
        Some(match (x_cmp, y_cmp) {
            (Equal, Less) => North,
            (Equal, Greater) => South,
            (Less, Equal) => West,
            (Greater, Equal) => East,
            _ => return None,
        })
    }
}

impl Rotation {
    pub fn from_connections(from: Connection, to: Connection) -> Option<Self> {
        let direction1 = from.direction()?;
        let direction2 = to.direction()?;
        use Direction::*;
        use Rotation::*;
        #[deny(non_snake_case)]
        Some(match (direction1, direction2) {
            (North, East) | (East, South) | (South, West) | (West, North) => Clockwise,
            (North, West) | (West, South) | (South, East) | (East, North) => CounterClockwise,
            _ => return None,
        })
    }
}

impl std::fmt::Display for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { from, to } = self;
        write!(f, "{from} -> {to}")
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { x, y } = self;
        write!(f, "({x}, {y})")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples_part1() {
        assert_eq!("50", Day09.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!("4748985168", Day09.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        assert_eq!("24", Day09.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn test_real_part2() {
        assert_eq!("1550760868", Day09.part2(Example::Real, Debug::NotDebug));
    }
}
