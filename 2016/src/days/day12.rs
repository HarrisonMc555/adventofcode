use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::days::{Day, Debug, Example, Part};

pub struct Day12;

const DESIRED_REGISTER: Register = 'a';
const INITALIZED_REGISTER: Register = 'c';
const INITIAL_VALUE: Value = 1;

impl Day for Day12 {
    fn number(&self) -> u32 {
        12
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day12 {
    fn part1(&self, example: Example, _debug: Debug) -> i32 {
        let instructions = Instruction::parse_lines(&self.read_file(example)).unwrap();
        let state = State::new(instructions);
        let result = state.run_to_completion();
        result.get(&DESIRED_REGISTER).copied().unwrap()
    }

    fn part2(&self, example: Example, _debug: Debug) -> i32 {
        let instructions = Instruction::parse_lines(&self.read_file(example)).unwrap();
        let mut state = State::new(instructions);
        state.registers.insert(INITALIZED_REGISTER, INITIAL_VALUE);
        let result = state.run_to_completion();
        result.get(&DESIRED_REGISTER).copied().unwrap()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Instruction {
    Cpy(Source, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Source, Offset),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Source {
    Register(Register),
    Immediate(Value),
}

type Value = i32;
type Offset = i32;
type Register = char;
type Registers = HashMap<Register, Value>;

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    instructions: Vec<Instruction>,
    index: isize,
    registers: Registers,
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum StepResult {
    Continue(State),
    End(Registers),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum NextIndex {
    Normal,
    Jump(Offset),
}

impl State {
    const DEFAULT_VALUE: Value = 0;

    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            index: 0,
            registers: HashMap::new(),
        }
    }

    pub fn run_to_completion(mut self) -> Registers {
        loop {
            match self.step() {
                StepResult::Continue(next_state) => self = next_state,
                StepResult::End(result) => return result,
            }
        }
    }

    fn step(mut self) -> StepResult {
        let Some(instruction) = self.instructions.get(self.index as usize) else {
            return StepResult::End(self.registers);
        };
        let increment_type = match instruction {
            Instruction::Cpy(source, register) => self.cpy(*source, *register),
            Instruction::Inc(register) => self.inc(*register),
            Instruction::Dec(register) => self.dec(*register),
            Instruction::Jnz(register, offset) => self.jnz(*register, *offset),
        };
        let offset = match increment_type {
            NextIndex::Normal => 1,
            NextIndex::Jump(offset) => offset,
        };
        self.index += offset as isize;
        StepResult::Continue(self)
    }

    fn cpy(&mut self, source: Source, register: Register) -> NextIndex {
        let value = self.source_value(source);
        self.registers.insert(register, value);
        NextIndex::Normal
    }

    fn inc(&mut self, register: Register) -> NextIndex {
        self.add(register, 1)
    }
    
    fn dec(&mut self, register: Register) -> NextIndex {
        self.add(register, -1)
    }

    fn add(&mut self, register: Register, value: Value) -> NextIndex {
        *self.registers.entry(register).or_insert(Self::DEFAULT_VALUE) += value;
        NextIndex::Normal
    }

    fn jnz(&mut self, source: Source, offset: Offset) -> NextIndex {
        if self.source_value(source) != 0 {
            NextIndex::Jump(offset)
        } else {
            NextIndex::Normal
        }
    }

    fn source_value(&self, source: Source) -> Value {
        match source {
            Source::Register(register) => self.register_value(register),
            Source::Immediate(value) => value,
        }
    }

    fn register_value(&self, register: Register) -> Value {
        self.registers.get(&register).copied().unwrap_or(Self::DEFAULT_VALUE)
    }
}

impl Instruction {
    fn parse_lines(instructions: &str) -> Option<Vec<Instruction>> {
        instructions.lines().map(Instruction::parse).collect()
    }
    
    fn parse(line: &str) -> Option<Self> {
        lazy_static! {
            static ref CPY_REG: Regex = Regex::new(r"cpy ([a-z]) ([a-z])").unwrap();
            static ref CPY_IMM: Regex = Regex::new(r"cpy (-?\d+) ([a-z])").unwrap();
            static ref INC: Regex = Regex::new(r"inc ([a-z])").unwrap();
            static ref DEC: Regex = Regex::new(r"dec ([a-z])").unwrap();
            static ref JNZ_REG: Regex = Regex::new(r"jnz ([a-z]) (-?\d+)").unwrap();
            static ref JNZ_IMM: Regex = Regex::new(r"jnz (-?\d+) (-?\d+)").unwrap();
        }

        fn parse_num(caps: &Captures, group: usize) -> Option<i32> {
            caps.get(group).unwrap().as_str().parse().ok()
        }
        fn get_register(caps: &Captures, group: usize) -> char {
            caps.get(group).unwrap().as_str().chars().next().unwrap()
        }
        if let Some(caps) = CPY_REG.captures(line) {
            let source = Source::Register(get_register(&caps, 1));
            let register = get_register(&caps, 2);
            Some(Instruction::Cpy(source, register))
        } else if let Some(caps) = CPY_IMM.captures(line) {
            let source = Source::Immediate(parse_num(&caps, 1)?);
            let register = get_register(&caps, 2);
            Some(Instruction::Cpy(source, register))
        } else if let Some(caps) = INC.captures(line) {
            let register = get_register(&caps, 1);
            Some(Instruction::Inc(register))
        } else if let Some(caps) = DEC.captures(line) {
            let register = get_register(&caps, 1);
            Some(Instruction::Dec(register))
        } else if let Some(caps) = JNZ_REG.captures(line) {
            let source = Source::Register(get_register(&caps, 1));
            let offset = parse_num(&caps, 2)?;
            Some(Instruction::Jnz(source, offset))
        } else if let Some(caps) = JNZ_IMM.captures(line) {
            let source = Source::Immediate(parse_num(&caps, 1)?);
            let offset = parse_num(&caps, 2)?;
            Some(Instruction::Jnz(source, offset))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_to_completion() {
        let state = State::new(vec![Instruction::Inc('a')]);
        let registers = state.run_to_completion();
        assert_eq!(Some(&1), registers.get(&'a'))
    }

    #[test]
    fn test_parse() {
        let text = "cpy 41 a\n\
            inc a\n\
            inc a\n\
            dec a\n\
            jnz a 2\n\
            dec a\n\
            ";
        let actual = Instruction::parse_lines(text).unwrap();
        let expected = vec![
            Instruction::Cpy(Source::Immediate(41), 'a'),
            Instruction::Inc('a'),
            Instruction::Inc('a'),
            Instruction::Dec('a'),
            Instruction::Jnz(Source::Register('a'), 2),
            Instruction::Dec('a'),
        ];
        assert_eq!(expected, actual);
    }
}