use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter};
use std::iter;
use std::ops::{Add, Sub};

use crate::days::{Day, Debug, Example, Part};
use crate::{debug_print, debug_println};

const DEBUG: bool = true;

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
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        let directions = parse_directions(&self.read_file(example)).unwrap();
        let mut chamber = Chamber::default();
        let blocks_offsets = blocks_to_offsets(&BLOCKS);
        chamber.calc_tower_height(&blocks_offsets, &directions, NUM_ROCKS)
    }

    fn part2(&self, example: Example, _debug: Debug) -> usize {
        let directions = parse_directions(&self.read_file(example)).unwrap();
        let mut chamber = Chamber::default();
        let blocks_offsets = blocks_to_offsets(&BLOCKS);
        chamber.calc_tower_height2(&blocks_offsets, &directions, NUM_ROCKS)
    }
}

const CHAMBER_WIDTH: usize = 7;
const INIT_DIST_FROM_LEFT: usize = 2;
const INIT_DIST_FROM_TOP: usize = 4;
const NUM_ROCKS: usize = 2022;
const MIN_IDENTICAL_COUNT: usize = 20;

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Cell {
    Empty,
    Rock,
}

type BlockOffsets = Vec<PositionOffset>;

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
struct Position {
    row: isize,
    column: isize,
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
struct PositionOffset {
    row_offset: isize,
    column_offset: isize,
}

#[derive(Debug, Default)]
struct Chamber {
    /// Each cell in the chamber after the stored offset. Everything below the [`offset`](Chamber::offset) row has been
    /// discarded but should not be able to affect further falling blocks.
    cells: VecDeque<[Cell; CHAMBER_WIDTH]>,
}

#[rustfmt::skip]
const BLOCKS: [[&str; 4]; 5] = [
    [
        "....",
        "....",
        "....",
        "####"
    ], [
        "....",
        ".#..",
        "###.",
        ".#.."
    ], [
        "....",
        "..#.",
        "..#.",
        "###."
    ], [
        "#...",
        "#...",
        "#...",
        "#..."
    ], [
        "....",
        "....",
        "##..",
        "##.."
    ],
];

impl Chamber {
    fn calc_tower_height(
        &mut self,
        blocks_offsets: &[BlockOffsets],
        directions: &[Direction],
        num_blocks: usize,
    ) -> usize {
        let block_offsets_iter = iter::repeat(blocks_offsets).flatten();
        let mut direction_cycle = directions.cycle();
        for block_offsets in block_offsets_iter.take(num_blocks) {
            self.drop_block(block_offsets, &mut direction_cycle);
            self.debug_print();
            debug_println!();
        }
        self.get_top_row_index().unwrap_or(0) + 1
    }

