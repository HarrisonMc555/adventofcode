use std::collections::HashSet;
use std::str::FromStr;

use counter::Counter;
use itertools::Itertools;

use crate::days::{Day, Debug, Example, Part};
use crate::{debug_print, debug_println};

const DEBUG: bool = false;

pub struct Day23;

impl Day for Day23 {
    fn number(&self) -> u32 {
        23
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Position {
    row: isize,
    col: isize,
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    elf_positions: HashSet<Position>,
    direction: Direction,
    min_row: isize,
    max_row: isize,
    min_col: isize,
    max_col: isize,
}

#[derive(Debug, Ordinalize, Default, Hash, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Direction {
    #[default]
    North,
    South,
    West,
    East,
}

impl State {
    fn step(&self) -> Self {
        let proposals = self.get_proposals();
        if DEBUG {
            debug_println!("Proposals:");
            for proposal in proposals.iter() {
                debug_println!("\t{:?}", proposal);
            }
        }
        let destination_counts = proposals
            .iter()
            .flat_map(|(_, dest)| *dest)
            .collect::<Counter<Position>>();
        let elf_positions = proposals
            .into_iter()
            .map(|(source, dest)| match dest {
                Some(dest) if destination_counts.get(&dest).copied().unwrap_or(0) == 1 => dest,
                _ => source,
            })
            .collect::<HashSet<_>>();
        let (min_row, max_row) = elf_positions
            .iter()
            .map(|p| p.row)
            .minmax()
            .into_option()
            .unwrap_or((0, 0));
        let (min_col, max_col) = elf_positions
            .iter()
            .map(|p| p.col)
            .minmax()
            .into_option()
            .unwrap_or((0, 0));
        let min_row = min_row.min(self.min_row);
        let max_row = max_row.max(self.min_row);
        let min_col = min_col.min(self.min_col);
        let max_col = max_col.max(self.min_col);
        State {
            elf_positions,
            direction: self.direction.next(),
            min_row,
            max_row,
            min_col,
            max_col,
        }
    }

    fn get_proposals(&self) -> Vec<(Position, Option<Position>)> {
        self.elf_positions
            .iter()
            .map(|position| (*position, self.get_proposal(*position)))
            .collect()
    }

    fn get_proposal(&self, position: Position) -> Option<Position> {
        if self.no_neighbors(position) {
            return None;
        }
        for direction in self.direction.get_sequence() {
            if !self.is_any_elves_in_direction(position, direction) {
                let dest = position.in_direction(direction);
                return Some(dest);
            }
        }
        None
    }

    fn no_neighbors(&self, position: Position) -> bool {
        position
            .neighbors()
            .into_iter()
            .all(|p| !self.elf_positions.contains(&p))
    }

    fn is_any_elves_in_direction(&self, position: Position, direction: Direction) -> bool {
        self.positions_in_direction(position, direction)
            .iter()
            .any(|p| self.elf_positions.contains(p))
    }

    fn positions_in_direction(&self, position: Position, direction: Direction) -> [Position; 3] {
        use Direction::*;
        let go = |d| position.in_direction(d);
        match direction {
            North => [go(North), go(North).go(East), go(North).go(West)],
            South => [go(South), go(South).go(East), go(South).go(West)],
            West => [go(West), go(West).go(North), go(West).go(South)],
            East => [go(East), go(East).go(North), go(East).go(South)],
        }
    }

    fn count_empty_tiles(&self) -> usize {
        let (min_pos, max_pos) = self.get_bounding_box();
        let num_rows = max_pos.row - min_pos.row + 1;
        let num_cols = max_pos.col - min_pos.col + 1;
        let grid_size = (num_rows * num_cols) as usize;
        grid_size - self.elf_positions.len()
    }

    fn get_bounding_box(&self) -> (Position, Position) {
        let (min_row, max_row) = self
            .elf_positions
            .iter()
            .map(|p| p.row)
            .minmax()
            .into_option()
            .unwrap_or((0, 0));
        let (min_col, max_col) = self
            .elf_positions
            .iter()
            .map(|p| p.col)
            .minmax()
            .into_option()
            .unwrap_or((0, 0));
        (
            Position {
                row: min_row,
                col: min_col,
            },
            Position {
                row: max_row,
                col: max_col,
            },
        )
    }

    fn debug_print(&self) {
        if DEBUG {
            for row in (self.min_row - 1)..=(self.max_row + 1) {
                for col in (self.min_col - 1)..=(self.max_col + 1) {
                    let letter = if self.elf_positions.contains(&Position { row, col }) {
                        '#'
                    } else {
                        '.'
                    };
                    debug_print!("{}", letter);
                }
                debug_println!();
            }
        }
    }
}

impl Direction {
    fn next(self) -> Self {
        let new_ordinal = (self.ordinal() + 1) as usize % Direction::variant_count();
        Direction::from_ordinal(new_ordinal as i8).unwrap()
    }

