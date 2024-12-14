use counter::Counter;

use crate::days::{Day, Debug, Example, Part};

pub struct Day01;

impl Day for Day01 {
    fn number(&self) -> u32 {
        1
    }

    fn part1(&self, example: Example, _debug: Debug) -> String {
        solve1(parse(self.get_lines(Part::Part1, example)).unwrap()).to_string()
    }

    fn part2(&self, example: Example, _debug: Debug) -> String {
        solve2(parse(self.get_lines(Part::Part2, example)).unwrap()).to_string()
    }
}

fn parse(lines: Vec<String>) -> Option<(Vec<isize>, Vec<isize>)> {
    lines.into_iter().map(|s| parse_line(&s)).collect()
}

fn parse_line(string: &str) -> Option<(isize, isize)> {
    let nums = string.split_ascii_whitespace().collect::<Vec<_>>();
    let [num1, num2] = nums.as_slice() else {
        return None;
    };
    Some((num1.parse().ok()?, num2.parse().ok()?))
}

fn solve1(input: (Vec<isize>, Vec<isize>)) -> isize {
    let (mut list1, mut list2) = input;
    list1.sort_unstable();
    list2.sort_unstable();
    list1.into_iter().zip(list2.into_iter()).map(|(x, y)| (x - y).abs()).sum()
}

fn solve2(input: (Vec<isize>, Vec<isize>)) -> isize {
    let (list1, list2) = input;
    let counter2 = list2.into_iter().collect::<Counter<_>>();
    list1.into_iter().map(|x| x * *counter2.get(&x).unwrap_or(&0) as isize).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_examples_part1() {
        assert_eq!("0", Day01.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    #[ignore]
    fn test_real_part1() {
        assert_eq!("0", Day01.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    #[ignore]
    fn test_examples_part2() {
        assert_eq!("0", Day01.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    #[ignore]
    fn test_real_part2() {
        assert_eq!("0", Day01.part2(Example::Real, Debug::NotDebug));
    }
}
