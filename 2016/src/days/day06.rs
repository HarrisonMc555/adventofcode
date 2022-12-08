use crate::days::{Day, Debug, Example, Part};
use counter::Counter;

pub struct Day06;

impl Day for Day06 {
    fn number(&self) -> u32 {
        6
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day06 {
    fn part1(&self, example: Example, _debug: Debug) -> String {
        error_correct(&self.get_lines(example))
    }

    fn part2(&self, example: Example, _debug: Debug) -> String {
        error_correct2(&self.get_lines(example))
    }
}

fn error_correct<T: AsRef<str>>(lines: &[T]) -> String {
    let num_letters = match lines.get(0) {
        None => return "".to_string(),
        Some(line) => line.as_ref().len(),
    };
    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.as_ref().chars().collect())
        .collect();
    (0..num_letters)
        .map(|index| find_most_common_letter(&grid, index))
        .collect()
}

fn error_correct2<T: AsRef<str>>(lines: &[T]) -> String {
    let num_letters = match lines.get(0) {
        None => return "".to_string(),
        Some(line) => line.as_ref().len(),
    };
    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.as_ref().chars().collect())
        .collect();
    (0..num_letters)
        .map(|index| find_least_common_letter(&grid, index))
        .collect()
}

fn find_most_common_letter<T: AsRef<[char]>>(lines: &[T], index: usize) -> char {
    *lines
        .iter()
        .filter_map(|line| line.as_ref().get(index))
        .collect::<Counter<_>>()
        .most_common()[0]
        .0
}

fn find_least_common_letter<T: AsRef<[char]>>(lines: &[T], index: usize) -> char {
    **lines
        .iter()
        .filter_map(|line| line.as_ref().get(index))
        .collect::<Counter<_>>()
        .iter()
        .min_by_key(|(_, count)| *count)
        .unwrap()
        .0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples_part1() {
        let text = include_str!("../../static/example06.txt");
        let lines = text.trim().lines().collect::<Vec<_>>();
        let corrected = error_correct(&lines);
        assert_eq!("easter", corrected)
    }

    #[test]
    fn test_real_part1() {
        assert_eq!("cyxeoccr", Day06.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        let text = include_str!("../../static/example06.txt");
        let lines = text.trim().lines().collect::<Vec<_>>();
        let corrected = error_correct2(&lines);
        assert_eq!("advent", corrected)
    }

    #[test]
    fn test_real_part2() {
        assert_eq!("batwpask", Day06.part2(Example::Real, Debug::NotDebug));
    }
}
