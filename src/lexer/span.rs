#[derive(Debug, PartialEq)]
pub struct Span {
  start: usize,
  end: usize,
}

impl Span {
  pub fn new(start: usize, end: usize) -> Span {
    Span { start, end }
  }

  pub fn get_bounds(&self) -> (usize, usize) {
    (self.start, self.end)
  }
}
