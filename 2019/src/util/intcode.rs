use crate::util::digits::DigitsRev;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::ops;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IntCode {
    memory: Vec<Value>,
    index: usize,
    inputs: Vec<Value>,
    outputs: Vec<Value>,
}

#[derive(Debug)]
pub struct Product {
    memory: Vec<Value>,
    outputs: Vec<Value>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Going {
    Continue,
    Stop,
}

// pub type Error = ();
pub type Error = String;
pub type Result<T> = std::result::Result<T, Error>;
pub type Value = i32;

const OPCODE_ADD: u8 = 1;
const OPCODE_MUL: u8 = 2;
const OPCODE_INPUT: u8 = 3;
const OPCODE_OUTPUT: u8 = 4;
const OPCODE_JUMP_IF_TRUE: u8 = 5;
const OPCODE_JUMP_IF_FALSE: u8 = 6;
const OPCODE_LESS_THAN: u8 = 7;
const OPCODE_EQUALS: u8 = 8;
const OPCODE_HALT: u8 = 99;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum OpCode {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

const MODE_POSITION: u8 = 0;
const MODE_IMMEDIATE: u8 = 1;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
}

#[derive(Debug)]
struct ParameterModes(Vec<ParameterMode>);

#[derive(Debug)]
struct Instruction {
    opcode: OpCode,
    parameter_modes: ParameterModes,
}

pub const MAX_NOUN: Value = 99;
pub const MAX_VERB: Value = 99;

type BinaryOp = fn(Value, Value) -> Value;
type CmpOp = fn(Value, Value) -> bool;

impl IntCode {
    pub fn from_str(s: &str) -> Result<Self> {
        s.parse()
    }

