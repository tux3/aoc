use anyhow::Result;
use smallvec::{SmallVec, smallvec};
use std::collections::HashSet;
use std::path::Path;

pub type Pos = (usize, usize);
pub type EdgeList = SmallVec<[Pos; 4]>;

pub struct Grid {
    pub grid: Vec<Vec<u8>>,
    pub edges: Vec<Vec<EdgeList>>,
}

impl Grid {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let grid = std::fs::read_to_string(path)?
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>()
            })
            .collect();
        let edges = Self::edges(&grid);
        Ok(Self { grid, edges })
    }

    fn find_edges_at(grid: &Vec<Vec<u8>>, (x, y): Pos) -> EdgeList {
        let nextval = grid[y][x] + 1;
        let mut edges = smallvec![];
        if x > 0 && grid[y][x - 1] == nextval {
            edges.push((x - 1, y))
        }
        if x < grid.len() - 1 && grid[y][x + 1] == nextval {
            edges.push((x + 1, y))
        }
        if y > 0 && grid[y - 1][x] == nextval {
            edges.push((x, y - 1))
        }
        if y < grid.len() - 1 && grid[y + 1][x] == nextval {
            edges.push((x, y + 1))
        }
        edges
    }

    fn edges(grid: &Vec<Vec<u8>>) -> Vec<Vec<EdgeList>> {
        let mut edges_grid = Vec::new();

        for (y, l) in grid.iter().enumerate() {
            let mut edges_row = Vec::new();
            for (x, _val) in l.iter().enumerate() {
                edges_row.push(Self::find_edges_at(grid, (x, y)))
            }
            edges_grid.push(edges_row);
        }
        edges_grid
    }

    pub fn trail_ends(&self, &(x, y): &Pos) -> HashSet<Pos> {
        if self.grid[y][x] == 9 {
            return [(x, y)].into();
        }
        self.edges[y][x]
            .iter()
            .fold(HashSet::new(), |mut acc, next| {
                acc.extend(self.trail_ends(next));
                acc
            })
    }

    pub fn trail_rating(&self, &(x, y): &Pos) -> usize {
        if self.grid[y][x] == 9 {
            return 1;
        }
        self.edges[y][x]
            .iter()
            .fold(0, |acc, next| acc + self.trail_rating(next))
    }

    pub fn sum_trailheads_stats(&self) -> (usize, usize) {
        let mut score = 0;
        let mut ratings = 0;
        for (y, l) in self.grid.iter().enumerate() {
            for (x, &val) in l.iter().enumerate() {
                if val == 0 {
                    score += self.trail_ends(&(x, y)).len();
                    ratings += self.trail_rating(&(x, y));
                }
            }
        }
        (score, ratings)
    }
}
