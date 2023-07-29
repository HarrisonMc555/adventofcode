use array2d::Array2D;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::VecDeque, str::FromStr};

use crate::days::{Day, Debug, Example, Part};

const DEBUG: bool = false;

const NUM_HEADER_LINES: usize = 2;

const ORIGIN: Index = (0, 0);
const TARGET_INDEX: Index = ORIGIN;

pub struct Day22;

impl Day for Day22 {
    fn number(&self) -> u32 {
        22
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day22 {
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        let nodes = NodeInfo::parse_lines(&self.read_file(example)).unwrap();
        count_viable_pairs(&nodes)
    }

    fn part2(&self, example: Example, _debug: Debug) -> usize {
        let nodes = NodeInfo::parse_lines(&self.read_file(example)).unwrap();
        let goal_index = get_goal_index(&nodes).unwrap();
        let grid = nodes_to_grid(&nodes, goal_index).unwrap();
        shortest_path_len(grid, TARGET_INDEX).unwrap()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct NodeInfo {
    x: usize,
    y: usize,
    size: usize,
    used: usize,
    avail: usize,
}

fn count_viable_pairs(nodes: &[NodeInfo]) -> usize {
    count_overlap_pairs(
        nodes.iter().map(|n| n.used).filter(|used| *used != 0),
        nodes.iter().map(|n| n.avail),
    )
}

fn count_overlap_pairs<IF, IT>(from: IF, into: IT) -> usize
where
    IF: IntoIterator<Item = usize>,
    IT: IntoIterator<Item = usize>,
{
    let from = from.into_iter().sorted_unstable();
    let into = into
        .into_iter()
        .sorted_unstable_by_key(|&x| std::cmp::Reverse(x))
        .collect::<Vec<_>>();
    let mut count = 0;
    let mut max_index = into.len();
    let mut prev = None;
    for val_from in from {
        if let Some((prev_val, prev_count)) = prev {
            if prev_val == val_from {
                count += prev_count;
                continue;
            }
        };
        debug_println!(
            "\tHow many does {} fit into? Max index is {} ({:?})",
            val_from,
            max_index,
            into.get(max_index)
        );
        max_index = into[..max_index].partition_point(|&val_into| val_from <= val_into);
        debug_println!(
            "\t\tFits {}. {:?} < {} <= {:?}",
            max_index,
            &into[max_index..],
            val_from,
            &into[..max_index],
        );
        if max_index == 0 {
            break;
        }
        count += max_index;
        prev = Some((val_from, max_index));
    }
    count
}

impl NodeInfo {
    pub fn parse_lines(s: &str) -> Result<Vec<Self>, <Self as FromStr>::Err> {
        s.lines()
            .skip(NUM_HEADER_LINES)
            .map(Self::from_str)
            .collect()
    }

    pub fn index(&self) -> Index {
        (self.y, self.x)
    }
}

impl FromStr for NodeInfo {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^/dev/grid/node-x(\d+)-y(\d+) +(\d+)T +(\d+)T +(\d+)T+ +(\d+)%$")
                    .unwrap();
        }
        let caps = RE
            .captures(s)
            .ok_or_else(|| format!("Invalid line: \"{}\"", s))?;
        let parse_usize = |group| caps.get(group).unwrap().as_str().parse::<usize>().unwrap();
        let x = parse_usize(1);
        let y = parse_usize(2);
        let size = parse_usize(3);
        let used = parse_usize(4);
        let avail = parse_usize(5);
        Ok(Self {
            x,
            y,
            size,
            used,
            avail,
        })
    }
}

fn get_goal_index(nodes: &[NodeInfo]) -> Option<Index> {
    nodes
        .iter()
        .filter(|node| node.y == 0)
        .max_by_key(|node| node.x)
        .map(|node| node.index())
}

fn nodes_to_grid(nodes: &[NodeInfo], goal_index: Index) -> Option<Grid> {
    let goal_node = nodes.iter().find(|node| node.index() == goal_index)?;
    let goal_size = goal_node.size;
    let goal_used = goal_node.used;
    debug_println!("Goal size: {goal_size}, used: {goal_used}");
    let cells = nodes
        .iter()
        .map(|node| {
            (
                node.index(),
                node_to_cell(node, goal_index, goal_used, goal_size),
            )
        })
        .collect::<Vec<_>>();
    let (max_row, max_column) = *cells.iter().map(|(index, _)| index).max()?;
    let mut grid = Array2D::filled_with(Cell::Immovable, max_row + 1, max_column + 1);
    for (index, cell) in cells {
        grid[index] = cell;
    }
    debug_println!("Used/Size");
    for row in 0..=max_row {
        for column in 0..=max_column {
            let index = (row, column);
            let node = nodes.iter().find(|node| node.index() == index).unwrap();
            debug_print!("{}/{} ", node.used, node.size);
        }
        debug_println!();
    }
    debug_println!();
    debug_println!("Cells");
    debug_print_grid(&grid);
    Some(grid)
}

fn node_to_cell(node: &NodeInfo, goal_index: Index, goal_used: usize, goal_size: usize) -> Cell {
    if node.index() == goal_index {
        Cell::Goal
    } else if node.used == 0 {
        Cell::Empty
    } else if node.size >= goal_used && node.used <= goal_size {
        Cell::Movable
    } else {
        Cell::Immovable
    }
}

type Grid = Array2D<Cell>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Cell {
    Goal,
    Empty,
    Movable,
    Immovable,
}

type Index = (usize, usize);

fn shortest_path_len(mut grid: Grid, destination: Index) -> Result<usize, String> {
    let mut empty_index = grid
        .indices_row_major()
        .find(|index| grid[*index] == Cell::Empty)
        .ok_or_else(|| "No empty cell found".to_string())?;
    let path = shortest_path_moving_goal(&grid, destination)?;
    debug_println!("Path for goal: {:?}", path);
    if path.len() <= 1 {
        return Ok(0);
    }
    let mut len = 0;
    debug_println!("Before starting:");
    debug_print_grid(&grid);
    for (&cur_goal_index, &next_goal_index) in path.iter().tuple_windows() {
        debug_println!();
        debug_println!(
            "Moving goal from {:?} to {:?}. {} steps so far.",
            cur_goal_index,
            next_goal_index,
            len,
        );
        // Move empty cell to position
        let num_steps = shortest_path_len_moving_empty(&grid, empty_index, next_goal_index)?;
        debug_println!(
            "Takes {} steps to move empty from {:?} to {:?}",
            num_steps,
            empty_index,
            next_goal_index
        );
        len += num_steps;
        // Swap empty cell with goal
        debug_println!(
            "Swap empty @ {:?} with goal @ {:?}",
            next_goal_index,
            cur_goal_index
        );
        len += 1;
        grid[empty_index] = Cell::Movable;
        grid[next_goal_index] = Cell::Goal;
        grid[cur_goal_index] = Cell::Empty;
        empty_index = cur_goal_index;
        debug_println!("Grid:");
        debug_print_grid(&grid);
        debug_println!();
    }
    Ok(len)
}

fn shortest_path_moving_goal(grid: &Grid, destination: Index) -> Result<Vec<Index>, String> {
    let goal_index = grid
        .indices_row_major()
        .find(|index| grid[*index] == Cell::Goal)
        .ok_or_else(|| "No goal cell found".to_string())?;

    let mut distances = Array2D::from_iter_row_major(
        grid.elements_row_major_iter().map(|_| None),
        grid.num_rows(),
        grid.num_columns(),
    )
    .unwrap();
    distances[goal_index] = Some(0);
    let mut queue = VecDeque::new();
    queue.push_back(goal_index);
    while let Some(index) = queue.pop_front() {
        let distance = distances[index].ok_or_else(|| {
            format!(
                "Internal error, distance for {:?} was not initialized",
                index
            )
        })?;
        if index == destination {
            return path_from_distances(&distances, destination);
        }
        for neighbor in neighbors(grid, index) {
            if distances[neighbor].is_some() {
                continue;
            }
            match grid[neighbor] {
                Cell::Movable | Cell::Empty => {}
                _ => continue,
            }
            let neighbor_distance = distance + 1;
            distances[neighbor] = Some(neighbor_distance);
            queue.push_back(neighbor);
        }
    }
    Err(format!(
        "No path from from {:?} to {:?}",
        goal_index, destination
    ))
}

fn path_from_distances(
    distances: &Array2D<Option<usize>>,
    destination: Index,
) -> Result<Vec<Index>, String> {
    let mut path = Vec::new();
    let mut distance = distances[destination].ok_or("No distance to destination")?;
    let mut index = destination;
    'outer: loop {
        path.push(index);
        if distance == 0 {
            break;
        }
        for neighbor in neighbors(distances, index) {
            match distances[neighbor] {
                Some(neighbor_distance) if neighbor_distance < distance => {
                    index = neighbor;
                    distance = neighbor_distance;
                    continue 'outer;
                }
                _ => {}
            }
        }
        return Err(format!("No path found from {:?}", destination));
    }
    path.reverse();
    Ok(path)
}

