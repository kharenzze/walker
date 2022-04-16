mod point;

use point::Point;
use std::convert::TryFrom;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use thiserror::Error;

pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

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
  pub fn run(config: AppConfig) -> DynResult<()> {
    let gm = GameMap::read_from_path(&config.path)?;
    Ok(())
  }
}

#[derive(Debug, Default)]
struct GameMap {
  data: Vec<Vec<CellType>>,
  dimensions: Point,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellType {
  Wall,
  Floor,
  Target,
  Origin
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
      'x' => Ok(CellType::Target),
      'o' => Ok(CellType::Origin),
      _ => Err(ConversionError::CellType(value)),
    }
  }
}

impl From<CellType> for char {
  fn from(value: CellType) -> Self {
    match value {
      CellType::Wall => '#',
      CellType::Floor => '.',
      CellType::Target => 'x',
      CellType::Origin => 'o',
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
      let v: Result<Vec<CellType>, _> = l.chars().map(|c| CellType::try_from(c)).collect();
      gm.data.push(v?);
    }
    gm.dimensions = Point::from((gm.data.len(), gm.data[0].len()));
    Ok(gm)
  }

  fn get_point(&self, p: Point) -> Option<&CellType> {
    if !self.dimensions.contains(&p) {
      return None;
    }
    self.data.get(p.x).unwrap().get(p.y)
  }
}

impl Display for GameMap {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for v in self.data.iter() {
      let text: String = v.iter().map(|c| char::from(*c)).collect();
      writeln!(f, "{}", &text)?;
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use indoc::indoc;

  use crate::{CellType, ConversionError, GameMap, Point};

  fn get_simple_gm() -> GameMap {
    GameMap::read_from_path("./resources/map_simple.map").expect("map should be read properly")
  }

  #[test]
  fn read_simple_map() {
    let gm = get_simple_gm();
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

  #[test]
  fn display() {
    let gm = get_simple_gm();
    let text = format!("{}", gm);
    let expected_text = indoc! { r#"
    ####
    #..#
    #..#
    ####
    "#
    };
    assert_eq!(&text, expected_text);
  }

  #[test]
  fn get_point() {
    let gm = get_simple_gm();
    let p = Point::new(0, 0);
    let cell =  gm.get_point(p);
    assert_eq!(cell, Some(&CellType::Wall));

    let p = Point::new(100, 0);
    let cell =  gm.get_point(p);
    assert_eq!(cell, None);
  }
}
