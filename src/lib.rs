use std::fs::File;
use std::io::{BufReader, BufRead};

type DynResult<T> = Result<T, Box<dyn std::error::Error>>;
#[derive(Debug, Default, Clone, Copy)]
struct Point {
  x: usize,
  y: usize,
}

#[derive(Debug, Default)]
struct GameMap {
  data: Vec<Vec<char>>,
  dimensions: Point
}

impl GameMap {
  pub fn read_from_path(path: &str) -> DynResult<Self> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let mut line_iter = reader.lines();
    let gm = GameMap::default();
    let row: usize = 0;
    while let Some(Ok(l)) = line_iter.next() {
      let v : Vec<char> = Vec::with_capacity(l.len());
      let v:  Vec<char> = l.chars().collect();
      for c in l.chars() {
        gm.data
      }
    }
    Ok(gm)
  }
}

#[cfg(test)]
mod tests {
    use crate::GameMap;

  fn read_simple_map() {
    let gm = GameMap::read_from_path("./resources/map_simple.map").expect("map should be read properly");

  }
}