const OP_ADD: u32 = 1;
const OP_MUL: u32 = 2;
const OP_HLT: u32 = 99;

fn main() {
  /*
  An Intcode program is a list of integers separated by commas (like 1,0,0,3,99). To run one, start by looking at the first integer (called position 0). Here, you will find an opcode - either 1, 2, or 99. The opcode indicates what to do; for example, 99 means that the program is finished and should immediately halt. Encountering an unknown opcode means something went wrong.

  Opcode 1 adds together numbers read from two positions and stores the result in a third position. The three integers immediately after the opcode tell you these three positions - the first two indicate the positions from which you should read the input values, and the third indicates the position at which the output should be stored.

  Opcode 2 works exactly like opcode 1, except it multiplies the two inputs instead of adding them. Again, the three integers after the opcode indicate where the inputs and outputs are, not their values.

  Once you have a working computer, the first step is to restore the gravity assist program (your puzzle input) to the "1202 program alarm" state it had just before the last computer caught fire. To do this, before running the program, replace position 1 with the value 12 and replace position 2 with the value 2. What value is left at position 0 after the program halts?
  */
  let input = include_str!("input.txt");
  let opcodes: &mut Vec<u32> = &mut parse(input);

  // before running the program, replace position 1 with the value 12 and replace position 2 with the value 2
  let res = calculate(&opcodes, 12, 2);
  println!("Part 1: {:?}", res); // 4576384

  /*
  To complete the gravity assist, you need to determine what pair of inputs produces the output 19690720."

  Find the input noun and verb that cause the program to produce the output 19690720. What is 100 * noun + verb? (For example, if noun=12 and verb=2, the answer would be 1202.)
  */
  
  let nv = find(&opcodes, 19690720);

  match nv {
    Ok((noun, verb)) => println!(
      "Part 2: noun={} verb={} answer={}",
      noun,
      verb,
      100 * noun + verb
    ),
    Err(s) => println!("Part 2: err={}", s),
  }
}

fn find(prog: &Vec<u32>, target: u32) -> Result<(u32, u32), String> {
  for noun in 0..=99 {
    for verb in 0..=99 {
      if let Ok(res) = calculate(&prog, noun, verb) {
        if res == target {
          return Ok((noun, verb));
        }
      }
    }
  }
  Err("No inputs found".into())
}

fn parse(input: &str) -> Vec<u32> {
  input
    .split(",")
    .into_iter()
    .filter_map(|s| s.parse().ok())
    .collect()
}

// In this program, the value placed in address 1 is called the noun, and the value placed in address 2 is called the verb. Each of the two input values will be between 0 and 99, inclusive.
fn calculate(opcodes: &Vec<u32>, noun: u32, verb: u32) -> Result<u32, String> {
  let mut tape = opcodes.clone();

  tape[1] = noun;
  tape[2] = verb;

  let mut i = 0;
  loop {
    match tape[i] {
      OP_ADD => {
        bin_assign(&mut tape, &i, std::ops::Add::add);
        i += 4;
      }
      OP_MUL => {
        bin_assign(&mut tape, &i, std::ops::Mul::mul);
        i += 4;
      }
      OP_HLT => return Ok(tape[0]),
      op => return Err(format!("Unknown opcode {}", op)),
    }
  }
}

fn bin_assign<F>(tape: &mut Vec<u32>, i: &usize, f: F)
where
  F: Fn(u32, u32) -> u32,
{
  let i_lhs = tape[i + 1] as usize;
  let i_rhs = tape[i + 2] as usize;
  let i_res = tape[i + 3] as usize;
  tape[i_res] = f(tape[i_lhs], tape[i_rhs]);
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
    for (prog, noun, verb, expected) in cases {
      let actual = calculate(&parse(prog), noun, verb);
      println!(
        "{} ({}, {}) => expected {}, actual {:?}",
        prog, noun, verb, expected, actual
      );
      assert_eq!(Ok(expected), actual);
    }
  }
}
