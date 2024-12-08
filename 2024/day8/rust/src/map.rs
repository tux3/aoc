use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::path::Path;

pub type Pos = (usize, usize);

pub struct Map {
    pub grid: Vec<Vec<char>>,
    pub antennas: HashMap<char, Vec<Pos>>,
}

impl Map {
    pub fn new(grid: Vec<Vec<char>>) -> Self {
        let mut antennas = HashMap::<char, Vec<Pos>>::new();
        for (y, l) in grid.iter().enumerate() {
            for (x, &c) in l.iter().enumerate() {
                if c != '.' {
                    antennas.entry(c).or_default().push((x, y));
                }
            }
        }
        Self { grid, antennas }
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let grid = std::fs::read_to_string(path)?
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect();
        Ok(Self::new(grid))
    }

    fn pos_diff((x1, y1): Pos, (x2, y2): Pos) -> (isize, isize) {
        (x1 as isize - x2 as isize, y1 as isize - y2 as isize)
    }

    fn add_pos_diff_inbounds(&self, (px, py): Pos, (dx, dy): (isize, isize)) -> Option<Pos> {
        let (x, y) = (px.checked_add_signed(dx)?, py.checked_add_signed(dy)?);
        (x < self.grid.len() && y < self.grid.len()).then(|| (x, y))
    }

    fn antinodes_simple(&self, mut nodes: Vec<Pos>, (&p1, &p2): (&Pos, &Pos)) -> Vec<Pos> {
        let diff = Self::pos_diff(p1, p2);
        if let Some(an) = self.add_pos_diff_inbounds(p1, diff) {
            nodes.push(an)
        }
        if let Some(an) = self.add_pos_diff_inbounds(p2, (-diff.0, -diff.1)) {
            nodes.push(an)
        }
        nodes
    }

    fn antinodes_harmonic(&self, mut nodes: Vec<Pos>, (&p1, &p2): (&Pos, &Pos)) -> Vec<Pos> {
        let diff = Self::pos_diff(p1, p2);
        let diff_gcd = num::integer::gcd(diff.0, diff.1);
        let diff = (diff.0 / diff_gcd, diff.1 / diff_gcd);

        let mut cur = p2;
        while let Some(an) = self.add_pos_diff_inbounds(cur, diff) {
            cur = an;
            nodes.push(an);
        }

        cur = p1;
        while let Some(an) = self.add_pos_diff_inbounds(cur, (-diff.0, -diff.1)) {
            cur = an;
            nodes.push(an)
        }
        nodes
    }

    pub fn antinodes(&self, harmonics: bool) -> HashSet<Pos> {
        let antinodes_fun = if harmonics {
            Self::antinodes_harmonic
        } else {
            Self::antinodes_simple
        };

        self.antennas
            .values()
            .flat_map(|antennas| {
                antennas
                    .iter()
                    .tuple_combinations()
                    .fold(vec![], |acc, val| antinodes_fun(&self, acc, val))
            })
            .collect()
    }
}
