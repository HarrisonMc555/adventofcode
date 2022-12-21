use std::num::ParseIntError;
use std::str::FromStr;

use crate::days::{Day, Debug, Example, Part};
use crate::{debug_print, debug_println};

const DEBUG: bool = false;

pub struct Day20;

impl Day for Day20 {
    fn number(&self) -> u32 {
        20
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day20 {
    fn part1(&self, example: Example, _debug: Debug) -> isize {
        let nums = parse_nums(&self.read_file(example)).unwrap();
        find_coordinate_sum(nums, &GROVE_INDICES).unwrap()
    }

    fn part2(&self, _example: Example, _debug: Debug) -> isize {
        todo!()
    }
}

const GROVE_INDEX_START: isize = 0;
const GROVE_INDICES: [usize; 3] = [1000, 2000, 3000];

#[derive(Debug, Clone, Eq, PartialEq)]
struct Mixer {
    values: Vec<isize>,
    links: Vec<Link>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Link {
    prev: usize,
    next: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct OffsetVec<T> {
    offset: isize,
    data: Vec<T>,
}

fn find_coordinate_sum(nums: Vec<isize>, indices_after_zero: &[usize]) -> Option<isize> {
    let mut mixer = Mixer::new(nums);
    mixer.mix();
    let mut sum = 0;
    for &index in indices_after_zero {
        sum += mixer.find_after(GROVE_INDEX_START, index)?;
    }
    Some(sum)
}

impl Mixer {
    fn new(values: Vec<isize>) -> Self {
        let mut links = vec![Link::default(); values.len()];
        for index in 0..values.len() {
            links[index].next = (index + 1) % links.len();
            links[index].prev = index.checked_sub(1).unwrap_or(values.len() - 1);
        }
        Mixer { values, links }
    }

    fn mix(&mut self) {
        debug_println!("Initial arrangement:");
        self.debug_print();
        debug_println!();
        for index in 0..self.links.len() {
            self.mix_one(index);
            if DEBUG {
                self.validate();
            }
            let value = self.values[index];
            if self.values[index] == 0 {
                debug_println!("{} does not move:", value);
            } else {
                let prev_value = self.values[self.links[index].prev];
                let next_value = self.values[self.links[index].next];
                debug_println!("{} moves between {} and {}:", value, prev_value, next_value);
            }
            self.debug_print();
            debug_println!();
        }
    }

    fn mix_one(&mut self, index: usize) {
        let value = self.values[index];
        if value == 0 {
            return;
        }
        let Link { prev, next } = self.links[index];
        self.links[prev].next = next;
        self.links[next].prev = prev;
        let mut dest_index = index;
        if value < 0 {
            for _ in value..=0 {
                dest_index = self.links[dest_index].prev;
            }
        } else {
            for _ in 0..value {
                dest_index = self.links[dest_index].next;
            }
        }
        let old_next_index = self.links[dest_index].next;
        self.links[old_next_index].prev = index;
        self.links[dest_index].next = index;
        self.links[index].prev = dest_index;
        self.links[index].next = old_next_index;
    }

    fn find_after(&self, start_value: isize, num_after: usize) -> Option<isize> {
        debug_println!("Finding {} after {}", num_after, start_value);
        let num_after = num_after % self.links.len();
        debug_println!("\tClamped to {}", num_after);
        let start_index = self
            .values
            .iter()
            .enumerate()
            .find(|(_, &value)| value == start_value)
            .map(|(index, _)| index)?;
        let mut cur_index = start_index;
        for i in 0..num_after {
            let cur_value = self.values[cur_index];
            let next_index = self.links[cur_index].next;
            let next_value = self.values[next_index];
            debug_println!("\t{: >3}: {} -> {}", i + 1, cur_value, next_value);
            cur_index = next_index;
        }
        let result_value = self.values[cur_index];
        debug_println!("Result index: {}", cur_index);
        debug_println!("Result value: {}", result_value);
        debug_println!();
        Some(result_value)
    }

    fn validate(&self) -> bool {
        if self.links.is_empty() {
            return true;
        }
        use std::collections::HashSet;
        let mut seen_indices = HashSet::new();
        let start_index = 0;
        let mut cur_index = start_index;
        loop {
            if seen_indices.contains(&cur_index) {
                debug_println!("Already seen index {}", cur_index);
                return false;
            }
            seen_indices.insert(cur_index);
            let next_index = self.links[cur_index].next;
            let next_prev_index = self.links[next_index].prev;
            if cur_index != next_prev_index {
                let cur_value = self.values[cur_index];
                let next_value = self.values[next_index];
                let next_prev_value = self.values[next_prev_index];
                debug_println!(
                    "Indices: {} -> {}, but {} <- {}",
                    cur_index,
                    next_index,
                    next_prev_index,
                    next_index
                );
                debug_println!(
                    "Values:  {} -> {}, but {} <- {}",
                    cur_value,
                    next_value,
                    next_prev_value,
                    next_value
                );
                return false;
            }
            cur_index = next_index;
            if cur_index == start_index {
                return true;
            }
        }
    }

    fn debug_print(&self) {
        let start_index = 0;
        let mut cur_index = start_index;
        debug_print!("{}", self.values[cur_index]);
        let mut count = 0;
        loop {
            count += 1;
            if count > self.values.len() {
                debug_println!("\nInfinite loop");
                return;
            }
            cur_index = self.links[cur_index].next;
            if cur_index == start_index {
                break;
            }
            debug_print!(", {}", self.values[cur_index]);
        }
        debug_println!();
    }
}

impl Default for Link {
    fn default() -> Self {
        Link {
            prev: usize::MAX,
            next: usize::MAX,
        }
    }
}

impl FromStr for Mixer {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<_> = s
            .trim()
            .split('\n')
            .map(|line| line.parse::<isize>())
            .collect::<Result<_, _>>()?;
        Ok(Mixer::new(nums))
    }
}

fn parse_nums(text: &str) -> Option<Vec<isize>> {
    text.trim()
        .split('\n')
        .map(|line| line.parse().ok())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples_part1() {
        assert_eq!(3, Day20.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(3466, Day20.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        // assert_eq!(0, Day20.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        // assert_eq!(0, Day20.part2(Example::Real, Debug::NotDebug));
    }
}
