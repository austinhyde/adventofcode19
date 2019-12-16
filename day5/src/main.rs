use intcode;
use intcode::io::{IteratorInput, StdoutOutput};

fn main() {
  let input = include_str!("input.txt");
  let prog = intcode::Program::parse(input).unwrap();

  /*
  The TEST diagnostic program will start by requesting from the user the ID of the system to test by running an input instruction - provide it 1, the ID for the ship's air conditioner unit.

  It will then perform a series of diagnostic tests confirming that various parts of the Intcode computer, like parameter modes, function correctly. For each test, it will run an output instruction indicating how far the result of the test was from the expected value, where 0 means the test was successful. Non-zero outputs mean that a function is not working correctly; check the instructions that were run before the output instruction to see which one failed.

  Finally, the program will output a diagnostic code and immediately halt. This final output isn't an error; an output followed immediately by a halt means the program finished. If all outputs were zero except the diagnostic code, the diagnostic program ran successfully.

  After providing 1 to the only input instruction and passing all the tests, what diagnostic code does the program produce?
  */
  println!("Part 1:");
  let mut prog_input = IteratorInput::new(vec![1]);
  let mut prog_output = StdoutOutput::new();
  prog.run_io(&mut prog_input, &mut prog_output).unwrap();

  /*
  This time, when the TEST diagnostic program runs its input instruction to get the ID of the system to test, provide it 5, the ID for the ship's thermal radiator controller. This diagnostic test suite only outputs one number, the diagnostic code.

  What is the diagnostic code for system ID 5?
  */
  println!("Part 2:");
  prog_input = IteratorInput::new(vec![5]);
  prog.run_io(&mut prog_input, &mut prog_output).unwrap();
}
