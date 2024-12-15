#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Move {
    Up,
    Right,
    Down,
    Left,
}

impl Move {
    pub fn from_char(c: char) -> Self {
        match c {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => panic!("Invalid move type '{c}'"),
        }
    }
}
