use self::Tile::*;
use crate::moves::Move::{self, *};
use anyhow::Result;
use ndarray::{Array2, ShapeBuilder};
use std::fmt::{Display, Formatter, Write};

pub type Pos = (usize, usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Floor,
    Box,
    Wall,
}

impl Tile {
    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Floor,
            'O' => Box,
            '#' => Wall,
            _ => panic!("Invalid tile type '{c}'"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Floor => f.write_char('.'),
            Box => f.write_char('O'),
            Wall => f.write_char('#'),
        }
    }
}

pub struct Grid {
    pub pos: Pos,
    pub map: Array2<Tile>,
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
    pub fn from_str(grid: &str) -> Result<Self> {
        let h = grid.lines().count();
        let w = grid.find('\n').unwrap();
        let start_off = grid.find('@').unwrap();
        let start = (start_off / (w + 1), start_off % (w + 1));

        let map = Array2::from_shape_vec(
            (h, w).f(),
            grid.trim()
                .chars()
                .filter(|&c| c != '\n')
                .map(|c| if c == '@' { Floor } else { Tile::from_char(c) })
                .collect::<Vec<_>>(),
        )?;

        Ok(Self { pos: start, map })
    }

    pub fn next_pos(&self, (x, y): Pos, dir: Move) -> Option<Pos> {
        match dir {
            Up => (y != 0).then(|| (x, y - 1)),
            Right => (x < self.map.ncols() - 1).then(|| (x + 1, y)),
            Down => (y < self.map.nrows() - 1).then(|| (x, y + 1)),
            Left => (x != 0).then(|| (x - 1, y)),
        }
    }

    pub fn next_hole(&self, mut pos: Pos, dir: Move) -> Option<Pos> {
        loop {
            match self.map[pos] {
                Floor => break Some(pos),
                Wall => break None,
                Box => pos = self.next_pos(pos, dir).unwrap(),
            }
        }
    }

    pub fn do_move(&mut self, dir: Move) {
        // There are walls, so we should never be able to try to walk out of bounds
        let next = self.next_pos(self.pos, dir).expect("out of bounds");
        if self.map[next] == Floor {
            self.pos = next
        } else if self.map[next] == Box
            && let Some(hole) = self.next_hole(next, dir)
        {
            self.map[next] = Floor;
            self.map[hole] = Box;
            self.pos = next
        }
    }

    pub fn gps_sum(&self) -> usize {
        self.map
            .indexed_iter()
            .map(|((x, y), &c)| if c == Box { 100 * y + x } else { 0 })
            .sum()
    }
}
