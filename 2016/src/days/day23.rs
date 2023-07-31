use crate::days::{Day, Debug, Example, Part};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::collections::HashMap;

pub struct Day23;

const DEBUG: bool = false;

const NUM_EGGS_PART1: Value = 7;
const NUM_EGGS_PART2: Value = 12;

const INITIAL_REGISTER: Register = 'a';
const DESIRED_REGISTER: Register = 'a';

const FACTOR1_INDEX: usize = 19;
const FACTOR2_INDEX: usize = 20;

impl Day for Day23 {
    fn number(&self) -> u32 {
        23
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day23 {
    fn part1(&self, example: Example, _debug: Debug) -> Value {
        for line in self.read_file(example).lines() {
            debug_println!("{} -> {:?}", line, Instruction::parse(line));
        }
        let instructions = Instruction::parse_lines(&self.read_file(example)).unwrap();
        let mut state = State::new(instructions);
        state.registers.insert(INITIAL_REGISTER, NUM_EGGS_PART1);
        let result = state.run_to_completion();
        result.get(&DESIRED_REGISTER).copied().unwrap()
    }

    fn part2(&self, example: Example, _debug: Debug) -> Value {
        let instructions = Instruction::parse_lines(&self.read_file(example)).unwrap();
        let Instruction::Cpy(Argument::Immediate(factor1), _) = instructions[FACTOR1_INDEX] else {
            panic!("Not pre-analyzed program");
        };
        let Instruction::Jnz(Argument::Immediate(factor2), _) = instructions[FACTOR2_INDEX] else {
            panic!("Not pre-analyzed program");
        };
        factorial(NUM_EGGS_PART2) + factor1 * factor2
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Instruction {
    Cpy(Argument, Argument),
    Inc(Argument),
    Dec(Argument),
    Jnz(Argument, Argument),
    Tgl(Argument),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Argument {
    Register(Register),
    Immediate(Value),
}

type Value = i64;
type Offset = i64;
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
    Error(Error, Registers),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum NextIndex {
    Normal,
    Jump(Offset),
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Error {
    IllegalArgumentType,
}

type Result<T> = std::result::Result<T, Error>;

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
        debug_println!("Before running:");
        self.debug_print();
        loop {
            if let Some(instruction) = self.instructions.get(self.index as usize) {
                debug_println!(
                    "After executing instruction {} {:?}",
                    self.index,
                    instruction
                );
            }
            let step_result = self.step();
            match step_result {
                StepResult::Continue(next_state) => self = next_state,
                StepResult::End(registers) => return registers,
                StepResult::Error(_, registers) => return registers,
            }
            self.debug_print();
        }
    }

    fn debug_print(&self) {
        debug_println!("Instructions:");
        for (index, instruction) in self.instructions.iter().enumerate() {
            let prefix = if self.index as usize == index {
                "* "
            } else {
                "  "
            };
            debug_println!("\t{prefix}{instruction:?}");
        }
        debug_println!();
        debug_println!("Registers:");
        for register in self.registers.iter() {
            debug_println!("\t{:?}", register);
        }
        debug_println!();
    }

    fn step(mut self) -> StepResult {
        let Some(instruction) = self.instructions.get(self.index as usize) else {
            return StepResult::End(self.registers);
        };
        let instruction_result = match instruction {
            Instruction::Cpy(source, register) => self.cpy(*source, *register),
            Instruction::Inc(register) => self.inc(*register),
            Instruction::Dec(register) => self.dec(*register),
            Instruction::Jnz(register, offset) => self.jnz(*register, *offset),
            Instruction::Tgl(register) => self.tgl(*register),
        };
        let increment_type = match instruction_result {
            Ok(increment_type) => increment_type,
            // Skip instructions with illegal arguments
            Err(Error::IllegalArgumentType) => NextIndex::Normal,
        };
        let offset = match increment_type {
            NextIndex::Normal => 1,
            NextIndex::Jump(offset) => offset,
        };
        self.index += offset as isize;
        StepResult::Continue(self)
    }

    fn cpy(&mut self, source: Argument, register: Argument) -> Result<NextIndex> {
        let value = self.get_value(source);
        let register = register.to_register()?;
        self.registers.insert(register, value);
        Ok(NextIndex::Normal)
    }

    fn inc(&mut self, register: Argument) -> Result<NextIndex> {
        self.add(register, Argument::Immediate(1))
    }

    fn dec(&mut self, register: Argument) -> Result<NextIndex> {
        self.add(register, Argument::Immediate(-1))
    }

    fn add(&mut self, register: Argument, value: Argument) -> Result<NextIndex> {
        let register = register.to_register()?;
        let value = self.get_value(value);
        *self
            .registers
            .entry(register)
            .or_insert(Self::DEFAULT_VALUE) += value;
        Ok(NextIndex::Normal)
    }

    fn jnz(&mut self, source: Argument, offset: Argument) -> Result<NextIndex> {
        let offset = self.get_value(offset);
        if self.get_value(source) != 0 {
            Ok(NextIndex::Jump(offset))
        } else {
            Ok(NextIndex::Normal)
        }
    }

    fn tgl(&mut self, offset: Argument) -> Result<NextIndex> {
        let value = self.get_value(offset);
        let index = self.index + value as isize;
        let Some(instruction) = self.instructions.get_mut(index as usize) else {
            return Ok(NextIndex::Normal);
        };
        *instruction = instruction.toggle();
        Ok(NextIndex::Normal)
    }

    fn get_value(&self, argument: Argument) -> Value {
        match argument {
            Argument::Register(register) => self.register_value(register),
            Argument::Immediate(value) => value,
        }
    }

    fn register_value(&self, register: Register) -> Value {
        self.registers
            .get(&register)
            .copied()
            .unwrap_or(Self::DEFAULT_VALUE)
    }
}

impl Argument {
    pub fn to_register(self) -> Result<Register> {
        match self {
            Self::Immediate(_) => Err(Error::IllegalArgumentType),
            Self::Register(r) => Ok(r),
        }
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
            static ref JNZ_REG_IMM: Regex = Regex::new(r"jnz ([a-z]) (-?\d+)").unwrap();
            static ref JNZ_IMM_IMM: Regex = Regex::new(r"jnz (-?\d+) (-?\d+)").unwrap();
            static ref JNZ_REG_REG: Regex = Regex::new(r"jnz ([a-z]) ([a-z])").unwrap();
            static ref JNZ_IMM_REG: Regex = Regex::new(r"jnz (-?\d+) ([a-z])").unwrap();
            static ref TGL_REG: Regex = Regex::new(r"tgl ([a-z])").unwrap();
            static ref TGL_IMM: Regex = Regex::new(r"tgl (-?\d+)").unwrap();
        }

        fn get_immediate(caps: &Captures, group: usize) -> Option<Argument> {
            caps.get(group)
                .unwrap()
                .as_str()
                .parse()
                .ok()
                .map(Argument::Immediate)
        }
        fn get_register(caps: &Captures, group: usize) -> Argument {
            Argument::Register(caps.get(group).unwrap().as_str().chars().next().unwrap())
        }
        if let Some(caps) = CPY_REG.captures(line) {
            let source = get_register(&caps, 1);
            let register = get_register(&caps, 2);
            Some(Instruction::Cpy(source, register))
        } else if let Some(caps) = CPY_IMM.captures(line) {
            let source = get_immediate(&caps, 1)?;
            let register = get_register(&caps, 2);
            Some(Instruction::Cpy(source, register))
        } else if let Some(caps) = INC.captures(line) {
            let register = get_register(&caps, 1);
            Some(Instruction::Inc(register))
        } else if let Some(caps) = DEC.captures(line) {
            let register = get_register(&caps, 1);
            Some(Instruction::Dec(register))
        } else if let Some(caps) = JNZ_REG_IMM.captures(line) {
            let source = get_register(&caps, 1);
            let offset = get_immediate(&caps, 2)?;
            Some(Instruction::Jnz(source, offset))
        } else if let Some(caps) = JNZ_IMM_IMM.captures(line) {
            let source = get_immediate(&caps, 1)?;
            let offset = get_immediate(&caps, 2)?;
            Some(Instruction::Jnz(source, offset))
        } else if let Some(caps) = JNZ_REG_REG.captures(line) {
            let source = get_register(&caps, 1);
            let offset = get_register(&caps, 2);
            Some(Instruction::Jnz(source, offset))
        } else if let Some(caps) = JNZ_IMM_REG.captures(line) {
            let source = get_immediate(&caps, 1)?;
            let offset = get_register(&caps, 2);
            Some(Instruction::Jnz(source, offset))
        } else if let Some(caps) = TGL_REG.captures(line) {
            let offset = get_register(&caps, 1);
            Some(Instruction::Tgl(offset))
        } else if let Some(caps) = TGL_IMM.captures(line) {
            let offset = get_immediate(&caps, 1)?;
            Some(Instruction::Tgl(offset))
        } else {
            None
        }
    }

    fn toggle(self) -> Instruction {
        match self {
            Instruction::Cpy(arg1, arg2) => Instruction::Jnz(arg1, arg2),
            Instruction::Inc(arg1) => Instruction::Dec(arg1),
            Instruction::Dec(arg1) => Instruction::Inc(arg1),
            Instruction::Jnz(arg1, arg2) => Instruction::Cpy(arg1, arg2),
            Instruction::Tgl(arg1) => Instruction::Inc(arg1),
        }
    }
}

fn factorial(x: i64) -> i64 {
    (1..=x).product()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples_part1() {
        let input = "cpy 2 a\n\
                     tgl a\n\
                     tgl a\n\
                     tgl a\n\
                     cpy 1 a\n\
                     dec a\n\
                     dec a";
        let instructions = Instruction::parse_lines(input).unwrap();
        let state = State::new(instructions);
        let registers = state.run_to_completion();
        assert_eq!(Some(&3), registers.get(&'a'))
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(10223, Day23.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        assert_eq!(479006783, Day23.part2(Example::Real, Debug::NotDebug));
    }
}

/*
# Original:
cpy a b
dec b
cpy a d
cpy 0 a
cpy b c
inc a
dec c
jnz c -2
dec d
jnz d -5
dec b
cpy b c
cpy c d
dec d
inc c
jnz d -2
tgl c
cpy -16 c
jnz 1 c
cpy 73 c
jnz 71 d
inc a
inc d
jnz d -2
inc c
jnz c -5

# Annotated:
cpy a b
dec b
cpy a d   ### ## a *= b
cpy 0 a   |   |
cpy b c   |   ##
inc a     |   | #
dec c     |   | |
jnz c -2  |   | # a += c
dec d     |   |
jnz d -5  |   ## a += b * d
dec b     |
cpy b c   |
cpy c d   |
dec d     |
inc c     |
jnz d -2  |
tgl c     |
cpy -16 c ### a = factorial(a)
jnz 1 c
cpy 73 c  ## a += 71 * 73
cpy 71 d  ## a += 71 * c
inc a     | #
dec d     | |
jnz d -2  | # a += d
dec c     |
jnz c -5  ##


*/
