mod point;

use point::Point;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Deref, DerefMut};
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
  target: Point,
  origin: Point,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellType {
  Wall,
  Floor,
  Target,
  Origin,
}

impl CellType {
  #[inline]
  fn is_target(&self) -> bool {
    self == &CellType::Target
  }

  #[inline]
  fn can_traverse(&self) -> bool {
    self == &CellType::Floor || self.is_target()
  }
}

#[derive(Debug, Error, PartialEq, Eq)]
enum ConversionError {
  #[error("Could not convert char '{0}' into CellType")]
  CellType(char),
}

#[derive(Debug, Error, PartialEq, Eq)]
enum GameMapError {
  #[error("Found more than one target")]
  MoreThanOneTarget,
  #[error("Missing target")]
  MissingTarget,
  #[error("Found more than one origin")]
  MoreThanOneOrigin,
  #[error("Missing origin")]
  MissingOrigin,
  #[error("Could not find a path")]
  CouldNotFindPath,
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
    let mut target: Option<Point> = None;
    let mut origin: Option<Point> = None;
    let mut row = 0_usize;
    while let Some(Ok(l)) = line_iter.next() {
      let v: Result<Vec<CellType>, _> = l.chars().map(|c| CellType::try_from(c)).collect();
      let v = v?;
      for (col, cell) in v.iter().enumerate() {
        match cell {
          &CellType::Target => {
            if target.is_none() {
              target = Some(Point::new(row, col));
            } else {
              Err(GameMapError::MoreThanOneTarget)?;
            }
          }
          &CellType::Origin => {
            if origin.is_none() {
              origin = Some(Point::new(row, col));
            } else {
              Err(GameMapError::MoreThanOneOrigin)?;
            }
          }
          _ => (),
        }
      }
      gm.data.push(v);
      row += 1;
    }
    gm.dimensions = Point::from((gm.data.len(), gm.data[0].len()));
    if target.is_none() {
      Err(GameMapError::MissingTarget)?;
    }
    if origin.is_none() {
      Err(GameMapError::MissingOrigin)?;
    }
    gm.target = target.unwrap();
    gm.origin = origin.unwrap();
    Ok(gm)
  }

  fn get_point(&self, p: Point) -> Option<&CellType> {
    if !self.dimensions.contains(&p) {
      return None;
    }
    self.data.get(p.x).unwrap().get(p.y)
  }

  #[inline]
  fn distance_to_target(&self, p: Point) -> usize {
    self.target.squared_distance(p)
  }

  pub fn solve(&self) -> Result<Path, GameMapError> {
    let mut cache: CostCache = Default::default();
    let mut opened: Vec<Point> = vec![self.origin];
    let origin_cost = CostMetric {
      parent: self.origin,
      to_origin: 0,
      heuristic: self.distance_to_target(self.origin),
      opened: true,
    };
    cache.insert(self.origin, origin_cost);
    let mut target_cost: Option<CostMetric> = None;

    while let Some(p) = opened.pop() {
      let next_points: Vec<Point> = p
        .get_points_around()
        .iter()
        .filter_map(|p| *p)
        .filter(|p| self.dimensions.contains(p))
        .filter(|p| self.get_point(*p).unwrap().can_traverse())
        .collect();
      let current_cost = cache.get_mut(&p).unwrap();
      let next_dist = current_cost.to_origin + 1;
      current_cost.opened = false;
      for next_p in next_points {
        let cell = self.get_point(next_p).unwrap();
        if cell.is_target() {
          let computed = CostMetric {
            opened: false,
            parent: p,
            to_origin: next_dist,
            heuristic: 0,
          };
          target_cost = Some(computed);
          cache.insert(next_p, computed);
          break;
        }
        let cached_cost = cache.get_mut(&next_p);
        if cached_cost.is_none() {
          let computed = CostMetric {
            opened: true,
            parent: p,
            to_origin: next_dist,
            heuristic: self.distance_to_target(next_p),
          };
          cache.insert(next_p, computed);
          opened.push(next_p);
        } else {
          let cost = cached_cost.unwrap();
          if cost.opened && cost.to_origin < next_dist {
            cost.to_origin = next_dist;
            cost.parent = next_p;
          }
        }
      }
      if target_cost.is_some() {
        break;
      }
    }
    if target_cost.is_none() {
      return Err(GameMapError::CouldNotFindPath);
    }
    cache
      .get_reverse_path(self.target)
      .ok_or(GameMapError::CouldNotFindPath)
  }
}

