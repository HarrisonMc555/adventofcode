#![allow(unused_variables, dead_code)]
use crate::util::digits::Digits;
use crate::util::iter::IteratorExtensions;

const INPUT: &str = include_str!("../../static/day15.txt");

type Value = isize;
type Digit = isize;
const BASE_PATTERN: [Digit; 4] = [0, 1, 0, -1];

pub fn main() {
    // let answer1 = solve1(INPUT);
    // println!("{:?}", answer1);
    // let answer2 = solve2(INPUT);
    // println!("{:?}", answer2);
}

fn fft(input: &[Digit], num_phases: usize) -> Vec<Digit> {
    unimplemented!()
}

fn next_fft(input: &[Digit]) -> Vec<Digit> {
    unimplemented!()
}

fn get_pattern(index: usize) -> impl Iterator<Item = Digit> {
    BASE_PATTERN
        .iter()
        .copied()
        .cycle()
        .duplicate_values(index + 1)
        .skip(1)
}

mod test {
    use super::*;

    fn digits(value: Value) -> Vec<Digit> {
        Digits::decimal(value.abs())
            .map(Digit::from)
            .collect::<Vec<_>>()
    }

    #[test]
    fn digits_pos() {
        assert_eq!(digits(123), vec![1, 2, 3]);
        assert_eq!(digits(7), vec![7]);
        assert_eq!(digits(1403), vec![1, 4, 0, 3]);
    }

    #[test]
    fn digits_zero() {
        assert_eq!(digits(0), vec![0]);
    }

    #[test]
    fn digits_neg() {
        assert_eq!(digits(-123), vec![1, 2, 3]);
    }
    
    #[test]
    fn pattern_first() {
        let mut pattern = get_pattern(0);
        assert_eq!(pattern.next(), Some(1));
        assert_eq!(pattern.next(), Some(0));
        assert_eq!(pattern.next(), Some(-1));
        assert_eq!(pattern.next(), Some(0));
        assert_eq!(pattern.next(), Some(1));
        assert_eq!(pattern.next(), Some(0));
        assert_eq!(pattern.next(), Some(-1));
        assert_eq!(pattern.next(), Some(0));
        assert_eq!(pattern.next(), Some(1));
    }

    #[test]
    fn pattern_second() {
        let mut pattern = get_pattern(1);
        assert_eq!(pattern.next(), Some(0));
        assert_eq!(pattern.next(), Some(1));
        assert_eq!(pattern.next(), Some(1));
        assert_eq!(pattern.next(), Some(0));
        assert_eq!(pattern.next(), Some(0));
        assert_eq!(pattern.next(), Some(-1));
        assert_eq!(pattern.next(), Some(-1));
        assert_eq!(pattern.next(), Some(0));
        assert_eq!(pattern.next(), Some(0));
        assert_eq!(pattern.next(), Some(1));
        assert_eq!(pattern.next(), Some(1));
        assert_eq!(pattern.next(), Some(0));
        assert_eq!(pattern.next(), Some(0));
        assert_eq!(pattern.next(), Some(-1));
        assert_eq!(pattern.next(), Some(-1));
        assert_eq!(pattern.next(), Some(0));
        assert_eq!(pattern.next(), Some(0));
        assert_eq!(pattern.next(), Some(1));
    }

    fn small_phase1() {
        let input = digits(12345678);
        let expected_output = digits(48226158);
        let output = next_fft(&input);
        assert_eq!(output, expected_output);
    }
}
