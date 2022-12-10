use crate::days::{Day, Debug, Example, Part};

pub struct Day10;

impl Day for Day10 {
    fn number(&self) -> u32 {
        10
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day10 {
    fn part1(&self, example: Example, _debug: Debug) -> i32 {
        let instructions = parse(&self.read_file(example)).unwrap();
        let register_history = run_instructions(&instructions);
        calc_signal_strength(&register_history, &SPECIAL_CYCLES)
    }

    fn part2(&self, _example: Example, _debug: Debug) -> i32 {
        todo!()
    }
}

const SPECIAL_CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn run_instructions(instructions: &[Instruction]) -> Vec<i32> {
    let mut register = 1;
    let mut register_history = Vec::new();
    for instruction in instructions {
        for _ in 0..instruction.num_cycles() {
            register_history.push(register);
        }
        instruction.apply_instruction(&mut register);
    }
    register_history.push(register);
    register_history
}

fn calc_signal_strength(register_history: &[i32], special_cycles: &[usize]) -> i32 {
    special_cycles.iter().map(|cycle| *cycle as i32 * register_history[*cycle - 1]).sum()
}

impl Instruction {
    fn num_cycles(&self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }

    fn apply_instruction(&self, register: &mut i32) {
        if let Instruction::Addx(amount) = self {
            *register += amount;
        }
    }
}

fn parse(text: &str) -> Option<Vec<Instruction>> {
    text.trim().split('\n').map(Instruction::parse).collect()
}

impl Instruction {
    fn parse(text: &str) -> Option<Self> {
        if text == "noop" {
            return Some(Instruction::Noop);
        }

        let addx_amount = text.strip_prefix("addx ")?.parse().ok()?;
        Some(Instruction::Addx(addx_amount))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let text = include_str!("../../static/example10.txt");
        let instructions = parse(text);
        assert!(instructions.is_some());

        let text = "noop\n\
                    addx 3\n\
                    addx -5";
        let actual = parse(text).unwrap();
        let expected = vec![
            Instruction::Noop,
            Instruction::Addx(3),
            Instruction::Addx(-5),
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_examples_part1() {
        let text = "noop\n\
                    addx 3\n\
                    addx -5";
        let instructions = parse(text).unwrap();
        let actual = run_instructions(&instructions);
        let expected = vec![1, 1, 1, 4, 4, -1];
        assert_eq!(expected, actual);

        let text = include_str!("../../static/example10.txt");
        let instructions = parse(text).unwrap();
        let register_history = run_instructions(&instructions);
        let actual = calc_signal_strength(&register_history, &SPECIAL_CYCLES);
        let expected = 13140;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(17180, Day10.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
    }
}
