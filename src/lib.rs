mod point;

use std::fs::File;
use std::io::{BufRead, BufReader};
use point::Point;

type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Default)]
struct GameMap {
  data: Vec<Vec<char>>,
  dimensions: Point,
}

impl GameMap {
  pub fn read_from_path(path: &str) -> DynResult<Self> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let mut line_iter = reader.lines();
    let mut gm = GameMap::default();
    while let Some(Ok(l)) = line_iter.next() {
      let v: Vec<char> = l.chars().collect();
      gm.data.push(v);
    }
    gm.dimensions = Point::from((gm.data.len(), gm.data[0].len()));
    Ok(gm)
  }
}

#[cfg(test)]
mod tests {
  use crate::{GameMap, Point};

  #[test]
  fn read_simple_map() {
    let gm =
      GameMap::read_from_path("./resources/map_simple.map").expect("map should be read properly");
    assert_eq!(gm.dimensions, Point::new(4, 4));
  }
}
