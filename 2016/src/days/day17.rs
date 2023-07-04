use std::collections::VecDeque;

use md5::{Context, Digest};

use crate::days::{Day, Debug, Example, Part};

const DEBUG: bool = false;

const NUM_ROWS: usize = 4;
const NUM_COLUMNS: usize = 4;

const INDEX_START: Index = Index::new(0, 0);
const INDEX_GOAL: Index = Index::new(NUM_ROWS - 1, NUM_COLUMNS - 1);

pub struct Day17;

impl Day for Day17 {
    fn number(&self) -> u32 {
        17
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day17 {
    fn part1(&self, example: Example, _debug: Debug) -> String {
        find_shortest_path(&self.read_file(example))
            .unwrap()
            .into_iter()
            .map(Direction::to_char)
            .collect()
    }

    fn part2(&self, example: Example, _debug: Debug) -> String {
        find_longest_path_len(&self.read_file(example))
            .unwrap()
            .to_string()
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Index {
    row: usize,
    column: usize,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    index: Index,
    path: Vec<Direction>,
}

fn find_shortest_path(passcode: &str) -> Option<Vec<Direction>> {
    let mut queue = VecDeque::new();
    queue.push_back(State::init());
    while let Some(state) = queue.pop_front() {
        let State { index, path } = state;
        if index == INDEX_GOAL {
            return Some(path);
        }
        for direction in get_open_directions(passcode, &path) {
            let Some(new_index) = index.direction(direction) else {
                continue;
            };
            let mut new_path = path.clone();
            new_path.push(direction);
            let new_state = State::new(new_index, new_path);
            queue.push_back(new_state);
        }
    }
    None
}

fn find_longest_path_len(passcode: &str) -> Option<usize> {
    let mut queue = VecDeque::new();
    queue.push_back(State::init());
    let mut longest_path_len: Option<usize> = None;
    while let Some(state) = queue.pop_front() {
        let State { index, path } = state;
        if index == INDEX_GOAL {
            match longest_path_len {
                Some(ref longest) if *longest > path.len() => {}
                _ => longest_path_len = Some(path.len()),
            }
            continue;
        }
        for direction in get_open_directions(passcode, &path) {
            let Some(new_index) = index.direction(direction) else {
                continue;
            };
            let mut new_path = path.clone();
            new_path.push(direction);
            let new_state = State::new(new_index, new_path);
            queue.push_back(new_state);
        }
    }
    longest_path_len
}

fn get_open_directions(passcode: &str, path: &[Direction]) -> Vec<Direction> {
    let hash = get_hash(passcode, path);
    let hash = format!("{:4x}", hash);
    let directions = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
    hash.chars()
        .zip(directions)
        .filter(|(c, _)| door_is_open(*c))
        .map(|(_, d)| d)
        .collect()
}

fn door_is_open(c: char) -> bool {
    match c {
        '0'..='a' => false,
        'b'..='f' => true,
        _ => {
            debug_println!("Invalid byte {}", c);
            false
        }
    }
}

fn get_hash(passcode: &str, path: &[Direction]) -> Digest {
    let mut context = Context::new();
    context.consume(passcode);
    for direction in path {
        context.consume([direction.to_char() as u8]);
    }
    context.compute()
}

impl State {
    pub fn init() -> Self {
        Self {
            index: INDEX_START,
            path: Vec::new(),
        }
    }

    pub fn new(index: Index, path: Vec<Direction>) -> Self {
        Self { index, path }
    }
}

impl Index {
    pub const fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    pub fn direction(self, direction: Direction) -> Option<Self> {
        let Index { row, column } = self;
        match direction {
            Direction::Up => row.checked_sub(1).map(|r| Index::new(r, column)),
            Direction::Down => Some(row + 1)
                .filter(|&r| r < NUM_ROWS)
                .map(|r| Index::new(r, column)),
            Direction::Left => column.checked_sub(1).map(|c| Index::new(row, c)),
            Direction::Right => Some(column + 1)
                .filter(|&c| c < NUM_COLUMNS)
                .map(|c| Index::new(row, c)),
        }
    }
}

impl Direction {
    pub fn to_char(self) -> char {
        match self {
            Direction::Up => 'U',
            Direction::Down => 'D',
            Direction::Left => 'L',
            Direction::Right => 'R',
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_open_directions() {
        let passcode = "hijkl";
        assert_eq!(dirs("UDL"), get_open_directions(passcode, &dirs("")));
        assert_eq!(dirs("ULR"), get_open_directions(passcode, &dirs("D")));
        assert_eq!(dirs(""), get_open_directions(passcode, &dirs("DR")));
        assert_eq!(dirs("R"), get_open_directions(passcode, &dirs("DU")));
        assert_eq!(
            Vec::<Direction>::new(),
            get_open_directions(
                passcode,
                &vec![Direction::Down, Direction::Up, Direction::Right]
            )
        );
    }

    #[test]
    fn test_find_shortest_path() {
        assert_eq!(None, find_shortest_path("hijkl"));
        assert_eq!(Some(dirs("DDRRRD")), find_shortest_path("ihgpwlah"));
        assert_eq!(Some(dirs("DDUDRLRRUDRD")), find_shortest_path("kglvqrro"));
        assert_eq!(
            Some(dirs("DRURDRUDDLLDLUURRDULRLDUUDDDRR")),
            find_shortest_path("ulqzkmiv")
        );
    }

    #[test]
    fn test_find_longest_path() {
        assert_eq!(None, find_longest_path_len("hijkl"));
        assert_eq!(Some(370), find_longest_path_len("ihgpwlah"));
        assert_eq!(Some(492), find_longest_path_len("kglvqrro"));
        assert_eq!(Some(830), find_longest_path_len("ulqzkmiv"));
    }

    fn dirs(string: &str) -> Vec<Direction> {
        string.chars().map(char_to_direction).collect()
    }

    fn char_to_direction(c: char) -> Direction {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction char '{}'", c),
        }
    }
}
