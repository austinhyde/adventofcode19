use intcode::*;

fn main() {
  /*
  An Intcode program is a list of integers separated by commas (like 1,0,0,3,99). To run one, start by looking at the first integer (called position 0). Here, you will find an opcode - either 1, 2, or 99. The opcode indicates what to do; for example, 99 means that the program is finished and should immediately halt. Encountering an unknown opcode means something went wrong.

  Opcode 1 adds together numbers read from two positions and stores the result in a third position. The three integers immediately after the opcode tell you these three positions - the first two indicate the positions from which you should read the input values, and the third indicates the position at which the output should be stored.

  Opcode 2 works exactly like opcode 1, except it multiplies the two inputs instead of adding them. Again, the three integers after the opcode indicate where the inputs and outputs are, not their values.

  Once you have a working computer, the first step is to restore the gravity assist program (your puzzle input) to the "1202 program alarm" state it had just before the last computer caught fire. To do this, before running the program, replace position 1 with the value 12 and replace position 2 with the value 2. What value is left at position 0 after the program halts?
  */
  let input = include_str!("input.txt");
  let prog = Program::parse(input).unwrap();

  // before running the program, replace position 1 with the value 12 and replace position 2 with the value 2
  let res = prog.run(12, 2);
  println!("Part 1: {:?}", res); // 4576384

  /*
  To complete the gravity assist, you need to determine what pair of inputs produces the output 19690720."

  Find the input noun and verb that cause the program to produce the output 19690720. What is 100 * noun + verb? (For example, if noun=12 and verb=2, the answer would be 1202.)
  */
  
  let nv = find(&prog, 19690720);

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

fn find(prog: &Program, target: Word) -> Result<(Word, Word), String> {
  for noun in 0..=99 {
    for verb in 0..=99 {
      if let Ok(res) = prog.run(noun, verb) {
        if res == target {
          return Ok((noun, verb));
        }
      }
    }
  }
  Err("No inputs found".into())
}
