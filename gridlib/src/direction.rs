use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
    NORTHEAST,
    SOUTHEAST,
    SOUTHWEST,
    NORTHWEST,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Direction::NORTH => "NORTH",
            Direction::EAST => "EAST",
            Direction::SOUTH => "SOUTH",
            Direction::WEST => "WEST",
            Direction::NORTHEAST => "NORTHEAST",
            Direction::SOUTHEAST => "SOUTHEAST",
            Direction::SOUTHWEST => "SOUTHWEST",
            Direction::NORTHWEST => "NORTHWEST",
        };
        return write!(f, "{}", s);
    }
}
