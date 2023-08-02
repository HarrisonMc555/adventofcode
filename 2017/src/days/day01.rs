use crate::days::{Day, Debug, Example, Part};
use itertools::Itertools;
pub struct Day01;

const RADIX: u32 = 10;

impl Day for Day01 {
    fn number(&self) -> u32 {
        1
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day01 {
    fn part1(&self, example: Example, _debug: Debug) -> u32 {
        let text = self.read_file(example);
        let digits = to_digits(&text).unwrap();
        circular_sum(&digits)
    }

    fn part2(&self, example: Example, _debug: Debug) -> u32 {
        let text = self.read_file(example);
        let digits = to_digits(&text).unwrap();
        circular_across_sum(&digits)
    }
}

fn circular_sum(digits: &[u32]) -> u32 {
    digits
        .iter()
        .circular_tuple_windows()
        .filter(|(x, y)| x == y)
        .map(|(x, _)| x)
        .sum()
}

fn circular_across_sum(digits: &[u32]) -> u32 {
    let offset = digits.len() / 2;
    digits[..offset]
        .iter()
        .enumerate()
        .filter(|(index, digit)| **digit == digits[index + offset])
        .map(|(_, digit)| digit * 2)
        .sum()
}

fn to_digits(string: &str) -> Result<Vec<u32>, char> {
    string
        .trim()
        .chars()
        .map(|c| c.to_digit(RADIX).ok_or(c))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples_part1() {
        assert_eq!(3, circular_sum("1122"));
        assert_eq!(4, circular_sum("1111"));
        assert_eq!(0, circular_sum("1234"));
        assert_eq!(9, circular_sum("91212129"));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(995, Day01.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        assert_eq!(6, circular_across_sum("1212"));
        assert_eq!(0, circular_across_sum("1221"));
        assert_eq!(4, circular_across_sum("123425"));
        assert_eq!(12, circular_across_sum("123123"));
        assert_eq!(4, circular_across_sum("12131415"));
    }

    #[test]
    fn test_real_part2() {
        assert_eq!(1130, Day01.part2(Example::Real, Debug::NotDebug));
    }

    fn circular_sum(string: &str) -> u32 {
        super::circular_sum(&to_digits(string).unwrap())
    }

    fn circular_across_sum(string: &str) -> u32 {
        super::circular_across_sum(&to_digits(string).unwrap())
    }
}
