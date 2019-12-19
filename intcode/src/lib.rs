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
    let mut rt = self.new_runtime();
    rt.set(1, noun)?;
    rt.set(2, verb)?;
    rt.resume(None)?;
    rt.get(0)
  }

  pub fn run_io(
    &self,
    input: &mut dyn io::Input,
    output: &mut dyn io::Output,
  ) -> Result<(), String> {
    self.new_runtime().run_io(input, output)
  }

  pub fn new_runtime(&self) -> Runtime {
    Runtime::new(self.memory.clone())
  }
}

pub struct Runtime {
  mem: Memory,
  pc: usize,
  state: RuntimeState,
  jump: Option<usize>,
  ops: ops::Operations,
  read_addr: Option<Word>,

  pub trace: bool,
  pub id: String,
}

impl Runtime {
  pub fn new(mem: Memory) -> Runtime {
    Runtime {
      mem: mem,
      pc: 0,
      state: RuntimeState::Ready,
      jump: None,
      ops: ops::Operations::new(),
      read_addr: None,
      trace: false,
      id: "".to_string(),
    }
  }
  pub fn state(&self) -> RuntimeState {
    self.state
  }
  pub fn set(&mut self, addr: Word, val: Word) -> Result<(), String> {
    if self.trace {
      println!("{}     set(addr={}, val={})", self.id, addr, val);
    }
    self.mem[addr as usize] = val;
    Ok(())
  }
  pub fn get(&self, addr: Word) -> Result<Word, String> {
    let val = self.mem[addr as usize];
    if self.trace {
      println!("{}     get(addr={}) -> {}", self.id, addr, val);
    }
    Ok(val)
  }
  pub fn get_word(&self, n: usize) -> Result<Word, String> {
    let val = self.mem[self.pc + n];
    if self.trace {
      println!(
        "{}     get_word(pc={}, n={}) -> {}",
        self.id, self.pc, n, val
      );
    }
    Ok(val)
  }
  pub fn read_instruction(&self) -> Result<ops::Instruction, String> {
    self.ops.parse(self)
  }
  pub fn set_jump(&mut self, addr: Word) -> Result<(), String> {
    self.jump = Some(addr as usize);
    Ok(())
  }
  pub fn halt(&mut self) -> Result<(), String> {
    if self.trace {
      println!("{}     halt()", self.id);
    }
    self.state = RuntimeState::Complete;
    Ok(())
  }

  pub fn read(&mut self, addr: Word) -> Result<(), String> {
    self.read_addr = Some(addr);
    self.state = RuntimeState::Resumable(None);
    if self.trace {
      println!("{}     read(addr={})", self.id, addr);
    }
    Ok(())
  }
  pub fn write(&mut self, val: Word) -> Result<(), String> {
    self.state = RuntimeState::Resumable(Some(val));
    if self.trace {
      println!("{}     write(val={})", self.id, val);
    }
    Ok(())
  }

  pub fn resume(&mut self, val: Option<Word>) -> Result<RuntimeState, String> {
    if self.trace {
      println!("{} resume({:?})", self.id, val);
    }
    if let RuntimeState::Complete = self.state {
      return Err("Cannot resume, program complete".to_string());
    }
    if let Some(addr) = self.read_addr {
      match val {
        Some(x) => self.set(addr, x)?,
        None => return Err("Expected to resume with a value and did not".to_string()),
      }
    }
    self.read_addr = None;
    self.state = RuntimeState::Ready;

    while let RuntimeState::Ready = self.state {
      if self.pc >= self.mem.len() {
        return Err("Reached end of program".to_string());
      }

      let inst = self.read_instruction()?;
      inst.execute(self)?;
      match self.jump {
        None => {
          self.pc += inst.operation.params as usize + 1;
        }
        Some(addr) => {
          self.pc = addr;
          self.jump = None;
        }
      }
    }
    Ok(self.state)
  }

  // helper for passing an input and retrieving an output
  // Some(val) is an output, None means we're done
  pub fn step(&mut self, val: Word) -> Result<Option<Word>, String> {
    match self.resume(Some(val))? {
      RuntimeState::Ready => Err("impossible state?".to_string()),
      RuntimeState::Resumable(None) => Err("unexpected ask for input".to_string()),
      RuntimeState::Resumable(Some(x)) => match self.resume(None)? {
        RuntimeState::Ready => Err("impossible state?".to_string()),
        RuntimeState::Resumable(None) => Ok(Some(x)),
        RuntimeState::Resumable(Some(y)) => Err(format!("unexpected output {}", y)),
        RuntimeState::Complete => Ok(None),
      },
      RuntimeState::Complete => Ok(None),
    }
  }

  pub fn run_io(
    &mut self,
    input: &mut dyn io::Input,
    output: &mut dyn io::Output,
  ) -> Result<(), String> {
    let mut next = None;
    while let RuntimeState::Resumable(val) = self.resume(next)? {
      match val {
        // output, expect nothing back
        Some(x) => {
          output.write(x)?;
          next = None;
        }
        // input, expect a new value
        None => {
          next = Some(input.read()?);
        }
      }
    }
    Ok(())
  }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RuntimeState {
  // fresh program
  Ready,
  // Resumable(Some(_)) broken to output some value, nothing expected back
  // Resumable(None) broken to collect some input
  Resumable(Option<Word>),
  // halted
  Complete,
}
