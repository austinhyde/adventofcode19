use super::*;

#[test]
fn day2_examples() {
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
fn negative() {
    Program::parse("1,-1,2,0,99").unwrap();
}

#[test]
fn day5_example1() {
    let prog = Program::parse("3,0,4,0,99").unwrap();
    let mut out = Vec::new();
    prog.run_io(
        &mut io::IteratorInput::new(vec![42]),
        &mut io::VecOutput::new(&mut out),
    )
    .unwrap();
    assert_eq!(vec![42], out);
}

#[test]
fn day5_example2() {
    let mut rt = Program::parse("1002,4,3,4,33").unwrap().new_runtime();
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

#[test]
fn day9_example0() {
    let mut rt = Runtime::new(vec![]);

    // if the relative base is 2000, then after the instruction 109,19, the relative base would be 2019.
    rt.relative_base = 2000;
    ops::Instruction {
        operation: &ops::OP_RBO,
        params: vec![ops::Param::Immediate(19)],
    }
    .execute(&mut rt)
    .unwrap();
    assert_eq!(rt.relative_base, 2019);

    // If the next instruction were 204,-34, then the value at address 1985 would be output
    rt.set(1985, 42).unwrap();
    ops::Instruction {
        operation: &ops::OP_OUT,
        params: vec![ops::Param::Relative(-34)],
    }
    .execute(&mut rt)
    .unwrap();
    assert_eq!(rt.state, Some(RuntimeState::Resumable(Some(42))));
}

#[test]
fn day9_example1() {
    // takes no input and produces a copy of itself as output.
    let v: Vec<i64> = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"
        .split(",")
        .filter_map(|s| s.parse().ok())
        .collect();
    let out = Program::new(v.clone()).run_collect_output(vec![]).unwrap();
    assert_eq!(v, out);
}

#[test]
fn day9_example2() {
    // should output a 16-digit number
    let out = Program::parse("1102,34915192,34915192,7,4,7,99,0")
        .unwrap()
        .run_collect_output(vec![])
        .unwrap();
    assert_eq!(16f64, (out[0] as f64).log10().ceil());
}

#[test]
fn day9_example3() {
    // should output the large number in the middle
    let out = Program::parse("104,1125899906842624,99")
        .unwrap()
        .run_collect_output(vec![])
        .unwrap();
    assert_eq!(1125899906842624, out[0]);
}
