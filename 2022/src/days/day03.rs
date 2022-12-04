use crate::days::{Day, Debug, Example, Part};
use std::collections::HashSet;

pub struct Day03;

impl Day for Day03 {
    fn number(&self) -> u32 {
        3
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day03 {
    fn part1(&self, example: Example, _debug: Debug) -> u32 {
        self.get_lines(example)
            .into_iter()
            .map(|line| parse_line(&line))
            .map(|(left, right)| get_shared_item(&left, &right).unwrap())
            .map(|item| to_priority(item).unwrap())
            .sum()
    }

    fn part2(&self, example: Example, _debug: Debug) -> u32 {
        parse_lines2(&self.get_lines(example))
            .into_iter()
            .map(|group| get_shared_item2(group).unwrap())
            .map(|item| to_priority(item).unwrap())
            .sum()
    }
}

fn parse_line(line: &str) -> (Vec<char>, Vec<char>) {
    assert_eq!(line.len() % 2, 0);
    let mid = line.len() / 2;
    (to_chars(&line[..mid]), to_chars(&line[mid..]))
}

fn parse_lines2<T>(lines: &[T]) -> Vec<(Vec<char>, Vec<char>, Vec<char>)>
where
    T: AsRef<str>,
{
    lines
        .chunks_exact(3)
        .map(|arr| {
            (
                to_chars(&arr[0].as_ref()),
                to_chars(&arr[1].as_ref()),
                to_chars(&arr[2].as_ref()),
            )
        })
        .collect()
}

fn to_chars(string: &str) -> Vec<char> {
    string.chars().collect()
}

fn get_shared_item(left: &[char], right: &[char]) -> Option<char> {
    let left = left.iter().collect::<HashSet<_>>();
    let right = right.iter().collect::<HashSet<_>>();
    let mut shared_iter = left.intersection(&right).cloned();
    if let (Some(shared), None) = (shared_iter.next(), shared_iter.next()) {
        Some(*shared)
    } else {
        None
    }
}

fn get_shared_item2((elf1, elf2, elf3): (Vec<char>, Vec<char>, Vec<char>)) -> Option<char> {
    let elf1 = elf1.iter().collect::<HashSet<_>>();
    let elf2 = elf2.iter().collect::<HashSet<_>>();
    let elf3 = elf3.iter().collect::<HashSet<_>>();
    let shared_1_and_2 = elf1.intersection(&elf2).cloned().collect::<HashSet<_>>();
    let mut shared_iter = shared_1_and_2.intersection(&elf3).cloned();
    if let (Some(shared), None) = (shared_iter.next(), shared_iter.next()) {
        Some(*shared)
    } else {
        None
    }
}

fn to_priority(item: char) -> Option<u32> {
    Some(match item {
        'a'..='z' => item as u32 - 'a' as u32 + 1,
        'A'..='Z' => item as u32 - 'A' as u32 + 27,
        _ => return None,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_shared_items_part1() {
        let first = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let (left, right) = parse_line(first);
        assert_eq!(left, "vJrwpWtwJgWr".chars().collect::<Vec<_>>());
        assert_eq!(right, "hcsFMMfFFhFp".chars().collect::<Vec<_>>());
        let shared_item = get_shared_item(first);
        assert_eq!(Some('p'), shared_item);
        assert_eq!(
            Some('L'),
            get_shared_item("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL")
        );
        assert_eq!(Some('P'), get_shared_item("PmmdzqPrVvPwwTWBwg"));
        assert_eq!(Some('v'), get_shared_item("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"));
        assert_eq!(Some('t'), get_shared_item("ttgJtRGJQctTZtZT"));
        assert_eq!(Some('s'), get_shared_item("CrZsJsPPZsGzwwsLwLmpwMDw"));
    }

    #[test]
    fn test_priorities_part1() {
        assert_eq!(Some(16), to_priority('p'));
        assert_eq!(Some(38), to_priority('L'));
        assert_eq!(Some(42), to_priority('P'));
        assert_eq!(Some(22), to_priority('v'));
        assert_eq!(Some(20), to_priority('t'));
        assert_eq!(Some(19), to_priority('s'));
    }

    #[test]
    fn test_examples_part2() {
        let group1 = (
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        );
        let group2 = (
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        );
        assert_eq!(Some('r'), get_shared_item2(group1));
        assert_eq!(Some('Z'), get_shared_item2(group2));
    }

    fn get_shared_item(line: &str) -> Option<char> {
        let (left, right) = parse_line(line);
        super::get_shared_item(&left, &right)
    }

    fn get_shared_item2((line1, line2, line3): (&str, &str, &str)) -> Option<char> {
        super::get_shared_item2((to_chars(line1), to_chars(line2), to_chars(line3)))
    }

    fn get_priority(line: &str) -> Option<u32> {
        to_priority(get_shared_item(line)?)
    }
}
