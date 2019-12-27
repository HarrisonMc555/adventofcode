use crate::util::digits::DigitsRev;
use num_bigint::BigInt;
use num_traits::identities::{One, Zero};
use std::collections::{HashMap, VecDeque};
use std::convert::TryFrom;

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
    memory: HashMap<usize, Value>,
    index: usize,
    inputs: VecDeque<Value>,
    outputs: VecDeque<Value>,
    relative_base: Value,
}

#[derive(Debug)]
pub struct Product {
    memory: HashMap<usize, Value>,
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
const OPCODE_ADJUST_RELATIVE_BASE: u8 = 9;
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
    AdjustRelativeBase,
    Halt,
}

const MODE_POSITION: u8 = 0;
const MODE_IMMEDIATE: u8 = 1;
const MODE_RELATIVE: u8 = 2;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
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
            memory: memory.into_iter().enumerate().collect(),
            index: 0,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
            relative_base: Value::from(0),
        }
    }

    pub fn run(mut self) -> Result<Product> {
        debug!("running");
        loop {
            match self.step()? {
                Going::Continue => {
                    debug!("Continuing");
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

    pub fn set_memory(&mut self, index: usize, value: Value) {
        self.memory.insert(index, value);
    }

    pub fn altered(mut self, noun: &Value, verb: &Value) -> Result<Self> {
        self.alter(noun, verb)?;
        Ok(self)
    }

    fn alter(&mut self, noun: &Value, verb: &Value) -> Result<()> {
        *self.get_mut(1) = noun.clone();
        *self.get_mut(2) = verb.clone();
        Ok(())
    }

    fn step(&mut self) -> Result<Going> {
        let value = self.next_value();
        debug!(&value.to_string());
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
            OpCode::AdjustRelativeBase => self.op_adjust_relative_base(modes),
            OpCode::Halt => Ok(Going::Stop),
        }
    }

    fn op_add(&mut self, modes: ParameterModes) -> Result<Going> {
        self.binary_op(modes, |x, y| x + y)
    }

    fn op_mul(&mut self, modes: ParameterModes) -> Result<Going> {
        self.binary_op(modes, |x, y| x * y)
    }

    fn op_input(&mut self, modes: ParameterModes) -> Result<Going> {
        let input = match self.inputs.pop_front() {
            Some(v) => v,
            None => {
                // Roll back index so we can be restarted
                self.index -= 1;
                return Ok(Going::NeedInput);
            }
        };
        let (dest,) = self.get_param_dest(modes.get(0))?;
        *dest = input;
        Ok(Going::Continue)
    }

    fn op_output(&mut self, modes: ParameterModes) -> Result<Going> {
        let (value,) = self.get_params1(modes.get(0))?;
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

    fn op_adjust_relative_base(&mut self, modes: ParameterModes) -> Result<Going> {
        let (value,) = self.get_params1(modes.get(0))?;
        self.relative_base += value;
        Ok(Going::Continue)
    }

    fn jump_op(&mut self, modes: ParameterModes, op: UnaryBoolOp) -> Result<Going> {
        let (value, dest_index) = self.get_params2(modes.get(0), modes.get(1))?;
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
        let (op1, op2, dest) =
            self.get_params2and_dest(modes.get(0), modes.get(1), modes.get(2))?;
        *dest = op(&op1, &op2);
        Ok(Going::Continue)
    }

    fn cmp_op(&mut self, modes: ParameterModes, op: CmpOp) -> Result<Going> {
        let (op1, op2, dest) =
            self.get_params2and_dest(modes.get(0), modes.get(1), modes.get(2))?;
        let value = if op(&op1, &op2) {
            Value::one()
        } else {
            Value::zero()
        };
        *dest = value;
        Ok(Going::Continue)
    }

    fn get_params1(&mut self, mode1: ParameterMode) -> Result<(Value,)> {
        let param1 = self.next_param(mode1)?;
        Ok((param1,))
    }

    fn get_param_dest(&mut self, mode1: ParameterMode) -> Result<(&mut Value,)> {
        let param1 = self.next_param_dest(mode1)?;
        Ok((param1,))
    }

    fn get_params2(
        &mut self,
        mode1: ParameterMode,
        mode2: ParameterMode,
    ) -> Result<(Value, Value)> {
        let param1 = self.next_param(mode1)?;
        let param2 = self.next_param(mode2)?;
        Ok((param1, param2))
    }

    fn get_params2and_dest(
        &mut self,
        mode1: ParameterMode,
        mode2: ParameterMode,
        mode3: ParameterMode,
    ) -> Result<(Value, Value, &mut Value)> {
        let param1 = self.next_param(mode1)?;
        let param2 = self.next_param(mode2)?;
        let param3 = self.next_param_dest(mode3)?;
        Ok((param1, param2, param3))
    }

    #[allow(dead_code)]
    fn get_params3(
        &mut self,
        mode1: ParameterMode,
        mode2: ParameterMode,
        mode3: ParameterMode,
    ) -> Result<(Value, Value, Value)> {
        let param1 = self.next_param(mode1)?;
        let param2 = self.next_param(mode2)?;
        let param3 = self.next_param(mode3)?;
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
        let param1 = self.next_param(mode1)?;
        let param2 = self.next_param(mode2)?;
        let param3 = self.next_param(mode3)?;
        let param4 = self.next_param(mode4)?;
        Ok((param1, param2, param3, param4))
    }

    fn next_param(&mut self, mode: ParameterMode) -> Result<Value> {
        let value = self.next_value();
        let value = match mode {
            ParameterMode::Position => self.get(to_usize(&value)?),
            ParameterMode::Immediate => value,
            ParameterMode::Relative => self.get(to_usize(&(&value + &self.relative_base))?),
        };
        Ok(value)
    }

    fn next_param_dest(&mut self, mode: ParameterMode) -> Result<&mut Value> {
        let value = self.next_value();
        let value = match mode {
            ParameterMode::Position => self.get_mut(to_usize(&value)?),
            ParameterMode::Immediate => return Err("Destination in immediate mode".to_string()),
            ParameterMode::Relative => self.get_mut(to_usize(&(&value + &self.relative_base))?),
        };
        Ok(value)
    }

    fn next_value(&mut self) -> Value {
        let value = self.get(self.index);
        self.index += 1;
        value
    }

    fn get(&mut self, index: usize) -> Value {
        self.memory.entry(index).or_insert_with(Value::zero).clone()
    }

    fn get_mut(&mut self, index: usize) -> &mut Value {
        self.memory.entry(index).or_insert_with(Value::zero)
    }

    #[allow(dead_code)]
    fn debug_print(&self) {
        if !DEBUG {
            return;
        }
        eprintln!("memory = [");
        for (&i, v) in self.memory.iter() {
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
    pub fn new(memory: HashMap<usize, Value>, outputs: VecDeque<Value>) -> Self {
        Product {
            memory,
            outputs: Vec::from(debug!(outputs)),
        }
    }

    #[allow(dead_code)]
    pub fn memory(&self) -> &HashMap<usize, Value> {
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

fn get<T>(slice: &HashMap<usize, T>, index: usize) -> Result<&T> {
    slice
        .get(&index)
        .ok_or_else(|| format!("Index {} out of bounds", index))
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
        let ones = digits.next().ok_or_else(|| "No opcode found".to_string())?;
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
            OPCODE_ADJUST_RELATIVE_BASE => OpCode::AdjustRelativeBase,
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
            MODE_RELATIVE => ParameterMode::Relative,
            _ => return Err(format!("Invalid opcode {}", value)),
        };
        Ok(mode)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day02_example1() -> Result<()> {
        test_memories(
            "1,9,10,3,2,3,11,0,99,30,40,50",
            &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        )
    }

    #[test]
    fn day02_example2() -> Result<()> {
        test_memories("1,0,0,0,99", &[2, 0, 0, 0, 99])
    }

    #[test]
    fn day02_example3() -> Result<()> {
        test_memories("2,4,4,5,99,0", &[2, 4, 4, 5, 99, 9801])
    }

    #[test]
    fn day02_example4() -> Result<()> {
        test_memories("1,1,1,4,99,5,6,0,99", &[30, 1, 1, 4, 2, 5, 6, 0, 99])
    }

    #[test]
    fn input_output() -> Result<()> {
        let program = IntCode::from_str("3,0,4,0,99")?;
        for input in &[1, 2, 3, 7, 100, 0, -101, 9999] {
            let input = Value::from(*input);
            let output = program
                .clone()
                .with_inputs(vec![input.clone()])
                .run()?
                .last_output()?;
            assert_eq!(output, input);
        }
        Ok(())
    }

    #[test]
    fn immediate() -> Result<()> {
        let product = IntCode::from_str("1002,4,3,4,33")?.run()?;
        let expected_memory: Vec<Value> = [1002, 4, 3, 4, 99]
            .iter()
            .map(|&x| Value::from(x))
            .collect::<Vec<_>>();
        assert_eq!(memory_to_vec(product.memory()), expected_memory);
        Ok(())
    }

    #[test]
    fn immediate_negative() -> Result<()> {
        test_memories("1101,100,-1,4,0", &[1101, 100, -1, 4, 99])
    }

    #[test]
    fn equal_position_mode() -> Result<()> {
        let program_str = "3,9,8,9,10,9,4,9,99,-1,8";
        let target = Value::from(8);
        let output_eq_target = IntCode::from_str(program_str)?
            .with_inputs(vec![target.clone()])
            .run()?
            .last_output()?;
        assert_eq!(output_eq_target, Value::from(1));
        let output_not_eq_target1 = IntCode::from_str(program_str)?
            .with_inputs(vec![target.clone() + 1])
            .run()?
            .last_output()?;
        assert_eq!(output_not_eq_target1, Value::from(0));
        let output_not_eq_target2 = IntCode::from_str(program_str)?
            .with_inputs(vec![target.clone() - 1])
            .run()?
            .last_output()?;
        assert_eq!(output_not_eq_target2, Value::from(0));
        Ok(())
    }

    #[test]
    fn less_than_position_mode() -> Result<()> {
        let program_str = "3,9,7,9,10,9,4,9,99,-1,8";
        let target = Value::from(8);
        let output_eq_target = IntCode::from_str(program_str)?
            .with_inputs(vec![target.clone()])
            .run()?
            .last_output()?;
        assert_eq!(output_eq_target, Value::from(0));
        let output_less_than_target = IntCode::from_str(program_str)?
            .with_inputs(vec![target.clone() - 1])
            .run()?
            .last_output()?;
        assert_eq!(output_less_than_target, Value::from(1));
        let output_greater_than_target = IntCode::from_str(program_str)?
            .with_inputs(vec![target.clone() + 1])
            .run()?
            .last_output()?;
        assert_eq!(output_greater_than_target, Value::from(0));
        Ok(())
    }

    #[test]
    fn equal_immediate_mode() -> Result<()> {
        let program_str = "3,3,1108,-1,8,3,4,3,99";
        let target = Value::from(8);
        let output_eq_target = IntCode::from_str(program_str)?
            .with_inputs(vec![target.clone()])
            .run()?
            .last_output()?;
        assert_eq!(output_eq_target, Value::from(1));
        let output_not_eq_target1 = IntCode::from_str(program_str)?
            .with_inputs(vec![target.clone() + 1])
            .run()?
            .last_output()?;
        assert_eq!(output_not_eq_target1, Value::from(0));
        let output_not_eq_target2 = IntCode::from_str(program_str)?
            .with_inputs(vec![target.clone() - 1])
            .run()?
            .last_output()?;
        assert_eq!(output_not_eq_target2, Value::from(0));
        Ok(())
    }

    #[test]
    fn less_than_immediate_mode() -> Result<()> {
        let program_str = "3,3,1107,-1,8,3,4,3,99";
        let target = Value::from(8);
        let output_eq_target = IntCode::from_str(program_str)?
            .with_inputs(vec![target.clone()])
            .run()?
            .last_output()?;
        assert_eq!(output_eq_target, Value::from(0));
        let output_less_than_target = IntCode::from_str(program_str)?
            .with_inputs(vec![target.clone() - 1])
            .run()?
            .last_output()?;
        assert_eq!(output_less_than_target, Value::from(1));
        let output_greater_than_target = IntCode::from_str(program_str)?
            .with_inputs(vec![target.clone() + 1])
            .run()?
            .last_output()?;
        assert_eq!(output_greater_than_target, Value::from(0));
        Ok(())
    }

    #[test]
    fn jump_position_mode() -> Result<()> {
        let program_str = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        let target = Value::from(0);
        let output_eq_target = IntCode::from_str(program_str)?
            .with_inputs(vec![target.clone()])
            .run()?
            .last_output()?;
        assert_eq!(output_eq_target, Value::from(0));
        let output_not_eq_target1 = IntCode::from_str(program_str)?
            .with_inputs(vec![target.clone() + 1])
            .run()?
            .last_output()?;
        assert_eq!(output_not_eq_target1, Value::from(1));
        let output_not_eq_target2 = IntCode::from_str(program_str)?
            .with_inputs(vec![target.clone() + 7])
            .run()?
            .last_output()?;
        assert_eq!(output_not_eq_target2, Value::from(1));
        Ok(())
    }

    #[test]
    fn jump_immediate_mode() -> Result<()> {
        let program_str = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        let target = Value::from(0);
        let output_eq_target = IntCode::from_str(program_str)?
            .with_inputs(vec![target.clone()])
            .run()?
            .last_output()?;
        assert_eq!(output_eq_target, Value::from(0));
        let output_not_eq_target1 = IntCode::from_str(program_str)?
            .with_inputs(vec![target.clone() + 1])
            .run()?
            .last_output()?;
        assert_eq!(output_not_eq_target1, Value::from(1));
        let output_not_eq_target2 = IntCode::from_str(program_str)?
            .with_inputs(vec![target.clone() + 7])
            .run()?
            .last_output()?;
        assert_eq!(output_not_eq_target2, Value::from(1));
        Ok(())
    }

    #[test]
    fn jump_large_example() -> Result<()> {
        let input_str = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        for value_less_than_8 in &[1, 2, 3, 7, 0, -1, -4, -999] {
            let input = Value::from(*value_less_than_8);
            expect_output(input_str, input, Value::from(999))?;
        }
        expect_output(input_str, Value::from(8), Value::from(1000))?;
        for value_greater_than_8 in &[9, 10, 11, 40, 999] {
            let input = Value::from(*value_greater_than_8);
            expect_output(input_str, input, Value::from(1001))?;
        }
        Ok(())
    }

    #[test]
    fn test_day09_example1() -> Result<()> {
        let program = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let product = IntCode::from_str(program)?.run()?;
        let memory = product.memory();
        let memory_as_str = (0..16)
            .map(|i| memory.get(&i).cloned().unwrap_or_else(|| Value::from(0)))
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");
        assert_eq!(memory_as_str, program);
        Ok(())
    }

    #[test]
    fn test_day09_example2() -> Result<()> {
        let program = "1102,34915192,34915192,7,4,7,99,0";
        let output = IntCode::from_str(program)?.run()?.first_output()?;
        let num_digits = output.to_string().chars().count();
        assert_eq!(num_digits, 16);
        Ok(())
    }

    #[test]
    fn test_day09_example3() -> Result<()> {
        let program = "104,1125899906842624,99";
        let output = IntCode::from_str(program)?.run()?.first_output()?;
        assert_eq!(
            output,
            "1125899906842624".parse().map_err(|_| "Cannot parse")?
        );
        Ok(())
    }

    fn expect_output(input_str: &str, input_value: Value, output_value: Value) -> Result<()> {
        let output = IntCode::from_str(input_str)?
            .with_inputs(vec![input_value])
            .run()?
            .first_output()?;
        assert_eq!(output, output_value);
        Ok(())
    }

    fn test_memories(input: &str, memory: &[i32]) -> Result<()> {
        let product = IntCode::from_str(input)?.run()?;
        let expected_memory: Vec<Value> =
            memory.iter().map(|&x| Value::from(x)).collect::<Vec<_>>();
        assert_eq!(memory_to_vec(product.memory()), expected_memory);
        Ok(())
    }

    use std::collections::HashMap;
    fn memory_to_vec(memory: &HashMap<usize, Value>) -> Vec<Value> {
        (0..memory.len())
            .map(|i| memory.get(&i).cloned().unwrap_or_else(|| Value::from(0)))
            .collect()
    }
}
