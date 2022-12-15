use std::collections::{HashMap, HashSet};

use crate::days::{Day, Debug, Example, Part};
use crate::debug_println;

const DEBUG: bool = false;

pub struct Day09;

impl Day for Day09 {
    fn number(&self) -> u32 {
        9
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day09 {
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        let commands = parse(&self.read_file(example)).unwrap();
        count_tail_positions(&commands)
    }

    fn part2(&self, example: Example, _debug: Debug) -> usize {
        let commands = parse(&self.read_file(example)).unwrap();
        count_tail_positions2(&commands)
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
struct State {
    head: Position,
    tail: Position,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State2 {
    knots: Vec<Position>,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Command {
    direction: Direction,
    amount: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn count_tail_positions(commands: &[Command]) -> usize {
    let mut state = State::default();
    let mut tail_positions = HashSet::new();
    tail_positions.insert(state.tail.clone());

    fn print_state(state: &State) {
        let display_x = (0, 6);
        let display_y = (0, 5);
        debug_println!(
            "{}",
            state
                .to_string_rows(Position::default(), display_x, display_y)
                .join("\n")
        );
        debug_println!();
    }
    debug_println!("== Initial State ==\n");
    print_state(&state);

    for command in commands {
        debug_println!("== {} {} ==\n", command.direction.to_char(), command.amount);

        for _ in 0..command.amount {
            state.step(command.direction);
            tail_positions.insert(state.tail.clone());
            debug_println!("Head: {:?}, Tail: {:?}", state.head, state.tail);
            print_state(&state);
        }
    }
    tail_positions.len()
}

const NUM_KNOTS: usize = 10;

fn count_tail_positions2(commands: &[Command]) -> usize {
    let mut state = State2::new(NUM_KNOTS);
    let mut tail_positions = HashSet::new();
    tail_positions.insert(state.knots.last().cloned().unwrap());

    fn print_state(state: &State2) {
        let display_x = (0, 6);
        let display_y = (0, 5);
        debug_println!(
            "{}",
            state
                .to_string_rows(Position::default(), display_x, display_y)
                .join("\n")
        );
        debug_println!();
    }
    debug_println!("== Initial State ==\n");
    print_state(&state);

    for command in commands {
        debug_println!("== {} {} ==\n", command.direction.to_char(), command.amount);

        for _ in 0..command.amount {
            state.step(command.direction);
            tail_positions.insert(state.knots.last().cloned().unwrap());
            print_state(&state);
        }
    }
    tail_positions.len()
}

impl State {
    fn step(&mut self, direction: Direction) {
        self.head.step(direction);
        if !self.head_tail_touching() {
            self.move_tail();
        }
    }

    fn head_tail_touching(&self) -> bool {
        (self.head.x - self.tail.x).abs() <= 1 && (self.head.y - self.tail.y).abs() <= 1
    }

    fn move_tail(&mut self) {
        let Position {
            x: head_x,
            y: head_y,
        } = self.head;
        let Position {
            x: tail_x,
            y: tail_y,
        } = self.tail;

        if head_x != tail_x {
            self.tail.x += State::get_step(tail_x, head_x);
        }
        if head_y != tail_y {
            self.tail.y += State::get_step(tail_y, head_y);
        }
    }

    fn get_step(from: i32, to: i32) -> i32 {
        (to - from).signum()
    }

    fn to_string_rows(
        &self,
        start: Position,
        (min_x, max_x): (i32, i32),
        (min_y, max_y): (i32, i32),
    ) -> Vec<String> {
        (min_y..max_y)
            .rev()
            .map(|y| {
                (min_x..max_x)
                    .map(|x| {
                        let position = Position { x, y };
                        if self.head == position {
                            'H'
                        } else if self.tail == position {
                            'T'
                        } else if start == position {
                            's'
                        } else {
                            '.'
                        }
                    })
                    .collect()
            })
            .collect()
    }
}

impl State2 {
    fn new(num_knots: usize) -> Self {
        State2 {
            knots: (0..num_knots).map(|_| Position::default()).collect(),
        }
    }

    fn step(&mut self, direction: Direction) {
        let head = self.knots.get_mut(0).unwrap();
        head.step(direction);
        for index in 1..self.knots.len() {
            let prev = self.knots[index - 1].clone();
            let cur = &mut self.knots[index];
            cur.follow(&prev);
        }
    }

    fn to_string_rows(
        &self,
        start: Position,
        (min_x, max_x): (i32, i32),
        (min_y, max_y): (i32, i32),
    ) -> Vec<String> {
        let mut grid = HashMap::new();
        grid.insert(start, 's');
        for (index, knot) in self.knots.iter().enumerate().rev() {
            let char = if index == 0 {
                'H'
            } else {
                index.to_string().chars().next().unwrap_or(' ')
            };
            grid.insert(knot.clone(), char);
        }

        (min_y..max_y)
            .rev()
            .map(|y| {
                (min_x..max_x)
                    .map(|x| {
                        let position = Position { x, y };
                        grid.get(&position).copied().unwrap_or('.')
                    })
                    .collect()
            })
            .collect()
    }
}

impl Position {
    fn step(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
        }
    }

    fn follow(&mut self, other: &Position) {
        if self.is_touching(other) {
            return;
        }
        self.x += Position::get_step(self.x, other.x);
        self.y += Position::get_step(self.y, other.y);
    }

    fn is_touching(&self, other: &Position) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    fn get_step(from: i32, to: i32) -> i32 {
        (to - from).signum()
    }
}

fn parse(text: &str) -> Option<Vec<Command>> {
    text.trim().split('\n').map(Command::parse).collect()
}

impl Command {
    fn new(direction: Direction, amount: usize) -> Self {
        Command { direction, amount }
    }

    fn parse(text: &str) -> Option<Self> {
        let mut iter = text.split_whitespace();
        let direction = iter.next()?;
        let amount = iter.next()?;
        if iter.next().is_some() {
            return None;
        }
        let direction = Direction::parse(direction)?;
        let amount = amount.parse().ok()?;
        Some(Command { direction, amount })
    }
}

impl Direction {
    fn parse(text: &str) -> Option<Self> {
        Some(match text {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => return None,
        })
    }

    fn to_char(self) -> char {
        match self {
            Direction::Up => 'U',
            Direction::Right => 'R',
            Direction::Down => 'D',
            Direction::Left => 'L',
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let text = include_str!("../../static/example09.txt");
        let commands = parse(text).unwrap();
        let mut iter = commands.into_iter();

        let expected = Command::new(Direction::Right, 4);
        assert_eq!(expected, iter.next().unwrap());

        let expected = Command::new(Direction::Up, 4);
        assert_eq!(expected, iter.next().unwrap());

        let expected = Command::new(Direction::Left, 3);
        assert_eq!(expected, iter.next().unwrap());

        let expected = Command::new(Direction::Down, 1);
        assert_eq!(expected, iter.next().unwrap());
    }

    #[test]
    fn test_examples_part1() {
        let mut state = State::default();
        state.head.x = 1;
        state.step(Direction::Right);
        assert_eq!(2, state.head.x);
        assert_eq!(1, state.tail.x);
        assert_eq!(0, state.head.y);
        assert_eq!(0, state.tail.y);

        let mut state = State::default();
        state.head.y = -1;
        state.step(Direction::Down);
        assert_eq!(0, state.head.x);
        assert_eq!(0, state.tail.x);
        assert_eq!(-2, state.head.y);
        assert_eq!(-1, state.tail.y);

        let mut state = State::default();
        state.head.x = 1;
        state.head.y = 1;
        state.step(Direction::Up);
        assert_eq!(1, state.head.x);
        assert_eq!(1, state.tail.x);
        assert_eq!(2, state.head.y);
        assert_eq!(1, state.tail.y);

        let mut state = State::default();
        state.head.x = 1;
        state.head.y = 1;
        state.step(Direction::Right);
        assert_eq!(2, state.head.x);
        assert_eq!(1, state.tail.x);
        assert_eq!(1, state.head.y);
        assert_eq!(1, state.tail.y);

        let text = include_str!("../../static/example09.txt");
        let commands = parse(text).unwrap();
        let num_tail_positions = count_tail_positions(&commands);
        assert_eq!(13, num_tail_positions);
    }

    #[test]
    fn test_examples_part2() {
        let text = include_str!("../../static/example09.txt");
        let commands = parse(text).unwrap();
        let num_tail_positions = count_tail_positions2(&commands);
        assert_eq!(1, num_tail_positions);
    }
}
