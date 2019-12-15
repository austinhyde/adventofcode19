use super::Word;

pub const OP_ADD: Word = 1;
pub const OP_MUL: Word = 2;
pub const OP_HLT: Word = 99;

pub fn bin_assign<F>(tape: &mut Vec<Word>, i: &usize, f: F)
where
  F: Fn(Word, Word) -> Word,
{
  let i_lhs = tape[i + 1] as usize;
  let i_rhs = tape[i + 2] as usize;
  let i_res = tape[i + 3] as usize;
  tape[i_res] = f(tape[i_lhs], tape[i_rhs]);
}
