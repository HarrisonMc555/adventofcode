use crate::days::{Day, Debug, Example, Part};

pub struct Day999;

impl Day for Day999 {
    fn number(&self) -> u32 {
        999
    }

    fn part1(&self, _example: Example, _debug: Debug) -> String {
        todo!()
    }

    fn part2(&self, _example: Example, _debug: Debug) -> String {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_examples_part1() {
        assert_eq!("0", Day999.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    #[ignore]
    fn test_real_part1() {
        assert_eq!("0", Day999.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    #[ignore]
    fn test_examples_part2() {
        assert_eq!("0", Day999.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    #[ignore]
    fn test_real_part2() {
        assert_eq!("0", Day999.part2(Example::Real, Debug::NotDebug));
    }
}
