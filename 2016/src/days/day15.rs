use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

use crate::days::{Day, Debug, Example, Part};

const DEBUG: bool = false;

const NEW_DISC_NUM_POSITIONS: usize = 11;
const NEW_DISC_START_POSITION: usize = 0;

pub struct Day15;

impl Day for Day15 {
    fn number(&self) -> u32 {
        15
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day15 {
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        let discs = parse_discs(&self.read_file(example)).unwrap();
        first_time_lined_up(&discs)
    }

    fn part2(&self, example: Example, _debug: Debug) -> usize {
        let mut discs = parse_discs(&self.read_file(example)).unwrap();
        let new_disc = Disc::new(
            discs.len() + 1,
            NEW_DISC_NUM_POSITIONS,
            NEW_DISC_START_POSITION,
        )
        .unwrap();
        discs.push(new_disc);
        first_time_lined_up(&discs)
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Disc {
    disc_number: usize,
    num_positions: usize,
    start_position: usize,
}

impl Disc {
    pub fn new(disc_number: usize, num_positions: usize, start_position: usize) -> Option<Self> {
        if start_position >= num_positions {
            return None;
        }
        Some(Self {
            disc_number,
            num_positions,
            start_position,
        })
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Cycle {
    period: usize,
    offset: usize,
}

impl Cycle {
    pub fn from_disc(disc: Disc) -> Self {
        Self {
            period: disc.num_positions,
            offset: Cycle::calc_offset(disc),
        }
    }

    fn calc_offset(disc: Disc) -> usize {
        let reverse_offset = (disc.disc_number + disc.start_position) as isize;
        let offset = (-reverse_offset).rem_euclid(disc.num_positions as isize);
        offset as usize
    }

    pub fn incorporate(self, other: Cycle) -> Self {
        let mut offset = self.offset;
        while offset % other.period != other.offset {
            offset += self.period;
        }
        Self {
            period: self.period * other.period,
            offset,
        }
    }
}

fn first_time_lined_up(discs: &[Disc]) -> usize {
    let mut cycles: Vec<_> = discs.iter().copied().map(Cycle::from_disc).collect();
    cycles.sort_unstable_by_key(|cycle| std::cmp::Reverse(cycle.period));
    let mut super_cycle = Cycle {
        period: 1,
        offset: 0,
    };
    for cycle in cycles {
        // Technically this should be the least common multiple (LCM), but since all of the provided periods are
        // coprime (and in fact are all strictly prime) just multiplying is equivalent for the provided data.
        super_cycle = super_cycle.incorporate(cycle);
    }
    super_cycle.offset
}

impl FromStr for Disc {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+).")
                    .unwrap();
        }
        let caps = RE
            .captures(s)
            .ok_or_else(|| format!("Invalid line \"{}\"", s))?;
        let parse = |group| {
            let num_str = caps.get(group).unwrap().as_str();
            num_str.parse::<usize>().map_err(|e| e.to_string())
        };
        let disc_number = parse(1)?;
        let num_positions = parse(2)?;
        let start_position = parse(3)?;
        Disc::new(disc_number, num_positions, start_position).ok_or_else(|| {
            format!(
                "Invalid start position {} for num positions {}",
                start_position, num_positions
            )
        })
    }
}

fn parse_discs(input: &str) -> Result<Vec<Disc>, String> {
    input.lines().map(Disc::from_str).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let line1 = "Disc #1 has 5 positions; at time=0, it is at position 4.";
        let disc1 = line1.parse().ok();
        assert_eq!(Disc::new(1, 5, 4), disc1);
        let line2 = "Disc #2 has 2 positions; at time=0, it is at position 1.";
        let disc2 = line2.parse().ok();
        assert_eq!(Disc::new(2, 2, 1), disc2);
    }
}
