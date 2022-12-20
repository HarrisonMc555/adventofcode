use std::collections::VecDeque;

use array2d::Array2D;

use crate::days::{Day, Debug, Example, Part};
use crate::debug_println;

const DEBUG: bool = false;

pub struct Day12;

impl Day for Day12 {
    fn number(&self) -> u32 {
        12
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day12 {
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        let height_map = HeightMap::parse(&self.read_file(example)).unwrap();
        let best_distances = find_best_distances(&height_map).unwrap();
        best_distances[height_map.end].unwrap()
    }

    fn part2(&self, example: Example, _debug: Debug) -> usize {
        let height_map = HeightMap::parse(&self.read_file(example)).unwrap();
        find_best_distance_reverse(&height_map).unwrap()
    }
}

#[derive(Debug)]
struct HeightMap {
    heights: Array2D<u8>,
    start: (usize, usize),
    end: (usize, usize),
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Cell {
    Start,
    End,
    Normal(u8),
}

fn find_best_distances(height_map: &HeightMap) -> Option<Array2D<Option<usize>>> {
    let mut best_distances = Array2D::filled_with(
        None,
        height_map.heights.num_rows(),
        height_map.heights.num_columns(),
    );
    best_distances[height_map.start] = Some(0);
    debug_println!("best distances: {:?}", best_distances.as_rows());
    let mut dirty_indices = VecDeque::new();
    dirty_indices.push_back(height_map.start);

    while let Some(index) = dirty_indices.pop_front() {
        debug_println!("Processing index: {:?}", index);
        let height = height_map.heights[index];
        debug_println!("Height          : {:?}", height);
        if best_distances[index].is_none() {
            debug_println!("No best distance found for {:?}", index);
        }
        let distance = best_distances[index]?;
        let next_distance = distance + 1;
        for neighbor_index in get_neighbor_indices(&best_distances, index) {
            debug_println!("\tProcessing neighbor: {:?}", neighbor_index);
            let neighbor_height = height_map.heights[neighbor_index];
            debug_println!("\tNeighbor height    : {:?}", neighbor_height);
            if neighbor_height > height + 1 {
                continue;
            }
            let neighbor_distance = &mut best_distances[neighbor_index];
            match neighbor_distance {
                Some(d) if *d <= next_distance => {}
                _ => {
                    *neighbor_distance = Some(next_distance);
                    dirty_indices.push_back(neighbor_index);
                }
            }
        }
    }
    Some(best_distances)
}

fn find_best_distance_reverse(height_map: &HeightMap) -> Option<usize> {
    let mut best_distances = Array2D::filled_with(
        None,
        height_map.heights.num_rows(),
        height_map.heights.num_columns(),
    );
    best_distances[height_map.end] = Some(0);
    debug_println!("best distances: {:?}", best_distances.as_rows());
    let mut dirty_indices = VecDeque::new();
    dirty_indices.push_back(height_map.end);

    while let Some(index) = dirty_indices.pop_front() {
        debug_println!("Processing index: {:?}", index);
        let height = height_map.heights[index];
        debug_println!("Height          : {:?}", height);
        if best_distances[index].is_none() {
            debug_println!("No best distance found for {:?}", index);
        }
        let distance = best_distances[index]?;
        if height == 0 {
            return Some(distance);
        }
        let next_distance = distance + 1;
        for neighbor_index in get_neighbor_indices(&best_distances, index) {
            debug_println!("\tProcessing neighbor: {:?}", neighbor_index);
            let neighbor_height = height_map.heights[neighbor_index];
            debug_println!("\tNeighbor height    : {:?}", neighbor_height);
            if height > 0 && neighbor_height < height - 1 {
                continue;
            }
            let neighbor_distance = &mut best_distances[neighbor_index];
            match neighbor_distance {
                Some(d) if *d <= next_distance => {}
                _ => {
                    *neighbor_distance = Some(next_distance);
                    dirty_indices.push_back(neighbor_index);
                }
            }
        }
    }
    None
}

fn get_neighbor_indices<T>(
    grid: &Array2D<T>,
    (row, column): (usize, usize),
) -> Vec<(usize, usize)> {
    let mut indices = Vec::new();
    if column > 0 {
        indices.push((row, column - 1));
    }
    if row > 0 {
        indices.push((row - 1, column));
    }
    if column < grid.num_columns() - 1 {
        indices.push((row, column + 1));
    }
    if row < grid.num_rows() - 1 {
        indices.push((row + 1, column));
    }
    indices
}

impl Cell {
    fn height(&self) -> u8 {
        match self {
            Cell::Start => 0,
            Cell::End => b'z' - b'a',
            Cell::Normal(height) => *height,
        }
    }
}

impl HeightMap {
    fn parse(text: &str) -> Option<Self> {
        let rows: Vec<Vec<_>> = text
            .trim()
            .split('\n')
            .map(|row| row.chars().map(Cell::parse).collect())
            .collect::<Option<_>>()?;
        let cells = Array2D::from_rows(&rows).ok()?;

        let mut start_indices_iter = cells
            .enumerate_row_major()
            .filter(|(_, cell)| **cell == Cell::Start)
            .map(|(index, _)| index);
        let start = start_indices_iter.next()?;
        if start_indices_iter.next().is_some() {
            return None;
        }

        let mut end_indices_iter = cells
            .enumerate_row_major()
            .filter(|(_, cell)| **cell == Cell::End)
            .map(|(index, _)| index);
        let end = end_indices_iter.next()?;
        if end_indices_iter.next().is_some() {
            return None;
        }

        let heights = Array2D::from_iter_row_major(
            cells.elements_row_major_iter().map(|cell| cell.height()),
            cells.num_rows(),
            cells.num_columns(),
        )
        .ok()?;
        Some(HeightMap {
            heights,
            start,
            end,
        })
    }
}

impl Cell {
    fn parse(c: char) -> Option<Cell> {
        Some(match c {
            'S' => Cell::Start,
            'E' => Cell::End,
            'a'..='z' => Cell::Normal(c as u8 - b'a'),
            _ => return None,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let height_map = HeightMap::parse(include_str!("../../static/example12.txt")).unwrap();
        assert_eq!(height_map.start, (0, 0));
        assert_eq!(height_map.heights[(0, 0)], 0);
        assert_eq!(height_map.end, (2, 5));
        assert_eq!(height_map.heights[(2, 5)], b'z' - b'a');
        assert_eq!(height_map.heights[(0, 6)], 13);
        assert_eq!(height_map.heights[(4, 2)], 3);
    }

    #[test]
    fn test_neighbor_indices() {
        let grid = Array2D::filled_with(0, 3, 4);

        let actual = get_neighbor_indices(&grid, (1, 1));
        let expected = vec![(1, 0), (0, 1), (1, 2), (2, 1)];
        assert_eq!(expected, actual);

        let actual = get_neighbor_indices(&grid, (0, 0));
        let expected = vec![(0, 1), (1, 0)];
        assert_eq!(expected, actual);

        let actual = get_neighbor_indices(&grid, (2, 1));
        let expected = vec![(2, 0), (1, 1), (2, 2)];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_find_best_distances() {
        let height_map = HeightMap::parse(include_str!("../../static/example12.txt")).unwrap();
        let actual = find_best_distances(&height_map).unwrap();
        let expected = vec![
            vec![0, 1, 2, 19, 18, 17, 16, 15],
            vec![1, 2, 3, 20, 29, 28, 27, 14],
            vec![2, 3, 4, 21, 30, 31, 26, 13],
            vec![3, 4, 5, 22, 23, 24, 25, 12],
            vec![4, 5, 6, 7, 8, 9, 10, 11],
        ];
        let expected: Vec<Vec<_>> = expected
            .into_iter()
            .map(|row| row.into_iter().map(Some).collect())
            .collect();
        assert_eq!(expected, actual.as_rows());
    }

    #[test]
    fn test_examples_part1() {
        assert_eq!(31, Day12.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(517, Day12.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        assert_eq!(29, Day12.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        assert_eq!(512, Day12.part2(Example::Real, Debug::NotDebug));
    }
}
