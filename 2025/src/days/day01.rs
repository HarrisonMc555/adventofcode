use std::str::FromStr;

use crate::days::{Day, Debug, Example, Part};

pub struct Day01;

impl Day for Day01 {
    fn number(&self) -> u32 {
        1
    }

    fn part1(&self, example: Example, debug: Debug) -> String {
        let text = self.read_file(Part::Part1, example);
        let commands = text
            .lines()
            .map(|line| line.parse::<Command>().unwrap())
            .collect::<Vec<_>>();
        let mut state = State::new(INITIAL_POSITION);
        let mut special_count = 0;
        for command in commands {
            state.run_command(command);
            if debug == Debug::Debug {
                println!("\t{}", state.position);
            }
            if state.position == SPECIAL_POSITION {
                special_count += 1;
            }
        }
        special_count.to_string()
    }

    fn part2(&self, example: Example, debug: Debug) -> String {
        let text = self.read_file(Part::Part2, example);
        let commands = text
            .lines()
            .map(|line| line.parse::<Command>().unwrap())
            .collect::<Vec<_>>();
        let mut state = State::new(INITIAL_POSITION);
        let mut special_count = 0;
        for command in commands {
            let position_before = state.position;
            let num_times_at_zero = state.run_command(command);
            let position_after = state.position;
            special_count += num_times_at_zero;
            if debug == Debug::Debug {
                println!("\tRun {command}, go from {position_before} -> {position_after}");
                if num_times_at_zero > 0 {
                    println!("\t\tpoints at zero {num_times_at_zero} times");
                }
            }
        }
        special_count.to_string()
    }
}

const NUM_POSITIONS: i32 = 100;
const INITIAL_POSITION: i32 = 50;
const SPECIAL_POSITION: i32 = 0;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Command {
    direction: Direction,
    amount: i32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct State {
    position: i32,
}

impl Direction {
    fn from_char(c: char) -> Result<Self, char> {
        Ok(match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => return Err(c),
        })
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let first_char = chars.next().ok_or_else(|| "Empty line")?;
        let direction =
            Direction::from_char(first_char).map_err(|c| format!("Invalid direction '{c}'"))?;
        let amount = chars.collect::<String>();
        let amount = amount
            .parse()
            .map_err(|e| format!("Invalid amount '{amount}' ({e:?})"))?;
        Ok(Self { direction, amount })
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let direction_char = match self.direction {
            Direction::Left => 'L',
            Direction::Right => 'R',
        };
        write!(f, "{direction_char}")?;
        write!(f, "{}", self.amount)?;
        Ok(())
    }
}

impl Command {
    pub fn offset(self) -> i32 {
        let factor = match self.direction {
            Direction::Left => -1,
            Direction::Right => 1,
        };
        self.amount * factor
    }
}

impl State {
    pub fn new(initial_position: i32) -> Self {
        Self {
            position: initial_position,
        }
    }

    pub fn run_command(&mut self, command: Command) -> usize {
        let old_position = self.position;
        let new_position = (self.position + (command.offset())).rem_euclid(NUM_POSITIONS);
        self.position = new_position;
        let complete_rotations = (command.amount / NUM_POSITIONS).abs() as usize;
        let remainder_rolled_over = match command.direction {
            Direction::Left => new_position > old_position && old_position != 0,
            Direction::Right => new_position < old_position,
        };
        let mut num_times_at_zero = complete_rotations;
        if remainder_rolled_over || new_position == 0{
            num_times_at_zero += 1;
        }
        num_times_at_zero
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples_part1() {
        assert_eq!("3", Day01.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!("1089", Day01.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        assert_eq!("6", Day01.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_run_command() {
        let command = |amount: i32| {
            if amount < 0 {
                Command {
                    direction: Direction::Left,
                    amount: amount.abs(),
                }
            } else {
                Command {
                    direction: Direction::Right,
                    amount: amount.abs(),
                }
            }
        };
        let run_command = |amount: i32| State::new(50).run_command(command(amount));

        assert_eq!(0, run_command(0));
        assert_eq!(0, run_command(10));
        assert_eq!(0, run_command(49));
        assert_eq!(1, run_command(50));
        assert_eq!(1, run_command(51));
        assert_eq!(1, run_command(100));
        assert_eq!(1, run_command(149));
        assert_eq!(2, run_command(150));
        assert_eq!(2, run_command(151));
        assert_eq!(2, run_command(200));
        assert_eq!(2, run_command(249));
        assert_eq!(3, run_command(250));
        assert_eq!(3, run_command(251));

        assert_eq!(0, run_command(0));
        assert_eq!(0, run_command(-1));
        assert_eq!(0, run_command(-10));
        assert_eq!(0, run_command(-49));
        assert_eq!(1, run_command(-50));
        assert_eq!(1, run_command(-51));
        assert_eq!(1, run_command(-100));
        assert_eq!(1, run_command(-149));
        assert_eq!(2, run_command(-150));
        assert_eq!(2, run_command(-151));
    }

    #[test]
    fn test_real_part2() {
        assert_eq!("6530", Day01.part2(Example::Real, Debug::NotDebug));
    }
}
