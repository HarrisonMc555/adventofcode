use crate::util::intcode::{Error, IntCode, Result, Stopped, Value};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;

const INPUT: &str = include_str!("../../static/day13.txt");

const QUARTERS_MEMORY_INDEX: usize = 0;

lazy_static! {
    static ref EMPTY_VALUE: Value = Value::from(0);
    static ref WALL_VALUE: Value = Value::from(1);
    static ref BLOCK_VALUE: Value = Value::from(2);
    static ref HORIZONTAL_PADDLE_VALUE: Value = Value::from(3);
    static ref BALL_VALUE: Value = Value::from(4);
    static ref SCORE_X_VALUE: Value = Value::from(-1);
    static ref SCORE_Y_VALUE: Value = Value::from(0);
    static ref NUM_QUARTERS: Value = Value::from(2);
    static ref JOYSTICK_LEFT: Value = Value::from(-1);
    static ref JOYSTICK_NEUTRAL: Value = Value::from(0);
    static ref JOYSTICK_RIGHT: Value = Value::from(1);
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
struct Vec2<T> {
    x: T,
    y: T,
}

type Location = Vec2<Value>;
type Grid = HashMap<Location, TileId>;
type Score = Value;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Instruction {
    Tile(Tile),
    Score(Score),
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Tile {
    location: Location,
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
    game: Game,
    program: IntCode,
}

struct Game {
    grid: Grid,
    score: Score,
    ball_loc: Location,
    paddle_loc: Location,
}

enum GameStatus {
    Stopped,
    Going,
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
    let grid = run_instructions(instructions);
    Ok(count_block_tiles(&grid))
}

fn solve2(input: &str) -> Result<Score> {
    let mut program = IntCode::from_str(input)?;
    program.set_memory(QUARTERS_MEMORY_INDEX, NUM_QUARTERS.clone());
    Player::play(program)
}

fn count_block_tiles(grid: &Grid) -> usize {
    grid.values()
        .filter(|tile_id| tile_id == &&TileId::Block)
        .count()
}

fn run_instructions(instructions: Vec<Instruction>) -> Grid {
    let mut grid = HashMap::new();
    for instruction in instructions {
        match instruction {
            Instruction::Tile(tile) => {
                grid.insert(tile.location.clone(), tile.tile_id);
            }
            Instruction::Score(_) => {
                eprintln!("No scores expected");
            }
        }
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
        .map(|chunk| Instruction::new(chunk[0].clone(), chunk[1].clone(), chunk[2].clone()))
        .collect::<Result<Vec<_>>>()?;
    Ok(instructions)
}

impl Instruction {
    fn new(i1: Value, i2: Value, i3: Value) -> Result<Self> {
        if i1 == *SCORE_X_VALUE && i2 == *SCORE_Y_VALUE {
            Ok(Instruction::Score(i3))
        } else {
            Ok(Instruction::Tile(Tile::new(i1, i2, TileId::try_from(&i3)?)))
        }
    }
}

impl Tile {
    fn new(x: Value, y: Value, tile_id: TileId) -> Self {
        Tile {
            location: Location::new(x, y),
            tile_id,
        }
    }
}

impl TryFrom<&Value> for TileId {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let tile_id = match () {
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
        Player::new(program)?.run()
    }

    fn new(mut program: IntCode) -> Result<Self> {
        match program.run_blocking_input()? {
            Stopped::Complete => return Err("Game stopped".to_string()),
            Stopped::NeedInput => (),
        }
        let instructions = parse_instructions(&program.take_outputs())?;
        let game = Game::init_from_instructions(instructions)?;
        Ok(Player { game, program })
    }

    fn run(mut self) -> Result<Score> {
        while let GameStatus::Going = self.step()? {
            self.program.push_input(self.get_command());
        }
        // Do one more step to account for the last outputs before the program
        // ended
        self.step()?;
        self.game.print();
        Ok(self.game.score)
    }

    fn get_command(&self) -> Value {
        Player::joystick_command(&self.game.paddle_loc.x, &self.game.ball_loc.x)
    }

    fn step(&mut self) -> Result<GameStatus> {
        match self.program.run_blocking_input()? {
            Stopped::Complete => return Ok(GameStatus::Stopped),
            Stopped::NeedInput => (),
        }
        let instructions = parse_instructions(&self.program.take_outputs())?;
        self.game.run_instructions(instructions);
        Ok(GameStatus::Going)
    }

    fn joystick_command(from: &Value, to: &Value) -> Value {
        match from.cmp(to) {
            Ordering::Less => JOYSTICK_RIGHT.clone(),
            Ordering::Equal => JOYSTICK_NEUTRAL.clone(),
            Ordering::Greater => JOYSTICK_LEFT.clone(),
        }
    }
}

impl Game {
    fn new(grid: Grid, score: Score, ball_loc: Location, paddle_loc: Location) -> Self {
        Game {
            grid,
            score,
            ball_loc,
            paddle_loc,
        }
    }

    fn init_from_instructions(instructions: Vec<Instruction>) -> Result<Self> {
        let mut grid = HashMap::new();
        let mut score = None;
        let mut ball_loc = None;
        let mut paddle_loc = None;
        for instruction in instructions {
            match instruction {
                Instruction::Tile(tile) => {
                    match tile.tile_id {
                        TileId::Ball => ball_loc = Some(tile.location.clone()),
                        TileId::HorizontalPaddle => paddle_loc = Some(tile.location.clone()),
                        _ => (),
                    }
                    grid.insert(tile.location, tile.tile_id);
                }
                Instruction::Score(score_value) => {
                    score = Some(score_value);
                }
            }
        }
        let score = score.unwrap_or_default();
        let ball_loc = ball_loc.ok_or_else(|| "No ball found".to_string())?;
        let paddle_loc = paddle_loc.ok_or_else(|| "No paddle found".to_string())?;
        Ok(Game::new(grid, score, ball_loc, paddle_loc))
    }

    fn run_instructions(&mut self, instructions: Vec<Instruction>) {
        for instruction in instructions {
            match instruction {
                Instruction::Tile(tile) => {
                    match tile.tile_id {
                        TileId::Ball => self.ball_loc = tile.location.clone(),
                        TileId::HorizontalPaddle => self.paddle_loc = tile.location.clone(),
                        _ => (),
                    }
                    self.grid.insert(tile.location, tile.tile_id);
                }
                Instruction::Score(score_value) => {
                    self.score = score_value;
                }
            }
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("Score: {}", self.score);
        let (min_x, max_x) = (0, 44);
        let (min_y, max_y) = (0, 20);
        for y in min_y..max_y {
            for x in min_x..max_x {
                let tile_id = self
                    .grid
                    .get(&Location::new(Value::from(x), Value::from(y)))
                    .unwrap();
                print!("{}", tile_id);
            }
            println!();
        }
    }
}

impl fmt::Display for TileId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            TileId::Empty => " ",
            TileId::Wall => "#",
            TileId::Block => "x",
            TileId::HorizontalPaddle => "-",
            TileId::Ball => "o",
        };
        write!(f, "{}", c)
    }
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }
}
