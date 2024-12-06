use anyhow::Result;
use hashbrown::HashSet;
use std::path::Path;

pub type Pos = (usize, usize);

pub struct Map {
    pub start_pos: Pos,
    pub grid: Vec<Vec<char>>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    pub fn turn(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl Map {
    pub fn new(grid: Vec<Vec<char>>) -> Self {
        let start_pos = Self::start_pos(&grid);
        Self { start_pos, grid }
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self::new(
            std::fs::read_to_string(path)?
                .lines()
                .map(|l| l.chars().collect())
                .collect(),
        ))
    }

    fn start_pos(grid: &Vec<Vec<char>>) -> Pos {
        for (i, l) in grid.iter().enumerate() {
            for (j, c) in l.iter().enumerate() {
                if ['^'].contains(c) {
                    return (j, i);
                }
            }
        }
        panic!("Start pos not found!")
    }

    pub fn next_pos(&self, (x, y): Pos, dir: Direction) -> Option<Pos> {
        match dir {
            Direction::Up => {
                if y == 0 {
                    None
                } else {
                    Some((x, y - 1))
                }
            }
            Direction::Right => {
                if x == self.grid.len() - 1 {
                    None
                } else {
                    Some((x + 1, y))
                }
            }
            Direction::Down => {
                if y == self.grid.len() - 1 {
                    None
                } else {
                    Some((x, y + 1))
                }
            }
            Direction::Left => {
                if x == 0 {
                    None
                } else {
                    Some((x - 1, y))
                }
            }
        }
    }

    #[allow(unused)]
    pub fn display_walk(&self, visited: &HashSet<Pos>, obstructions: &HashSet<Pos>) {
        for (y, l) in self.grid.iter().enumerate() {
            for (x, c) in l.iter().enumerate() {
                if obstructions.contains(&(x, y)) {
                    print!("O");
                } else if visited.contains(&(x, y)) {
                    print!("X");
                } else {
                    print!("{c}")
                }
            }
            println!();
        }
    }

    // Simple walk, can be replaced by can_obstruct_ahead_fast for extra zoomies :)
    #[allow(unused)]
    pub fn can_obstruct_ahead(&self, obs_pos: Pos) -> bool {
        let mut visited = HashSet::<(Pos, Direction)>::with_capacity(32);

        let mut pos = self.start_pos;
        let mut dir = Direction::Up;
        while let Some((x, y)) = self.next_pos(pos, dir) {
            if self.grid[y][x] == '#' || (x, y) == obs_pos {
                dir = dir.turn();
                if !visited.insert((pos, dir)) {
                    return true;
                }
            } else {
                pos = (x, y);
            }
        }
        false
    }

    /// Returns (walk_length, possible_obstructions)
    pub fn walk_results(&self) -> (usize, usize) {
        let mut pos = self.start_pos;
        let mut dir = Direction::Up;
        let mut visited = HashSet::<Pos>::from_iter([pos]);
        let mut possible_obstructions = HashSet::<Pos>::with_capacity(1024);

        let mut move_table = self.fast_move_table();
        move_table.insert(
            (pos, Direction::Left),
            self.find_move_target((pos, Direction::Left)),
        );

        while let Some((x, y)) = self.next_pos(pos, dir) {
            if self.grid[y][x] == '#' {
                dir = dir.turn();
            } else {
                pos = (x, y);

                if visited.insert(pos) && self.can_obstruct_ahead_fast(pos, dir, &move_table) {
                    possible_obstructions.insert(pos);
                }
            }
        }

        // The display eats almost 1ms, but it looks neat =)
        self.display_walk(&visited, &possible_obstructions);

        (visited.len(), possible_obstructions.len())
    }
}