fn shortest_path_len_moving_empty(
    grid: &Grid,
    source @ (source_row, source_column): Index,
    destination @ (destination_row, destination_column): Index,
) -> Result<usize, String> {
    if grid.get(destination_row, destination_column).is_none() {
        return Err(format!("Destination {:?} is out of bounds", destination));
    }
    let source_cell = grid.get(source_row, source_column);
    if source_cell != Some(&Cell::Empty) {
        return Err(format!(
            "Source: {:?} is not empty (it is {:?})",
            source, source_cell
        ));
    }
    let mut distances = Array2D::from_iter_row_major(
        grid.elements_row_major_iter().map(|_| None),
        grid.num_rows(),
        grid.num_columns(),
    )
    .unwrap();
    distances[source] = Some(0);
    let mut queue = VecDeque::new();
    queue.push_back(source);
    while let Some(index) = queue.pop_front() {
        let distance = distances[index].ok_or_else(|| {
            format!(
                "Internal error, distance for {:?} was not initialized",
                index
            )
        })?;
        if index == destination {
            return Ok(distance);
        }
        for neighbor in neighbors(grid, index) {
            if grid[neighbor] != Cell::Movable || distances[neighbor].is_some() {
                continue;
            }
            let neighbor_distance = distance + 1;
            distances[neighbor] = Some(neighbor_distance);
            queue.push_back(neighbor);
        }
    }
    Err(format!(
        "No path from from {:?} to {:?}",
        source, destination
    ))
}

