#![allow(unused_variables, dead_code)]
use crate::util::iter::IteratorExtensions;
use itertools::unfold;
use std::convert::TryFrom;

const INPUT: &str = include_str!("../../static/day16.txt");

type Value = isize;
type Digit = isize;
const BASE_PATTERN: [Digit; 4] = [0, 1, 0, -1];

// Notes: print(np.matrix([[0 if col - row < 0 else math.comb(n + col - row - 1, col - row) for col in range(4, 8)] for row in range(4, 8)]))
// A8 = np.matrix([[1, 0, -1, 0, 1, 0, -1, 0], [0, 1, 1, 0, 0, -1, -1, 0], [0, 0, 1, 1, 1, 0, 0, 0], [0, 0, 0, 1, 1, 1, 1, 0], [0, 0, 0, 0, 1, 1, 1, 1], [0, 0, 0, 0, 0, 1, 1, 1], [0, 0, 0, 0, 0, 0, 1, 1], [0, 0, 0, 0, 0, 0, 0, 1]])

pub fn main() {
    let answer1 = solve1(INPUT);
    println!("{}", answer1);
    // let answer2 = solve2(INPUT);
    // println!("{:?}", answer2);
}

fn solve1(input: &str) -> String {
    let input = input.trim();

    const NUM_PHASES: usize = 100;
    let input = digits(input);
    let output = fft(&input, NUM_PHASES);
    let first_8_digits = &output[..8];
    first_8_digits
        .iter()
        .map(|digit| digit.to_string())
        .collect()
}

fn digits(value: &str) -> Vec<Digit> {
    value
        .chars()
        .map(|c| {
            let digit = c.to_digit(10).expect("Invalid digit");
            Digit::try_from(digit).expect("Failed conversion")
        })
        .collect::<Vec<_>>()
}

fn fft(input: &[Digit], num_phases: usize) -> Vec<Digit> {
    itertools::iterate(input.to_vec(), |digits| next_fft(&digits))
        .nth(num_phases)
        .expect("Infinite application of itertools::iterate ended prematurely")
}

fn next_fft(input: &[Digit]) -> Vec<Digit> {
    (0..input.len()).map(|i| calc_elem(input, i)).collect()
}

fn calc_elem(input: &[Digit], index: usize) -> Digit {
    let pattern = get_pattern(index);
    let sum: Digit = input.iter().zip(pattern).map(|(d, p)| d * p).sum();
    (sum % 10).abs()
}

fn get_pattern(index: usize) -> impl Iterator<Item = Digit> {
    BASE_PATTERN
        .iter()
        .copied()
        .cycle()
        .duplicate_values(index + 1)
        .skip(1)
}

fn binoms(k: isize) -> impl Iterator<Item = isize> {
    let mut n = k;
    let mut number = 1;
    unfold(0, move |divisor| {
        let return_number = number;
        n += 1;
        number *= n;
        *divisor += 1;
        number /= *divisor;
        Some(return_number)
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn digits_pos() {
        assert_eq!(digits("123"), vec![1, 2, 3]);
        assert_eq!(digits("7"), vec![7]);
        assert_eq!(digits("1403"), vec![1, 4, 0, 3]);
    }

    #[test]
    fn digits_zero() {
        assert_eq!(digits("0"), vec![0]);
    }

    #[test]
    fn digits_neg() {
        assert_eq!(digits("123"), vec![1, 2, 3]);
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

    #[test]
    fn detailed_example() {
        let input = digits("12345678");
        let phase1 = next_fft(&input);
        assert_eq!(phase1, digits("48226158"));
        let phase2 = next_fft(&phase1);
        assert_eq!(phase2, digits("34040438"));
        let phase3 = next_fft(&phase2);
        assert_eq!(phase3, digits("03415518"));
        let phase4 = next_fft(&phase3);
        assert_eq!(phase4, digits("01029498"));
    }

    #[test]
    fn example1() {
        const NUM_PHASES: usize = 100;
        let input = digits("80871224585914546619083218645595");
        let output = fft(&input, NUM_PHASES);
        let output_first_8 = output[..8].to_vec();
        let expected_output_first_8 = digits("24176176");
        assert_eq!(output_first_8, expected_output_first_8);
    }

    #[test]
    fn example2() {
        const NUM_PHASES: usize = 100;
        let input = digits("19617804207202209144916044189917");
        let output = fft(&input, NUM_PHASES);
        let output_first_8 = output[..8].to_vec();
        let expected_output_first_8 = digits("73745418");
        assert_eq!(output_first_8, expected_output_first_8);
    }

    #[test]
    fn example3() {
        const NUM_PHASES: usize = 100;
        let input = digits("69317163492948606335995924319873");
        let output = fft(&input, NUM_PHASES);
        let output_first_8 = output[..8].to_vec();
        let expected_output_first_8 = digits("52432133");
        assert_eq!(output_first_8, expected_output_first_8);
    }

    #[test]
    fn test_binoms() {
        assert_eq!(
            binoms(2).take(10).collect::<Vec<_>>(),
            vec![1, 3, 6, 10, 15, 21, 28, 36, 45, 55]
        );
        assert_eq!(
            binoms(3).take(10).collect::<Vec<_>>(),
            vec![1, 4, 10, 20, 35, 56, 84, 120, 165, 220]
        );
    }
}
