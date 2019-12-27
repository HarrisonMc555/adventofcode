use crate::util::intcode::{Error, IntCode, Result, Value};
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

const INPUT: &str = include_str!("../../static/day13.txt");

lazy_static! {
    static ref EMPTY_VALUE: Value = Value::from(0);
    static ref WALL_VALUE: Value = Value::from(1);
    static ref BLOCK_VALUE: Value = Value::from(2);
    static ref HORIZONTAL_PADDLE_VALUE: Value = Value::from(3);
    static ref BALL_VALUE: Value = Value::from(4);
}

type Location = (Value, Value);
type Grid = HashMap<Location, TileId>;
type Score = Value;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Instruction {
    x: Value,
    y: Value,
    tile_id: TileId,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum TileId {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

struct Player {
    grid: Grid,
    paddle_destination: Location,
    ball_vel: (isize, isize),
}

pub fn main() {
    let answer1 = solve1(INPUT);
    println!("{:?}", answer1);
    let answer2 = solve2(INPUT);
    println!("{:?}", answer2);
}

fn solve1(input: &str) -> Result<usize> {
    let product = IntCode::from_str(input)?.run()?;
    let instructions = parse_instructions(&product.outputs())?;
    let grid = run_instructions(&instructions);
    Ok(count_block_tiles(&grid))
}

fn solve2(input: &str) -> Result<usize> {
    let mut program = IntCode::from_str(input)?;
    program.set_memory(0, Value::from(2));
    Player::play(program);
    Ok(0)
}

fn count_block_tiles(grid: &Grid) -> usize {
    grid.values()
        .filter(|tile_id| tile_id == &&TileId::Block)
        .count()
}

fn run_instructions(instructions: &[Instruction]) -> Grid {
    let mut grid = HashMap::new();
    for instruction in instructions {
        grid.insert(instruction.location(), instruction.tile_id);
    }
    grid
}

fn parse_instructions(outputs: &[Value]) -> Result<Vec<Instruction>> {
    if outputs.len() % 3 != 0 {
        return Err(format!(
            "There are {} outputs, which is not a multiple of 3",
            outputs.len()
        ));
    }
    let instructions = outputs
        .chunks_exact(3)
        .map(|chunk| {
            Ok(Instruction::new(
                chunk[0].clone(),
                chunk[1].clone(),
                (&chunk[2]).try_into()?,
            ))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(instructions)
}

impl Instruction {
    fn new(x: Value, y: Value, tile_id: TileId) -> Self {
        Instruction { x, y, tile_id }
    }

    fn location(&self) -> Location {
        (self.x.clone(), self.y.clone())
    }
}

impl TryFrom<&Value> for TileId {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let tile_id = match value {
            _ if value == &*EMPTY_VALUE => TileId::Empty,
            _ if value == &*WALL_VALUE => TileId::Wall,
            _ if value == &*BLOCK_VALUE => TileId::Block,
            _ if value == &*HORIZONTAL_PADDLE_VALUE => TileId::HorizontalPaddle,
            _ if value == &*BALL_VALUE => TileId::Ball,
            _ => return Err(format!("Invalid tile id {}", value)),
        };
        Ok(tile_id)
    }
}

impl Player {
    pub fn play(program: IntCode) -> Result<Score> {
        Ok(Value::from(0))
    }
}
