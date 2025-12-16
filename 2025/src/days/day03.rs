use itertools::Itertools;

use crate::days::{Day, Debug, Example, Part};

pub struct Day03;

impl Day for Day03 {
    fn number(&self) -> u32 {
        3
    }

    fn part1(&self, example: Example, _debug: Debug) -> String {
        let lines = self.get_lines(Part::Part1, example);
        let banks = lines
            .iter()
            .map(|line| Bank::from_line(line).unwrap_or_else(|| panic!("Invalid bank {line}")));
        let highest_joltages = banks.map(|bank| {
            bank.highest_joltage(NUM_DIGITS)
                .unwrap_or_else(|| panic!("Cannot find highest joltage for bank {bank:?}"))
        });
        let sum = highest_joltages
            .into_iter()
            .map(|joltage| joltage.0)
            .sum::<u64>();
        sum.to_string()
    }

    fn part2(&self, example: Example, _debug: Debug) -> String {
        let lines = self.get_lines(Part::Part2, example);
        let banks = lines
            .iter()
            .map(|line| Bank::from_line(line).unwrap_or_else(|| panic!("Invalid bank {line}")));
        let highest_joltages = banks.map(|bank| {
            bank.highest_joltage(NUM_DIGITS_PART_2)
                .unwrap_or_else(|| panic!("Cannot find highest joltage for bank {bank:?}"))
        });
        let sum = highest_joltages
            .into_iter()
            .map(|joltage| joltage.0)
            .sum::<u64>();
        sum.to_string()
    }
}

const BASE: u32 = 10;
const NUM_DIGITS: usize = 2;
const NUM_DIGITS_PART_2: usize = 12;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Joltage(u64);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Battery {
    joltage: Joltage,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Bank {
    batteries: Vec<Battery>,
}

impl Battery {
    pub fn from_char(c: char) -> Option<Self> {
        let joltage = Joltage(c.to_digit(BASE)? as u64);
        Some(Self { joltage })
    }
}

impl Bank {
    pub fn from_line(line: &str) -> Option<Self> {
        let batteries = line
            .chars()
            .map(Battery::from_char)
            .collect::<Option<_>>()?;
        Some(Self { batteries })
    }

    pub fn highest_joltage(&self, num_digits: usize) -> Option<Joltage> {
        let batteries = &self.batteries;
        if batteries.len() < num_digits {
            return None;
        }
        let mut digits: Vec<u64> = Vec::new();
        let mut starting_index = 0;
        for digit_index in 0..num_digits {
            let ending_index_exclusive = self.batteries.len() - num_digits + digit_index + 1;
            let highest_batteries_with_indices = &batteries[starting_index..ending_index_exclusive]
                .iter()
                .enumerate()
                .max_set_by_key(|(_, battery)| *battery);
            let (index_offset, battery) = highest_batteries_with_indices.first()?;
            digits.push(battery.joltage.0);
            starting_index = starting_index + index_offset + 1;
        }
        let joltage = Joltage(digits_to_u64(&digits));
        Some(joltage)
    }
}

fn digits_to_u64(digits: &[u64]) -> u64 {
    let mut sum = 0;
    let mut factor = 1;
    for digit in digits.into_iter().rev() {
        sum += digit * factor;
        factor *= BASE as u64;
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples_part1() {
        let highest_joltage = |line: &str| {
            Bank::from_line(line)
                .unwrap()
                .highest_joltage(NUM_DIGITS)
                .unwrap()
                .0
        };
        assert_eq!(98, highest_joltage("987654321111111"));
        assert_eq!(89, highest_joltage("811111111111119"));
        assert_eq!(78, highest_joltage("234234234234278"));
        assert_eq!(92, highest_joltage("818181911112111"));
        assert_eq!("357", Day03.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!("17301", Day03.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        let highest_joltage = |line: &str| {
            Bank::from_line(line)
                .unwrap()
                .highest_joltage(NUM_DIGITS_PART_2)
                .unwrap()
                .0
        };
        assert_eq!(987654321111, highest_joltage("987654321111111"));
        assert_eq!(811111111119, highest_joltage("811111111111119"));
        assert_eq!(434234234278, highest_joltage("234234234234278"));
        assert_eq!(888911112111, highest_joltage("818181911112111"));
        assert_eq!("3121910778619", Day03.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        assert_eq!("172162399742349", Day03.part2(Example::Real, Debug::NotDebug));
    }
}
