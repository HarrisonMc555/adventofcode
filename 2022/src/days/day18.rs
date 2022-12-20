use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

use itertools::{Itertools, MinMaxResult};

use crate::days::{Day, Debug, Example, Part};
use crate::{debug_print, debug_println};

const DEBUG: bool = false;

pub struct Day18;

impl Day for Day18 {
    fn number(&self) -> u32 {
        18
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day18 {
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        let cube_positions = parse_cube_positions(&self.read_file(example)).unwrap();
        let cube_positions = cube_positions.into_iter().collect();
        total_exposed_surface_area(&cube_positions)
    }

    fn part2(&self, example: Example, _debug: Debug) -> usize {
        let cube_positions = parse_cube_positions(&self.read_file(example)).unwrap();
        let cube_positions = cube_positions.into_iter().collect();
        total_exterior_surface_area(&cube_positions)
    }
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
struct Position {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, Hash, Default, Copy, Clone, Eq, PartialEq)]
struct BoundingBox {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
    z_min: isize,
    z_max: isize,
}

fn total_exposed_surface_area(cube_positions: &HashSet<Position>) -> usize {
    cube_positions
        .iter()
        .map(|cube| cube.exposed_surface_area(cube_positions))
        .sum()
}

fn total_exterior_surface_area(cube_positions: &HashSet<Position>) -> usize {
    let mut explorer = Explorer::new(cube_positions);
    debug_println!("Bounding box: {:?}", explorer.bounding_box);
    let air_cube_iter = cube_positions
        .iter()
        .flat_map(Position::neighbor_cube_positions)
        .filter(|cube| !cube_positions.contains(cube));
    let mut exterior_surface_area = 0;
    for air_cube in air_cube_iter {
        debug_println!("Starting exploration at {}", air_cube);
        if explorer.explore_air_cube(air_cube) == Cell::ExteriorAir {
            exterior_surface_area += 1;
        }
        debug_print!("After exploration at {}: ", air_cube);
        explorer.debug_print_cells();
        debug_println!();
    }

    debug_println!("=== Final count ===");
    let mut sorted_cube_positions = cube_positions.iter().copied().collect::<Vec<_>>();
    sorted_cube_positions.sort_unstable_by_key(|position| (position.x, position.y, position.z));
    let [c1, c2, c3, ..] = sorted_cube_positions.as_slice() else {
        panic!();
    };
    debug_println!("First 3 cubes: {}, {}, {}", c1, c2, c3);
    let [.., c1, c2, c3] = sorted_cube_positions.as_slice() else {
        panic!();
    };
    debug_println!("Last  3 cubes: {}, {}, {}", c1, c2, c3);
    for cube_position in sorted_cube_positions.iter() {
        debug_println!("\tPosition {}", cube_position);
        for neighbor_position in cube_position.neighbor_cube_positions() {
            debug_print!("\t\tNeighbor: {} is ", neighbor_position);
            match explorer.cells.get(&neighbor_position) {
                None => panic!("Did not explore neighbor position {}!", neighbor_position),
                Some(Cell::ExteriorAir) => debug_println!("Exterior"),
                Some(Cell::Cube) => debug_println!("Cube"),
                Some(Cell::InteriorAir) => debug_println!("Interior"),
            }
        }
    }

    let after_count = cube_positions
        .iter()
        .flat_map(Position::neighbor_cube_positions)
        .filter(|neighbor| matches!(explorer.cells.get(neighbor), Some(Cell::ExteriorAir)))
        .count();
    debug_println!("Count as you go: {}", exterior_surface_area);
    debug_println!("Count afterward: {}", after_count);
    if exterior_surface_area != after_count {
        panic!(
            "Counts did not match. Count as you go was {}, count afterward was {}",
            exterior_surface_area, after_count
        );
    }
    exterior_surface_area
}

struct Explorer {
    cells: HashMap<Position, Cell>,
    bounding_box: BoundingBox,
    currently_exploring: HashSet<Position>,
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
enum Cell {
    ExteriorAir,
    InteriorAir,
    Cube,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum ExploreResult {
    Exterior,
    Interior,
    Unknown,
}

impl Explorer {
    fn new(cube_positions: &HashSet<Position>) -> Self {
        let cells = cube_positions
            .iter()
            .map(|position| (*position, Cell::Cube))
            .collect();
        Explorer {
            cells,
            bounding_box: find_bounding_box(cube_positions).unwrap_or_default(),
            currently_exploring: HashSet::new(),
        }
    }

    fn explore_air_cube(&mut self, air_cube: Position) -> Cell {
        let cell = match self.helper(air_cube) {
            ExploreResult::Exterior => Cell::ExteriorAir,
            ExploreResult::Interior | ExploreResult::Unknown => Cell::InteriorAir,
        };
        for explored in self.currently_exploring.iter() {
            debug_println!("\t{} is {:?}", explored, cell);
            self.cells.insert(*explored, cell);
        }
        self.currently_exploring.clear();
        cell
    }