    fn calc_tower_height2(
        &mut self,
        blocks_offsets: &[BlockOffsets],
        directions: &[Direction],
        num_blocks: usize,
    ) -> usize {
        debug_println!("Calculating tower height 2 ({} blocks)", num_blocks);
        let mut direction_index = 0;
        let mut last_dropped_offsets = HashMap::new();
        let mut identical_count = 0;
        let mut cycle_start_position = Position { row: 0, column: 0 };
        let mut previous_dropped_position =
            self.drop_block2(&blocks_offsets[0], &directions, &mut direction_index);
        let mut total_index = 1;
        let (cycle_end_position, cycle_len) = loop {
            if total_index >= num_blocks {
                break (previous_dropped_position, 0);
            }
            total_index += 1;
            if total_index > 2023 {
                panic!();
            }
            let block_index = total_index % blocks_offsets.len();
            debug_println!(
                "Block index: {}, {} total",
                block_index,
                blocks_offsets.len()
            );
            let block_offsets = &blocks_offsets[block_index];
            let dropped_position =
                self.drop_block2(block_offsets, &directions, &mut direction_index);
            let dropped_offset = dropped_position - previous_dropped_position;
            let key = (block_index, direction_index);
            debug_println!(
                "Dropped position: {:?}, previous position: {:?}, offset: {:?}",
                dropped_position,
                previous_dropped_position,
                dropped_offset
            );
            debug_println!(
                "Last value for key {:?} was {:?}",
                key,
                last_dropped_offsets.get(&key)
            );
            match last_dropped_offsets.get(&key) {
                Some((offset, index)) if *offset == dropped_offset => {
                    debug_println!("Offset matches: {:?}", dropped_offset);
                    if identical_count == 0 {
                        debug_println!(
                            "\tInitializing cycle: {:?}, {:?}",
                            total_index,
                            dropped_position
                        );
                        cycle_start_position = dropped_position;
                    } else {
                        debug_println!(
                            "\tIdentical count: {} -> {}",
                            identical_count,
                            identical_count + 1
                        );
                    }
                    identical_count += 1;
                    if identical_count >= MIN_IDENTICAL_COUNT {
                        debug_println!("Found {} identical drops in a row", identical_count);
                        break (dropped_position, total_index - index);
                    }
                }
                _ => {
                    debug_println!("Offset does NOT match, resetting identical count");
                    identical_count = 0;
                }
            }

            previous_dropped_position = dropped_position;
            last_dropped_offsets.insert(key, (dropped_offset, total_index));
            self.debug_print();
            debug_println!();
        };
        self.debug_print();
        debug_println!();

        let cycle_height = cycle_end_position.row - cycle_start_position.row;
        dbg!(total_index, cycle_len, cycle_end_position, cycle_start_position, cycle_height);

        if cycle_len <= 0 {
            debug_println!(
                "Cycle length is 0, returning top row index {:?}",
                self.get_top_row_index()
            );
            return self.get_top_row_index().unwrap();
        }

        let num_blocks_remaining = num_blocks - total_index;
        let num_cycles = num_blocks_remaining / cycle_len;
        let num_blocks_after_cycles = num_blocks_remaining % cycle_len;
        dbg!(num_blocks_remaining, num_cycles, num_blocks_after_cycles);
        for _ in 0..num_blocks_after_cycles {
            let block_index = total_index % blocks_offsets.len();
            let block_offsets = &blocks_offsets[block_index];
            total_index += 1;
            self.drop_block2(block_offsets, &directions, &mut direction_index);
        }

        // before cycle + (cycle * num_cycles) + after cycle
        dbg!(
            total_index,
            num_blocks,
            num_blocks_remaining,
            num_cycles,
            num_blocks_after_cycles,
            cycle_start_position,
            cycle_height,
            num_cycles,
            self.get_top_row_index(),
            cycle_end_position
        );
        cycle_start_position.row as usize
            + (cycle_height as usize * num_cycles)
            + (self.get_top_row_index().unwrap() - cycle_end_position.row as usize)
    }

    fn drop_block(
        &mut self,
        block_offsets: &BlockOffsets,
        direction_iter: &mut SliceCycle<Direction>,
    ) {
        debug_println!("Dropping block: {:?}", block_offsets);
        let mut position = self.get_block_init_position();
        self.get_row_mut(position.row + 4);
        debug_println!("Starting position: {}", position);
        loop {
            let direction = direction_iter.next();
            debug_println!("\tAttempting to move {:?}", direction);
            if let Some(position_after_direction) = position.move_direction(*direction) {
                if self.is_valid_position(block_offsets, position_after_direction) {
                    debug_println!("\t\tNew position {} is valid", position_after_direction);
                    position = position_after_direction;
                } else {
                    debug_println!("\t\tNew position {} is NOT valid", position_after_direction);
                }
            } else {
                debug_println!("\t\tCannot move position {} {:?}", position, direction);
            }
            let Some(position_after_fall) = position.fall() else {
                debug_println!("\t\tPosition {} cannot fall (presumably on the floor)", position);
                break;
            };
            debug_println!("\t\tAfter falling, new position is {}", position_after_fall);
            if self.is_valid_position(block_offsets, position_after_fall) {
                debug_println!(
                    "\t\tAfter falling, new position {} is valid",
                    position_after_fall
                );
                position = position_after_fall;
            } else {
                debug_println!(
                    "\t\tAfter falling, new position {} is NOT valid, now settled",
                    position_after_fall
                );
                break;
            }
        }

        debug_println!("Now settled at {}", position);
        for offset in block_offsets {
            debug_println!("\tOffset: {}", offset);
            let Position { row, column } = position + offset;
            let row_index = row;
            debug_println!("\t\tNew position: ({},{})", row, column);
            let row = self.get_row_mut(row);
            debug_println!("\t\tBefore, row {}: {:?}", row_index, row);
            row[column as usize] = Cell::Rock;
            debug_println!("\t\tAfter,  row {}: {:?}", row_index, row);
        }
    }

