use crate::util::intcode::{Error, IntCode, Result, Stopped, Value};
use crate::val;
use std::collections::HashMap;

const INPUT: &str = include_str!("../../static/day15.txt");

lazy_static! {
    static ref VALUE_NORTH: Value = val!(1);
    static ref VALUE_EAST: Value = val!(2);
    static ref VALUE_SOUTH: Value = val!(3);
    static ref VALUE_WEST: Value = val!(4);
    static ref VALUE_HIT_WALL: Value = val!(0);
    static ref VALUE_MOVED: Value = val!(1);
    static ref VALUE_MOVED_FOUND_GOAL: Value = val!(2);
    static ref VALUE_ONE: Value = val!(1);
}

struct Explorer {
    location: Location,
    breadcrumbs: Vec<Breadcrumb>,
    goal_location: Option<Location>,
    grid: Grid,
    program: IntCode,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
struct Breadcrumb {
    location: Location,
    direction: Direction,
}

type Location = Vec2<Value>;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
struct Vec2<T> {
    x: T,
    y: T,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
enum StatusCode {
    HitWall,
    Moved,
    MovedFoundGoal,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
enum Tile {
    Wall,
    Empty,
    Goal,
}

type Grid = HashMap<Location, Tile>;

enum Success {
    Success,
    Failure,
}

pub fn main() {
    let answer1 = solve1(INPUT);
    println!("{:?}", answer1);
    // let answer2 = solve2(INPUT);
    // println!("{:?}", answer2);
}

fn solve1(input: &str) -> Result<()> {
    let program = IntCode::from_str(input)?;
    let (grid, goal_location) = Explorer::new(program).explore()?;
    Err("unimplemented".to_string())
}

impl Explorer {
    pub fn new(program: IntCode) -> Self {
        let location = Location::new(val!(0), val!(0));
        let breadcrumbs = vec![Breadcrumb::new(location.clone(), Direction::North)];
        let goal_location = None;
        let grid = Grid::new();
        Explorer {
            location,
            breadcrumbs,
            goal_location,
            grid,
            program,
        }
    }

    pub fn explore(mut self) -> Result<(Grid, Location)> {
        while let Some(breadcrumb) = self.breadcrumbs.pop() {
            assert_eq!(breadcrumb.location, self.location);
            let status_code = self.try_move(breadcrumb.direction)?;
            self.handle_status_code(status_code, breadcrumb)?;
        }
        let goal_location = self
            .goal_location
            .ok_or_else(|| "Never found goal location")?;
        Ok((self.grid, goal_location))
    }

    fn handle_status_code(
        &mut self,
        status_code: StatusCode,
        breadcrumb: Breadcrumb,
    ) -> Result<()> {
        let Breadcrumb {
            location,
            direction,
        } = breadcrumb;
        match status_code {
            StatusCode::HitWall => {
                self.grid.insert(self.location.go(direction), Tile::Wall);
                self.hit_wall_in_direction(direction);
            }
            StatusCode::Moved => {
                self.grid.insert(location, Tile::Empty);
                self.moved_in_direction(breadcrumb.direction);
            }
            StatusCode::MovedFoundGoal => {
                self.found_goal_location(&location)?;
                self.grid.insert(location, Tile::Goal);
                self.moved_in_direction(direction);
            }
        }
        Ok(())
    }

    fn moved_in_direction(&mut self, direction: Direction) {
        self.breadcrumbs.push(Breadcrumb::new(self.location.clone(), direction));
        self.location = self.location.go(direction);
        // if let Some(next_breadcrumb) = self.next_breadcrumb(direction) {
        //     self.breadcrumbs.push(next_breadcrumb);
        // }
        // let forward_breadcrumb = self.step_forward(direction);
        // self.breadcrumbs.push(forward_breadcrumb);
        // self.location = location.go(direction);
    }

    fn hit_wall_in_direction(&mut self, direction: Direction) {
        
    }

    fn found_goal_location(&mut self, location: &Location) -> Result<()> {
        if let Some(old_goal_location) = &self.goal_location {
            if location != old_goal_location {
                return Err(format!(
                    "Found goal location at {:?}, but it was already found at {:?}. ",
                    location, old_goal_location
                ));
            }
        }
        Ok(())
    }

    fn try_move(&mut self, direction: Direction) -> Result<StatusCode> {
        println!("Pushing input: {:?}", direction.to_command());
        self.program.push_input(direction.to_command());
        println!("Pushed input, running");
        let stopped = self.program.run_blocking_input()?;
        assert!(stopped == Stopped::NeedInput);
        println!("Ran, popping output");
        let value = self.program.pop_output()?;
        println!("Output: {:?}", value);
        StatusCode::try_from_value(value)
    }
}

impl Direction {
    pub fn to_command(&self) -> Value {
        match self {
            Direction::North => VALUE_NORTH.clone(),
            Direction::East => VALUE_EAST.clone(),
            Direction::South => VALUE_SOUTH.clone(),
            Direction::West => VALUE_WEST.clone(),
        }
    }
}

impl StatusCode {
    pub fn try_from_value(value: Value) -> Result<Self> {
        Ok(match () {
            _ if value == *VALUE_HIT_WALL => StatusCode::HitWall,
            _ if value == *VALUE_MOVED => StatusCode::Moved,
            _ if value == *VALUE_MOVED_FOUND_GOAL => StatusCode::MovedFoundGoal,
            _ => return Err(format!("Invalid status code {}", value)),
        })
    }
}

impl Breadcrumb {
    pub fn new(location: Location, direction: Direction) -> Self {
        Breadcrumb {
            location,
            direction,
        }
    }
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }
}

impl Location {
    pub fn go(&self, direction: Direction) -> Self {
        let (x, y) = (self.x.clone(), self.y.clone());
        let (new_x, new_y) = match direction {
            Direction::North => (x, y + &*VALUE_ONE),
            Direction::East => (x + &*VALUE_ONE, y),
            Direction::South => (x, y - &*VALUE_ONE),
            Direction::West => (x - &*VALUE_ONE, y),
        };
        Location::new(new_x, new_y)
    }
}
