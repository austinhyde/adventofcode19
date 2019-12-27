pub mod io;
mod ops;
#[cfg(test)]
mod tests;

use std::collections::HashMap;

pub type Word = i64;

pub struct Program {
    operations: Vec<Word>,
}

impl Program {
    pub fn new(mem: Vec<Word>) -> Self {
        Program {
            operations: mem.clone(),
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

    pub fn run_collect_output(&self, input: Vec<Word>) -> Result<Vec<Word>, String> {
        let mut i = io::IteratorInput::new(input);
        let mut v = Vec::new();
        let mut o = io::VecOutput::new(&mut v);
        self.run_io(&mut i, &mut o)?;
        Ok(v)
    }

    pub fn run_loop<F>(
        &self,
        inputs: Vec<Word>,
        n_outputs: usize,
        mut handler: F,
    ) -> Result<(), String>
    where
        F: FnMut(Vec<Word>) -> Vec<Word>,
    {
        let mut rt = self.new_runtime();
        let mut curr_inputs = inputs.clone();

        while rt.start()? {
            let out = rt.stepn(curr_inputs, n_outputs)?;
            curr_inputs = handler(out);
        }
        Ok(())
    }

    pub fn new_runtime(&self) -> Runtime {
        Runtime::new(self.operations.clone())
    }
}

pub struct Runtime {
    mem: HashMap<Word, Word>,
    pc: Word,
    state: Option<RuntimeState>,
    jump: Option<Word>,
    ops: ops::Operations,
    read_addr: Option<Word>,
    relative_base: Word,

    pub trace: bool,
    pub trace_state: bool,
    pub id: String,
}

impl Runtime {
    pub fn new(initial: Vec<Word>) -> Runtime {
        let mut m = HashMap::new();
        m.reserve(initial.len());
        Runtime {
            mem: initial.iter().enumerate().fold(m, |mut m, (i, w)| {
                m.insert(i as Word, *w);
                m
            }),
            pc: 0,
            state: None,
            jump: None,
            ops: ops::Operations::new(),
            read_addr: None,
            trace: false,
            trace_state: false,
            relative_base: 0,
            id: "".to_string(),
        }
    }
    pub fn state(&self) -> RuntimeState {
        self.state.unwrap()
    }
    pub fn set(&mut self, addr: Word, val: Word) -> Result<(), String> {
        if self.trace {
            println!("{}     set(addr={}, val={})", self.id, addr, val);
        }
        self.mem.insert(addr, val);
        Ok(())
    }
    pub fn get(&self, addr: Word) -> Result<Word, String> {
        let val = *self.mem.get(&addr).unwrap_or(&0);
        if self.trace {
            println!("{}     get(addr={}) -> {}", self.id, addr, val);
        }
        Ok(val)
    }
    pub fn get_word(&self, n: Word) -> Result<Word, String> {
        let val = *self.mem.get(&(self.pc + n)).unwrap_or(&0);
        // if self.trace && !self.trace_state {
        //     println!(
        //         "{}     get_word(pc={}, n={}) -> {}",
        //         self.id, self.pc, n, val
        //     );
        // }
        Ok(val)
    }
    pub fn adjust_relative_base(&mut self, delta: Word) -> Result<(), String> {
        self.relative_base += delta;
        if self.trace {
            println!(
                "{}     adjust_relative_base(delta={}) -> rb={}",
                self.id, delta, self.relative_base
            );
        }
        Ok(())
    }
    pub fn read_instruction(&self) -> Result<ops::Instruction, String> {
        self.ops.parse(self)
    }
    pub fn set_jump(&mut self, addr: Word) -> Result<(), String> {
        self.jump = Some(addr);
        Ok(())
    }
    pub fn halt(&mut self) -> Result<(), String> {
        if self.trace {
            println!("{}     halt()", self.id);
        }
        self.state = Some(RuntimeState::Complete);
        Ok(())
    }

    pub fn read(&mut self, addr: Word) -> Result<(), String> {
        self.read_addr = Some(addr);
        self.state = Some(RuntimeState::Resumable(None));
        if self.trace {
            println!("{}     read(addr={})", self.id, addr);
        }
        Ok(())
    }
    pub fn write(&mut self, val: Word) -> Result<(), String> {
        self.state = Some(RuntimeState::Resumable(Some(val)));
        if self.trace {
            println!("{}     write(val={})", self.id, val);
        }
        Ok(())
    }

    pub fn start(&mut self) -> Result<bool, String> {
        match self.resume(None)? {
            RuntimeState::Resumable(None) => Ok(true),
            RuntimeState::Complete => Ok(false),
            RuntimeState::Resumable(Some(x)) => Err(format!("Unexpected output: {}", x)),
        }
    }

    pub fn resume(&mut self, val: Option<Word>) -> Result<RuntimeState, String> {
        if self.trace {
            println!("{} resume({:?})", self.id, val);
        }
        if let Some(RuntimeState::Complete) = self.state {
            return Err("Cannot resume, program complete".to_string());
        }
        if let Some(addr) = self.read_addr {
            match val {
                Some(x) => self.set(addr, x)?,
                None => return Err("Expected to resume with a value and did not".to_string()),
            }
        }
        self.read_addr = None;
        self.state = None;

        while let None = self.state {
            if self.trace {
                println!("\n{} pc={} rb={}", self.id, self.pc, self.relative_base);
            }
            if self.trace_state {
                self.print_state()
            }
            // if self.pc >= self.mem.len() {
            //     return Err("Reached end of program".to_string());
            // }

            let inst = self.read_instruction()?;
            inst.execute(self)?;
            match self.jump {
                None => {
                    self.pc += inst.operation.params as Word + 1;
                }
                Some(addr) => {
                    self.pc = addr;
                    self.jump = None;
                }
            }
        }
        Ok(self.state.unwrap())
    }
    fn print_state(&mut self) {
        let mut x = 0;
        let mut addr_pos = HashMap::new();
        // 0   1 2   3  4    5   6 7   8    9   10 11  12   13  14 15
        // 109 1 204 -1 1001 100 1 100 1008 100 16 101 1006 101 0  99
        //              ^----i---p-p--

        // line 1: addresses
        let max_addr = self.mem.keys().max().unwrap();
        let padding = 2;
        print!("\n");
        for addr in 0..=*max_addr {
            let addr_s = format!("{}", addr);
            let val_s = format!("{}", self.mem.get(&addr).unwrap_or(&0));
            let width = addr_s.len().max(val_s.len()) + padding;
            addr_pos.insert(addr, (x, width));
            x += width;
            print!("{:<1$}", addr_s, width);
        }
        print!("\n");

        // line 2: values
        for addr in 0..=*max_addr {
            let (_x, width) = addr_pos.get(&addr).unwrap_or(&(0, 1));
            print!("{:<1$}", self.mem.get(&addr).unwrap_or(&0), width);
        }
        print!("\n");

        // line 3: current instruction
        let pc = addr_pos.get(&self.pc).unwrap();
        let inst = self.read_instruction().unwrap();
        let inst_width = (0..inst.operation.params)
            .map(|i| addr_pos.get(&(self.pc + i as i64 + 1)).unwrap_or(&(0, 1)).1)
            .sum::<usize>();
        print!(
            "{}^{}\n{:?}\n",
            " ".repeat(pc.0),
            "-".repeat(pc.1 + inst_width - 1),
            inst,
        );
    }

    // helper for passing an input, retrieving an output
    // (output, done)
    pub fn step(&mut self, val: Word) -> Result<(Word, bool), String> {
        let r = self.stepn(vec![val], 1)?;
        // advance one more time and return
        match self.resume(None)? {
            RuntimeState::Complete => Ok((r[0], true)),
            RuntimeState::Resumable(None) => Ok((r[0], false)),
            RuntimeState::Resumable(Some(_)) => {
                Err("Unexpected output after reading all outputs".to_string())
            }
        }
    }

    // helper for passing any number of inputs, then getting n outputs
    pub fn stepn(&mut self, vals: Vec<Word>, n: usize) -> Result<Vec<Word>, String> {
        // provide all but one input
        if vals.len() > 0 {
            for v in &vals[0..vals.len() - 1] {
                match self.resume(Some(*v))? {
                    RuntimeState::Complete => {
                        return Err("Unexpected complete during inputs".to_string())
                    }
                    RuntimeState::Resumable(Some(_)) => {
                        return Err("Unexpected output during inputs".to_string())
                    }
                    RuntimeState::Resumable(None) => (),
                }
            }
        }

        // provide the last input, read n outputs
        let mut out = Vec::new();
        let mut val = if vals.len() > 0 {
            Some(vals[vals.len() - 1])
        } else {
            None
        };
        for i in 0..n {
            match self.resume(val)? {
                RuntimeState::Complete => {
                    return Err(format!("Unexpected complete after output {}", i));
                }
                RuntimeState::Resumable(None) => {
                    return Err(format!("Unexpected ask for input after output {}", i));
                }
                RuntimeState::Resumable(Some(x)) => {
                    val = None;
                    out.push(x);
                }
            }
        }

        Ok(out)
    }

    pub fn step_read(&mut self, n: usize) -> Result<Option<Vec<Word>>, String> {
        let mut out = vec![0; n];
        for i in 0..n {
            match self.resume(None)? {
                RuntimeState::Complete => return Ok(None),
                RuntimeState::Resumable(None) => {
                    return Err(format!("Unexpected ask for input at output {}", i))
                }
                RuntimeState::Resumable(Some(x)) => out[i] = x,
            }
        }
        Ok(Some(out))
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
    // Resumable(Some(_)) broken to output some value, nothing expected back
    // Resumable(None) broken to collect some input
    Resumable(Option<Word>),
    // halted
    Complete,
}
