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
  let (max, _) = find_max(&prog, run_chain, vec![0, 1, 2, 3, 4]);
  println!("Part 1: {}", max); // 199988

  /*
  Most of the amplifiers are connected as they were before; amplifier A's output is connected to amplifier B's input, and so on. However, the output from amplifier E is now connected into amplifier A's input. This creates the feedback loop: the signal will be sent through the amplifiers many times.

  In feedback loop mode, the amplifiers need totally different phase settings: integers from 5 to 9, again each used exactly once. These settings will cause the Amplifier Controller Software to repeatedly take input and produce output many times before halting. Provide each amplifier its phase setting at its first input instruction; all further input/output instructions are for signals.

  Don't restart the Amplifier Controller Software on any amplifier during this process. Each one should continue receiving and sending signals until it halts.

  All signals sent or received in this process will be between pairs of amplifiers except the very first signal and the very last signal. To start the process, a 0 signal is sent to amplifier A's input exactly once.

  Eventually, the software on the amplifiers will halt after they have processed the final loop. When this happens, the last output signal from amplifier E is sent to the thrusters. Your job is to find the largest output signal that can be sent to the thrusters using the new phase settings and feedback loop arrangement.
  */
  let (max, _) = find_max(&prog, run_feedback, vec![5, 6, 7, 8, 9]);
  println!("Part 2: {}", max); // 17519904
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
    // feed input, retrieve output, advance to next input
    let (result, _done) = amp.step(input).unwrap();
    input = result;
  }
  input
}

fn run_feedback(prog: &Program, phases: &Vec<Word>) -> Word {
  // run each amp to consume the phase setting, then wait for input
  let mut amps: Vec<_> = phases
    .iter()
    .enumerate()
    .map(|(i, phase)| {
      let mut rt = prog.new_runtime();
      // rt.trace = true;
      rt.id = i.to_string();
      rt.resume(None).unwrap();
      rt.resume(Some(*phase)).unwrap();
      // assert_eq!(rt.state(), RuntimeState::Resumable(None));
      rt
    })
    .collect();

  let mut input = 0;
  loop {
    let mut complete = 0;
    for amp in &mut amps {
      let (result, done) = amp.step(input).unwrap();
      input = result;
      if done {
        complete += 1  
      }
    }
    if complete == phases.len() {
      break;
    }
  }
  input
}

fn find_max<F>(prog: &Program, circuit: F, phase_settings: Vec<Word>) -> (Word, Vec<Word>)
where
  F: Fn(&Program, &Vec<Word>) -> Word,
{
  let mut max = 0;
  let mut max_phases = Vec::new();
  for phases in permute(phase_settings) {
    let output = circuit(&prog, &phases);
    if output > max {
      max = output;
      max_phases = phases.clone();
    }
  }
  (max, max_phases)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample1() {
    let prog = Program::parse("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0").unwrap();
    assert_eq!(43210, run_chain(&prog, &vec![4, 3, 2, 1, 0]));
    assert_eq!(
      (43210, vec![4, 3, 2, 1, 0]),
      find_max(&prog, run_chain, vec![0, 1, 2, 3, 4])
    );
  }
  #[test]
  fn sample2() {
    let prog =
      Program::parse("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0")
        .unwrap();
    assert_eq!(54321, run_chain(&prog, &vec![0, 1, 2, 3, 4]));
    assert_eq!(
      (54321, vec![0, 1, 2, 3, 4]),
      find_max(&prog, run_chain, vec![0, 1, 2, 3, 4])
    );
  }
  #[test]
  fn sample3() {
    let prog = Program::parse("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0").unwrap();
    assert_eq!(65210, run_chain(&prog, &vec![1, 0, 4, 3, 2]));
    assert_eq!(
      (65210, vec![1, 0, 4, 3, 2]),
      find_max(&prog, run_chain, vec![0, 1, 2, 3, 4])
    );
  }
  #[test]
  fn sample4() {
    let prog = Program::parse(
      "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
    )
    .unwrap();
    assert_eq!(139629729, run_feedback(&prog, &vec![9, 8, 7, 6, 5]));
    assert_eq!(
      (139629729, vec![9, 8, 7, 6, 5]),
      find_max(&prog, run_feedback, vec![5, 6, 7, 8, 9])
    );
  }
  #[test]
  fn sample5() {
    let prog = Program::parse("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10").unwrap();
    assert_eq!(18216, run_feedback(&prog, &vec![9, 8, 7, 6, 5]));
    assert_eq!(
      (18216, vec![9, 8, 7, 6, 5]),
      find_max(&prog, run_feedback, vec![5, 6, 7, 8, 9])
    );
  }
}
