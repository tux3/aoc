use self::Tile::*;
use super::direction::Direction::{self, *};
use anyhow::Result;
use hashbrown::{HashMap, HashSet};
use ndarray::{Array2, ShapeBuilder};
use std::fmt::{Display, Formatter, Write};

pub type Pos = (usize, usize);
pub type EdgeCosts = [Option<usize>; 4]; // Where None means a wall

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Floor,
    Wall,
}

impl Tile {
    pub fn from_char(c: char) -> Self {
        match c {
            'S' | 'E' => Floor,
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
    pub start: Pos,
    pub end: Pos,
    pub map: Array2<Tile>,
    pub costs: HashMap<(Pos, Direction), EdgeCosts>,
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
            grid.trim_end()
                .chars()
                .filter(|&c| c != '\n')
                .map(Tile::from_char)
                .collect::<Vec<_>>(),
        )?;
        let costs = Self::map_costs(&map, start);

        Ok(Self {
            start,
            end,
            map,
            costs,
        })
    }

    pub fn next_pos((x, y): Pos, dir: Direction) -> Pos {
        match dir {
            Up => (x, y - 1),
            Right => (x + 1, y),
            Down => (x, y + 1),
            Left => (x - 1, y),
        }
    }

    fn edge_costs(map: &Array2<Tile>, pos: Pos, from_dir: Direction) -> [Option<usize>; 4] {
        [Up, Right, Down, Left].map(|to_dir| {
            let npos = Self::next_pos(pos, to_dir);
            (map[npos] == Floor).then(|| 1 + from_dir.turn_cost(to_dir))
        })
    }

    pub fn map_costs(map: &Array2<Tile>, start: Pos) -> HashMap<(Pos, Direction), EdgeCosts> {
        let mut map_costs = HashMap::<(Pos, Direction), EdgeCosts>::new();
        let mut stack: Vec<(Pos, Direction)> = vec![(start, Right)];

        while let Some((pos, dir)) = stack.pop() {
            let cur_edges = Self::edge_costs(map, pos, dir);
            map_costs.insert((pos, dir), cur_edges);

            for (idx, _cost) in cur_edges
                .iter()
                .enumerate()
                .filter_map(|(i, oc)| oc.map(|c| (i, c)))
            {
                let to_dir = Direction::from_num(idx);
                let to_pos = Self::next_pos(pos, to_dir);
                if !map_costs.contains_key(&(to_pos, to_dir)) {
                    stack.push((to_pos, to_dir));
                }
            }
        }
        map_costs
    }

    pub fn solve(&self) -> (usize, usize) {
        let visited = &mut HashMap::<Pos, (Direction, usize)>::new();
        let mut path = vec![];
        let mut stack: Vec<(usize, Pos, Direction, usize)> = vec![(0, self.start, Right, 0)];
        let mut best_cost: Option<usize> = None;
        let mut best_path_cells = HashSet::<Pos>::new();

        while let Some((path_size, pos, dir, cur_cost)) = stack.pop() {
            path.truncate(path_size);
            path.push(pos);

            visited.insert(pos, (dir, cur_cost));
            if pos == self.end {
                if best_cost != Some(cur_cost) {
                    best_path_cells.clear()
                }
                best_path_cells.extend(&path);
                best_cost = Some(cur_cost);
                continue;
            }

            for (to_dir, to_cost) in self.costs[&(pos, dir)]
                .iter()
                .enumerate()
                .filter_map(|(i, oc)| oc.map(|c| (Direction::from_num(i), c)))
                .filter(|&(i, _)| i != dir.rev())
            {
                if let Some(best_cost) = best_cost
                    && cur_cost + to_cost > best_cost
                {
                    continue;
                }
                let to_pos = Self::next_pos(pos, to_dir);
                if let Some(&(prev_dir, prev_cost)) = visited.get(&to_pos) {
                    if cur_cost + to_cost > prev_cost + to_dir.turn_cost(prev_dir) {
                        continue;
                    }
                }
                stack.push((path.len(), to_pos, to_dir, cur_cost + to_cost))
            }
        }

        let (_, cost) = *visited.get(&(self.end)).unwrap();
        self.draw_path(&best_path_cells);

        (cost, best_path_cells.len())
    }

    fn draw_path(&self, visited: &HashSet<Pos>) {
        let mut str = String::new();
        for y in 0..self.map.ncols() {
            for x in 0..self.map.nrows() {
                if (x, y) == self.start {
                    str.push('S');
                } else if (x, y) == self.end {
                    str.push('S');
                } else if visited.contains(&(x, y)) {
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
