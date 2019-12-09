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
        let op: BinaryOp = match opcode {
            OPCODE_ADD => ops::Add::add,
            OPCODE_MUL => ops::Mul::mul,
            OPCODE_HALT => return Ok(Going::Stop),
            _ => return Err(()),
        };
        let op1_index = self.next()?;
        let op2_index = self.next()?;
        let dest_index = self.next()?;
        let op1 = self.get(op1_index)?;
        let op2 = self.get(op2_index)?;
        let dest = self.get_mut(dest_index)?;
        *dest = op(op1, op2);
        Ok(Going::Continue)
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