    fn drop_block2(
        &mut self,
        block_offsets: &BlockOffsets,
        directions: &[Direction],
        direction_index: &mut usize,
    ) -> Position {
        debug_println!("Dropping block: {:?}", block_offsets);
        let mut position = self.get_block_init_position();
        self.get_row_mut(position.row + 4);
        debug_println!("Starting position: {}", position);
        loop {
            let direction = directions[*direction_index];
            *direction_index = (*direction_index + 1) % directions.len();
            debug_println!("\tAttempting to move {:?}", direction);
            if let Some(position_after_direction) = position.move_direction(direction) {
                if self.is_valid_position(block_offsets, position_after_direction) {
                    debug_println!("\t\tNew position {} is valid", position_after_direction);
                    position = position_after_direction;
                } else {
                    debug_println!("\t\tNew position {} is NOT valid", position_after_direction);
                }
            } else {
                debug_println!("\t\tCannot move position {} {:?}", position, direction);
            }
            let Some(position_after_fall) = position.fall() else {
                debug_println!("\t\tPosition {} cannot fall (presumably on the floor)", position);
                break;
            };
            debug_println!("\t\tAfter falling, new position is {}", position_after_fall);
            if self.is_valid_position(block_offsets, position_after_fall) {
                debug_println!(
                    "\t\tAfter falling, new position {} is valid",
                    position_after_fall
                );
                position = position_after_fall;
            } else {
                debug_println!(
                    "\t\tAfter falling, new position {} is NOT valid, now settled",
                    position_after_fall
                );
                break;
            }
        }

        debug_println!("Now settled at {}", position);
        for offset in block_offsets {
            debug_println!("\tOffset: {}", offset);
            let Position { row, column } = position + offset;
            let row_index = row;
            debug_println!("\t\tNew position: ({},{})", row, column);
            let row = self.get_row_mut(row);
            debug_println!("\t\tBefore, row {}: {:?}", row_index, row);
            row[column as usize] = Cell::Rock;
            debug_println!("\t\tAfter,  row {}: {:?}", row_index, row);
        }

        position
    }

    fn get_block_init_position(&self) -> Position {
        let row = self
            .get_top_row_index()
            .map(|top_row| top_row + INIT_DIST_FROM_TOP)
            .unwrap_or(INIT_DIST_FROM_TOP - 1);
        let column = INIT_DIST_FROM_LEFT;
        Position {
            row: row as isize,
            column: column as isize,
        }
    }

    fn is_valid_position(&self, block_offsets: &BlockOffsets, position: Position) -> bool {
        for offset in block_offsets {
            let Position { row, column } = position + offset;
            if row < 0 || column < 0 {
                return false;
            }
            let row_index = row;
            let Some(row) = self.cells.get(row as usize) else {
                debug_println!("\t\t\tNo row for new position ({},{})", row, column);
                return true;
            };
            let Some(cell) = row.get(column as usize) else {
                debug_println!("\t\t\tNo cell for new position ({},{})", row_index, column);
                return false;
            };
            if *cell == Cell::Rock {
                debug_println!(
                    "\t\t\tPosition ({},{}) is occupied by a rock",
                    row_index,
                    column
                );
                return false;
            }
        }
        debug_println!("\t\t\tPosition is valid");
        true
    }

    fn get_row_mut(&mut self, row_index: isize) -> &mut [Cell; CHAMBER_WIDTH] {
        // loop {
        //     if let Some(row) = self.cells.get_mut(row_index) {
        //         return row;
        //     }
        //     self.cells.push_back([Cell::Empty; CHAMBER_WIDTH]);
        // }
        assert!(row_index >= 0);
        let row_index = row_index as usize;
        while self.cells.len() <= row_index {
            self.cells.push_back([Cell::Empty; CHAMBER_WIDTH]);
        }
        &mut self.cells[row_index]
    }

    fn get_top_row_index(&self) -> Option<usize> {
        self.cells
            .iter()
            .enumerate()
            .rev()
            .find(|(_, row)| row.iter().any(|cell| *cell == Cell::Rock))
            .map(|(row_index, _)| row_index)
    }

