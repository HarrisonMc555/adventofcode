use crate::days::{Day, Debug, Example, Part};
use std::collections::HashSet;

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
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        let message = to_chars(self.read_file(example).trim());
        find_first_marker(&message).unwrap()
    }

    fn part2(&self, example: Example, _debug: Debug) -> usize {
        let message = to_chars(self.read_file(example).trim());
        find_first_message(&message).unwrap()
    }
}

const NUM_FIRST_MARKER_CHARS: usize = 4;
const NUM_FIRST_MESSAGE_CHARS: usize = 14;

fn find_first_marker(message: &[char]) -> Option<usize> {
    message
        .windows(NUM_FIRST_MARKER_CHARS)
        .enumerate()
        .find(|(_, window)| none_match(window))
        .map(|(index, _)| index + NUM_FIRST_MARKER_CHARS)
}

fn find_first_message(message: &[char]) -> Option<usize> {
    message
        .windows(NUM_FIRST_MESSAGE_CHARS)
        .enumerate()
        .find(|(_, window)| none_match(window))
        .map(|(index, _)| index + NUM_FIRST_MESSAGE_CHARS)
}

fn none_match<T>(window: &[T]) -> bool
where
    T: Eq + std::hash::Hash,
{
    count_unique(window) == window.len()
}

fn count_unique<T, I>(iter: I) -> usize
where
    I: IntoIterator<Item = T>,
    T: Eq + std::hash::Hash,
{
    iter.into_iter().collect::<HashSet<_>>().len()
}

fn to_chars<T: AsRef<str>>(string: T) -> Vec<char> {
    string.as_ref().chars().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples_part1() {
        assert_eq!(Some(7), find_first_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(Some(5), find_first_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(Some(6), find_first_marker("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(
            Some(10),
            find_first_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")
        );
        assert_eq!(
            Some(11),
            find_first_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")
        );
    }

    #[test]
    fn test_examples_part2() {
        assert_eq!(
            Some(19),
            find_first_message("mjqjpqmgbljsphdztnvjfqwrcgsmlb")
        );
        assert_eq!(Some(23), find_first_message("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(Some(23), find_first_message("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(
            Some(29),
            find_first_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")
        );
        assert_eq!(
            Some(26),
            find_first_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")
        );
    }

    fn find_first_marker(string: &str) -> Option<usize> {
        super::find_first_marker(&to_chars(string))
    }

    fn find_first_message(string: &str) -> Option<usize> {
        super::find_first_message(&to_chars(string))
    }
}
