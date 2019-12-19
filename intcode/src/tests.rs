use super::*;

#[test]
fn test_day2_examples() {
  let cases = vec![
    ("1,9,10,3,2,3,11,0,99,30,40,50", 9, 10, 3500),
    ("1,0,0,0,99", 0, 0, 2),
    ("2,4,4,0,99", 4, 4, 9801),
    ("1,1,1,4,99,5,6,0,99", 1, 1, 30),
  ];
  for (inp, noun, verb, expected) in cases {
    let actual = Program::parse(inp).unwrap().run(noun, verb);
    println!(
      "{} ({}, {}) => expected {}, actual {:?}",
      inp, noun, verb, expected, actual
    );
    assert_eq!(Ok(expected), actual);
  }
}

#[test]
fn test_negative() {
  Program::parse("1,-1,2,0,99").unwrap();
}

#[test]
fn test_day5_example1() {
  let prog = Program::parse("3,0,4,0,99").unwrap();
  let mut out = Vec::new();
  prog
    .run_io(
      &mut io::IteratorInput::new(vec![42]),
      &mut io::VecOutput::new(&mut out),
    )
    .unwrap();
  assert_eq!(vec![42], out);
}

#[test]
fn test_day5_example2() {
  let mut rt = Program::parse("1002,4,3,4,33")
    .unwrap()
    .new_runtime();
  rt.trace = true;

  // test that we parse correctly
  let inst = rt.read_instruction().unwrap();
  let expected = ops::Instruction {
    operation: &ops::OP_MUL,
    params: vec![
      ops::Param::Position(4),
      ops::Param::Immediate(3),
      ops::Param::Position(4),
    ],
  };
  assert_eq!(expected, inst);

  // test that we execute correctly
  inst.execute(&mut rt).unwrap();
  assert_eq!(99, rt.get(4).unwrap());
}
