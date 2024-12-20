use self::Tile::*;
use crate::direction::Direction::{self, *};
use anyhow::Result;
use itertools::Itertools;
use ndarray::{Array2, ShapeBuilder, s};
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
            'S' | 'E' | '.' => Floor,
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
    pub start: Pos,
    pub end: Pos,
    pub map: Array2<Tile>,
    pub distances: HashMap<Pos, usize>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.map.nrows() {
            for x in 0..self.map.ncols() {
                if (x, y) == self.start {
                    f.write_char('S')?
                } else if (x, y) == self.end {
                    f.write_char('E')?
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
        let start_off = grid.find('S').unwrap();
        let start = (start_off % (w + 1), start_off / (w + 1));
        let end_off = grid.find('E').unwrap();
        let end = (end_off % (w + 1), end_off / (w + 1));

        let map = Array2::from_shape_vec(
            (w, h).f(),
            grid.trim()
                .chars()
                .filter(|&c| c != '\n')
                .map(|c| Tile::from_char(c))
                .collect::<Vec<_>>(),
        )?;

        let mut grid = Self {
            start,
            end,
            map,
            distances: Default::default(),
        };
        grid.distances = grid.plain_distances();
        Ok(grid)
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

    pub fn plain_distances(&self) -> HashMap<Pos, usize> {
        let mut distances = HashMap::<Pos, usize>::new();

        let mut last_pos = self.end;
        let mut pos = self.end;
        let mut dist = 0;
        while pos != self.start {
            distances.insert(pos, dist);

            for dir in [Up, Right, Down, Left] {
                let Some(to_pos) = self.next_pos(pos, dir) else {
                    continue;
                };
                if to_pos == last_pos {
                    continue;
                }
                last_pos = pos;
                pos = to_pos;
                dist += 1;
                break;
            }
        }
        distances.insert(pos, dist);
        distances
    }

    pub fn solve(&self) -> usize {
        let mut shortcuts = Vec::<(Pos, Pos, usize)>::new();
        for ((x, y), &tile) in self.map.indexed_iter() {
            if tile != Wall {
                continue;
            }
            if x > 0
                && x < self.map.ncols() - 1
                && self.map[(x - 1, y)] == Floor
                && self.map[(x + 1, y)] == Floor
            {
                let (l, r) = (self.distances[&(x - 1, y)], self.distances[&(x + 1, y)]);
                if l < r {
                    shortcuts.push(((x - 1, y), (x + 1, y), r - l - 2));
                } else {
                    shortcuts.push(((x + 1, y), (x - 1, y), l - r - 2));
                }
            }
            if y > 0
                && y < self.map.nrows() - 1
                && self.map[(x, y - 1)] == Floor
                && self.map[(x, y + 1)] == Floor
            {
                let (u, d) = (self.distances[&(x, y - 1)], self.distances[&(x, y + 1)]);
                if u < d {
                    shortcuts.push(((x, y - 1), (x, y + 1), d - u - 2));
                } else {
                    shortcuts.push(((x, y + 1), (x, y - 1), u - d - 2));
                }
            }
        }

        shortcuts
            .iter()
            .filter_map(|&(_s, _e, g)| (g >= 100).then_some(g))
            .count()
    }

    pub fn solve_p2(&self, max_dist: usize, min_saved: usize) -> usize {
        let mut shortcuts = Vec::<(Pos, Pos, usize)>::new();

        for ((ap, ad), (bp, bd)) in self.distances.iter().tuple_combinations() {
            let taxicab = usize::abs_diff(ap.0, bp.0) + usize::abs_diff(ap.1, bp.1);
            if taxicab > max_dist || taxicab < 2 {
                continue;
            }
            if bd > ad {
                shortcuts.push((*ap, *bp, bd - ad - taxicab));
            } else {
                shortcuts.push((*bp, *ap, ad - bd - taxicab));
            }
        }

        shortcuts
            .iter()
            .filter_map(|&(_s, _e, g)| (g >= min_saved).then_some(g))
            .count()
    }
}
