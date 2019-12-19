use intcode::{Program, Word};
use permute::permute;

fn main() {
  let input = include_str!("input.txt");
  let prog = Program::parse(input).unwrap();

  /*
  When a copy of the program starts running on an amplifier, it will first use an input instruction to ask the amplifier for its current phase setting (an integer from 0 to 4). Each phase setting is used exactly once, but the Elves can't remember which amplifier needs which phase setting.

  The program will then call another input instruction to get the amplifier's input signal, compute the correct output signal, and supply it back to the amplifier with an output instruction. (If the amplifier has not yet received an input signal, it waits until one arrives.)

  Try every combination of phase settings on the amplifiers. What is the highest signal that can be sent to the thrusters?
  */
  let (max, _) = find_max(&prog);
  println!("Part 1: {}", max); // 199988
}

fn find_max(prog: &Program) -> (Word, Vec<Word>) {
  let mut max = 0;
  let mut max_phases = Vec::new();
  for phases in permute(vec![0, 1, 2, 3, 4]) {
    let output = run_chain(&prog, &phases);
    if output > max {
      max = output;
      max_phases = phases.clone();
    }
  }
  (max, max_phases)
}

fn run_chain(prog: &Program, phases: &Vec<Word>) -> Word {
  let amps: Vec<_> = phases
    .iter()
    .map(|phase| {
      let mut amp = prog.new_runtime();
      amp.resume(None).unwrap();
      amp.resume(Some(*phase)).unwrap();
      amp
    })
    .collect();

  let mut input = 0;
  for mut amp in amps {
    input = amp.step(input).unwrap().unwrap();
  }
  input
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample1() {
    let prog = Program::parse("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0").unwrap();
    assert_eq!(43210, run_chain(&prog, &vec![4, 3, 2, 1, 0]));
    assert_eq!((43210, vec![4, 3, 2, 1, 0]), find_max(&prog));
  }
  #[test]
  fn sample2() {
    let prog =
      Program::parse("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0")
        .unwrap();
    assert_eq!(54321, run_chain(&prog, &vec![0, 1, 2, 3, 4]));
    assert_eq!((54321, vec![0, 1, 2, 3, 4]), find_max(&prog));
  }
  #[test]
  fn sample3() {
    let prog = Program::parse("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0").unwrap();
    assert_eq!(65210, run_chain(&prog, &vec![1, 0, 4, 3, 2]));
    assert_eq!((65210, vec![1, 0, 4, 3, 2]), find_max(&prog));
  }
}
