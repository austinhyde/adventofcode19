use super::{Runtime, Word};
use std::collections::HashMap;

type Opcode = u8;

pub const OP_ADD: Operation = Operation {
  opcode: 1,
  name: "add",
  params: 3,
  action: &BinAction(std::ops::Add::add),
};

pub const OP_MUL: Operation = Operation {
  opcode: 2,
  name: "mult",
  params: 3,
  action: &BinAction(std::ops::Mul::mul),
};

pub const OP_INP: Operation = Operation {
  opcode: 3,
  name: "input",
  params: 1,
  action: &InputAction {},
};

pub const OP_OUT: Operation = Operation {
  opcode: 4,
  name: "output",
  params: 1,
  action: &OutputAction {},
};

pub const OP_HLT: Operation = Operation {
  opcode: 99,
  name: "halt",
  params: 0,
  action: &HaltAction {},
};

struct BinAction<F: Fn(Word, Word) -> Word>(F);
impl<F: Fn(Word, Word) -> Word> OpAction for BinAction<F> {
  fn execute(&self, rt: &mut Runtime, params: &Vec<Param>) -> Result<(), String> {
    let lhs = params[0].resolve(rt)?;
    let rhs = params[1].resolve(rt)?;
    match params[2] {
      Param::Position(addr) => {
        let val = (self.0)(lhs, rhs);
        if rt.trace {
          println!("  setting addr {} = {} = f({}, {})", addr, val, lhs, rhs);
        }
        rt.set(addr, val)
      }
      Param::Immediate(_) => Err("third param must be position mode".to_string()),
    }
  }
}

struct InputAction;
impl OpAction for InputAction {
  fn execute(&self, rt: &mut Runtime, params: &Vec<Param>) -> Result<(), String> {
    match params[0] {
      Param::Position(addr) => {
        let v = rt.read()?;
        if rt.trace {
          println!("  setting addr {} = {}", addr, v);
        }
        rt.set(addr, v)
      }
      Param::Immediate(_) => Err("param must be position mode".to_string()),
    }
  }
}

struct OutputAction;
impl OpAction for OutputAction {
  fn execute(&self, rt: &mut Runtime, params: &Vec<Param>) -> Result<(), String> {
    let val = params[0].resolve(rt)?;
    rt.write(val)
  }
}

struct HaltAction;
impl OpAction for HaltAction {
  fn execute(&self, rt: &mut Runtime, _params: &Vec<Param>) -> Result<(), String> {
    rt.halt()
  }
}

// Types

pub struct Operations {
  ops: HashMap<Opcode, &'static Operation>,
}
impl Operations {
  pub fn new() -> Self {
    let mut ops = HashMap::new();
    for op in vec![&OP_ADD, &OP_MUL, &OP_INP, &OP_OUT, &OP_HLT] {
      ops.insert(op.opcode, op);
    }
    Operations { ops }
  }
  pub fn parse(&self, rt: &Runtime) -> Result<Instruction, String> {
    if rt.trace {
      println!("  parse instruction");
    }
    let mut word = rt.get_word(0)?;
    let opcode = word % 100;
    let operation = self
      .ops
      .get(&(opcode as Opcode))
      .ok_or(format!("No such opcode {}", opcode))?;
    word /= 100;

    let mut params = Vec::new();
    for i in 0..operation.params {
      params.push(match word % 10 {
        0 => Param::Position(rt.get_word(i as usize + 1)?),
        _ => Param::Immediate(rt.get_word(i as usize + 1)?),
      });
      word /= 10;
    }

    Ok(Instruction { operation, params })
  }
}

pub struct Operation {
  pub opcode: u8,
  pub name: &'static str,
  pub params: u8,
  pub action: &'static dyn OpAction,
}
impl Operation {
  fn execute(&self, rt: &mut Runtime, params: &Vec<Param>) -> Result<(), String> {
    self.action.execute(rt, params)
  }
}
impl PartialEq for Operation {
  fn eq(&self, other: &Self) -> bool {
    return self.opcode == other.opcode;
  }
}

pub trait OpAction {
  fn execute(&self, rt: &mut Runtime, params: &Vec<Param>) -> Result<(), String>;
}

#[derive(PartialEq)]
pub struct Instruction {
  pub operation: &'static Operation,
  pub params: Vec<Param>,
}
impl Instruction {
  pub fn execute(&self, rt: &mut Runtime) -> Result<(), String> {
    if rt.trace {
      println!("{:?}", self);
    }
    self.operation.execute(rt, &self.params)
  }
}
impl std::fmt::Debug for Instruction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{:?}", self.operation.name, self.params)
  }
}

#[derive(Debug, PartialEq)]
pub enum Param {
  Position(Word),
  Immediate(Word),
}
impl Param {
  pub fn resolve(&self, rt: &Runtime) -> Result<Word, String> {
    match self {
      Param::Position(addr) => rt.get(*addr),
      Param::Immediate(val) => Ok(*val),
    }
  }
}