#[derive(Debug, Default)]
struct CostCache(HashMap<Point, CostMetric>);

impl Deref for CostCache {
  type Target = HashMap<Point, CostMetric>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for CostCache {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

type PathInner = Vec<Point>;

struct Path {
  inner: PathInner,
}

impl Deref for Path {
  type Target = PathInner;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl DerefMut for Path {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.inner
  }
}

impl Path {
  fn new(vec: PathInner) -> Self {
    Self { inner: vec }
  }

  fn to_point_set(&self) -> HashSet<Point> {
    HashSet::from_iter(self.inner.iter().map(|p| *p))
  }
}

impl Display for Path {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    for p in self.inner.iter() {
      writeln!(f, "{}", p)?;
    }
    Ok(())
  }
}

impl CostCache {
  fn get_reverse_path(&self, end: Point) -> Option<Path> {
    let mut path: Path = Path::new(vec![end]);
    let mut p = end;
    loop {
      let cost = self.get(&p)?;
      if cost.parent == p {
        break;
      }
      p = cost.parent;
      path.push(p);
    }
    Some(path)
  }
}

#[derive(Debug, Default, Clone, Copy)]
struct CostMetric {
  to_origin: usize,
  parent: Point,
  heuristic: usize,
  opened: bool,
}

impl CostMetric {
  #[inline]
  fn cost(&self) -> usize {
    self.heuristic + self.to_origin
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

  use crate::{CellType, ConversionError, DynResult, GameMap, Point};

  fn get_simple_gm() -> DynResult<GameMap> {
    GameMap::read_from_path("./resources/map_simple.map")
  }

  fn get_medium_gm() -> DynResult<GameMap> {
    GameMap::read_from_path("./resources/medium.map")
  }

  fn get_double_origin_gm() -> DynResult<GameMap> {
    GameMap::read_from_path("./resources/double_origin.map")
  }

  fn get_missing_target_gm() -> DynResult<GameMap> {
    GameMap::read_from_path("./resources/missing_target.map")
  }

  #[test]
  fn read_simple_map() {
    let gm = get_simple_gm();
    assert!(gm.is_ok());
    let gm = gm.unwrap();
    assert_eq!(gm.dimensions, Point::new(4, 4));
    assert_eq!(gm.origin, Point::new(2, 1));
    assert_eq!(gm.target, Point::new(1, 2));

    let gm = get_double_origin_gm();
    assert!(gm.is_err());

    let gm = get_missing_target_gm();
    assert!(gm.is_err());
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
    let gm = get_simple_gm().unwrap();
    let text = format!("{}", gm);
    let expected_text = indoc! { r#"
    ####
    #.x#
    #o.#
    ####
    "#
    };
    assert_eq!(&text, expected_text);
  }

  #[test]
  fn get_point() {
    let gm = get_simple_gm().unwrap();
    let p = Point::new(0, 0);
    let cell = gm.get_point(p);
    assert_eq!(cell, Some(&CellType::Wall));

    let p = Point::new(100, 0);
    let cell = gm.get_point(p);
    assert_eq!(cell, None);
  }

  #[test]
  fn solve() {
    let gm = get_simple_gm().unwrap();
    let path = gm.solve().unwrap();
    assert_eq!(path.len(), 3);
    let gm = get_medium_gm().unwrap();
    let path = gm.solve().unwrap();
    println!("{}", &path)
  }
}
