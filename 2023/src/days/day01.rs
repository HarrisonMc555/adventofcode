use crate::days::{Day, Debug, Example};

pub struct Day01;

impl Day for Day01 {
    fn number(&self) -> u32 {
        1
    }

    fn part1(&self, example: Example, _debug: Debug) -> String {
        self.get_lines(example)
            .into_iter()
            .map(|line| calibration_value(&line).unwrap())
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self, _example: Example, _debug: Debug) -> String {
        todo!()
    }
}

const BASE: u32 = 10;

fn calibration_value(string: &str) -> Option<u32> {
    let first = first_digit(string)?;
    let last = last_digit(string)?;
    Some(first * 10 + last)
}

fn first_digit(string: &str) -> Option<u32> {
    extract_first_digit(string.chars())
}

fn last_digit(string: &str) -> Option<u32> {
    extract_first_digit(string.chars().rev())
}

fn extract_first_digit<I>(iterator: I) -> Option<u32>
where
    I: IntoIterator<Item = char>,
{
    iterator.into_iter().flat_map(|c| c.to_digit(BASE)).next()
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
}
