use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

// use itertools::Itertools;

use crate::days::{Day, Debug, Example, Part};

pub struct Day08;

impl Day for Day08 {
    fn number(&self) -> u32 {
        8
    }

    fn part1(&self, example: Example, _debug: Debug) -> String {
        let lines = self.get_lines(Part::Part1, example);
        // println!("lines: {lines:?}");
        let points: Vec<Point<NUM_DIMENSIONS>> = parse_points(lines).unwrap();
        // println!("points: {points:?}");
        let num_connections = match example {
            Example::Real => NUM_CONNECTIONS,
            Example::Example => NUM_CONNECTIONS_EXAMPLE,
        };
        let connections = closest_n_connections(&points, num_connections);
        // println!("connections:");
        // for connection in connections.iter() {
        //     println!(
        //         "\tP{} <-> P{} squared distance {}",
        //         connection.point_id1.0, connection.point_id2.0, connection.squared_distance
        //     );
        // }
        // println!();
        let circuits = calc_circuits(&connections);
        // println!("circuits: {circuits:?}");
        let circuit_sizes = {
            let mut circuit_sizes = circuits
                .iter()
                .map(|circuit| circuit.len())
                .collect::<Vec<_>>();
            circuit_sizes.sort_unstable_by_key(|len| Reverse(*len));
            circuit_sizes
        };
        let answer = circuit_sizes[..NUM_LARGEST_CIRCUITS]
            .iter()
            .product::<usize>();
        answer.to_string()
    }

    fn part2(&self, example: Example, _debug: Debug) -> String {
        {
            let lines = self.get_lines(Part::Part2, example);
            // println!("lines: {lines:?}");
            let points: Vec<Point<NUM_DIMENSIONS>> = parse_points(lines).unwrap();
            // println!("points: {points:?}");
            let connections = {
                let mut connections = calc_connections(&points).collect::<Vec<_>>();
                connections.sort_unstable_by_key(|con| con.squared_distance);
                connections
            };
            // println!("connections:");
            // for connection in connections.iter() {
            //     println!(
            //         "\tP{} <-> P{} squared distance {}",
            //         connection.point_id1.0, connection.point_id2.0, connection.squared_distance
            //     );
            // }
            // println!();
            let last_connection =
                find_last_connection_for_all_connected(&connections, points.len()).unwrap();
            let point1 = points[last_connection.point_id1.0];
            let point2 = points[last_connection.point_id2.0];
            let answer = point1[0] * point2[0];
            answer.to_string()
        }
    }
}

const NUM_DIMENSIONS: usize = 3;
const NUM_CONNECTIONS: usize = 1000;
const NUM_CONNECTIONS_EXAMPLE: usize = 10;
const NUM_LARGEST_CIRCUITS: usize = 3;

