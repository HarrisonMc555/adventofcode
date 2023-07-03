use crate::days::{Day, Debug, Example, Part};
use itertools::Itertools;

const DEBUG: bool = false;

const LENGTH_PART1: usize = 272;
const LENGTH_PART2: usize = 35651584;

pub struct Day16;

impl Day for Day16 {
    fn number(&self) -> u32 {
        16
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day16 {
    fn part1(&self, example: Example, _debug: Debug) -> String {
        let input = self.read_file(example);
        let bytes = from_ascii(&input).unwrap();
        let checksum = fill_disk(bytes, LENGTH_PART1);
        to_ascii(checksum)
    }

    fn part2(&self, example: Example, _debug: Debug) -> String {
        let input = self.read_file(example);
        let bytes = from_ascii(&input).unwrap();
        let checksum = fill_disk(bytes, LENGTH_PART2);
        to_ascii(checksum)
    }
}

fn fill_disk(initial_state: Vec<bool>, len: usize) -> Vec<bool> {
    let mut state = initial_state;
    while state.len() < len {
        state = step(&state);
    }
    let state = &state[..len];
    checksum(state.to_vec())
}

fn step(bytes: &[bool]) -> Vec<bool> {
    let a = bytes;
    let b = bytes.iter().copied().rev().map(|b| !b);
    a.iter()
        .copied()
        .chain(std::iter::once(false))
        .chain(b)
        .collect()
}

fn checksum(mut bytes: Vec<bool>) -> Vec<bool> {
    loop {
        bytes = checksum_step(&bytes);
        if bytes.len() % 2 != 0 {
            break;
        }
    }
    bytes
}

fn checksum_step(bytes: &[bool]) -> Vec<bool> {
    bytes
        .iter()
        .copied()
        .tuples()
        .map(|(b1, b2)| b1 == b2)
        .collect()
}

fn from_ascii(string: &str) -> Result<Vec<bool>, u8> {
    string
        .bytes()
        .map(|b| match b {
            b'0' => Ok(false),
            b'1' => Ok(true),
            _ => Err(b),
        })
        .collect()
}

fn to_ascii(bytes: Vec<bool>) -> String {
    bytes
        .into_iter()
        .map(|b| if b { '1' } else { '0' })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_step() {
        assert_eq!(step("1"), "100");
        assert_eq!(step("0"), "001");
        assert_eq!(step("11111"), "11111000000");
        assert_eq!(step("111100001010"), "1111000010100101011110000");
    }

    #[test]
    fn test_checksum() {
        assert_eq!(checksum_step("110010110100"), "110101");
        assert_eq!(checksum_step("110101"), "100");
        assert_eq!(checksum("110010110100"), "100");
    }

    fn step(bytes: &str) -> String {
        to_ascii(super::step(&from_ascii(bytes)))
    }

    fn checksum_step(bytes: &str) -> String {
        to_ascii(super::checksum_step(&from_ascii(bytes)))
    }

    fn checksum(bytes: &str) -> String {
        to_ascii(super::checksum(from_ascii(bytes)))
    }

    fn from_ascii(bytes: &str) -> Vec<bool> {
        super::from_ascii(bytes).unwrap()
    }
}