fn neighbors<T>(grid: &Array2D<T>, (row, column): Index) -> impl Iterator<Item = Index> {
    let up_row = row.checked_sub(1);
    let down_row = row.checked_add(1).filter(|r| *r < grid.num_rows());
    let left_column = column.checked_sub(1);
    let right_column = column.checked_add(1).filter(|c| *c < grid.num_columns());
    [
        (Some(row), right_column),
        (down_row, Some(column)),
        (Some(row), left_column),
        (up_row, Some(column)),
    ]
    .into_iter()
    .filter_map(|i| match i {
        (Some(r), Some(c)) => Some((r, c)),
        _ => None,
    })
}

fn debug_print_grid(grid: &Array2D<Cell>) {
    if !DEBUG {
        return;
    }
    for row in grid.rows_iter() {
        for cell in row {
            debug_print!("{} ", cell.debug_char());
        }
        debug_println!();
    }
}

impl Cell {
    pub fn debug_char(self) -> char {
        match self {
            Cell::Goal => 'G',
            Cell::Empty => '_',
            Cell::Movable => '.',
            Cell::Immovable => '#',
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_parse() {
        let line = "/dev/grid/node-x0-y0     92T   68T    24T   73%";
        let actual = line.parse();
        let expected = NodeInfo {
            x: 0,
            y: 0,
            size: 92,
            used: 68,
            avail: 24,
        };
        assert_eq!(Ok(expected.clone()), actual);

        let lines = "root@ebhq-gridcenter# df -h\n\
            Filesystem              Size  Used  Avail  Use%\n\
            /dev/grid/node-x0-y0     92T   68T    24T   73%\n\
            /dev/grid/node-x0-y1     90T   68T    22T   75%";
        let actual = NodeInfo::parse_lines(lines);
        let expected = vec![
            expected,
            NodeInfo {
                x: 0,
                y: 1,
                size: 90,
                used: 68,
                avail: 22,
            },
        ];
        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn test_count_overlap_pairs() {
        assert_eq!(0, count_overlap_pairs(&[], &[]));
        assert_eq!(0, count_overlap_pairs(&[1], &[]));
        assert_eq!(0, count_overlap_pairs(&[], &[1]));
        assert_eq!(1, count_overlap_pairs(&[0], &[1]));
        assert_eq!(1, count_overlap_pairs(&[1], &[1]));
        assert_eq!(0, count_overlap_pairs(&[2], &[1]));
        assert_eq!(2, count_overlap_pairs(&[1], &[1, 2]));
        assert_eq!(1, count_overlap_pairs(&[2], &[1, 2]));
        assert_eq!(0, count_overlap_pairs(&[3], &[1, 2]));
        assert_eq!(
            6 + 4 + 2,
            count_overlap_pairs(&[1, 4, 6], &[1, 3, 4, 5, 6, 7])
        );
    }

    fn count_overlap_pairs(from: &[usize], into: &[usize]) -> usize {
        super::count_overlap_pairs(from.iter().copied(), into.iter().copied())
    }

    #[test]
    fn test_examples_part1() {}

    #[test]
    fn test_real_part1() {
        assert_eq!(985, Day22.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_neighbors() {
        let grid = Array2D::from_rows(&[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]).unwrap();

        let actual = neighbors(&grid, (0, 0));
        let expected = [(0, 1), (1, 0)].into_iter().collect::<HashSet<_>>();
        assert_eq!(expected, actual);

        let actual = neighbors(&grid, (1, 0));
        let expected = [(0, 0), (2, 0), (1, 1)].into_iter().collect::<HashSet<_>>();
        assert_eq!(expected, actual);

        let actual = neighbors(&grid, (0, 1));
        let expected = [(0, 0), (0, 2), (1, 1)].into_iter().collect::<HashSet<_>>();
        assert_eq!(expected, actual);

        let actual = neighbors(&grid, (1, 1));
        let expected = [(0, 1), (2, 1), (1, 0), (1, 2)]
            .into_iter()
            .collect::<HashSet<_>>();
        assert_eq!(expected, actual);

        let actual = neighbors(&grid, (2, 2));
        let expected = [(2, 1), (1, 2)].into_iter().collect::<HashSet<_>>();
        assert_eq!(expected, actual);

        let actual = neighbors(&grid, (2, 0));
        let expected = [(2, 1), (1, 0)].into_iter().collect::<HashSet<_>>();
        assert_eq!(expected, actual);

        let actual = neighbors(&grid, (0, 2));
        let expected = [(0, 1), (1, 2)].into_iter().collect::<HashSet<_>>();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_shortest_path_len_moving_empty() {
        let e = Cell::Empty;
        let m = Cell::Movable;
        let i = Cell::Immovable;
        let g = Cell::Goal;

        let grid = Array2D::from_rows(&[
            vec![m, g, e], //
            vec![m, m, m], //
            vec![i, m, m], //
        ])
        .unwrap();
        let actual = shortest_path_len_moving_empty(&grid, (0, 2), (0, 0));
        assert_eq!(Ok(4), actual);

        let grid = Array2D::from_rows(&[
            vec![m, g, e], //
            vec![m, i, m], //
            vec![m, m, m], //
        ])
        .unwrap();
        let actual = shortest_path_len_moving_empty(&grid, (0, 2), (0, 0));
        assert_eq!(Ok(6), actual);
    }

    #[test]
    fn test_path_from_distances() {
        let distances = Array2D::from_rows(&[
            vec![Some(2), Some(1), Some(0)],
            vec![Some(3), Some(2), Some(1)],
            vec![None, Some(3), Some(2)],
        ])
        .unwrap();
        let actual = path_from_distances(&distances, (0, 0));
        let expected = Ok(vec![(0, 2), (0, 1), (0, 0)]);
        assert_eq!(expected, actual);

        let distances = Array2D::from_rows(&[
            vec![Some(4), None, Some(0)],
            vec![Some(3), Some(2), Some(1)],
            vec![None, Some(3), Some(2)],
        ])
        .unwrap();
        let actual = path_from_distances(&distances, (0, 0));
        let expected = Ok(vec![(0, 2), (1, 2), (1, 1), (1, 0), (0, 0)]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_examples_part2() {
        let e = Cell::Empty;
        let m = Cell::Movable;
        let i = Cell::Immovable;
        let g = Cell::Goal;

        let grid = Array2D::from_rows(&[vec![m, m, g], vec![m, e, m], vec![i, m, m]]).unwrap();
        let actual = shortest_path_len(grid, (0, 0));
        assert_eq!(Ok(7), actual);
    }

    #[test]
    fn test_real_part2() {
        assert_eq!(179, Day22.part2(Example::Real, Debug::NotDebug));
    }

    fn neighbors<T>(grid: &Array2D<T>, index: Index) -> HashSet<Index> {
        super::neighbors(grid, index).collect()
    }
}
