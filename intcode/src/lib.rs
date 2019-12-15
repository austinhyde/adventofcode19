mod ops;

pub type Word = u32;

pub struct Program {
    memory: Vec<Word>,
}

impl Program {
    pub fn new(mem: Vec<Word>) -> Self {
        Program {
            memory: mem.clone(),
        }
    }
    pub fn parse(input: &str) -> Self {
        Self::new(
            input
                .split(",")
                .into_iter()
                .filter_map(|s| s.parse().ok())
                .collect(),
        )
    }

    pub fn run(&self, noun: Word, verb: Word) -> Result<Word, String> {
        let mut tape = self.memory.clone();
        tape[1] = noun;
        tape[2] = verb;

        let mut pc = 0;
        loop {
            let inst = tape[pc];
            match inst {
                ops::OP_ADD => {
                    ops::bin_assign(&mut tape, &pc, std::ops::Add::add);
                    pc += 4;
                }
                ops::OP_MUL => {
                    ops::bin_assign(&mut tape, &pc, std::ops::Mul::mul);
                    pc += 4;
                }
                ops::OP_HLT => return Ok(tape[0]),
                op => return Err(format!("Unknown opcode {}", op)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let cases = vec![
            ("1,9,10,3,2,3,11,0,99,30,40,50", 9, 10, 3500),
            ("1,0,0,0,99", 0, 0, 2),
            ("2,4,4,0,99", 4, 4, 9801),
            ("1,1,1,4,99,5,6,0,99", 1, 1, 30),
        ];
        for (inp, noun, verb, expected) in cases {
            let actual = Program::parse(inp).run(noun, verb);
            println!(
                "{} ({}, {}) => expected {}, actual {:?}",
                inp, noun, verb, expected, actual
            );
            assert_eq!(Ok(expected), actual);
        }
    }
}
