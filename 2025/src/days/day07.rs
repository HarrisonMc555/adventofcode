use std::collections::HashSet;

use crate::days::{Day, Debug, Example, Part};

pub struct Day07;

impl Day for Day07 {
    fn number(&self) -> u32 {
        7
    }

    fn part1(&self, example: Example, _debug: Debug) -> String {
        let lines = self.get_lines(Part::Part1, example);
        let (first_line, lines) = lines.split_first().unwrap();
        let mut tachyon_columns = first_line
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == TACHYON_START_CHAR)
            .map(|(i, _)| i)
            .collect::<HashSet<_>>();
        let mut num_splits = 0;
        for line in lines {
            let chars = line.chars().collect::<Vec<_>>();
            let mut new_tachyon_columns = HashSet::new();
            for i in tachyon_columns.iter().copied() {
                let c = chars[i];
                if c == TACHYON_SPLITTER_CHAR {
                    num_splits += 1;
                    if let Some(prev_i) = i.checked_sub(1) {
                        new_tachyon_columns.insert(prev_i);
                    }
                    let next_i = i + 1;
                    if next_i < chars.len() {
                        new_tachyon_columns.insert(next_i);
                    }
                } else {
                    new_tachyon_columns.insert(i);
                }
            }
            tachyon_columns = new_tachyon_columns;
        }
        num_splits.to_string()
    }

    fn part2(&self, example: Example, _debug: Debug) -> String {
        let lines = self.get_lines(Part::Part2, example);
        let (first_line, lines) = lines.split_first().unwrap();
        let mut tachyon_columns = first_line
            .chars()
            .map(|c| if c == TACHYON_START_CHAR { 1u64 } else { 0u64 })
            .collect::<Vec<_>>();
        for line in lines {
            let chars = line.chars().collect::<Vec<_>>();
            let mut new_tachyon_columns = vec![0; tachyon_columns.len()];
            for (i, count) in tachyon_columns.iter().copied().enumerate() {
                let c = chars[i];
                if c == TACHYON_SPLITTER_CHAR {
                    if let Some(prev_i) = i.checked_sub(1) {
                        new_tachyon_columns[prev_i] += count;
                    }
                    let next_i = i + 1;
                    if next_i < chars.len() {
                        new_tachyon_columns[next_i] += count;
                    }
                } else {
                    new_tachyon_columns[i] += count;
                }
            }
            tachyon_columns = new_tachyon_columns;
        }
        tachyon_columns.into_iter().sum::<u64>().to_string()
    }
}

const TACHYON_START_CHAR: char = 'S';
const TACHYON_SPLITTER_CHAR: char = '^';

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples_part1() {
        assert_eq!("21", Day07.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!("1543", Day07.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        assert_eq!("40", Day07.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        assert_eq!("3223365367809", Day07.part2(Example::Real, Debug::NotDebug));
    }
}
