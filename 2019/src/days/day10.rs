use array2d::Array2D;

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

fn solve2(_input: &str) -> Result<Value> {
    Err(String::new())
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

fn reduced_delta((x1, y1): Location, (x2, y2): Location) -> (isize, isize) {
    let dx = x2 as isize - x1 as isize;
    let dy = y2 as isize - y1 as isize;
    let divisor = gcd(dx.abs() as usize, dy.abs() as usize) as isize;
    (dx / divisor, dy / divisor)
}

fn find_asteroid_locations(grid: &Array2D<Cell>) -> Vec<Location> {
    (0..grid.num_elements())
        .map(|i| (i / grid.num_rows(), i % grid.num_rows()))
        .filter(|&loc| grid[loc] == Cell::Asteroid)
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
    vec.sort();
    vec.dedup();
    vec.len()
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn answer10a() {
        assert_eq!(solve1(INPUT), Ok(284));
    }
}
