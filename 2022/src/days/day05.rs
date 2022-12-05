use lazy_static::lazy_static;
use regex::Regex;

use crate::days::{Day, Debug, Example, Part};

pub struct Day05;

impl Day for Day05 {
    fn number(&self) -> u32 {
        5
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer.iter().collect::<String>());
    }
}

impl Day05 {
    fn part1(&self, example: Example, _debug: Debug) -> Vec<char> {
        let (mut stacks, instructions) = parse(&self.read_file(example)).unwrap();
        execute_instructions(&mut stacks, &instructions);
        get_answer(stacks)
    }

    fn part2(&self, example: Example, _debug: Debug) -> Vec<char> {
        let (mut stacks, instructions) = parse(&self.read_file(example)).unwrap();
        execute_instructions2(&mut stacks, &instructions);
        get_answer(stacks)
    }
}

type Stacks = Vec<Vec<char>>;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn execute_instructions(stacks: &mut Stacks, instructions: &[Instruction]) {
    for instruction in instructions {
        execute_instruction(stacks, *instruction);
    }
}

fn execute_instructions2(stacks: &mut Stacks, instructions: &[Instruction]) {
    for instruction in instructions {
        execute_instruction2(stacks, *instruction);
    }
}

fn execute_instruction(stacks: &mut Stacks, instruction: Instruction) {
    for _ in 0..instruction.count {
        let item = stacks[instruction.from - 1].pop().unwrap();
        stacks[instruction.to - 1].push(item);
    }
}

fn execute_instruction2(stacks: &mut Stacks, instruction: Instruction) {
    let items = take_last_n(&mut stacks[instruction.from - 1], instruction.count);
    stacks[instruction.to - 1].extend(items);
}

fn take_last_n<T>(vec: &mut Vec<T>, count: usize) -> Vec<T> {
    let new_len = vec.len().saturating_sub(count);
    vec.split_off(new_len)
}

fn get_answer(stacks: Stacks) -> Vec<char> {
    stacks
        .into_iter()
        .map(|stack| stack.last().copied().unwrap())
        .collect()
}

impl Instruction {
    fn new(count: usize, from: usize, to: usize) -> Self {
        Self { count, from, to }
    }
}

fn parse<T: AsRef<str>>(text: T) -> Option<(Stacks, Vec<Instruction>)> {
    let parts: Vec<&str> = text.as_ref().split("\n\n").collect::<Vec<_>>();
    let (stacks_part, instructions_part) = match parts[..] {
        [stacks_part, instructions_part] => (stacks_part, instructions_part),
        _ => return None,
    };
    Some((
        parse_stacks(stacks_part)?,
        parse_instructions(instructions_part)?,
    ))
}

fn parse_stacks<T: AsRef<str>>(text: T) -> Option<Stacks> {
    let mut lines = text.as_ref().lines().collect::<Vec<_>>();
    // Drop the "labels" line
    let labels_line = lines.pop()?;
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?: (\d+)  )* (\d+) $").unwrap();
    }
    let caps = RE.captures(labels_line.as_ref())?;
    let num_stacks = caps.get(2).unwrap().as_str().parse().ok()?;
    let cells: Vec<_> = lines.into_iter().map(parse_cells).collect::<Option<_>>()?;
    let mut stacks = Vec::with_capacity(num_stacks);
    for stack_index in 0..num_stacks {
        let mut stack = Vec::new();
        for row in cells.iter().rev() {
            match row.get(stack_index)? {
                Some(cell) => stack.push(*cell),
                None => continue,
            }
        }
        stacks.push(stack);
    }
    Some(stacks)
}

fn parse_cells<T: AsRef<str>>(line: T) -> Option<Vec<Option<char>>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(   |\[\w\])( (   |\[\w\]))*$").unwrap();
    }
    let line = line.as_ref();
    if !RE.is_match(line) {
        return None;
    }
    let chars = line.chars().collect::<Vec<_>>();
    let mut cells = Vec::new();
    for cell_index in 0.. {
        let char_index = 1 + cell_index * 4;
        match chars.get(char_index) {
            Some(' ') => cells.push(None),
            Some(c) => cells.push(Some(*c)),
            None => break,
        }
    }
    Some(cells)
}

fn parse_instructions<T: AsRef<str>>(text: T) -> Option<Vec<Instruction>> {
    text.as_ref()
        .trim()
        .lines()
        .map(Instruction::parse)
        .collect()
}

impl Instruction {
    fn parse<T: AsRef<str>>(line: T) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }
        let caps = RE.captures(line.as_ref())?;
        Some(Instruction {
            count: caps.get(1).unwrap().as_str().parse().ok()?,
            from: caps.get(2).unwrap().as_str().parse().ok()?,
            to: caps.get(3).unwrap().as_str().parse().ok()?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples_part1() {
        let mut stacks = vec![to_chars("ZN"), to_chars("MCD"), to_chars("P")];
        let instructions = vec![
            Instruction::new(1, 2, 1),
            Instruction::new(3, 1, 3),
            Instruction::new(2, 2, 1),
            Instruction::new(1, 1, 2),
        ];
        execute_instructions(&mut stacks, &instructions);
        let expected_result = vec![to_chars("C"), to_chars("M"), to_chars("PDNZ")];
        assert_eq!(stacks, expected_result);
        assert_eq!(get_answer(stacks), to_chars("CMZ"));
    }

    #[test]
    fn test_examples_part2() {
        let mut stacks = vec![to_chars("ZN"), to_chars("MCD"), to_chars("P")];
        let instructions = vec![
            Instruction::new(1, 2, 1),
            Instruction::new(3, 1, 3),
            Instruction::new(2, 2, 1),
            Instruction::new(1, 1, 2),
        ];
        execute_instructions2(&mut stacks, &instructions);
        let expected_result = vec![to_chars("M"), to_chars("C"), to_chars("PZND")];
        assert_eq!(stacks, expected_result);
        assert_eq!(get_answer(stacks), to_chars("MCD"));
    }

    #[test]
    fn test_parse() {
        let text = "    [D]    \n\
                    [N] [C]    \n\
                    [Z] [M] [P]\n 1   2   3 \n\
                    \n\
                    move 1 from 2 to 1\n\
                    move 3 from 1 to 3\n\
                    move 2 from 2 to 1\n\
                    move 1 from 1 to 2";
        let (stacks_actual, instructions_actual) = parse(text).unwrap();
        let stacks_expected = vec![to_chars("ZN"), to_chars("MCD"), to_chars("P")];
        let instructions_expected = vec![
            Instruction::new(1, 2, 1),
            Instruction::new(3, 1, 3),
            Instruction::new(2, 2, 1),
            Instruction::new(1, 1, 2),
        ];
        assert_eq!(stacks_expected, stacks_actual);
        assert_eq!(instructions_expected, instructions_actual);
    }

    fn to_chars(string: &str) -> Vec<char> {
        string.chars().collect()
    }
}
