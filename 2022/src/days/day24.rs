#![allow(unused_variables)]

use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use array2d::Array2D;
use bitflags::bitflags;

use crate::days::{Day, Debug, Example, Part};
use crate::debug_println;

const DEBUG: bool = true;

pub struct Day24;

impl Day for Day24 {
    fn number(&self) -> u32 {
        24
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day24 {
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        let state = self.read_file(example).parse::<State>().unwrap();
        let start = state.find_start().unwrap();
        let end = state.find_end().unwrap();
        Solver::find_min_path_len(state, start, end)
    }

    fn part2(&self, _example: Example, _debug: Debug) -> usize {
        todo!()
    }
}

bitflags! {
    struct Blizzards: u8 {
        const UP    = 0b0001;
        const RIGHT = 0b0010;
        const DOWN  = 0b0100;
        const LEFT  = 0b1000;
    }
}

#[derive(Debug, Ordinalize, Hash, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Cell {
    Empty(Blizzards),
    Wall,
}

#[derive(Debug, Eq, PartialEq)]
struct Solver {
    state: State,
    round: usize,
    accessible: HashSet<Index>,
    end: Index,
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    cells: Array2D<Cell>,
}

type Index = (usize, usize);

impl Solver {
    pub fn find_min_path_len(state: State, start: Index, end: Index) -> usize {
        let mut solver = Solver::new(state, start, end);
        while !solver.is_start_end_connected() {
            solver = solver.step();
        }
        solver.round
    }

    fn new(state: State, start: Index, end: Index) -> Self {
        let mut from_start = HashSet::new();
        from_start.insert(start);
        Solver {
            state,
            round: 0,
            accessible: from_start,
            end,
        }
    }

    fn step(&self) -> Self {
        let state = self.state.step();
        let round = self.round + 1;
        let accessible = self
            .accessible
            .iter()
            .copied()
            .flat_map(|index| state.accessible_from(index))
            .collect();
        Solver {
            state,
            round,
            accessible,
            end: self.end,
        }
    }

    fn is_start_end_connected(&self) -> bool {
        self.accessible.contains(&self.end)
    }
}

impl State {
    pub fn step(&self) -> Self {
        let iter = self
            .cells
            .enumerate_row_major()
            .map(|(_, cell)| match cell {
                Cell::Empty(_) => Cell::Empty(Blizzards::empty()),
                Cell::Wall => Cell::Wall,
            });
        let cells =
            Array2D::from_iter_row_major(iter, self.cells.num_rows(), self.cells.num_columns())
                .unwrap();
        let mut state = State { cells };
        state.step_from(&self);
        state
    }

