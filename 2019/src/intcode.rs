use std::ops;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IntCode {
    memory: Vec<Value>,
    index: usize,
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

pub const MAX_NOUN: Value = 99;
pub const MAX_VERB: Value = 99;

type BinaryOp = fn(Value, Value) -> Value;

impl IntCode {
    pub fn from_str(s: &str) -> Result<Self> {
        s.parse()
    }

    fn new(memory: Vec<Value>) -> Self {
        IntCode { memory, index: 0 }
    }

    pub fn run(mut self) -> Result<Value> {
        loop {
            match self.step()? {
                Going::Continue => (),
                Going::Stop => break,
            }
        }
        self.get(0)
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
        let opcode = self.next()?;
        let _ = match opcode {
            OPCODE_ADD => self.op_add()?,
            OPCODE_MUL => self.op_mul()?,
            OPCODE_INPUT => self.op_input()?,
            OPCODE_OUTPUT => self.op_output()?,
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
        Err(())
    }

    fn op_output(&mut self) -> Result<()> {
        Err(())
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
        self.memory.get(index).copied().ok_or(())
    }

    fn get_mut(&mut self, index: usize) -> Result<&mut Value> {
        self.memory.get_mut(index).ok_or(())
    }
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
