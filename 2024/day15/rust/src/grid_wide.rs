use self::Tile::*;
use crate::moves::Move::{self, *};
use anyhow::Result;
use itertools::Itertools;
use ndarray::{Array2, ShapeBuilder};
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};

pub type Pos = (usize, usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Floor,
    LBox,
    RBox,
    Wall,
}

impl Tile {
    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Floor,
            '[' => LBox,
            ']' => RBox,
            '#' => Wall,
            _ => panic!("Invalid tile type '{c}'"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Floor => f.write_char('.'),
            LBox => f.write_char('['),
            RBox => f.write_char(']'),
            Wall => f.write_char('#'),
        }
    }
}

pub struct GridWide {
    pub pos: Pos,
    pub map: Array2<Tile>,
}

impl Display for GridWide {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.map.ncols() {
            for x in 0..self.map.nrows() {
                if (x, y) == self.pos {
                    f.write_char('@')?
                } else {
                    self.map[(x, y)].fmt(f)?
                }
            }
            if y < self.map.ncols() - 1 {
                f.write_char('\n')?
            }
        }
        Ok(())
    }
}

impl GridWide {
    pub fn from_str(grid: &str) -> Result<Self> {
        let grid = grid
            .replace('.', "..")
            .replace('@', "@.")
            .replace('#', "##")
            .replace('O', "[]");

        let h = grid.lines().count();
        let w = grid.find('\n').unwrap();
        let start_off = grid.find('@').unwrap();
        let start = (start_off % (w + 1), start_off / (w + 1));

        let map = Array2::from_shape_vec(
            (w, h).f(),
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
            Right => (x < self.map.nrows() - 1).then(|| (x + 1, y)),
            Down => (y < self.map.ncols() - 1).then(|| (x, y + 1)),
            Left => (x != 0).then(|| (x - 1, y)),
        }
    }

    pub fn next_hole(&self, mut pos: Pos, dir: Move) -> Option<Pos> {
        loop {
            match self.map[pos] {
                Floor => break Some(pos),
                Wall => break None,
                LBox | RBox => pos = self.next_pos(pos, dir).unwrap(),
            }
        }
    }

    // If we're moving obstacles up/down, we may drag along a bunch of boxes
    // We don't necessarily move all the boxes in the bounding box, e.g.:
    //                        []
    // [] []                   [][] <- doesn't move
    //  []       becomes        []
    //   []                      @
    //    @
    // So let's recursively find a list of boxes that need to move
    // If we return None, the whole thing cannot move
    pub fn vertical_move_list(&self, pos: Pos, dir: Move) -> Option<HashSet<Pos>> {
        let (lpos, rpos) = match self.map[pos] {
            Floor => return Some(HashSet::new()),
            Wall => return None,
            LBox => (pos, (pos.0 + 1, pos.1)),
            RBox => ((pos.0 - 1, pos.1), pos),
        };
        let next_lpos = self.next_pos(lpos, dir).unwrap();
        let next_rpos = self.next_pos(rpos, dir).unwrap();
        let mut list = self.vertical_move_list(next_lpos, dir)?;
        list.extend(self.vertical_move_list(next_rpos, dir)?);
        list.extend(&[lpos, rpos]);
        Some(list)
    }

    pub fn do_move(&mut self, dir: Move) {
        // There are walls, so we should never be able to try to walk out of bounds
        let next = self.next_pos(self.pos, dir).expect("out of bounds");
        if self.map[next] == Floor {
            self.pos = next
        } else if (self.map[next] == LBox || self.map[next] == RBox)
            && (dir == Left || dir == Right)
            && let Some(hole) = self.next_hole(next, dir)
        {
            if dir == Left {
                for x in hole.0..next.0 {
                    self.map[(x, next.1)] = self.map[(x + 1, next.1)];
                }
            } else {
                for x in (next.0..hole.0).rev() {
                    self.map[(x + 1, next.1)] = self.map[(x, next.1)];
                }
            }
            self.map[next] = Floor;
            self.pos = next
        } else if (self.map[next] == LBox || self.map[next] == RBox) && (dir == Up || dir == Down) {
            let Some(move_list) = self.vertical_move_list(next, dir) else {
                return;
            };
            let move_list = if dir == Up {
                move_list.iter().sorted_by(|a, b| Ord::cmp(&a.1, &b.1))
            } else {
                move_list.iter().sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
            };
            for &(x, y) in move_list {
                if dir == Up {
                    self.map[(x, y - 1)] = self.map[(x, y)];
                } else {
                    self.map[(x, y + 1)] = self.map[(x, y)];
                }
                self.map[(x, y)] = Floor
            }
            self.pos = next
        }
    }

    pub fn gps_sum(&self) -> usize {
        self.map
            .indexed_iter()
            .map(|((x, y), &c)| if c == LBox { 100 * y + x } else { 0 })
            .sum()
    }
}