    fn helper(&mut self, air_cube: Position) -> ExploreResult {
        if self.currently_exploring.contains(&air_cube) {
            debug_println!("\t\tSkip {}: already exploring", air_cube);
            return ExploreResult::Unknown;
        }
        match self.cells.get(&air_cube) {
            Some(Cell::ExteriorAir) => {
                debug_println!("\t\tSkip {}: already exterior", air_cube);
                return ExploreResult::Exterior;
            }
            Some(Cell::InteriorAir) => {
                debug_println!("\t\tSkip {}: already interior", air_cube);
                return ExploreResult::Interior;
            }
            Some(Cell::Cube) => {
                debug_println!("\t\tSkip {}: it's a cube", air_cube);
                return ExploreResult::Unknown;
            }
            None => {
                debug_println!("\t\tProcess {}, haven't seen yet", air_cube);
            }
        }
        if !self.bounding_box.contains(&air_cube) {
            self.cells.insert(air_cube, Cell::ExteriorAir);
            debug_println!("\t*** {} is outside bounding box, EXTERIOR ***", air_cube);
            return ExploreResult::Exterior;
        }

        self.currently_exploring.insert(air_cube);
        for neighbor in air_cube.neighbor_cube_positions().into_iter() {
            match self.helper(neighbor) {
                ExploreResult::Exterior => {
                    self.cells.insert(air_cube, Cell::ExteriorAir);
                    self.currently_exploring.remove(&air_cube);
                    debug_println!(
                        "\t*** {} has exterior neighbor {}, EXTERIOR ***",
                        air_cube,
                        neighbor
                    );
                    return ExploreResult::Exterior;
                }
                ExploreResult::Interior => {
                    self.cells.insert(air_cube, Cell::InteriorAir);
                    self.currently_exploring.remove(&air_cube);
                    debug_println!(
                        "\t*** {} has interior neighbor {}, INTERIOR ***",
                        air_cube,
                        neighbor
                    );
                    return ExploreResult::Interior;
                }
                ExploreResult::Unknown => continue,
            }
        }
        ExploreResult::Unknown
    }

    fn debug_print_cells(&self) {
        let mut first = true;
        for (position, cell) in self.cells.iter() {
            if *cell == Cell::Cube {
                continue;
            }
            if first {
                first = false;
            } else {
                debug_print!(", ");
            }
            let c = match cell {
                Cell::ExteriorAir => 'E',
                Cell::InteriorAir => 'I',
                Cell::Cube => 'C',
            };
            debug_print!("{}: {}", position, c);
        }
        debug_println!("}}");
    }
}

fn find_bounding_box<I, T>(cube_positions: I) -> Option<BoundingBox>
where
    I: IntoIterator<Item = T> + Copy,
    T: AsRef<Position>,
{
    let (x_min, x_max) = match cube_positions
        .into_iter()
        .map(|cube| cube.as_ref().x)
        .minmax()
    {
        MinMaxResult::NoElements => return None,
        MinMaxResult::OneElement(x) => (x, x),
        MinMaxResult::MinMax(x_min, x_max) => (x_min, x_max),
    };
    let (y_min, y_max) = match cube_positions
        .into_iter()
        .map(|cube| cube.as_ref().y)
        .minmax()
    {
        MinMaxResult::NoElements => return None,
        MinMaxResult::OneElement(y) => (y, y),
        MinMaxResult::MinMax(y_min, y_max) => (y_min, y_max),
    };
    let (z_min, z_max) = match cube_positions
        .into_iter()
        .map(|cube| cube.as_ref().z)
        .minmax()
    {
        MinMaxResult::NoElements => return None,
        MinMaxResult::OneElement(z) => (z, z),
        MinMaxResult::MinMax(z_min, z_max) => (z_min, z_max),
    };
    Some(BoundingBox {
        x_min,
        x_max,
        y_min,
        y_max,
        z_min,
        z_max,
    })
}

impl Position {
    fn exposed_surface_area(&self, cube_positions: &HashSet<Position>) -> usize {
        self.neighbor_cube_positions()
            .iter()
            .filter(|neighbor| !cube_positions.contains(neighbor))
            .count()
    }

    fn neighbor_cube_positions(&self) -> [Position; 6] {
        let Position { x, y, z } = *self;
        [
            Position { x: x + 1, y, z },
            Position { x: x - 1, y, z },
            Position { x, y: y + 1, z },
            Position { x, y: y - 1, z },
            Position { x, y, z: z + 1 },
            Position { x, y, z: z - 1 },
        ]
    }
}

impl BoundingBox {
    fn contains(&self, cube: &Position) -> bool {
        let BoundingBox {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
        } = self;
        let Position { x, y, z } = cube;
        x_min <= x && x <= x_max && y_min <= y && y <= y_max && z_min <= z && z <= z_max
    }
}

fn parse_cube_positions(text: &str) -> Option<Vec<Position>> {
    text.trim().split('\n').map(Position::parse).collect()
}

impl Position {
    fn parse(text: &str) -> Option<Self> {
        let mut nums = text.split(',').map(|w| w.parse().ok());
        let x = nums.next()??;
        let y = nums.next()??;
        let z = nums.next()??;
        if nums.next().is_some() {
            return None;
        }
        Some(Position { x, y, z })
    }
}

impl AsRef<Position> for Position {
    fn as_ref(&self) -> &Position {
        self
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let text = "2,2,2\n1,2,2\n3,2,2\n2,1,2";
        let cube_positions = parse_cube_positions(text).unwrap();
        assert_eq!(4, cube_positions.len());
        assert_eq!(Position { x: 2, y: 2, z: 2 }, cube_positions[0]);
        assert_eq!(Position { x: 1, y: 2, z: 2 }, cube_positions[1]);
        assert_eq!(Position { x: 3, y: 2, z: 2 }, cube_positions[2]);
        assert_eq!(Position { x: 2, y: 1, z: 2 }, cube_positions[3]);
    }

    #[test]
    fn test_examples_part1() {
        assert_eq!(64, Day18.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(3576, Day18.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        assert_eq!(58, Day18.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        assert_eq!(2066, Day18.part2(Example::Real, Debug::NotDebug));
    }
}