    fn new(memory: Vec<Value>) -> Self {
        IntCode {
            memory,
            index: 0,
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    pub fn run(mut self) -> Result<Product> {
        loop {
            match self.step()? {
                Going::Continue => (),
                Going::Stop => break,
            }
        }
        // Outputs are currently in reverse order, un-reverse them
        self.outputs.reverse();
        Ok(Product::new(self.memory, self.outputs))
    }

    pub fn with_inputs(mut self, mut inputs: Vec<Value>) -> Self {
        // We want inputs in reverse order so we can pop them off one by one
        inputs.reverse();
        self.inputs = inputs;
        self
    }

    pub fn altered(mut self, noun: Value, verb: Value) -> Result<Self> {
        self.alter(noun, verb)?;
        Ok(self)
    }

    fn alter(&mut self, noun: Value, verb: Value) -> Result<()> {
        *self.get_mut(1)? = noun;
        *self.get_mut(2)? = verb;
        Ok(())
    }

    fn step(&mut self) -> Result<Going> {
        let value = self.next_value()?;
        let instruction = Instruction::try_from(value)?;
        self.execute(instruction)
    }

    fn execute(&mut self, instruction: Instruction) -> Result<Going> {
        let modes = instruction.parameter_modes;
        let result = match instruction.opcode {
            OpCode::Add => self.op_add(modes),
            OpCode::Mul => self.op_mul(modes),
            OpCode::Input => self.op_input(modes),
            OpCode::Output => self.op_output(modes),
            OpCode::JumpIfTrue => self.op_jump_if_true(modes),
            OpCode::JumpIfFalse => self.op_jump_if_false(modes),
            OpCode::LessThan => self.op_less_than(modes),
            OpCode::Equals => self.op_equals(modes),
            OpCode::Halt => return Ok(Going::Stop),
        };
        result.map(|()| Going::Continue)
    }

    fn op_add(&mut self, modes: ParameterModes) -> Result<()> {
        self.binary_op(modes, ops::Add::add)
    }

    fn op_mul(&mut self, modes: ParameterModes) -> Result<()> {
        self.binary_op(modes, ops::Mul::mul)
    }

    fn op_input(&mut self, _modes: ParameterModes) -> Result<()> {
        let input = self.inputs.pop().ok_or("Ran out of inputs".to_string())?;
        let param_index = self.next_value()?;
        let param_index = to_usize(param_index)?;
        let dest = self.get_mut(param_index)?;
        *dest = input;
        Ok(())
    }

    fn op_output(&mut self, modes: ParameterModes) -> Result<()> {
        let value = self.next_param(modes.get(0))?;
        self.outputs.push(value);
        Ok(())
    }

    fn op_jump_if_true(&mut self, modes: ParameterModes) -> Result<()> {
        let value = self.next_param(modes.get(0))?;
        if value != 0 {
            let _ = self.do_jump(modes.get(1))?;
        }
        Ok(())
    }

    fn op_jump_if_false(&mut self, modes: ParameterModes) -> Result<()> {
        let value = self.next_param(modes.get(0))?;
        if value == 0 {
            let _ = self.do_jump(modes.get(1))?;
        }
        Ok(())
    }

    fn op_less_than(&mut self, modes: ParameterModes) -> Result<()> {
        self.cmp_op(modes, |x, y| x < y)
    }

    fn op_equals(&mut self, modes: ParameterModes) -> Result<()> {
        self.cmp_op(modes, |x, y| x == y)
    }

    fn do_jump(&mut self, mode: ParameterMode) -> Result<()> {
        let dest_index = self.next_param(mode)?;
        self.index = to_usize(dest_index)?;
        Ok(())
    }

    fn binary_op(&mut self, modes: ParameterModes, op: BinaryOp) -> Result<()> {
        let op1 = self.next_param(modes.get(0))?;
        let op2 = self.next_param(modes.get(1))?;
        let dest_index = self.next_value()?;
        let dest_index = to_usize(dest_index)?;
        let dest = self.get_mut(dest_index)?;
        *dest = op(op1, op2);
        Ok(())
    }

    fn cmp_op(&mut self, modes: ParameterModes, op: CmpOp) -> Result<()> {
        let op1 = self.next_param(modes.get(0))?;
        let op2 = self.next_param(modes.get(1))?;
        let dest_index = self.next_value()?;
        let dest_index = to_usize(dest_index)?;
        let dest = self.get_mut(dest_index)?;
        let value = if op(op1, op2) { 1 } else { 0 };
        *dest = value;
        Ok(())
    }

    fn next_param(&mut self, mode: ParameterMode) -> Result<Value> {
        let value = self.next_value()?;
        let value = match mode {
            ParameterMode::Position => self
                .get(to_usize(value)?)
                .map_err(|_| format!("Param index {} out of bound", value))?,
            ParameterMode::Immediate => value,
        };
        Ok(value)
    }

    fn next_value(&mut self) -> Result<Value> {
        let value = self.get(self.index)?;
        self.index += 1;
        Ok(value)
    }

    fn get(&self, index: usize) -> Result<Value> {
        get(&self.memory, index)
    }

    fn get_mut(&mut self, index: usize) -> Result<&mut Value> {
        // self.memory.get_mut(index).ok_or(())
        self.memory
            .get_mut(index)
            .ok_or("index out of bounds".to_string())
    }
}

impl Product {
    pub fn new(memory: Vec<Value>, outputs: Vec<Value>) -> Self {
        Product { memory, outputs }
    }

    #[allow(dead_code)]
    pub fn memory(&self) -> &[Value] {
        &self.memory
    }

    #[allow(dead_code)]
    pub fn outputs(&self) -> &[Value] {
        &self.outputs
    }

    pub fn get(&self, index: usize) -> Result<Value> {
        get(&self.memory, index)
    }
}

impl ParameterModes {
    fn get(&self, index: usize) -> ParameterMode {
        self.0
            .get(index)
            .copied()
            .unwrap_or(ParameterMode::Position)
    }
}

fn get<T>(slice: &[T], index: usize) -> Result<T>
where
    T: Copy,
{
    slice
        .get(index)
        .copied()
        .ok_or(format!("Index {} out of bounds", index))
}

fn to_usize(input: Value) -> Result<usize> {
    input
        .try_into()
        .map_err(|_| format!("Cannot convert {} to usize", input))
}

impl std::str::FromStr for IntCode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let vec = s
            .trim()
            .split(',')
            .map(str::parse)
            .collect::<std::result::Result<_, _>>()
            .map_err(|e| format!("Invalid operation: {:?}", e))?;
        Ok(IntCode::new(vec))
    }
}

impl TryFrom<Value> for Instruction {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self> {
        let mut digits = DigitsRev::decimal(value);
        // let ones = digits.next().ok_or(())?;
        let ones = digits.next().ok_or("No opcode found".to_string())?;
        let tens = digits.next().unwrap_or(0);
        let opcode = tens * 10 + ones;
        let parameter_modes = digits.map(ParameterMode::try_from).collect::<Result<_>>()?;
        Ok(Instruction {
            opcode: OpCode::try_from(opcode)?,
            parameter_modes: ParameterModes(parameter_modes),
        })
    }
}

impl TryFrom<u8> for OpCode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        let opcode = match value {
            OPCODE_ADD => OpCode::Add,
            OPCODE_MUL => OpCode::Mul,
            OPCODE_INPUT => OpCode::Input,
            OPCODE_OUTPUT => OpCode::Output,
            OPCODE_JUMP_IF_TRUE => OpCode::JumpIfTrue,
            OPCODE_JUMP_IF_FALSE => OpCode::JumpIfFalse,
            OPCODE_LESS_THAN => OpCode::LessThan,
            OPCODE_EQUALS => OpCode::Equals,
            OPCODE_HALT => OpCode::Halt,
            _ => return Err(format!("Invalid opcode {}", value)),
            // _ => return Err(()),
        };
        Ok(opcode)
    }
}

impl TryFrom<u8> for ParameterMode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        let mode = match value {
            MODE_POSITION => ParameterMode::Position,
            MODE_IMMEDIATE => ParameterMode::Immediate,
            _ => return Err(format!("Invalid opcode {}", value)),
            // _ => return Err(()),
        };
        Ok(mode)
    }
}
