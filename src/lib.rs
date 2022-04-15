mod point;

use point::Point;
use std::convert::TryFrom;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use thiserror::Error;

type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

pub struct AppConfig {
  path: String,
}

impl AppConfig {
  pub fn new(path: String) -> Self {
    Self { path }
  }
}

pub struct App;

impl App {
  pub fn run(config: AppConfig) {}
}

#[derive(Debug, Default)]
struct GameMap {
  data: Vec<Vec<char>>,
  dimensions: Point,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellType {
  Wall,
  Floor,
}

#[derive(Debug, Error, PartialEq, Eq)]
enum ConversionError {
  #[error("Could not convert char '{0}' into CellType")]
  CellType(char),
}

impl TryFrom<char> for CellType {
  type Error = ConversionError;

  fn try_from(value: char) -> Result<Self, Self::Error> {
    match value {
      '#' => Ok(CellType::Wall),
      '.' => Ok(CellType::Floor),
      _ => Err(ConversionError::CellType(value)),
    }
  }
}

impl From<CellType> for char {
  fn from(value: CellType) -> Self {
    match value {
      CellType::Wall => '#',
      CellType::Floor => '.',
    }
  }
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

impl Display for GameMap {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use crate::{CellType, GameMap, Point, ConversionError};

  #[test]
  fn read_simple_map() {
    let gm =
      GameMap::read_from_path("./resources/map_simple.map").expect("map should be read properly");
    assert_eq!(gm.dimensions, Point::new(4, 4));
  }

  #[test]
  fn cell_type_conversion() {
    let c: char = CellType::Floor.into();
    assert_eq!(c, '.');
    let c: char = CellType::Wall.into();
    assert_eq!(c, '#');

    let cell = CellType::try_from('.');
    assert_eq!(Ok(CellType::Floor), cell);
    let cell = CellType::try_from('K');
    assert_eq!(Err(ConversionError::CellType('K')), cell);
  }
}