    fn debug_print(&self) {
        for row in self.cells.iter().rev() {
            debug_print!("|");
            for cell in row {
                let c = match cell {
                    Cell::Rock => '#',
                    Cell::Empty => '.',
                };
                debug_print!("{}", c);
            }
            debug_println!();
        }
        debug_print!("+");
        for _ in 0..CHAMBER_WIDTH {
            debug_print!("-");
        }
        debug_println!("+");
    }
}

impl Position {
    fn move_direction(self, direction: Direction) -> Option<Position> {
        let Position { row, column } = self;
        let column = match direction {
            Direction::Left => column.checked_sub(1)?,
            Direction::Right => Some(column + 1).filter(|c| *c < CHAMBER_WIDTH as isize)?,
        };
        Some(Position { row, column })
    }

    fn fall(self) -> Option<Position> {
        let Position { row, column } = self;
        let row = row.checked_sub(1)?;
        Some(Position { row, column })
    }
}

impl Add<PositionOffset> for Position {
    type Output = Position;

    fn add(self, rhs: PositionOffset) -> Self::Output {
        Position {
            row: self.row + rhs.row_offset,
            column: self.column + rhs.column_offset,
        }
    }
}

impl Add<&'_ PositionOffset> for Position {
    type Output = Position;

    fn add(self, rhs: &'_ PositionOffset) -> Self::Output {
        self + *rhs
    }
}

impl Sub for Position {
    type Output = PositionOffset;

    fn sub(self, rhs: Self) -> Self::Output {
        PositionOffset {
            row_offset: rhs.row - self.row,
            column_offset: rhs.column - self.column,
        }
    }
}

fn blocks_to_offsets(blocks: &[[&str; 4]]) -> Vec<BlockOffsets> {
    blocks
        .iter()
        .map(|block| block_to_offsets(*block))
        .collect()
}

fn block_to_offsets(block: [&str; 4]) -> BlockOffsets {
    block
        .iter()
        .rev()
        .enumerate()
        .flat_map(move |(row_index, text)| block_line_to_offests(row_index as isize, text))
        .collect()
}

fn block_line_to_offests(
    offset_row: isize,
    line: &str,
) -> impl Iterator<Item = PositionOffset> + '_ {
    line.chars()
        .enumerate()
        .filter(move |(_, c)| *c == '#')
        .map(move |(offset_column, _)| PositionOffset {
            row_offset: offset_row,
            column_offset: offset_column as isize,
        })
}

fn parse_directions(text: &str) -> Option<Vec<Direction>> {
    text.trim().chars().map(Direction::parse).collect()
}

impl Direction {
    fn parse(c: char) -> Option<Direction> {
        Some(match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => return None,
        })
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.row, self.column)
    }
}

impl Display for PositionOffset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.row_offset, self.column_offset)
    }
}

struct SliceCycle<'a, T> {
    slice: &'a [T],
    index: usize,
}

impl<'a, T> SliceCycle<'a, T> {
    fn next(&mut self) -> &'a T {
        let result = &self.slice[self.index];
        self.index = (self.index + 1) % self.slice.len();
        result
    }
}

trait SliceExt<T> {
    fn cycle(&self) -> SliceCycle<T>;
}

impl<T, const N: usize> SliceExt<T> for [T; N] {
    fn cycle(&self) -> SliceCycle<T> {
        assert!(N > 0);
        SliceCycle {
            slice: &self[..],
            index: 0,
        }
    }
}

impl<'a, T> Iterator for SliceCycle<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next())
    }
}

impl<'a, T, const N: usize> SliceExt<T> for &'a [T; N] {
    fn cycle(&self) -> SliceCycle<T> {
        assert!(N > 0);
        SliceCycle {
            slice: &self[..],
            index: 0,
        }
    }
}

impl<'a, T> SliceExt<T> for &'a [T] {
    fn cycle(&self) -> SliceCycle<T> {
        if self.is_empty() {
            panic!("Cannot create cycle with empty slice");
        }
        SliceCycle {
            slice: &self[..],
            index: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_slice_cycle() {
        let array = [5, 10, 15];
        let mut iter = array.cycle();
        assert_eq!(&5, iter.next());
        assert_eq!(&10, iter.next());
        assert_eq!(&15, iter.next());
        assert_eq!(&5, iter.next());
        assert_eq!(&10, iter.next());
        assert_eq!(&15, iter.next());
        assert_eq!(&5, iter.next());
        for _ in 0..100 {
            assert_eq!(0, iter.next() % 5);
        }
    }

    #[test]
    fn test_examples_part1() {
        assert_eq!(3068, Day17.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(3184, Day17.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        // assert_eq!(0, Day17.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        // assert_eq!(0, Day17.part2(Example::Real, Debug::NotDebug));
    }
}
