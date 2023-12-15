pub use crate::direction::Direction;

use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridCoordinate {
    pub x: usize,
    pub y: usize,
}

impl GridCoordinate {
    pub fn new(x: usize, y: usize) -> GridCoordinate {
        return GridCoordinate { x: x, y: y };
    }
}

impl Display for GridCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "({}, {})", self.x, self.y);
    }
}

impl std::ops::Add for GridCoordinate {
    type Output = GridCoordinate;

    fn add(self, other: GridCoordinate) -> GridCoordinate {
        return GridCoordinate {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Ord for GridCoordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        return other.x.cmp(&self.x).then_with(|| other.y.cmp(&self.y));
    }
}

impl PartialOrd for GridCoordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridCoordinateInf {
    pub x: i32,
    pub y: i32,
}

impl GridCoordinateInf {
    pub fn new(x: i32, y: i32) -> GridCoordinateInf {
        return GridCoordinateInf { x: x, y: y };
    }

    pub fn move_dir(&self, direction: Direction) -> GridCoordinateInf {
        let north_move = GridCoordinateInf::new(0, -1);
        let south_move = GridCoordinateInf::new(0, 1);
        let west_move = GridCoordinateInf::new(-1, 0);
        let east_move = GridCoordinateInf::new(1, 0);

        return *self
            + match direction {
                Direction::NORTH => north_move,
                Direction::EAST => east_move,
                Direction::SOUTH => south_move,
                Direction::WEST => west_move,
                Direction::NORTHEAST => north_move + east_move,
                Direction::NORTHWEST => north_move + west_move,
                Direction::SOUTHEAST => south_move + east_move,
                Direction::SOUTHWEST => south_move + west_move,
            };
    }
}

impl Display for GridCoordinateInf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "({}, {})", self.x, self.y);
    }
}

impl std::ops::Add for GridCoordinateInf {
    type Output = GridCoordinateInf;

    fn add(self, other: GridCoordinateInf) -> GridCoordinateInf {
        return GridCoordinateInf {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridCoordinateInf64 {
    pub x: i64,
    pub y: i64,
}

impl GridCoordinateInf64 {
    pub fn new(x: i64, y: i64) -> GridCoordinateInf64 {
        return GridCoordinateInf64 { x: x, y: y };
    }

    pub fn move_dir(&self, direction: Direction) -> GridCoordinateInf64 {
        let north_move = GridCoordinateInf64::new(0, -1);
        let south_move = GridCoordinateInf64::new(0, 1);
        let west_move = GridCoordinateInf64::new(-1, 0);
        let east_move = GridCoordinateInf64::new(1, 0);

        return *self
            + match direction {
                Direction::NORTH => north_move,
                Direction::EAST => east_move,
                Direction::SOUTH => south_move,
                Direction::WEST => west_move,
                Direction::NORTHEAST => north_move + east_move,
                Direction::NORTHWEST => north_move + west_move,
                Direction::SOUTHEAST => south_move + east_move,
                Direction::SOUTHWEST => south_move + west_move,
            };
    }
}

impl Display for GridCoordinateInf64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "({}, {})", self.x, self.y);
    }
}

impl std::ops::Add for GridCoordinateInf64 {
    type Output = GridCoordinateInf64;

    fn add(self, other: GridCoordinateInf64) -> GridCoordinateInf64 {
        return GridCoordinateInf64 {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_coord() {
        let a = GridCoordinate::new(2321, 9875);
        let b = format!("{}", a);
        assert_eq!(b, "(2321, 9875)");

        let c = GridCoordinateInf::new(-3213, -9932);
        let d = format!("{}", c);
        assert_eq!(d, "(-3213, -9932)");
    }

    #[test]
    fn test_move_on_infinite_grid() {
        let start = GridCoordinateInf::new(0, 0);
        let mut cur = start.move_dir(Direction::NORTH);
        assert_eq!(cur, GridCoordinateInf::new(0, -1));
        cur = cur.move_dir(Direction::WEST);
        assert_eq!(cur, GridCoordinateInf::new(-1, -1));
        cur = cur.move_dir(Direction::NORTHWEST);
        assert_eq!(cur, GridCoordinateInf::new(-2, -2));
        cur = cur.move_dir(Direction::NORTHEAST);
        assert_eq!(cur, GridCoordinateInf::new(-1, -3));
        cur = cur.move_dir(Direction::EAST);
        assert_eq!(cur, GridCoordinateInf::new(-0, -3));
        cur = cur.move_dir(Direction::SOUTH);
        assert_eq!(cur, GridCoordinateInf::new(0, -2));
        cur = cur.move_dir(Direction::SOUTHEAST);
        assert_eq!(cur, GridCoordinateInf::new(1, -1));
        cur = cur.move_dir(Direction::SOUTHWEST);
        assert_eq!(cur, GridCoordinateInf::new(0, 0));
    }

    #[test]
    fn test_move_on_infinite_grid64() {
        let start = GridCoordinateInf64::new(0, 0);
        let mut cur = start.move_dir(Direction::NORTH);
        assert_eq!(cur, GridCoordinateInf64::new(0, -1));
        cur = cur.move_dir(Direction::WEST);
        assert_eq!(cur, GridCoordinateInf64::new(-1, -1));
        cur = cur.move_dir(Direction::NORTHWEST);
        assert_eq!(cur, GridCoordinateInf64::new(-2, -2));
        cur = cur.move_dir(Direction::NORTHEAST);
        assert_eq!(cur, GridCoordinateInf64::new(-1, -3));
        cur = cur.move_dir(Direction::EAST);
        assert_eq!(cur, GridCoordinateInf64::new(-0, -3));
        cur = cur.move_dir(Direction::SOUTH);
        assert_eq!(cur, GridCoordinateInf64::new(0, -2));
        cur = cur.move_dir(Direction::SOUTHEAST);
        assert_eq!(cur, GridCoordinateInf64::new(1, -1));
        cur = cur.move_dir(Direction::SOUTHWEST);
        assert_eq!(cur, GridCoordinateInf64::new(0, 0));
    }
}
