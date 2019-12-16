use super::Word;

pub trait Input {
  fn read(&mut self) -> Result<Word, String>;
}
pub trait Output {
  fn write(&mut self, val: Word) -> Result<(), String>;
}

pub struct NotImplemented;
impl Input for NotImplemented {
  fn read(&mut self) -> Result<Word, String> {
    Err("Not implemented".to_string())
  }
}
impl Output for NotImplemented {
  fn write(&mut self, _: Word) -> Result<(), String> {
    Err("Not implemented".to_string())
  }
}

pub struct IteratorInput<I, W>
where
  I: Iterator<Item = W>,
  W: Into<Word>,
{
  iter: I,
}

impl<I, W> IteratorInput<I, W>
where
  I: Iterator<Item = W>,
  W: Into<Word>,
{
  pub fn new<II>(i: II) -> Self
  where
    II: IntoIterator<IntoIter = I, Item = W>,
  {
    Self {
      iter: i.into_iter(),
    }
  }
}

impl<I, W> Input for IteratorInput<I, W>
where
  I: Iterator<Item = W>,
  W: Into<Word>,
{
  fn read(&mut self) -> Result<Word, String> {
    match self.iter.next() {
      Some(x) => Ok(x.into()),
      None => Err("No more input".to_string()),
    }
  }
}

pub struct StdoutOutput;
impl StdoutOutput {
  pub fn new() -> Self {
    StdoutOutput {}
  }
}
impl Output for StdoutOutput {
  fn write(&mut self, val: Word) -> Result<(), String> {
    println!("{}", val);
    Ok(())
  }
}

pub struct VecOutput<'a> {
  vec: &'a mut Vec<Word>,
}
impl<'a> VecOutput<'a> {
  pub fn new(vec: &'a mut Vec<Word>) -> Self {
    Self { vec }
  }
}
impl Output for VecOutput<'_> {
  fn write(&mut self, val: Word) -> Result<(), String> {
    self.vec.push(val);
    Ok(())
  }
}
