use self::Direction::*;
use std::fmt::{Display, Formatter};

#[repr(usize)]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    pub fn from_num(idx: usize) -> Self {
        match idx {
            0 => Up,
            1 => Right,
            2 => Down,
            3 => Left,
            _ => panic!("oh no"),
        }
    }

    pub fn turn_cost(&self, to: Direction) -> usize {
        if *self == to {
            0
        } else if (*self as usize + 2) % 4 == to as usize {
            2000
        } else {
            1000
        }
    }

    pub fn rev(&self) -> Self {
        match self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Up => f.write_str("up"),
            Right => f.write_str("right"),
            Down => f.write_str("down"),
            Left => f.write_str("left"),
        }
    }
}
