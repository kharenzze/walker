use std::ops::Add;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Point {
  x: usize,
  y: usize,
}

const X: Point = Point::new(1, 0);
const Y: Point = Point::new(0, 1);

impl From<(usize, usize)> for Point {
  fn from(input: (usize, usize)) -> Self {
    Self {
      x: input.0,
      y: input.1,
    }
  }
}

impl Add<Point> for Point {
  type Output = Point;

  fn add(self, rhs: Point) -> Self::Output {
    Point::from((self.x + rhs.x, self.y + rhs.y))
  }
}

impl Point {
  pub const fn new(x: usize, y: usize) -> Self {
    Self { x, y }
  }

  pub fn is_contained_in(&self, reference: &Self) -> bool {
    self.x < reference.x && self.y < reference.y
  }

  pub fn get_points_arround(&self) -> Vec<Point> {
    todo!()
  }

  pub fn checked_sub(&self, rhs: &Point) -> Option<Point> {
    let x = self.x.checked_sub(rhs.x)?;
    let y = self.y.checked_sub(rhs.y)?;
    Some(Point::new(x, y))
  }
}

#[cfg(test)]
mod tests {
  use super::{Point, X, Y};

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

  #[test]
  fn add() {
    assert_eq!(X + Y, Point::new(1,1))
  }

  #[test]
  fn checked_sub() {
    assert_eq!(X.checked_sub(&Y), None);
    assert_eq!(X.checked_sub(&X), Some(Point::new(0,0)));
  }
}
