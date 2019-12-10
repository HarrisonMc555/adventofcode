use std::convert::TryFrom;
use std::ops;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IntCode {
    memory: Vec<Value>,
    index: usize,
    inputs: Vec<Value>,
    outputs: Vec<Value>,
}

pub struct Product {
    memory: Vec<Value>,
    outputs: Vec<Value>,
}

enum Going {
    Continue,
    Stop,
}

pub type Error = ();
pub type Result<T> = std::result::Result<T, Error>;
pub type Value = usize;

const OPCODE_ADD: Value = 1;
const OPCODE_MUL: Value = 2;
const OPCODE_INPUT: Value = 3;
const OPCODE_OUTPUT: Value = 4;
const OPCODE_HALT: Value = 99;

enum OpCode {
    Add,
    Mul,
    Input,
    Output,
    Halt,
}

enum ParameterMode {
    Position,
    Immediate,
}

struct Instruction {
    opcode: OpCode,
    parameter_mode: ParameterMode,
}

pub const MAX_NOUN: Value = 99;
pub const MAX_VERB: Value = 99;

type BinaryOp = fn(Value, Value) -> Value;

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
        // let value = self.next()?;
        // let instruction = Instruction::from(value);
        // instruction.run(&mut )
        let opcode = self.next()?;
        let _ = match opcode {
            OPCODE_ADD => self.op_add(),
            OPCODE_MUL => self.op_mul(),
            OPCODE_INPUT => self.op_input(),
            OPCODE_OUTPUT => self.op_output(),
            OPCODE_HALT => return Ok(Going::Stop),
            _ => return Err(()),
        };
        Ok(Going::Continue)
    }

    fn op_add(&mut self) -> Result<()> {
        self.binary_op(ops::Add::add)
    }

    fn op_mul(&mut self) -> Result<()> {
        self.binary_op(ops::Mul::mul)
    }

    fn op_input(&mut self) -> Result<()> {
        let input = self.inputs.pop().ok_or(())?;
        let param_index = self.next()?;
        let dest = self.get_mut(param_index)?;
        *dest = input;
        Ok(())
    }

    fn op_output(&mut self) -> Result<()> {
        let param_index = self.next()?;
        let value = self.get(param_index)?;
        self.outputs.push(value);
        Ok(())
    }

    fn binary_op(&mut self, op: BinaryOp) -> Result<()> {
        let op1_index = self.next()?;
        let op2_index = self.next()?;
        let dest_index = self.next()?;
        let op1 = self.get(op1_index)?;
        let op2 = self.get(op2_index)?;
        let dest = self.get_mut(dest_index)?;
        *dest = op(op1, op2);
        Ok(())
    }

    fn next(&mut self) -> Result<Value> {
        let value = self.get(self.index)?;
        self.index += 1;
        Ok(value)
    }

    fn get(&self, index: usize) -> Result<Value> {
        get(&self.memory, index)
    }

    fn get_mut(&mut self, index: usize) -> Result<&mut Value> {
        self.memory.get_mut(index).ok_or(())
    }
}

impl Product {
    pub fn new(memory: Vec<Value>, outputs: Vec<Value>) -> Self {
        Product { memory, outputs }
    }

    pub fn memory(&self) -> &[Value] {
        &self.memory
    }

    pub fn outputs(&self) -> &[Value] {
        &self.outputs
    }

    pub fn get(&self, index: usize) -> Result<Value> {
        get(&self.memory, index)
    }
}

fn get<T>(slice: &[T], index: usize) -> Result<T>
where
    T: Copy,
{
    slice.get(index).copied().ok_or(())
}

impl std::str::FromStr for IntCode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let vec = s
            .trim()
            .split(',')
            .map(str::parse)
            .collect::<std::result::Result<_, _>>()
            .map_err(|_| ())?;
        Ok(IntCode::new(vec))
    }
}

// impl TryFrom<Value> for Instruction {
//     type Error = Error;

//     fn try_from(value: Value) -> Result<Self> {
        
//     }
// }
