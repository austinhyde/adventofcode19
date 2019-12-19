use super::{Runtime, Word};
use std::collections::HashMap;

type Opcode = u8;

pub const OP_ADD: Operation = Operation {
  opcode: 1,
  name: "add",
  params: 3,
  action: &ClosureAction(|rt, params| {
    let lhs = params[0].resolve(rt)?;
    let rhs = params[1].resolve(rt)?;
    let addr = params[2].position()?;
    let val = lhs + rhs;
    if rt.trace {
      println!("{}   setting addr {} = {} = f({}, {})", rt.id, addr, val, lhs, rhs);
    }
    rt.set(addr, val)
  }),
};

pub const OP_MUL: Operation = Operation {
  opcode: 2,
  name: "mult",
  params: 3,
  action: &ClosureAction(|rt, params| {
    let lhs = params[0].resolve(rt)?;
    let rhs = params[1].resolve(rt)?;
    let addr = params[2].position()?;
    let val = lhs * rhs;
    if rt.trace {
      println!("{}   setting addr {} = {} = f({}, {})", rt.id, addr, val, lhs, rhs);
    }
    rt.set(addr, val)
  }),
};

pub const OP_INP: Operation = Operation {
  opcode: 3,
  name: "input",
  params: 1,
  action: &ClosureAction(|rt, params| rt.read(params[0].position()?)),
};

pub const OP_OUT: Operation = Operation {
  opcode: 4,
  name: "output",
  params: 1,
  action: &ClosureAction(|rt, params| rt.write(params[0].resolve(rt)?)),
};

pub const OP_JIT: Operation = Operation {
  opcode: 5,
  name: "jump-if-true",
  params: 2,
  action: &ClosureAction(|rt, params| {
    let pred = params[0].resolve(rt)?;
    if pred != 0 {
      rt.set_jump(params[1].resolve(rt)?)?;
    }
    Ok(())
  }),
};

pub const OP_JIF: Operation = Operation {
  opcode: 6,
  name: "jump-if-false",
  params: 2,
  action: &ClosureAction(|rt, params| {
    let pred = params[0].resolve(rt)?;
    if pred == 0 {
      rt.set_jump(params[1].resolve(rt)?)?;
    }
    Ok(())
  }),
};

pub const OP_LT: Operation = Operation {
  opcode: 7,
  name: "less-than",
  params: 3,
  action: &ClosureAction(|rt, params| {
    let lhs = params[0].resolve(rt)?;
    let rhs = params[1].resolve(rt)?;
    match params[2] {
      Param::Position(addr) => rt.set(addr, if lhs < rhs { 1 } else { 0 }),
      Param::Immediate(_) => Err("param must be positional".to_string()),
    }
  }),
};

pub const OP_EQ: Operation = Operation {
  opcode: 8,
  name: "equals",
  params: 3,
  action: &ClosureAction(|rt, params| {
    let lhs = params[0].resolve(rt)?;
    let rhs = params[1].resolve(rt)?;
    match params[2] {
      Param::Position(addr) => rt.set(addr, if lhs == rhs { 1 } else { 0 }),
      Param::Immediate(_) => Err("param must be positional".to_string()),
    }
  }),
};

pub const OP_HLT: Operation = Operation {
  opcode: 99,
  name: "halt",
  params: 0,
  action: &ClosureAction(|rt, _params| rt.halt()),
};

struct ClosureAction<F: Fn(&mut Runtime, &Vec<Param>) -> Result<(), String>>(F);
impl<F: Fn(&mut Runtime, &Vec<Param>) -> Result<(), String>> OpAction for ClosureAction<F> {
  fn execute(&self, rt: &mut Runtime, params: &Vec<Param>) -> Result<(), String> {
    (self.0)(rt, params)
  }
}

// Types

pub struct Operations {
  ops: HashMap<Opcode, &'static Operation>,
}
impl Operations {
  pub fn new() -> Self {
    let mut ops = HashMap::new();
    for op in vec![
      &OP_ADD, &OP_MUL, &OP_INP, &OP_OUT, &OP_HLT, &OP_JIT, &OP_JIF, &OP_EQ, &OP_LT,
    ] {
      ops.insert(op.opcode, op);
    }
    Operations { ops }
  }
  pub fn parse(&self, rt: &Runtime) -> Result<Instruction, String> {
    if rt.trace {
      println!("{}   parse instruction", rt.id);
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
      println!("{} {:?}", rt.id, self);
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
  pub fn position(&self) -> Result<Word, String> {
    match self {
      Param::Position(addr) => Ok(*addr),
      Param::Immediate(_) => Err("must be positional parameter".to_string()),
    }
  }
}