    pub fn accessible_from(&self, index: Index) -> impl Iterator<Item = Index> + '_ {
        [
            index,
            self.step_index_up(index),
            self.step_index_right(index),
            self.step_index_down(index),
            self.step_index_left(index),
        ]
        .into_iter()
        .filter(|(row, col)| self.cells.get(*row, *col) == Some(&Cell::Empty(Blizzards::empty())))
    }

    pub fn find_start(&self) -> Option<Index> {
        let first_row_iter = self.cells.rows_iter().next()?;
        let empty = first_row_iter
            .enumerate()
            .filter(|(_, c)| **c == Cell::Empty(Blizzards::empty()))
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        match empty.as_slice() {
            [index] => Some((0, *index)),
            _ => None,
        }
    }

    pub fn find_end(&self) -> Option<Index> {
        let last_row_iter = self.cells.rows_iter().rev().next()?;
        let empty = last_row_iter
            .enumerate()
            .filter(|(_, c)| **c == Cell::Empty(Blizzards::empty()))
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        match empty.as_slice() {
            [index] => Some((self.cells.num_rows() - 1, *index)),
            _ => None,
        }
    }

    fn step_from(&mut self, other: &State) {
        for (row, col) in self.cells.indices_row_major() {
            let Some(Cell::Empty(blizzards)) = other.cells.get(row, col) else {
                continue;
            };
            for direction in blizzards.directions() {
                self.step_blizzard((row, col), direction);
            }
        }
    }

    fn step_blizzard(&mut self, index: Index, direction: Direction) {
        let mut index = index;
        let blizzards = loop {
            index = self.step_index(index, direction);
            match self.cells.get_mut(index.0, index.1) {
                None => panic!("Bad index step: {:?}", index),
                Some(Cell::Wall) => continue,
                Some(Cell::Empty(blizzards)) => break blizzards,
            }
        };
        blizzards.insert(direction.into());
    }

    fn step_index(&self, index: Index, direction: Direction) -> Index {
        match direction {
            Direction::Up => self.step_index_up(index),
            Direction::Right => self.step_index_right(index),
            Direction::Down => self.step_index_down(index),
            Direction::Left => self.step_index_left(index),
        }
    }

    fn step_index_up(&self, (row, col): Index) -> Index {
        if row <= 0 {
            (self.cells.num_rows() - 1, col)
        } else {
            (row - 1, col)
        }
    }

    fn step_index_right(&self, (row, col): Index) -> Index {
        if col >= self.cells.num_columns() - 1 {
            (row, 0)
        } else {
            (row, col + 1)
        }
    }

    fn step_index_down(&self, (row, col): Index) -> Index {
        if row >= self.cells.num_rows() - 1 {
            (0, col)
        } else {
            (row + 1, col)
        }
    }

    fn step_index_left(&self, (row, col): Index) -> Index {
        if col <= 0 {
            (row, self.cells.num_columns() - 1)
        } else {
            (row, col - 1)
        }
    }

    fn debug_lines(&self) -> impl Iterator<Item = String> + '_ {
        self.cells.rows_iter().map(|row| {
            use std::fmt::Write;
            let mut line = String::new();
            for cell in row {
                write!(line, "{}", cell).unwrap();
            }
            line
        })
    }

    fn debug_print(&self) {
        if !DEBUG {
            return;
        }
        for line in self.debug_lines() {
            debug_println!("{}", line);
        }
        // for row in self.cells.rows_iter() {
        //     for cell in row {
        //         debug_print!("{}", cell);
        //     }
        //     debug_println!();
        // }
    }
}

impl FromStr for State {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s
            .lines()
            .map(|line| line.chars().map(Cell::parse).collect::<Result<Vec<_>, _>>())
            .collect::<Result<Vec<_>, _>>()?;
        let cells = Array2D::from_rows(&rows).map_err(|e| format!("{:?}", e))?;
        Ok(State { cells })
        // let width = s.lines().next().ok_or_else(|| "Empty".to_owned())?.len();
        // let rows_iter = s
        //     .lines()
        //     .map(|line| line.chars().map(Cell::parse).collect::<Result<Vec<_>, _>>())
        //     .collect::<Result<Vec<_>, _>>()?;
        // let walls = (0..width).map(|_| Cell::Wall).collect::<Vec<_>>();
        // let rows = std::iter::once(walls.clone())
        //     .chain(rows_iter)
        //     .chain(std::iter::once(walls))
        //     .collect::<Vec<_>>();
        // let cells = Array2D::from_rows(&rows).map_err(|e| format!("{:?}", e))?;
        // Ok(State { cells })
    }
}

impl Cell {
    fn parse(c: char) -> Result<Cell, char> {
        Ok(match c {
            '#' => Cell::Wall,
            '.' => Cell::Empty(Blizzards::empty()),
            '^' => Cell::Empty(Blizzards::UP),
            '>' => Cell::Empty(Blizzards::RIGHT),
            'v' => Cell::Empty(Blizzards::DOWN),
            '<' => Cell::Empty(Blizzards::LEFT),
            _ => return Err(c),
        })
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty(blizzards) => write!(f, "{}", blizzards),
            Cell::Wall => write!(f, "#"),
        }
    }
}

impl Display for Blizzards {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl Blizzards {
    fn directions(self) -> impl Iterator<Item = Direction> {
        Direction::variants()
            .into_iter()
            .filter(move |d| self.intersects(d.into()))
    }

    fn chars() -> impl Iterator<Item = (Blizzards, char)> {
        [
            (Blizzards::UP, '^'),
            (Blizzards::RIGHT, '>'),
            (Blizzards::DOWN, 'v'),
            (Blizzards::LEFT, '<'),
        ]
        .into_iter()
    }

