#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Point {
  x: usize,
  y: usize,
}

impl From<(usize, usize)> for Point {
  fn from(input: (usize, usize)) -> Self {
    Self {
      x: input.0,
      y: input.1,
    }
  }
}

impl Point {
  pub fn new(x: usize, y: usize) -> Self {
    Self { x, y }
  }

  pub fn is_contained_in(&self, reference: &Self) -> bool {
    self.x < reference.x && self.y < reference.y
  }
}

#[cfg(test)]
mod tests {
  use super::Point;

  #[test]
  fn is_contained() {
    let ref_ = Point::new(4, 4);
    let a = Point::new(2, 2);
    assert!(a.is_contained_in(&ref_));

    let a = Point::new(4, 2);
    assert!(!a.is_contained_in(&ref_));

    let a = Point::new(2, 4);
    assert!(!a.is_contained_in(&ref_));
  }
}
