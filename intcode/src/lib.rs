pub mod io;
mod ops;
#[cfg(test)]
mod tests;

pub type Word = i32;
pub type Memory = Vec<Word>;

pub struct Program {
  memory: Memory,
}

impl Program {
  pub fn new(mem: Vec<Word>) -> Self {
    Program {
      memory: mem.clone(),
    }
  }
  pub fn parse(input: &str) -> Result<Self, String> {
    let parsed = input
      .trim()
      .split(",")
      .into_iter()
      .map(|s| s.parse::<Word>())
      .collect::<Result<Vec<_>, _>>()
      .or_else(|e| Err(format!("{}: '{}'", e.to_string(), input)))?;
    Ok(Self::new(parsed))
  }
  pub fn run(&self, noun: Word, verb: Word) -> Result<Word, String> {
    let mut input = io::NotImplemented {};
    let mut output = io::NotImplemented {};
    let mut rt = Runtime::new(self.memory.clone(), &mut input, &mut output);
    rt.set(1, noun)?;
    rt.set(2, verb)?;
    rt.execute()?;
    rt.get(0)
  }

  pub fn run_io(
    &self,
    input: &mut dyn io::Input,
    output: &mut dyn io::Output,
  ) -> Result<(), String> {
    let mut rt = Runtime::new(self.memory.clone(), input, output);
    rt.execute()
  }

  pub fn new_runtime<'a>(
    &self,
    input: &'a mut dyn io::Input,
    output: &'a mut dyn io::Output,
  ) -> Runtime<'a> {
    Runtime::new(self.memory.clone(), input, output)
  }
}

pub struct Runtime<'a> {
  mem: Memory,
  pc: usize,
  halted: bool,
  input: &'a mut dyn io::Input,
  output: &'a mut dyn io::Output,
  ops: ops::Operations,
  pub trace: bool,
}

impl Runtime<'_> {
  pub fn new<'a>(
    mem: Memory,
    input: &'a mut dyn io::Input,
    output: &'a mut dyn io::Output,
  ) -> Runtime<'a> {
    Runtime {
      mem: mem,
      pc: 0,
      halted: false,
      input,
      output,
      ops: ops::Operations::new(),
      trace: false,
    }
  }
  pub fn set(&mut self, addr: Word, val: Word) -> Result<(), String> {
    if self.trace {
      println!("    set(addr={}, val={})", addr, val);
    }
    self.mem[addr as usize] = val;
    Ok(())
  }
  pub fn get(&self, addr: Word) -> Result<Word, String> {
    let val = self.mem[addr as usize];
    if self.trace {
      println!("    get(addr={}) -> {}", addr, val);
    }
    Ok(val)
  }
  pub fn get_word(&self, n: usize) -> Result<Word, String> {
    let val = self.mem[self.pc + n];
    if self.trace {
      println!("    get_word(pc={}, n={}) -> {}", self.pc, n, val);
    }
    Ok(val)
  }
  pub fn read_instruction(&self) -> Result<ops::Instruction, String> {
    self.ops.parse(self)
  }
  pub fn execute(&mut self) -> Result<(), String> {
    self.pc = 0;
    self.halted = false;
    while !self.halted {
      if self.pc >= self.mem.len() {
        return Err("Reached end of program".to_string());
      }
      let inst = self.read_instruction()?;
      inst.execute(self)?;
      self.pc += inst.operation.params as usize + 1;
    }
    Ok(())
  }
  pub fn halt(&mut self) -> Result<(), String> {
    if self.trace {
      println!("    halt()");
    }
    self.halted = true;
    Ok(())
  }
  pub fn read(&mut self) -> Result<Word, String> {
    let val = self.input.read()?;
    if self.trace {
      println!("    read() -> {}", val);
    }
    Ok(val)
  }
  pub fn write(&mut self, val: Word) -> Result<(), String> {
    if self.trace {
      println!("    write(val={})", val);
    }
    self.output.write(val)
  }
}