type Point<const N: usize> = [u64; N];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PointId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Connection {
    point_id1: PointId,
    point_id2: PointId,
    squared_distance: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CircuitId(usize);

fn calc_circuits(connections: &[Connection]) -> Vec<HashSet<PointId>> {
    let mut point_id_to_circuit_id = HashMap::<PointId, CircuitId>::new();
    let mut circuit_id_to_point_ids = HashMap::<CircuitId, HashSet<PointId>>::new();
    let mut next_circuit_id = CircuitId(0);
    for connection in connections {
        // println!("Point -> Circuit");
        // for (point_id, circuit_id) in point_id_to_circuit_id.iter() {
        //     println!("\tP{} -> C{}", point_id.0, circuit_id.0);
        // }
        // println!("Circuit -> Point");
        // for (circuit_id, point_ids) in circuit_id_to_point_ids.iter() {
        //     println!(
        //         "\tC{} -> {}",
        //         circuit_id.0,
        //         point_ids
        //             .iter()
        //             .sorted()
        //             .map(|p| format!("P{}", p.0))
        //             .join(", ")
        //     );
        // }
        let Connection {
            point_id1,
            point_id2,
            ..
        } = *connection;
        // println!("Processing P{} <-> P{}", point_id1.0, point_id2.0);
        // println!();
        let circuit_id1 = point_id_to_circuit_id.get(&point_id1).cloned();
        let circuit_id2 = point_id_to_circuit_id.get(&point_id2).cloned();
        match (circuit_id1, circuit_id2) {
            (None, None) => {
                let circuit_id = next_circuit_id;
                next_circuit_id = CircuitId(next_circuit_id.0 + 1);
                point_id_to_circuit_id.insert(point_id1, circuit_id);
                point_id_to_circuit_id.insert(point_id2, circuit_id);
                circuit_id_to_point_ids
                    .entry(circuit_id)
                    .or_default()
                    .extend([point_id1, point_id2]);
            }
            (None, Some(circuit_id2)) => {
                point_id_to_circuit_id.insert(point_id1, circuit_id2);
                circuit_id_to_point_ids
                    .get_mut(&circuit_id2)
                    .unwrap()
                    .insert(point_id1);
            }
            (Some(circuit_id1), None) => {
                point_id_to_circuit_id.insert(point_id2, circuit_id1);
                circuit_id_to_point_ids
                    .get_mut(&circuit_id1)
                    .unwrap()
                    .insert(point_id2);
            }
            (Some(circuit_id1), Some(circuit_id2)) => {
                if circuit_id1 == circuit_id2 {
                    continue;
                }
                let min_circuit_id = std::cmp::min(circuit_id1, circuit_id2);
                let max_circuit_id = std::cmp::max(circuit_id1, circuit_id2);
                let max_point_ids = circuit_id_to_point_ids.remove(&max_circuit_id).unwrap();
                for point_id in max_point_ids.iter().copied() {
                    point_id_to_circuit_id.insert(point_id, min_circuit_id);
                }
                circuit_id_to_point_ids
                    .get_mut(&min_circuit_id)
                    .unwrap()
                    .extend(max_point_ids);
            }
        }
    }
    circuit_id_to_point_ids
        .into_iter()
        .map(|(_, point_ids)| point_ids)
        .collect()
}

fn find_last_connection_for_all_connected(
    connections: &[Connection],
    num_points: usize,
) -> Option<Connection> {
    let mut point_id_to_circuit_id = HashMap::<PointId, CircuitId>::new();
    let mut circuit_id_to_point_ids = HashMap::<CircuitId, HashSet<PointId>>::new();
    let mut next_circuit_id = CircuitId(0);
    for connection in connections {
        // println!("Point -> Circuit");
        // for (point_id, circuit_id) in point_id_to_circuit_id.iter() {
        //     println!("\tP{} -> C{}", point_id.0, circuit_id.0);
        // }
        // println!("Circuit -> Point");
        // for (circuit_id, point_ids) in circuit_id_to_point_ids.iter() {
        //     println!(
        //         "\tC{} -> {}",
        //         circuit_id.0,
        //         point_ids
        //             .iter()
        //             .sorted()
        //             .map(|p| format!("P{}", p.0))
        //             .join(", ")
        //     );
        // }
        let Connection {
            point_id1,
            point_id2,
            ..
        } = *connection;
        // println!("Processing P{} <-> P{}", point_id1.0, point_id2.0);
        // println!();
        let circuit_id1 = point_id_to_circuit_id.get(&point_id1).cloned();
        let circuit_id2 = point_id_to_circuit_id.get(&point_id2).cloned();
        match (circuit_id1, circuit_id2) {
            (None, None) => {
                let circuit_id = next_circuit_id;
                next_circuit_id = CircuitId(next_circuit_id.0 + 1);
                point_id_to_circuit_id.insert(point_id1, circuit_id);
                point_id_to_circuit_id.insert(point_id2, circuit_id);
                circuit_id_to_point_ids
                    .entry(circuit_id)
                    .or_default()
                    .extend([point_id1, point_id2]);
            }
            (None, Some(circuit_id2)) => {
                point_id_to_circuit_id.insert(point_id1, circuit_id2);
                circuit_id_to_point_ids
                    .get_mut(&circuit_id2)
                    .unwrap()
                    .insert(point_id1);
            }
            (Some(circuit_id1), None) => {
                point_id_to_circuit_id.insert(point_id2, circuit_id1);
                circuit_id_to_point_ids
                    .get_mut(&circuit_id1)
                    .unwrap()
                    .insert(point_id2);
            }
            (Some(circuit_id1), Some(circuit_id2)) => {
                if circuit_id1 == circuit_id2 {
                    continue;
                }
                let min_circuit_id = std::cmp::min(circuit_id1, circuit_id2);
                let max_circuit_id = std::cmp::max(circuit_id1, circuit_id2);
                let max_point_ids = circuit_id_to_point_ids.remove(&max_circuit_id).unwrap();
                for point_id in max_point_ids.iter().copied() {
                    point_id_to_circuit_id.insert(point_id, min_circuit_id);
                }
                circuit_id_to_point_ids
                    .get_mut(&min_circuit_id)
                    .unwrap()
                    .extend(max_point_ids);
            }
        }

        if point_id_to_circuit_id.len() == num_points && circuit_id_to_point_ids.len() == 1 {
            return Some(*connection);
        }
    }
    None
}

fn closest_n_connections<const N: usize>(points: &[Point<N>], limit: usize) -> Vec<Connection> {
    #[derive(PartialEq, Eq, Debug)]
    struct Wrapper(Connection);
    impl PartialOrd for Wrapper {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for Wrapper {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0.squared_distance.cmp(&other.0.squared_distance)
        }
    }

    // Keep track of smallest N values by kicking out the largest value after inserting a value (once we reach N items)
    let mut heap = BinaryHeap::with_capacity(limit + 1);

    for connection in calc_connections(points) {
        heap.push(Wrapper(connection));
        while heap.len() > limit {
            // println!(
            //     "Current heap is too big: {}",
            //     heap.iter()
            //         .map(|w| w.0.squared_distance)
            //         .sorted()
            //         .join(", ")
            // );
            heap.pop();
            // let removed = heap.pop();
            // if let Some(removed) = removed {
            //     let removed = removed.0;
            //     println!(
            //         "\tRemoving connection P{} <-> P{} with squared distance {}",
            //         removed.point_id1.0, removed.point_id2.0, removed.squared_distance
            //     );
            // }
        }
    }

    heap.into_sorted_vec()
        .into_iter()
        .map(|wrapper| wrapper.0)
        .collect()
}

fn calc_connections<const N: usize>(points: &[Point<N>]) -> impl Iterator<Item = Connection> {
    let num_points = points.len();
    (0..num_points)
        .flat_map(move |index1| {
            ((index1 + 1)..num_points).map(move |index2| (PointId(index1), PointId(index2)))
        })
        .map(|(index1, index2)| Connection::calc(points, index1, index2))
}

impl Connection {
    pub fn calc<const N: usize>(points: &[Point<N>], index1: PointId, index2: PointId) -> Self {
        let point1 = points[index1.0];
        let point2 = points[index2.0];
        Self {
            point_id1: index1,
            point_id2: index2,
            squared_distance: squared_distance(point1, point2),
        }
    }
}

fn squared_distance<const N: usize>(point1: Point<N>, point2: Point<N>) -> u64 {
    let mut sum = 0;
    for i in 0..N {
        let diff = point1[i].abs_diff(point2[i]);
        sum += diff * diff;
    }
    sum
}

fn parse_points<const N: usize, I, S>(lines: I) -> Result<Vec<Point<N>>, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    lines
        .into_iter()
        .map(|line| parse_point(line.as_ref()))
        .collect()
}

fn parse_point<const N: usize>(line: &str) -> Result<Point<N>, String> {
    let numbers = line
        .split(",")
        .map(|word| word.parse())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Invalid number: {e}"))?;
    let array = numbers
        .try_into()
        .map_err(|_| format!("Invalid point format: {line}"))?;
    Ok(array)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples_part1() {
        assert_eq!("40", Day08.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!("54600", Day08.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        assert_eq!("25272", Day08.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        assert_eq!("107256172", Day08.part2(Example::Real, Debug::NotDebug));
    }
}
