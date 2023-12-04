use crate::days::{Day, Debug, Example, Part};
use crate::{debug_print, debug_println};

pub struct Day01;

impl Day for Day01 {
    fn number(&self) -> u32 {
        1
    }

    fn part1(&self, example: Example, debug: Debug) -> String {
        let runner = Runner::new(Part::Part1, example, debug);
        self.get_lines(Part::Part1, example)
            .into_iter()
            .map(|line| runner.calibration_value(&line).unwrap())
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self, example: Example, debug: Debug) -> String {
        let runner = Runner::new(Part::Part1, example, debug);
        self.get_lines(Part::Part2, example)
            .into_iter()
            .map(|line| runner.calibration_value_part2(&line).unwrap())
            .sum::<u32>()
            .to_string()
    }
}

struct Runner {
    part: Part,
    example: Example,
    debug: Debug,
}

impl Runner {
    const BASE: u32 = 10;
    const ENGLISH_DIGIT_NAMES: [&'static [u8]; Self::BASE as usize] = [
        b"zero",  //
        b"one",   //
        b"two",   //
        b"three", //
        b"four",  //
        b"five",  //
        b"six",   //
        b"seven", //
        b"eight", //
        b"nine",  //
    ];

    pub fn new(part: Part, example: Example, debug: Debug) -> Self {
        Self {
            part,
            example,
            debug,
        }
    }

    fn calibration_value(&self, string: &str) -> Option<u32> {
        let first = self.first_digit(string)?;
        let last = self.last_digit(string)?;
        Some(first * 10 + last)
    }

    fn first_digit(&self, string: &str) -> Option<u32> {
        self.extract_first_digit(string.chars())
    }

    fn last_digit(&self, string: &str) -> Option<u32> {
        self.extract_first_digit(string.chars().rev())
    }

    fn extract_first_digit<I>(&self, iterator: I) -> Option<u32>
    where
        I: IntoIterator<Item = char>,
    {
        iterator
            .into_iter()
            .flat_map(|c| c.to_digit(Self::BASE))
            .next()
    }

    fn calibration_value_part2(&self, string: &str) -> Option<u32> {
        // println!("Calling on {string}");
        // std::io::Write::flush(&mut std::io::stdout()).expect("Could not flush");
        debug_print!(self.debug, "Calibration value for {string}: ");
        let first = self.first_digit_part2(string)?;
        debug_print!(self.debug, "{first}");
        let last = self.last_digit_part2(string)?;
        debug_print!(self.debug, "{last}");
        debug_println!(self.debug);
        Some(first * 10 + last)
    }

    fn first_digit_part2(&self, string: &str) -> Option<u32> {
        let characters = string.bytes().collect::<Vec<_>>();
        for index in 0..characters.len() {
            if let Some(digit) = self.match_digit_start(&characters[index..]) {
                return Some(digit);
            }
        }
        None
    }

    fn match_digit_start(&self, characters: &[u8]) -> Option<u32> {
        let first = *characters.first()?;
        if let Some(digit) = self.ascii_byte_to_digit(first) {
            return Some(digit);
        }
        for (digit, digit_name) in Self::ENGLISH_DIGIT_NAMES.into_iter().enumerate() {
            if characters.starts_with(digit_name) {
                return Some(digit as u32);
            }
        }
        None
    }

    fn last_digit_part2(&self, string: &str) -> Option<u32> {
        let characters = string.bytes().collect::<Vec<_>>();
        for index in (0..=characters.len()).rev() {
            debug_println!(
                self.debug,
                "Trying to match from end with {}",
                String::from_utf8(characters[..index].to_vec()).expect("Invalid string"),
            );
            if let Some(digit) = self.match_digit_end(&characters[..index]) {
                return Some(digit);
            }
        }
        None
    }

    fn match_digit_end(&self, characters: &[u8]) -> Option<u32> {
        let last = *characters.last()?;
        if let Some(digit) = self.ascii_byte_to_digit(last) {
            return Some(digit);
        }
        for (digit, digit_name) in Self::ENGLISH_DIGIT_NAMES.into_iter().enumerate() {
            if characters.ends_with(digit_name) {
                debug_println!(
                    self.debug,
                    "Matched {} with {} to make digit {digit}",
                    String::from_utf8(characters.to_vec()).expect("Invalid string"),
                    String::from_utf8(digit_name.to_vec()).expect("Invalid string"),
                );
                return Some(digit as u32);
            }
        }
        None
    }

    fn ascii_byte_to_digit(&self, byte: u8) -> Option<u32> {
        if !byte.is_ascii_digit() {
            return None;
        }
        Some((byte - b'0') as u32)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Debug, Example};

    #[test]
    fn test_examples_part1() {
        assert_eq!("142", Day01.part1(Example::Example, Debug::NotDebug))
    }

    #[test]
    fn test_real_part1() {
        assert_eq!("54630", Day01.part1(Example::Real, Debug::NotDebug))
    }

    #[test]
    fn test_examples_part2() {
        assert_eq!("281", Day01.part2(Example::Example, Debug::NotDebug))
    }

    #[test]
    fn test_real_part2() {
        assert_eq!("54770", Day01.part2(Example::Real, Debug::NotDebug))
    }
}
