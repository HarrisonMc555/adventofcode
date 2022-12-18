use crate::days::{Day, Debug, Example, Part};

pub struct Day11;

impl Day for Day11 {
    fn number(&self) -> u32 {
        11
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day11 {
    fn part1(&self, _example: Example, _debug: Debug) -> usize {
        todo!()
    }

    fn part2(&self, _example: Example, _debug: Debug) -> usize {
        todo!()
    }
}

fn parse(text: &str) {
    todo!( )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples_part1() {
    }

    #[test]
    fn test_real_part1() {
        // assert_eq!(0, Day11.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
    }

    #[test]
    fn test_real_part2() {
        // assert_eq!(0, Day11.part2(Example::Real, Debug::NotDebug));
    }
}