    fn get_sequence(self) -> [Direction; Direction::variant_count()] {
        const COUNT: usize = Direction::variant_count();
        let mut sequence = [Direction::default(); COUNT];
        let mut direction = self;
        for sequence_direction in sequence.iter_mut() {
            *sequence_direction = direction;
            direction = direction.next();
        }
        sequence
    }
}

impl Position {
    fn in_direction(self, direction: Direction) -> Position {
        let Position { row, col } = self;
        match direction {
            Direction::North => Position { row: row - 1, col },
            Direction::South => Position { row: row + 1, col },
            Direction::West => Position { row, col: col - 1 },
            Direction::East => Position { row, col: col + 1 },
        }
    }

    #[doc(alias = "in_direction")]
    fn go(self, direction: Direction) -> Position {
        self.in_direction(direction)
    }

    fn neighbors(self) -> [Position; 8] {
        use Direction::*;
        [
            self.go(North),
            self.go(North).go(East),
            self.go(East),
            self.go(East).go(South),
            self.go(South),
            self.go(South).go(West),
            self.go(West),
            self.go(West).go(North),
        ]
    }
}

impl FromStr for State {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elf_positions = HashSet::new();
        for (row, line) in s.lines().enumerate() {
            for (col, letter) in line.chars().enumerate() {
                match letter {
                    '#' => {
                        elf_positions.insert(Position {
                            row: row as isize,
                            col: col as isize,
                        });
                    }
                    '.' => {}
                    _ => return Err(()),
                }
            }
        }
        let max_row = s.lines().count() - 1;
        let max_col = s
            .lines()
            .map(|line| line.chars().count() - 1)
            .max()
            .unwrap_or(0);
        Ok(State {
            elf_positions,
            direction: Direction::default(),
            min_row: 0,
            max_row: max_row as isize,
            min_col: 0,
            max_col: max_col as isize,
        })
    }
}

impl Day23 {
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        let mut state = self.read_file(example).parse::<State>().unwrap();
        debug_println!("Initial:");
        state.debug_print();
        debug_println!();
        for round in 0..NUM_ROUNDS {
            state = state.step();
            debug_println!("== End of Round {} ==", round + 1);
            state.debug_print();
            debug_println!();
        }
        state.count_empty_tiles()
    }

    fn part2(&self, _example: Example, _debug: Debug) -> usize {
        todo!()
    }
}

const NUM_ROUNDS: usize = 10;

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_parse() {
        let actual = State::from_str(
            ".....\n\
             ..##.\n\
             ..#..\n\
             .....\n\
             ..##.\n\
             .....\n",
        )
        .unwrap();
        let elf_positions = [p(1, 2), p(1, 3), p(2, 2), p(4, 2), p(4, 3)]
            .into_iter()
            .collect::<HashSet<_>>();
        let expected = State {
            elf_positions,
            direction: Direction::North,
            min_row: 0,
            max_row: 5,
            min_col: 0,
            max_col: 4,
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_step() {
        let mut state = State::from_str(
            ".....\n\
             ..##.\n\
             ..#..\n\
             .....\n\
             ..##.\n\
             .....\n",
        )
        .unwrap();
        let next_states = vec![
            State::from_str(
                "..##.\n\
                 .....\n\
                 ..#..\n\
                 ...#.\n\
                 ..#..\n\
                 .....\n",
            ),
            State::from_str(
                ".....\n\
                 ..##.\n\
                 .#...\n\
                 ....#\n\
                 .....\n\
                 ..#..\n",
            ),
            State::from_str(
                "..#..\n\
                 ....#\n\
                 #....\n\
                 ....#\n\
                 .....\n\
                 ..#..\n",
            ),
        ];

        debug_println!("Initial:");
        state.debug_print();
        debug_println!();
        for next_state in next_states {
            state = state.step();
            debug_println!("=====");
            debug_println!();
            debug_println!("Actual:");
            state.debug_print();
            debug_println!();
            let expected = &next_state.unwrap();
            debug_println!("Expected:");
            expected.debug_print();
            debug_println!();
            assert_elf_positions_match(&expected, &state);
        }
    }

    #[test]
    fn test_examples_part1() {
        assert_eq!(110, Day23.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(4116, Day23.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        // assert_eq!(0, Day23.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        // assert_eq!(0, Day23.part2(Example::Real, Debug::NotDebug));
    }

    fn assert_elf_positions_match(state1: &State, state2: &State) {
        let row_offset = state2.min_row - state1.min_row;
        let col_offset = state2.min_col - state1.min_col;
        let elf_positions2 = state2
            .elf_positions
            .iter()
            .map(|Position { row, col }| Position {
                row: row + row_offset,
                col: col + col_offset,
            })
            .collect();
        debug_println!(
            "State 1: min_row={}, max_row={}, min_col={}, max_col={}",
            state1.min_row,
            state1.max_row,
            state1.min_col,
            state1.max_col
        );
        debug_println!(
            "State 2: min_row={}, max_row={}, min_col={}, max_col={}",
            state2.min_row,
            state2.max_row,
            state2.min_col,
            state2.max_col
        );
        debug_println!();
        assert_eq!(state1.elf_positions, elf_positions2);
    }

    fn p(row: isize, col: isize) -> Position {
        Position { row, col }
    }
}
