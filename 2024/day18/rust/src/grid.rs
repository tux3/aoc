use self::Tile::*;
use crate::direction::Direction::{self, *};
use anyhow::Result;
use ndarray::{Array2, ShapeBuilder};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Write};

pub type Pos = (usize, usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Floor,
    Wall,
}

impl Tile {
    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Floor,
            '#' => Wall,
            _ => panic!("Invalid tile type '{c}'"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Floor => f.write_char('.'),
            Wall => f.write_char('#'),
        }
    }
}

pub struct Grid {
    pub pos: Pos,
    pub map: Array2<Tile>,
    pub orig_walls: Vec<(usize, usize)>, // In reverse order
    pub walls: Vec<(usize, usize)>,      // In reverse order
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.map.nrows() {
            for x in 0..self.map.ncols() {
                if (x, y) == self.pos {
                    f.write_char('@')?
                } else {
                    self.map[(x, y)].fmt(f)?
                }
            }
            if y < self.map.nrows() - 1 {
                f.write_char('\n')?
            }
        }
        Ok(())
    }
}

impl Grid {
    pub fn from_str(grid: &str, size: usize) -> Result<Self> {
        let walls = grid
            .lines()
            .map(|line| {
                let (l, r) = line.split_once(',').unwrap();
                (l.parse().unwrap(), r.parse().unwrap())
            })
            .rev()
            .collect::<Vec<(usize, usize)>>();

        let map = Array2::from_shape_fn((size, size), |_| Floor);

        Ok(Self {
            pos: (0, 0),
            map,
            orig_walls: walls.clone(),
            walls,
        })
    }

    pub fn clear_walls(&mut self) {
        self.walls = self.orig_walls.clone();
        self.map = Array2::from_shape_fn((self.map.ncols(), self.map.nrows()), |_| Floor);
    }

    pub fn add_walls(&mut self, num: usize) {
        let num = num.min(self.walls.len());
        let start = self.walls.len() - num;
        for &wall in &self.walls[start..] {
            self.map[wall] = Wall;
        }
        self.walls.truncate(start);
    }

    pub fn next_pos(&self, (x, y): Pos, dir: Direction) -> Option<Pos> {
        match dir {
            Up => (y != 0).then(|| (x, y - 1)),
            Right => (x < self.map.ncols() - 1).then(|| (x + 1, y)),
            Down => (y < self.map.nrows() - 1).then(|| (x, y + 1)),
            Left => (x != 0).then(|| (x - 1, y)),
        }
        .and_then(|pos| (self.map[pos] == Floor).then_some(pos))
    }

    pub fn solve(&self) -> usize {
        let end = (self.map.ncols() - 1, self.map.nrows() - 1);
        let visited = &mut HashMap::<Pos, usize>::new();
        let mut stack: Vec<(Pos, usize)> = vec![((0, 0), 0)];
        let mut best_cost: Option<usize> = None;

        while let Some((pos, cur_cost)) = stack.pop() {
            visited.insert(pos, cur_cost);
            if pos == end {
                best_cost = Some(cur_cost);
                continue;
            }

            for to_dir in [Up, Right, Down, Left] {
                if let Some(best_cost) = best_cost
                    && cur_cost + 1 > best_cost
                {
                    continue;
                }
                let Some(to_pos) = self.next_pos(pos, to_dir) else {
                    continue;
                };
                if let Some(&last_cost) = visited.get(&to_pos)
                    && last_cost <= cur_cost + 1
                {
                    continue;
                }
                stack.push((to_pos, cur_cost + 1))
            }
        }

        let cost = *visited.get(&(end)).unwrap();
        cost
    }

    pub fn solvable(&self) -> bool {
        let end = (self.map.ncols() - 1, self.map.nrows() - 1);
        let visited = &mut HashSet::<Pos>::new();
        let mut stack: Vec<(Pos, usize)> = vec![((0, 0), 0)];

        while let Some((pos, cur_cost)) = stack.pop() {
            visited.insert(pos);
            if pos == end {
                return true;
            }

            for to_dir in [Up, Right, Down, Left] {
                let Some(to_pos) = self.next_pos(pos, to_dir) else {
                    continue;
                };
                if visited.contains(&to_pos) {
                    continue;
                }
                stack.push((to_pos, cur_cost + 1))
            }
        }

        false
    }

    fn draw_path(&self, visited: &HashSet<Pos>) {
        let mut str = String::new();
        for y in 0..self.map.ncols() {
            for x in 0..self.map.nrows() {
                if visited.contains(&(x, y)) {
                    str.push('O');
                } else {
                    str += &self.map[(x, y)].to_string();
                }
            }
            if y < self.map.ncols() - 1 {
                str.push('\n');
            }
        }
        println!("{str}");
    }
}
