use std::collections::{HashMap, VecDeque};

use array2d::Array2D;
use itertools::Itertools;

use crate::days::{Day, Debug, Example, Part};

const DEBUG: bool = false;
const RADIX: u32 = 10;
const START: PointOfInterestID = PointOfInterestID(0);

pub struct Day24;

impl Day for Day24 {
    fn number(&self) -> u32 {
        24
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day24 {
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        let grid = parse_lines(&self.read_file(example)).unwrap();
        shortest_path_len_points_of_interest(&grid, START).unwrap()
    }

    fn part2(&self, example: Example, _debug: Debug) -> usize {
        let grid = parse_lines(&self.read_file(example)).unwrap();
        shortest_path_len_points_of_interest_return_to_start(&grid, START).unwrap()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum Cell {
    Open,
    Wall,
    PointOfInterest(PointOfInterestID),
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct PointOfInterestID(usize);

type Grid = Array2D<Cell>;
type Index = (usize, usize);

fn parse_lines(text: &str) -> Option<Grid> {
    let rows = text
        .lines()
        .map(|line| line.chars().map(Cell::parse).collect())
        .collect::<Option<Vec<_>>>()?;
    Array2D::from_rows(&rows).ok()
}

impl Cell {
    fn parse(c: char) -> Option<Self> {
        match c {
            '.' => return Some(Cell::Open),
            '#' => return Some(Cell::Wall),
            _ => {}
        }
        let point_of_interest_id = c.to_digit(RADIX)?;
        Some(Cell::PointOfInterest(PointOfInterestID(
            point_of_interest_id as usize,
        )))
    }
}

fn shortest_path_len_points_of_interest(grid: &Grid, start: PointOfInterestID) -> Option<usize> {
    let point_of_interest_ids = get_point_of_interest_ids(grid).collect::<Vec<_>>();
    let start_index = point_of_interest_ids.iter().position(|id| *id == start)?;
    let distances = get_pairwise_distances(grid, point_of_interest_ids.iter().copied());
    let mut remaining_point_of_interest_ids = point_of_interest_ids;
    remaining_point_of_interest_ids.remove(start_index);
    let routes = remaining_point_of_interest_ids
        .iter()
        .permutations(remaining_point_of_interest_ids.len())
        .map(|route| std::iter::once(start).chain(route.into_iter().copied()));
    for route in routes.clone() {
        for PointOfInterestID(id) in route.clone() {
            debug_print!("{id} ");
        }
        debug_println!("-> {}", get_route_len(&distances, route.clone()).unwrap());
    }
    routes
        .map(|route| get_route_len(&distances, route).unwrap())
        .min()
}

fn shortest_path_len_points_of_interest_return_to_start(
    grid: &Grid,
    start: PointOfInterestID,
) -> Option<usize> {
    let point_of_interest_ids = get_point_of_interest_ids(grid).collect::<Vec<_>>();
    let start_index = point_of_interest_ids.iter().position(|id| *id == start)?;
    let distances = get_pairwise_distances(grid, point_of_interest_ids.iter().copied());
    let mut remaining_point_of_interest_ids = point_of_interest_ids;
    remaining_point_of_interest_ids.remove(start_index);
    let routes = remaining_point_of_interest_ids
        .iter()
        .permutations(remaining_point_of_interest_ids.len())
        .map(|route| {
            std::iter::once(start)
                .chain(route.into_iter().copied())
                .chain(std::iter::once(start))
        });
    for route in routes.clone() {
        for PointOfInterestID(id) in route.clone() {
            debug_print!("{id} ");
        }
        debug_println!("-> {}", get_route_len(&distances, route.clone()).unwrap());
    }
    routes
        .map(|route| get_route_len(&distances, route).unwrap())
        .min()
}

fn get_route_len<T>(
    distances: &HashMap<(PointOfInterestID, PointOfInterestID), usize>,
    route: T,
) -> Option<usize>
where
    T: IntoIterator<Item = PointOfInterestID>,
{
    let mut sum = 0;
    for pair @ (_, _) in route.into_iter().tuple_windows() {
        sum += distances.get(&pair)?;
    }
    Some(sum)
}

fn get_point_of_interest_ids(grid: &Grid) -> impl Iterator<Item = PointOfInterestID> + '_ {
    grid.elements_row_major_iter().filter_map(|cell| {
        if let Cell::PointOfInterest(id) = cell {
            Some(*id)
        } else {
            None
        }
    })
}

fn get_pairwise_distances<T>(
    grid: &Grid,
    point_of_interest_ids: T,
) -> HashMap<(PointOfInterestID, PointOfInterestID), usize>
where
    T: Iterator<Item = PointOfInterestID>,
{
    let mut result = HashMap::new();
    for from_id in point_of_interest_ids {
        let distances = get_distances_from(grid, from_id).unwrap();
        for (to_id, distance) in distances {
            result.insert((from_id, to_id), distance);
        }
    }
    result
}

fn get_distances_from(
    grid: &Grid,
    start: PointOfInterestID,
) -> Option<HashMap<PointOfInterestID, usize>> {
    let mut result = HashMap::new();
    let mut distances = Array2D::filled_with(None, grid.num_rows(), grid.num_columns());
    let start_index = grid
        .enumerate_row_major()
        .find(|(_, cell)| **cell == Cell::PointOfInterest(start))
        .map(|(index, _)| index)?;
    distances[start_index] = Some(0);
    let mut queue = VecDeque::new();
    queue.push_back(start_index);
    while let Some(index) = queue.pop_front() {
        let distance = distances[index]?;
        if let Cell::PointOfInterest(id) = grid[index] {
            result.insert(id, distance);
        }
        let next_distance = distance + 1;
        for neighbor_index in neighbor_indices(&distances, index) {
            match grid[neighbor_index] {
                Cell::Open | Cell::PointOfInterest(_) => {
                    if distances[neighbor_index].is_none() {
                        distances[neighbor_index] = Some(next_distance);
                        queue.push_back(neighbor_index);
                    }
                }
                Cell::Wall => {}
            }
        }
    }
    Some(result)
}

fn neighbor_indices<T>(grid: &Array2D<T>, (row, column): Index) -> impl Iterator<Item = Index> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // #[ignore]
    fn test_examples_part1() {
        let text = "###########\n\
                    #0.1.....2#\n\
                    #.#######.#\n\
                    #4.......3#\n\
                    ###########";
        let actual = parse_lines(text).unwrap();
        let ww = Cell::Wall;
        let oo = Cell::Open;
        let p0 = Cell::PointOfInterest(PointOfInterestID(0));
        let p1 = Cell::PointOfInterest(PointOfInterestID(1));
        let p2 = Cell::PointOfInterest(PointOfInterestID(2));
        let p3 = Cell::PointOfInterest(PointOfInterestID(3));
        let p4 = Cell::PointOfInterest(PointOfInterestID(4));
        let expected = vec![
            vec![ww, ww, ww, ww, ww, ww, ww, ww, ww, ww, ww],
            vec![ww, p0, oo, p1, oo, oo, oo, oo, oo, p2, ww],
            vec![ww, oo, ww, ww, ww, ww, ww, ww, ww, oo, ww],
            vec![ww, p4, oo, oo, oo, oo, oo, oo, oo, p3, ww],
            vec![ww, ww, ww, ww, ww, ww, ww, ww, ww, ww, ww],
        ];
        let expected = Array2D::from_rows(&expected).unwrap();
        assert_eq!(expected, actual);

        let grid = actual;
        let path_len = shortest_path_len_points_of_interest(&grid, PointOfInterestID(0)).unwrap();
        assert_eq!(14, path_len);
    }

    #[test]
    fn test_get_distances_from() {
        let text = "###########\n\
                    #0.1.....2#\n\
                    #.#######.#\n\
                    #4.......3#\n\
                    ###########";
        let grid = parse_lines(text).unwrap();
        let actual = get_distances_from(&grid, PointOfInterestID(0)).unwrap();
        let expected = [(0, 0), (1, 2), (2, 8), (3, 10), (4, 2)]
            .iter()
            .map(|(id, distance)| (PointOfInterestID(*id), *distance))
            .collect::<HashMap<_, _>>();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_pairwise_distances() {
        let text = "###########\n\
                    #0.1.....2#\n\
                    #.#######.#\n\
                    #4.......3#\n\
                    ###########";
        let grid = parse_lines(text).unwrap();
        let distances = get_pairwise_distances(&grid);
        let dist = |(from, to)| distances[&(PointOfInterestID(from), PointOfInterestID(to))];
        assert_eq!(2, dist((0, 1)));
        assert_eq!(2, dist((1, 0)));
        assert_eq!(6, dist((1, 2)));
        assert_eq!(6, dist((2, 1)));
        assert_eq!(2, dist((2, 3)));
        assert_eq!(2, dist((3, 2)));
        assert_eq!(8, dist((3, 4)));
        assert_eq!(8, dist((4, 3)));
        assert_eq!(8, dist((0, 2)));
        assert_eq!(8, dist((2, 0)));
        assert_eq!(10, dist((0, 3)));
        assert_eq!(10, dist((3, 0)));
    }

    #[test]
    fn test_get_route_len() {
        let text = "###########\n\
            #0.1.....2#\n\
            #.#######.#\n\
            #4.......3#\n\
            ###########";
        let grid = parse_lines(text).unwrap();
        let distances = get_pairwise_distances(&grid);
        let route_len = |route| get_route_len(&distances, route);
        assert_eq!(2, route_len(&[0, 1]));
        assert_eq!(2 + 6, route_len(&[0, 1, 2]));
        assert_eq!(8 + 6, route_len(&[0, 2, 1]));
        assert_eq!(8 + 6 + 4, route_len(&[0, 2, 1, 4]));
        assert_eq!(10 + 8 + 10, route_len(&[3, 0, 2, 4]));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(462, Day24.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {}

    #[test]
    fn test_real_part2() {
        assert_eq!(676, Day24.part2(Example::Real, Debug::NotDebug));
    }

    fn get_pairwise_distances(
        grid: &Grid,
    ) -> HashMap<(PointOfInterestID, PointOfInterestID), usize> {
        super::get_pairwise_distances(grid, get_point_of_interest_ids(grid))
    }

    fn get_route_len(
        distances: &HashMap<(PointOfInterestID, PointOfInterestID), usize>,
        route: &[usize],
    ) -> usize {
        let route = route.into_iter().map(|id| PointOfInterestID(*id));
        super::get_route_len(distances, route).unwrap()
    }
}
