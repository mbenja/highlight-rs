#[derive(Debug, PartialEq)]
pub struct Span {
  start: usize,
  end: usize,
}

impl Span {
  pub fn new(start: usize, end: usize) -> Span {
    Span { start, end }
  }
}
