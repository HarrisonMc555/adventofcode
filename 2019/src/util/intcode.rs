use crate::util::digits::DigitsRev;
use std::collections::VecDeque;
use std::convert::TryFrom;
use num_bigint::BigInt;
use num_traits::identities::{One, Zero};

const DEBUG: bool = false;

macro_rules! debug {
    ( $e:expr ) => {
        if DEBUG {
            dbg!($e)
        } else {
            $e
        }
    };
    ( $($e:expr),* ) => {
        if DEBUG {
            dbg!(( $(&$e),* ))
        } else {
            ( $(&$e),* )
        }
    };
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IntCode {
    memory: Vec<Value>,
    index: usize,
    inputs: VecDeque<Value>,
    outputs: VecDeque<Value>,
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
    NeedInput,
}

#[derive(Debug)]
pub enum Stopped {
    NeedInput(IntCode),
    Complete(Product),
}

pub type Error = String;
pub type Result<T> = std::result::Result<T, Error>;
pub type Value = BigInt;

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

type BinaryOp = fn(&Value, &Value) -> Value;
type CmpOp = fn(&Value, &Value) -> bool;
type UnaryBoolOp = fn(&Value) -> bool;

impl IntCode {
    pub fn from_str(s: &str) -> Result<Self> {
        s.parse()
    }

    fn new(memory: Vec<Value>) -> Self {
        IntCode {
            memory,
            index: 0,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
        }
    }

    pub fn run(mut self) -> Result<Product> {
        debug!("running");
        loop {
            match self.step()? {
                Going::Continue => {
                    debug!("Continuing");
                    ()
                }
                Going::Stop => {
                    debug!("Stopping");
                    break;
                }
                Going::NeedInput => return Err("Ran out of inputs".to_string()),
            }
        }
        debug!("done running");
        Ok(Product::new(self.memory, self.outputs))
    }

    pub fn run_blocking_input(mut self) -> Result<Stopped> {
        debug!("running blocking");
        loop {
            match self.step()? {
                Going::Continue => {
                    debug!("Continuing");
                    ()
                }
                Going::Stop => {
                    debug!("Stopping, complete");
                    return Ok(Stopped::Complete(Product::new(self.memory, self.outputs)));
                }
                Going::NeedInput => {
                    debug!("Stopping, need input");
                    return Ok(Stopped::NeedInput(self));
                }
            }
        }
    }

    pub fn with_inputs(mut self, inputs: Vec<Value>) -> Self {
        self.inputs = VecDeque::from(inputs);
        self
    }

    pub fn push_input(&mut self, input: &Value) {
        self.inputs.push_back(input.clone());
    }

    pub fn pop_output(&mut self) -> Result<Value> {
        self.outputs
            .pop_front()
            .ok_or_else(|| "No outputs".to_string())
    }

    pub fn altered(mut self, noun: &Value, verb: &Value) -> Result<Self> {
        self.alter(noun, verb)?;
        Ok(self)
    }

    fn alter(&mut self, noun: &Value, verb: &Value) -> Result<()> {
        *self.get_mut(1)? = noun.clone();
        *self.get_mut(2)? = verb.clone();
        Ok(())
    }

    fn step(&mut self) -> Result<Going> {
        let value = self.next_value()?;
        debug!(&value);
        let instruction = Instruction::try_from(&value)?;
        debug!(&instruction);
        self.execute(instruction)
    }

    fn execute(&mut self, instruction: Instruction) -> Result<Going> {
        let modes = instruction.parameter_modes;
        match instruction.opcode {
            OpCode::Add => self.op_add(modes),
            OpCode::Mul => self.op_mul(modes),
            OpCode::Input => self.op_input(modes),
            OpCode::Output => self.op_output(modes),
            OpCode::JumpIfTrue => self.op_jump_if_true(modes),
            OpCode::JumpIfFalse => self.op_jump_if_false(modes),
            OpCode::LessThan => self.op_less_than(modes),
            OpCode::Equals => self.op_equals(modes),
            OpCode::Halt => Ok(Going::Stop),
        }
    }

    fn op_add(&mut self, modes: ParameterModes) -> Result<Going> {
        self.binary_op(modes, |x, y| x + y)
    }

    fn op_mul(&mut self, modes: ParameterModes) -> Result<Going> {
        self.binary_op(modes, |x, y| x * y)
    }

    fn op_input(&mut self, _modes: ParameterModes) -> Result<Going> {
        let input = match self.inputs.pop_front() {
            Some(v) => v,
            None => {
                // Roll back index so we can be restarted
                self.index -= 1;
                return Ok(Going::NeedInput);
            }
        };
        let (param_index,) = self.get_params1(ParameterMode::Immediate)?;
        let param_index = to_usize(&param_index)?;
        let dest = self.get_mut(param_index)?;
        *dest = input;
        Ok(Going::Continue)
    }

    fn op_output(&mut self, modes: ParameterModes) -> Result<Going> {
        let (value,) = self.get_params1(modes.get(0))?;
        let value = value.clone();
        self.outputs.push_back(value);
        Ok(Going::Continue)
    }

    fn op_jump_if_true(&mut self, modes: ParameterModes) -> Result<Going> {
        self.jump_op(modes, |v| v != &Value::zero())
    }

    fn op_jump_if_false(&mut self, modes: ParameterModes) -> Result<Going> {
        self.jump_op(modes, |v| v == &Value::zero())
    }

    fn op_less_than(&mut self, modes: ParameterModes) -> Result<Going> {
        self.cmp_op(modes, |x, y| x < y)
    }

    fn op_equals(&mut self, modes: ParameterModes) -> Result<Going> {
        self.cmp_op(modes, |x, y| x == y)
    }

    fn jump_op(&mut self, modes: ParameterModes, op: UnaryBoolOp) -> Result<Going> {
        let (value, dest_index) = self.get_params2(modes.get(0), modes.get(1))?;
        let value = value.clone();
        let dest_index = dest_index.clone();
        if op(&value) {
            self.do_jump(&dest_index)
        } else {
            Ok(Going::Continue)
        }
    }

    fn do_jump(&mut self, dest_index: &Value) -> Result<Going> {
        self.index = to_usize(dest_index)?;
        Ok(Going::Continue)
    }

    fn binary_op(&mut self, modes: ParameterModes, op: BinaryOp) -> Result<Going> {
        let (op1, op2, dest_index) =
            self.get_params3(modes.get(0), modes.get(1), ParameterMode::Immediate)?;
        let dest_index = to_usize(&dest_index)?;
        let dest = self.get_mut(dest_index)?;
        *dest = op(&op1, &op2);
        Ok(Going::Continue)
    }

    fn cmp_op(&mut self, modes: ParameterModes, op: CmpOp) -> Result<Going> {
        let (op1, op2, dest_index) =
            self.get_params3(modes.get(0), modes.get(1), ParameterMode::Immediate)?;
        let dest_index = to_usize(&dest_index)?;
        let dest = self.get_mut(dest_index)?;
        let value = if op(&op1, &op2) { Value::one() } else { Value::zero() };
        *dest = value;
        Ok(Going::Continue)
    }

    fn get_params1(&mut self, mode1: ParameterMode) -> Result<(Value,)> {
        let param1 = self.next_param(mode1)?.clone();
        Ok((param1,))
    }

    fn get_params2(
        &mut self,
        mode1: ParameterMode,
        mode2: ParameterMode,
    ) -> Result<(Value, Value)> {
        let param1 = self.next_param(mode1)?.clone();
        let param2 = self.next_param(mode2)?.clone();
        Ok((param1, param2))
    }

    fn get_params3(
        &mut self,
        mode1: ParameterMode,
        mode2: ParameterMode,
        mode3: ParameterMode,
    ) -> Result<(Value, Value, Value)> {
        let param1 = self.next_param(mode1)?.clone();
        let param2 = self.next_param(mode2)?.clone();
        let param3 = self.next_param(mode3)?.clone();
        Ok((param1, param2, param3))
    }

    #[allow(dead_code)]
    fn get_params4(
        &mut self,
        mode1: ParameterMode,
        mode2: ParameterMode,
        mode3: ParameterMode,
        mode4: ParameterMode,
    ) -> Result<(Value, Value, Value, Value)> {
        let param1 = self.next_param(mode1)?.clone();
        let param2 = self.next_param(mode2)?.clone();
        let param3 = self.next_param(mode3)?.clone();
        let param4 = self.next_param(mode4)?.clone();
        Ok((param1, param2, param3, param4))
    }

    fn next_param(&mut self, mode: ParameterMode) -> Result<Value> {
        let value = self.next_value()?.clone();
        let value = match mode {
            ParameterMode::Position => self
                .get(to_usize(&value)?)
                .map(|v| v.clone())
                .map_err(|_| "Param index out of bound".to_string())?,
            ParameterMode::Immediate => value.clone(),
        };
        Ok(value)
    }

    fn next_value(&mut self) -> Result<Value> {
        let value = self.get(self.index)?.clone();
        self.index += 1;
        Ok(value)
    }

    fn get(&self, index: usize) -> Result<&Value> {
        get(&self.memory, index)
    }

    fn get_mut(&mut self, index: usize) -> Result<&mut Value> {
        self.memory
            .get_mut(index)
            .ok_or("index out of bounds".to_string())
    }

    #[allow(dead_code)]
    fn debug_print(&self) {
        if !DEBUG {
            return;
        }
        eprintln!("memory = [");
        for (i, v) in self.memory.iter().enumerate() {
            let cur_marker = if i == self.index { " <--" } else { "" };
            eprintln!("\t[{:>3}] = {}{}", i, v, cur_marker);
        }
        eprintln!("]");
        eprintln!("inputs: {:?}", self.inputs);
        eprintln!("outputs: {:?}", self.outputs);
        eprintln!();
    }
}

impl Product {
    pub fn new(memory: Vec<Value>, outputs: VecDeque<Value>) -> Self {
        Product {
            memory,
            outputs: Vec::from(debug!(outputs)),
        }
    }

    #[allow(dead_code)]
    pub fn memory(&self) -> &[Value] {
        &self.memory
    }

    #[allow(dead_code)]
    pub fn outputs(&self) -> &[Value] {
        &self.outputs
    }

    pub fn get_memory_at(&self, index: usize) -> Result<Value> {
        get(&self.memory, index).map(|v| v.clone())
    }

    #[allow(dead_code)]
    pub fn first_output(&self) -> Result<Value> {
        self.outputs
            .first()
            .cloned()
            .ok_or_else(|| "No outputs".to_string())
    }

    pub fn last_output(&self) -> Result<Value> {
        self.outputs
            .last()
            .cloned()
            .ok_or_else(|| "No outputs".to_string())
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

fn get<T>(slice: &[T], index: usize) -> Result<&T>
{
    slice
        .get(index)
        .ok_or(format!("Index {} out of bounds", index))
}

fn to_usize(input: &Value) -> Result<usize> {
    use num_traits::cast::ToPrimitive;
    input
        .to_usize()
        .ok_or_else(|| format!("Cannot convert {} to usize", input))
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

impl TryFrom<&Value> for Instruction {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let mut digits = DigitsRev::decimal(value.clone());
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
        };
        Ok(mode)
    }
}
