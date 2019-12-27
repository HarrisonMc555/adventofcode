use crate::util::math;
use array2d::Array2D;
use std::collections::HashMap;

const INPUT: &str = include_str!("../../static/day10.txt");

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cell {
    Asteroid,
    Empty,
}

type Result<T> = std::result::Result<T, String>;
type Value = usize;
type Location = (usize, usize);

pub fn main() {
    let answer1 = solve1(INPUT);
    println!("{:?}", answer1);
    let answer2 = solve2(INPUT);
    println!("{:?}", answer2);
}

fn solve1(input: &str) -> Result<Value> {
    let grid = parse_input(input)?;
    Ok(most_asteroids_seen(&grid))
}

fn solve2(input: &str) -> Result<Value> {
    let grid = parse_input(input)?;
    let locations = asteroids_destroyed_in_order(&grid);
    let (x, y) = locations
        .get(200 - 1)
        .ok_or_else(|| "Not enough asteroids")?;
    Ok(100 * x + y)
}

fn asteroids_destroyed_in_order(grid: &Array2D<Cell>) -> Vec<Location> {
    let asteroid_locations = find_asteroid_locations(&grid);
    let station_location = match find_station_location(&asteroid_locations) {
        Some(location) => location,
        None => return Vec::new(),
    };
    let reduced_delta_to_delta = asteroid_locations
        .iter()
        .filter(|&&loc| loc != station_location)
        .map(|&loc| {
            (
                reduced_delta(station_location, loc),
                delta(station_location, loc),
            )
        })
        .fold(HashMap::new(), |mut map, (delta, loc)| {
            let mut vec = map.entry(delta).or_insert_with(Vec::new);
            let key_function = |&(x, y): &(isize, isize)| (x.abs(), y.abs());
            insert_into_sorted_by_key(&mut vec, loc, key_function);
            map
        });
    let mut reduced_delta_to_delta = reduced_delta_to_delta.iter().collect::<Vec<_>>();
    reduced_delta_to_delta.sort_unstable_by(|&(&(dx1, dy1), _), &(&(dx2, dy2), _)| {
        let angle1 = atan2(dx1, dy1);
        let angle2 = atan2(dx2, dy2);
        angle1.partial_cmp(&angle2).unwrap()
    });
    let mut deltas = reduced_delta_to_delta
        .iter()
        .map(|&(_, v)| v)
        .collect::<Vec<_>>();
    let mut result = Vec::new();
    let mut outer_index = 0;
    let mut inner_index = 0;
    while !deltas.is_empty() {
        if let Some(delta) = deltas.get(outer_index).and_then(|v| v.get(inner_index)) {
            result.push(undelta(station_location, *delta));
            outer_index += 1;
        } else {
            deltas.remove(outer_index);
        }
        if outer_index == deltas.len() {
            outer_index = 0;
            inner_index += 1;
        }
    }
    result
}

fn undelta((x, y): Location, (dx, dy): (isize, isize)) -> Location {
    ((x as isize + dx) as usize, (y as isize + dy) as usize)
}

fn find_station_location(asteroid_locations: &[Location]) -> Option<Location> {
    asteroid_locations
        .iter()
        .max_by_key(|&&loc| asteroids_seen_from(loc, &asteroid_locations))
        .cloned()
}

fn most_asteroids_seen(grid: &Array2D<Cell>) -> usize {
    let asteroid_locations = find_asteroid_locations(&grid);
    asteroid_locations
        .iter()
        .map(|&loc| asteroids_seen_from(loc, &asteroid_locations))
        .max()
        .unwrap_or(0)
}

fn asteroids_seen_from(source: Location, asteroid_locations: &[Location]) -> usize {
    let deltas = asteroid_locations
        .iter()
        .filter(|&&dest| dest != source)
        .map(|&dest| reduced_delta(source, dest))
        .collect::<Vec<_>>();
    num_unique(deltas)
}

fn delta((x1, y1): Location, (x2, y2): Location) -> (isize, isize) {
    let dx = x2 as isize - x1 as isize;
    let dy = y2 as isize - y1 as isize;
    (dx, dy)
}

fn reduced_delta(loc1: Location, loc2: Location) -> (isize, isize) {
    let (dx, dy) = delta(loc1, loc2);
    let divisor = math::gcd(dx.abs() as usize, dy.abs() as usize) as isize;
    let divisor = if divisor == 0 { 1 } else { divisor };
    (dx / divisor, dy / divisor)
}

fn find_asteroid_locations(grid: &Array2D<Cell>) -> Vec<Location> {
    (0..grid.num_elements())
        .map(|i| (i % grid.row_len(), i / grid.row_len()))
        .filter(|&(x, y)| grid[(y, x)] == Cell::Asteroid)
        .collect::<Vec<_>>()
}

fn parse_input(input: &str) -> Result<Array2D<Cell>> {
    let grid = input
        .trim()
        .lines()
        .map(|line| line.chars().map(char_to_cell).collect::<Result<Vec<_>>>())
        .collect::<Result<Vec<_>>>()?;
    Array2D::from_rows(&grid).map_err(|_| "Bad grid dimensions".to_string())
}

fn char_to_cell(c: char) -> Result<Cell> {
    Ok(match c {
        '#' => Cell::Asteroid,
        '.' => Cell::Empty,
        _ => return Err(format!("Invalid cell {}", c)),
    })
}

fn num_unique<T>(mut vec: Vec<T>) -> usize
where
    T: Eq + Ord,
{
    vec.sort_unstable();
    vec.dedup();
    vec.len()
}

fn atan2(x: isize, y: isize) -> f32 {
    std::f32::consts::PI - (x as f32).atan2(y as f32)
}

fn insert_into_sorted_by_key<T, B, F>(vec: &mut Vec<T>, item: T, mut f: F)
where
    B: Ord,
    F: FnMut(&'_ T) -> B,
{
    match vec.binary_search_by_key(&f(&item), f) {
        Ok(_) => {} // Already present
        Err(pos) => vec.insert(pos, item),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn answer1() {
        assert_eq!(solve1(INPUT), Ok(284));
    }

    #[test]
    fn answer2() {
        assert_eq!(solve2(INPUT), Ok(404));
    }
}
