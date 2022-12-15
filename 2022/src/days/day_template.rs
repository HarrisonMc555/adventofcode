use crate::days::{Day, Debug, Example, Part};

pub struct Day999;

impl Day for Day999 {
    fn number(&self) -> u32 {
        999
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day999 {
    fn part1(&self, _example: Example, _debug: Debug) -> usize {
        todo!()
    }

    fn part2(&self, _example: Example, _debug: Debug) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples_part1() {
        // assert_eq!(0, Day999.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        // assert_eq!(0, Day999.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        // assert_eq!(0, Day999.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        // assert_eq!(0, Day999.part2(Example::Real, Debug::NotDebug));
    }
}
