use crate::days::{Day, Debug, Example, Part};

const DEBUG: bool = false;

pub struct Day19;

impl Day for Day19 {
    fn number(&self) -> u32 {
        19
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day19 {
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        let num_elves = self.read_file(example).parse().unwrap();
        calculate_winning_elf_part1(num_elves) + 1
    }

    fn part2(&self, example: Example, _debug: Debug) -> usize {
        let num_elves = self.read_file(example).parse().unwrap();
        calculate_winning_elf_part2(num_elves) + 1
    }
}

fn simulate_winning_elf_part1(num_elves: usize) -> usize {
    if num_elves == 0 {
        panic!("No elves");
    }
    let mut elves: Vec<_> = (0..num_elves).collect();
    let mut index: usize = 0;
    while elves.len() > 1 {
        index = (index + 1).rem_euclid(elves.len());
        elves.remove(index);
    }
    elves[0]
}

fn calculate_winning_elf_part1(num_elves: usize) -> usize {
    if num_elves == 0 {
        panic!("No elves");
    }
    let log = num_elves.ilog2();
    let without_leading_1 = num_elves ^ (1 << log);
    without_leading_1 << 1
}

fn simulate_winning_elf_part2(num_elves: usize) -> usize {
    if num_elves == 0 {
        panic!("No elves");
    }
    let mut elves: Vec<_> = (0..num_elves).collect();
    let mut index: usize = 0;
    while elves.len() > 1 {
        let offset = elves.len() / 2;
        let remove_index = (index + offset).rem_euclid(elves.len());
        elves.remove(remove_index);
        if remove_index <= index {
            index = index.rem_euclid(elves.len());
        } else {
            index = (index + 1).rem_euclid(elves.len());
        }
    }
    elves[0]
}

fn calculate_winning_elf_part2(num_elves: usize) -> usize {
    let mut previous_offset = 0;
    for num in 1..=num_elves {
        let offset = 1 + previous_offset;
        let offset = if offset < num / 2 { offset } else { offset + 1 };
        previous_offset = offset.rem_euclid(num);
    }
    previous_offset
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simulate_vs_forecast_part1() {
        for num_elves in 1..=1000 {
            assert_eq!(
                simulate_winning_elf_part1(num_elves),
                calculate_winning_elf_part1(num_elves)
            );
        }
    }

    #[test]
    fn test_simulate_vs_forecast_part2() {
        for num_elves in 1..=1000 {
            assert_eq!(
                simulate_winning_elf_part2(num_elves),
                calculate_winning_elf_part2(num_elves),
                "Simulation and forecast don't match for {} elves",
                num_elves
            );
        }
    }
}
