use std::fmt::{Display, Formatter};
use std::iter::Sum;
use std::ops::Add;
use std::str::FromStr;

use crate::days::{Day, Debug, Example, Part};
use crate::debug_println;

const DEBUG: bool = false;

pub struct Day25;

impl Day for Day25 {
    fn number(&self) -> u32 {
        25
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day25 {
    fn part1(&self, example: Example, _debug: Debug) -> String {
        parse_snafus(&self.read_file(example))
            .unwrap()
            .into_iter()
            .sum::<Snafu>()
            .to_string()
    }

    fn part2(&self, _example: Example, _debug: Debug) -> String {
        panic!("Day 25 has no part 2");
    }
}

type Snafu = SignedDigitsNum<5>;

#[derive(Debug, Hash, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct SignedDigitsNum<const BASE: u8> {
    digits: Vec<SignedDigit<BASE>>,
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct SignedDigit<const BASE: u8> {
    digit: i8,
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Carry {
    Positive,
    Zero,
    Negative,
}

impl<const BASE: u8> Add for &SignedDigitsNum<BASE> {
    type Output = SignedDigitsNum<BASE>;

    fn add(self, rhs: Self) -> Self::Output {
        debug_println!("Performing {:?} + {:?}", self, rhs);
        self.check();
        rhs.check();
        let mut digits_self = self.digits.iter().copied();
        let mut digits_rhs = rhs.digits.iter().copied();
        let mut digits = Vec::new();
        let mut carry = Carry::Zero;
        loop {
            let (digit_self, digit_rhs) = match (digits_self.next(), digits_rhs.next()) {
                (Some(digit_self), Some(digit_rhs)) => (digit_self, digit_rhs),
                (Some(digit_self), None) => (digit_self, SignedDigit { digit: 0 }),
                (None, Some(digit_rhs)) => (SignedDigit { digit: 0 }, digit_rhs),
                (None, None) => break,
            };
            let (digit, new_carry) = digit_self.add(digit_rhs, carry);
            digits.push(digit);
            carry = new_carry;
        }
        match carry {
            Carry::Positive => digits.push(SignedDigit { digit: 1 }),
            Carry::Zero => {}
            Carry::Negative => digits.push(SignedDigit { digit: -1}),
        }
        SignedDigitsNum { digits }
    }
}

impl<const BASE: u8> SignedDigitsNum<BASE> {
    fn check(&self) {
        let max = BASE as i8 / 2;
        for digit in self.digits.iter() {
            let digit = digit.digit;
            if digit < -max || digit > max {
                panic!("Invalid digit {}", digit);
            }
        }
    }
}

impl<const BASE: u8> Sum for SignedDigitsNum<BASE> {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        let mut sum = SignedDigitsNum { digits: Vec::new()};
        for num in iter {
            sum = &sum + &num;
        }
        sum
    }
}


impl<const BASE: u8> SignedDigit<BASE> {
    fn add(self, rhs: Self, carry: Carry) -> (Self, Carry) {
        let base = BASE as i8;
        let max = base / 2;
        let carry_digit = match carry {
            Carry::Positive => 1,
            Carry::Zero => 0,
            Carry::Negative => -1,
        };
        let result = self.digit + rhs.digit + carry_digit;
        let (digit, carry) = if result > max {
            (result - base, Carry::Positive)
        } else if result < -max {
            (result + base, Carry::Negative)
        } else {
            (result, Carry::Zero)
        };
        if digit < -max || digit > max {
            panic!("Invalid digit {}, sum of {:?} + {:?} + {:?}", digit, self, rhs, carry);
        }
        (SignedDigit { digit }, carry)
    }
}

impl<const BASE: u8> SignedDigit<BASE> {
    pub fn new(digit: i8) -> Option<Self> {
        let max = BASE / 2;
        if digit < -(max as i8) || digit > max as i8 {
            return None;
        }
        Some(SignedDigit { digit })
    }
}

fn parse_snafus(text: &str) -> Result<Vec<Snafu>, char> {
    text.lines().map(|line| line.parse()).collect()
}

impl Snafu {
    fn parse_snafu_digit(c: char) -> Result<SignedDigit<5>, char> {
        Ok(SignedDigit::new(match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => return Err(c),
        })
        .unwrap())
    }
}

impl FromStr for Snafu {
    type Err = char;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits = s
            .chars()
            .map(Snafu::parse_snafu_digit)
            .rev()
            .collect::<Result<_, _>>()?;
        Ok(Snafu { digits })
    }
}

impl Display for SignedDigitsNum<5> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for digit in self.digits.iter().rev() {
            write!(f, "{}", digit)?;
        }
        Ok(())
    }
}

impl Display for SignedDigit<5> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self.digit {
            2 => '2',
            1 => '1',
            0 => '0',
            -1 => '-',
            -2 => '=',
            // _ => unreachable!(),
            _ => panic!("Invalid digit {}", self.digit),
        };
        write!(f, "{}", c)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let pairs = [
            ("1=-0-2", vec![2, -1, 0, -1, -2, 1]),
            ("12111", vec![1, 1, 1, 2, 1]),
            ("2=0=", vec![-2, 0, -2, 2]),
        ];
        for (snafu, digits) in pairs {
            assert_eq!(
                snafu.parse::<Snafu>().unwrap(),
                s(digits.clone()),
                "Expected \"{}\" to parse to {:?}",
                snafu,
                digits
            );
        }
    }

    #[test]
    fn test_add() {
        let tests = [
            // 5     15     20
            ("10", "1=0", "1-0"),
            // 6    2     8
            ("11", "2", "2="),
        ];
        for (lhs, rhs, sum) in tests {
            let lhs_snafu: Snafu = lhs.parse().expect(&format!("{} did not parse", lhs));
            let rhs_snafu: Snafu = rhs.parse().expect(&format!("{} did not parse", rhs));
            let expected_snafu: Snafu = sum.parse().expect(&format!("{} did not parse", sum));
            let actual_snafu = &lhs_snafu + &rhs_snafu;
            assert_eq!(
                expected_snafu,
                actual_snafu,
                "Expected {} + {} == {}\nInstead got {} + {} => {}\nOr {:?} + {:?} != {:?}",
                lhs,
                rhs,
                sum,
                lhs_snafu,
                rhs_snafu,
                actual_snafu,
                lhs_snafu,
                rhs_snafu,
                actual_snafu,
            );
        }
    }

    #[test]
    fn test_examples_part1() {
        assert_eq!("2=-1=0", Day25.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!("2--1=0=-210-1=00=-=1", Day25.part1(Example::Real, Debug::NotDebug));
    }

    fn s(digits: Vec<i8>) -> Snafu {
        let digits = digits
            .into_iter()
            .map(SignedDigit::new)
            .collect::<Option<_>>()
            .unwrap();
        Snafu { digits }
    }
}