    fn to_char(self) -> char {
        let chars = Blizzards::chars()
            .filter(|(d, _)| self.intersects(*d))
            .map(|(_, c)| c)
            .collect::<Vec<_>>();
        match chars.as_slice() {
            [_, _, ..] => char::from_digit(chars.len() as u32, 10).unwrap(),
            [c] => *c,
            [] => '.',
        }
    }
}

impl From<Direction> for Blizzards {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => Blizzards::UP,
            Direction::Right => Blizzards::RIGHT,
            Direction::Down => Blizzards::DOWN,
            Direction::Left => Blizzards::LEFT,
        }
    }
}

impl From<&Direction> for Blizzards {
    fn from(direction: &Direction) -> Self {
        match direction {
            Direction::Up => Blizzards::UP,
            Direction::Right => Blizzards::RIGHT,
            Direction::Down => Blizzards::DOWN,
            Direction::Left => Blizzards::LEFT,
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_parse() {
        let text = "\
            #.#####\n\
            #.....#\n\
            #>....#\n\
            #.....#\n\
            #...v.#\n\
            #.....#\n\
            #####.#\n";
        let num_rows = text.lines().count();
        let num_cols = text.lines().next().unwrap().chars().count();
        let state = State::from_str(text).unwrap();
        let blizzards = [((2, 1), Blizzards::RIGHT), ((4, 4), Blizzards::DOWN)]
            .into_iter()
            .collect::<HashMap<_, _>>();
        for row in 1..num_rows - 1 {
            for col in 1..num_cols - 1 {
                let actual = *state.cells.get(row, col).unwrap();
                let expected = Cell::Empty(
                    blizzards
                        .get(&(row, col))
                        .copied()
                        .unwrap_or(Blizzards::empty()),
                );
                assert_eq!(expected, actual, "Row {} and column {}", row, col);
            }
        }

        let rows = state.cells.as_rows();

        let actual_first_row = rows.first().unwrap();
        let mut expected_first_row = vec![Cell::Wall; 7];
        expected_first_row[1] = Cell::Empty(Blizzards::empty());
        assert_eq!(&expected_first_row, actual_first_row);

        let actual_last_row = rows.last().unwrap();
        let mut expected_last_row = vec![Cell::Wall; 7];
        expected_last_row[5] = Cell::Empty(Blizzards::empty());
        assert_eq!(&expected_last_row, actual_last_row);
    }

    #[test]
    fn test_step() {
        let state_strings = [
            //
            "#.#####\n\
             #.....#\n\
             #>....#\n\
             #.....#\n\
             #...v.#\n\
             #.....#\n\
             #####.#\n",
            //
            "#.#####\n\
             #.....#\n\
             #.>...#\n\
             #.....#\n\
             #.....#\n\
             #...v.#\n\
             #####.#\n",
            //
            "#.#####\n\
             #...v.#\n\
             #..>..#\n\
             #.....#\n\
             #.....#\n\
             #.....#\n\
             #####.#\n",
            //
            "#.#####\n\
             #.....#\n\
             #...2.#\n\
             #.....#\n\
             #.....#\n\
             #.....#\n\
             #####.#\n",
            //
            "#.#####\n\
             #.....#\n\
             #....>#\n\
             #...v.#\n\
             #.....#\n\
             #.....#\n\
             #####.#\n",
            //
            "#.#####\n\
             #.....#\n\
             #>....#\n\
             #.....#\n\
             #...v.#\n\
             #.....#\n\
             #####.#\n",
        ];
        let mut state = State::from_str(state_strings[0]).unwrap();
        for (index, expected) in state_strings.into_iter().enumerate() {
            debug_println!("== After {} steps ==", index);
            debug_println!("Expected:");
            println!("{}", expected);
            debug_println!();
            debug_println!("Actual:");
            state.debug_print();
            debug_println!();
            let expected = expected.lines().collect::<Vec<_>>();
            // let mut actual = state.debug_lines().skip(1).collect::<Vec<_>>();
            // actual.pop();
            let actual = state.debug_lines().collect::<Vec<_>>();
            assert_eq!(expected, actual, "After {} steps", index);
            state = state.step();
        }
    }

    #[test]
    fn test_examples_part1() {
        assert_eq!(18, Day24.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(332, Day24.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        // assert_eq!(0, Day24.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        // assert_eq!(0, Day24.part2(Example::Real, Debug::NotDebug));
    }
}
