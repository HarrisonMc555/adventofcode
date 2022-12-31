use std::collections::{HashMap, HashSet};

use crate::days::{Day, Debug, Example, Part};
use crate::{debug_print, debug_println};

const DEBUG: bool = false;

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
    fn part1(&self, example: Example, _debug: Debug) -> isize {
        let (board, commands) = parse(&self.read_file(example)).unwrap();
        let state = simulate(board, &commands).unwrap();
        get_password(state)
    }

    fn part2(&self, example: Example, _debug: Debug) -> isize {
        let (board, commands) = parse(&self.read_file(example)).unwrap();
        let square_width = match example {
            Example::Real => SQUARE_WIDTH,
            Example::Example => SQUARE_WIDTH_EXAMPLE,
        };
        let state = simulate2(board, &commands, square_width)
            .unwrap()
            .into_state();
        get_password(state)
    }
}

const BASE: u32 = 10;
const SQUARE_WIDTH: isize = 50;
const SQUARE_WIDTH_EXAMPLE: isize = 4;
type Commands = Vec<Command>;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Board {
    cells: HashMap<(isize, isize), Cell>,
    num_rows: isize,
    num_cols: isize,
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
enum Cell {
    Open,
    Solid,
    Nonexistent,
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
enum Command {
    Forward(u32),
    Turn(TurnDirection),
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
enum TurnDirection {
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    board: Board,
    index: Index,
    orientation: Orientation,
}

#[derive(Debug, Ordinalize, Hash, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Orientation {
    Up,
    #[default]
    Right,
    Down,
    Left,
}

fn get_password(state: State) -> isize {
    let (row, col) = state.index;
    let row = row + 1;
    let col = col + 1;
    let facing = match state.orientation {
        Orientation::Right => 0,
        Orientation::Down => 1,
        Orientation::Left => 2,
        Orientation::Up => 3,
    };
    1000 * row + 4 * col + facing
}

fn simulate(board: Board, commands: &Commands) -> Option<State> {
    let mut state = State::new(board)?;
    for command in commands {
        state.run_command(*command)
    }
    Some(state)
}

impl State {
    fn new(board: Board) -> Option<Self> {
        let index = board.first_open()?;
        Some(State {
            board,
            index,
            orientation: Orientation::default(),
        })
    }

    fn run_command(&mut self, command: Command) {
        match command {
            Command::Forward(amount) => self.forward(amount),
            Command::Turn(direction) => self.turn(direction),
        }
    }

    fn forward(&mut self, amount: u32) {
        for _ in 0..amount {
            self.step();
        }
    }

    fn step(&mut self) {
        let (diff_row, diff_col) = match self.orientation {
            Orientation::Up => (-1, 0),
            Orientation::Right => (0, 1),
            Orientation::Down => (1, 0),
            Orientation::Left => (0, -1),
        };

        let (mut row, mut col) = self.index;
        loop {
            let new_row = (row + diff_row).rem_euclid(self.board.num_rows);
            let new_col = (col + diff_col).rem_euclid(self.board.num_cols);
            match self.board.cells.get(&(new_row, new_col)) {
                Some(Cell::Open) => {
                    self.index = (new_row, new_col);
                    return;
                }
                Some(Cell::Solid) => return,
                Some(Cell::Nonexistent) | None => {
                    row = new_row;
                    col = new_col;
                    continue;
                }
            }
        }
    }

    fn turn(&mut self, direction: TurnDirection) {
        match direction {
            TurnDirection::Left => self.orientation = self.orientation.counter_clockwise(),
            TurnDirection::Right => self.orientation = self.orientation.clockwise(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct State2 {
    board: Board,
    index: Index,
    orientation: Orientation,
    face_connections: HashMap<(Index, Orientation), (Index, Orientation)>,
    square_width: isize,
}

type Index = (isize, isize);

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
struct Orientation3D {
    axis: Axis,
    sign: Sign,
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
struct FaceOrientation {
    normal: Orientation3D,
    up: Orientation3D,
}

fn simulate2(board: Board, commands: &Commands, square_width: isize) -> Option<State2> {
    let mut state = State2::new(board, square_width)?;
    for (i, command) in commands.iter().enumerate() {
        debug_println!(
            "Running command {}/{}: {:?}",
            i + 1,
            commands.len(),
            command
        );
        state.run_command(*command)
    }
    Some(state)
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
enum MoveResult {
    Moved,
    HitWall,
}

impl State2 {
    fn new(board: Board, square_width: isize) -> Option<Self> {
        debug_println!("Creating new board with width {}", square_width);
        let index = board.first_open()?;
        let face_connections = create_face_connections(&board, square_width)?;
        debug_println!("Face connections:");
        let mut vec = face_connections.iter().collect::<Vec<_>>();
        vec.sort_unstable_by_key(|(from, _)| **from);
        for (from, to) in vec {
            debug_println!("\t{:?}    \t-> {:?}", from, to);
        }
        Some(State2 {
            board,
            index,
            orientation: Orientation::default(),
            face_connections,
            square_width,
        })
    }

    fn run_command(&mut self, command: Command) {
        match command {
            Command::Forward(amount) => self.forward(amount),
            Command::Turn(direction) => self.turn(direction),
        }
    }

    fn forward(&mut self, amount: u32) {
        debug_println!("%%% Forward: {} %%%", amount);
        for _ in 0..amount {
            if self.step() == MoveResult::HitWall {
                debug_println!("Hit wall, not performing remaining steps forward");
                break;
            }
        }
    }

    fn step(&mut self) -> MoveResult {
        debug_println!("% Step %");
        let (diff_row, diff_col) = match self.orientation {
            Orientation::Up => (-1, 0),
            Orientation::Right => (0, 1),
            Orientation::Down => (1, 0),
            Orientation::Left => (0, -1),
        };

        let (row, col) = self.index;
        let new_row = row + diff_row;
        let new_col = col + diff_col;
        let new_index = (new_row, new_col);
        let result = match self.board.cells.get(&new_index) {
            Some(Cell::Open) => {
                self.index = new_index;
                MoveResult::Moved
            }
            Some(Cell::Solid) => MoveResult::HitWall,
            Some(Cell::Nonexistent) | None => self.step_to_next_face(),
        };
        self.debug_print();
        debug_println!();
        debug_println!();
        result
    }

    fn step_to_next_face(&mut self) -> MoveResult {
        let cur_face_index = self.face_index(self.index);
        let (row, col) = self.index;
        let cur_edge_distance = match self.orientation {
            Orientation::Up => col.rem_euclid(self.square_width),
            Orientation::Down => (-col - 1).rem_euclid(self.square_width),
            Orientation::Right => row.rem_euclid(self.square_width),
            Orientation::Left => (-row - 1).rem_euclid(self.square_width),
        };
        let (new_face_index, new_orientation) = self
            .face_connections
            .get(&(cur_face_index, self.orientation))
            .copied()
            .unwrap();
        let (new_row_diff, new_col_diff) = match new_orientation {
            Orientation::Up => (self.square_width - 1, cur_edge_distance),
            Orientation::Down => (0, (-cur_edge_distance - 1).rem_euclid(self.square_width)),
            Orientation::Right => (cur_edge_distance, 0),
            Orientation::Left => (
                (-cur_edge_distance - 1).rem_euclid(self.square_width),
                self.square_width - 1,
            ),
        };
        let (face_row, face_col) = new_face_index;
        let new_row = face_row * self.square_width + new_row_diff;
        let new_col = face_col * self.square_width + new_col_diff;
        let new_index = (new_row, new_col);
        debug_println!(
            "Orientation: {:?}, index: {:?}, cur_edge_distance: {}, new_orientation: {:?}, new diffs: {:?}, \
            new_index: {:?}",
            self.orientation,
            self.index,
            cur_edge_distance,
            new_orientation,
            (new_row_diff, new_col_diff),
            new_index,
        );

        if let Some(Cell::Solid) = self.board.cells.get(&new_index) {
            debug_println!("Stepping to next face hits wall, not moving");
            return MoveResult::HitWall;
        }

        self.index = new_index;
        self.orientation = new_orientation;
        MoveResult::Moved
    }

    fn face_index(&self, (row, col): Index) -> Index {
        (row / self.square_width, col / self.square_width)
    }

    fn turn(&mut self, direction: TurnDirection) {
        debug_println!("%%% Turn: {:?} %%%", direction);
        match direction {
            TurnDirection::Left => self.orientation = self.orientation.counter_clockwise(),
            TurnDirection::Right => self.orientation = self.orientation.clockwise(),
        }
        self.debug_print();
        debug_println!();
    }

    fn into_state(self) -> State {
        State {
            board: self.board,
            index: self.index,
            orientation: self.orientation,
        }
    }

    fn debug_print(&self) {
        if !DEBUG {
            return;
        }
        for row in 0..self.board.num_rows {
            for col in 0..self.board.num_cols {
                let index = (row, col);
                let c = if index == self.index {
                    self.orientation.to_char()
                } else {
                    self.board
                        .cells
                        .get(&(row, col))
                        .map(|cell| cell.to_char())
                        .unwrap_or(' ')
                };
                debug_print!("{}", c);
            }
            debug_println!();
        }
    }
}

fn create_face_connections(
    board: &Board,
    square_width: isize,
) -> Option<HashMap<(Index, Orientation), (Index, Orientation)>> {
    let face_orientations = create_face_orientations(board, square_width)?;
    let orientations = Orientation::variants();
    face_orientations
        .iter()
        .flat_map(|(normal, (index, up))| {
            orientations.iter().map(|orientation| {
                let face_orientation = FaceOrientation {
                    normal: *normal,
                    up: *up,
                };
                create_face_connection(&face_orientations, *index, face_orientation, *orientation)
            })
        })
        .collect()
}

fn create_face_connection(
    face_orientations: &HashMap<Orientation3D, (Index, Orientation3D)>,
    index: Index,
    face_orientation: FaceOrientation,
    orientation: Orientation,
) -> Option<((Index, Orientation), (Index, Orientation))> {
    let rotated = face_orientation.rotate(orientation)?;
    let (next_index, next_up) = face_orientations.get(&rotated.normal)?;
    let mut next_orientation: Orientation = orientation;
    let mut rotated_up = rotated.up;
    debug_println!(
        "Leaving square {:?} @ {:?} going {:?}",
        index,
        face_orientation,
        orientation
    );
    debug_println!("Next square: {:?}, next up: {:?}", next_index, next_up);
    while rotated_up != *next_up {
        debug_println!(
            "\tRotated up {:?} does NOT match next up {:?}",
            rotated_up,
            next_up
        );
        rotated_up = rotated_up.clockwise_around(rotated.normal)?;
        next_orientation = next_orientation.counter_clockwise();
    }
    debug_println!(
        "Rotated up {:?} matches next up {:?}, next orientation: {:?}",
        rotated_up,
        next_up,
        next_orientation
    );
    Some(((index, orientation), (*next_index, next_orientation)))
}

fn create_face_orientations(
    board: &Board,
    square_width: isize,
) -> Option<HashMap<Orientation3D, (Index, Orientation3D)>> {
    if square_width == 0 {
        return None;
    }
    let (row, col) = board.first_open()?;
    let index = (row / square_width, col / square_width);
    let face_orientation = FaceOrientation {
        normal: Orientation3D {
            axis: Axis::Z,
            sign: Sign::Positive,
        },
        up: Orientation3D {
            axis: Axis::Y,
            sign: Sign::Positive,
        },
    };
    let mut face_orientation_map = HashMap::new();
    let possible_indices = (0..=board.num_rows / square_width)
        .flat_map(|row| {
            (0..=board.num_cols / square_width)
                .filter(move |col| {
                    board
                        .cells
                        .get(&(row * square_width, col * square_width))
                        .is_some()
                })
                .map(move |col| (row, col))
        })
        .collect();

    fn helper(
        index_to_orientation3d: &mut HashMap<Orientation3D, (Index, Orientation3D)>,
        possible_indices: &HashSet<Index>,
        index: Index,
        face_orientation: FaceOrientation,
        count: &mut usize,
    ) {
        if *count > 1000 {
            panic!("Infinite loop");
        }
        *count += 1;
        debug_println!("Helper for {:?} and {:?}", index, face_orientation);
        let FaceOrientation { normal, up } = face_orientation;
        if index_to_orientation3d.contains_key(&normal) {
            debug_println!("Already found orientation for {:?}", index);
            return;
        }
        if !possible_indices.contains(&index) {
            debug_println!("Index {:?} is not a possible index", index);
            return;
        }
        index_to_orientation3d.insert(normal, (index, up));
        for orientation in Orientation::variants() {
            let new_index @ (row, col) = shift(index, orientation);
            debug_println!("row: {}, col: {}", row, col);
            let new_orientation = face_orientation.rotate(orientation).unwrap();
            helper(
                index_to_orientation3d,
                possible_indices,
                new_index,
                new_orientation,
                count,
            );
        }
    }

    helper(
        &mut face_orientation_map,
        &possible_indices,
        index,
        face_orientation,
        &mut 0,
    );

    debug_println!("Face orientation map:");
    let mut items = face_orientation_map.iter().collect::<Vec<_>>();
    items.sort_unstable_by_key(|(_, (i, _))| *i);
    for x in items {
        debug_println!("\t{:?}", x);
    }

    Some(face_orientation_map)
}

fn shift((row, col): Index, orientation: Orientation) -> Index {
    match orientation {
        Orientation::Up => (row - 1, col),
        Orientation::Right => (row, col + 1),
        Orientation::Down => (row + 1, col),
        Orientation::Left => (row, col - 1),
    }
}

impl FaceOrientation {
    fn rotate(self, orientation: Orientation) -> Option<FaceOrientation> {
        let FaceOrientation { normal, up } = self;
        Some(match orientation {
            Orientation::Up => FaceOrientation {
                normal: up,
                up: -normal,
            },
            Orientation::Down => FaceOrientation {
                normal: -up,
                up: normal,
            },
            Orientation::Right => FaceOrientation {
                normal: -normal.cross(up)?,
                up,
            },
            Orientation::Left => FaceOrientation {
                normal: normal.cross(up)?,
                up,
            },
        })
    }
}

impl std::ops::Neg for Orientation3D {
    type Output = Orientation3D;

    fn neg(self) -> Self::Output {
        Orientation3D {
            sign: -self.sign,
            ..self
        }
    }
}

impl std::ops::Neg for Sign {
    type Output = Sign;

    fn neg(self) -> Self::Output {
        match self {
            Sign::Positive => Sign::Negative,
            Sign::Negative => Sign::Positive,
        }
    }
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
enum Sign {
    Positive,
    Negative,
}

impl Orientation3D {
    fn cross(self, rhs: Self) -> Option<Orientation3D> {
        use Axis::*;
        use Sign::*;
        let axis = match (self.axis, rhs.axis) {
            (X, Y) | (Y, X) => Z,
            (X, Z) | (Z, X) => Y,
            (Y, Z) | (Z, Y) => X,
            _ => {
                return None;
            }
        };

        let cross_sign = match (self.axis, rhs.axis) {
            (X, Y) | (Y, Z) | (Z, X) => Positive,
            (Y, X) | (Z, Y) | (X, Z) => Negative,
            _ => {
                return None;
            }
        };
        let sign = self.sign * rhs.sign * cross_sign;
        Some(Orientation3D { axis, sign })
    }

    fn clockwise_around(self, rotation_orientation: Orientation3D) -> Option<Orientation3D> {
        self.cross(rotation_orientation)
    }
}

impl std::ops::Mul for Sign {
    type Output = Sign;

    fn mul(self, rhs: Self) -> Self::Output {
        use Sign::*;
        match (self, rhs) {
            (Positive, Positive) | (Negative, Negative) => Positive,
            (Positive, Negative) | (Negative, Positive) => Negative,
        }
    }
}

impl Orientation {
    fn clockwise(self) -> Self {
        match self {
            Orientation::Up => Orientation::Right,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
        }
    }

    fn counter_clockwise(self) -> Self {
        match self {
            Orientation::Up => Orientation::Left,
            Orientation::Left => Orientation::Down,
            Orientation::Down => Orientation::Right,
            Orientation::Right => Orientation::Up,
        }
    }

    fn to_char(self) -> char {
        match self {
            Orientation::Up => '^',
            Orientation::Right => '>',
            Orientation::Down => 'v',
            Orientation::Left => '<',
        }
    }
}

impl Board {
    fn first_open(&self) -> Option<(isize, isize)> {
        self.cells
            .iter()
            .filter(|(_, cell)| **cell == Cell::Open)
            .map(|(index, _)| index)
            .copied()
            .min()
    }
}

fn parse(text: &str) -> Option<(Board, Commands)> {
    let (board_text, commands_text) = text.split_once("\n\n")?;
    debug_println!("Board text:\n{}\n", board_text);
    let board = parse_board(board_text)?;
    debug_println!("Commands text:\n{}\n", commands_text);
    let commands = parse_commands(commands_text)?;
    Some((board, commands))
}

fn parse_board(text: &str) -> Option<Board> {
    let rows = text
        .lines()
        .map(|line| line.chars().map(Cell::parse).collect::<Option<Vec<_>>>())
        .collect::<Option<Vec<_>>>()?;
    for row in rows.iter() {
        for cell in row.iter() {
            debug_print!("{}", cell.to_char());
        }
        debug_println!();
    }

    let num_rows = rows.len() as isize;
    let num_cols = rows.iter().map(|row| row.len()).max()? as isize;
    let mut cells = HashMap::new();
    for (row_index, row) in rows.into_iter().enumerate() {
        for (col_index, cell) in row.into_iter().enumerate() {
            if cell != Cell::Nonexistent {
                cells.insert((row_index as isize, col_index as isize), cell);
            }
        }
    }

    Some(Board {
        cells,
        num_rows,
        num_cols,
    })
}

fn parse_commands(text: &str) -> Option<Commands> {
    let mut chars = text.trim().chars();
    let mut digits = Vec::new();
    let mut commands = Vec::new();
    loop {
        let Some(c) = chars.next() else {
            break;
        };
        match c.to_digit(BASE) {
            Some(digit) => digits.push(digit),
            None => {
                let amount = from_digits(&digits);
                digits.clear();
                let direction = TurnDirection::parse(c)?;
                commands.push(Command::Forward(amount));
                commands.push(Command::Turn(direction));
            }
        }
    }
    if digits.is_empty() {
        debug_println!("Should end with forward command");
        return None;
    }
    let amount = from_digits(&digits);
    commands.push(Command::Forward(amount));
    Some(commands)
}

impl Cell {
    fn parse(c: char) -> Option<Self> {
        Some(match c {
            '.' => Cell::Open,
            '#' => Cell::Solid,
            ' ' => Cell::Nonexistent,
            _ => {
                debug_println!("Unexpected cell character: {:?}", c);
                return None;
            }
        })
    }

    fn to_char(self) -> char {
        match self {
            Cell::Open => '.',
            Cell::Solid => '#',
            Cell::Nonexistent => ' ',
        }
    }
}

impl TurnDirection {
    fn parse(c: char) -> Option<Self> {
        Some(match c {
            'L' => TurnDirection::Left,
            'R' => TurnDirection::Right,
            _ => {
                debug_println!("Unexpected direction character: {:?}", c);
                return None;
            }
        })
    }
}

fn from_digits(digits: &[u32]) -> u32 {
    digits.iter().fold(0, |acc, digit| acc * BASE + *digit)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let commands_text = "10R5L5R10L4R5L5";
        let actual = parse_commands(commands_text).unwrap();
        let expected = vec![
            Command::Forward(10),
            Command::Turn(TurnDirection::Right),
            Command::Forward(5),
            Command::Turn(TurnDirection::Left),
            Command::Forward(5),
            Command::Turn(TurnDirection::Right),
            Command::Forward(10),
            Command::Turn(TurnDirection::Left),
            Command::Forward(4),
            Command::Turn(TurnDirection::Right),
            Command::Forward(5),
            Command::Turn(TurnDirection::Left),
            Command::Forward(5),
        ];
        assert_eq!(expected, actual);

        assert!(parse(include_str!("../../static/example22.txt")).is_some());
    }

    #[test]
    fn test_examples_part1() {
        assert_eq!(6032, Day22.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(146092, Day22.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        assert_eq!(5031, Day22.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        assert_eq!(110342, Day22.part2(Example::Real, Debug::NotDebug));
    }
}
