//! This optimization is a bit more code, but takes us from 50ms runtime to 6-7ms =)
//!
//! For the whole map, we precompute a lookup table of jumps from point A to point B
//! Except for when the path crosses an obstacle, we can do all moves through the table
//! This is much faster than walking every square of the map

use crate::map::{Direction, Map, Pos};
use hashbrown::{HashMap, HashSet};

impl Map {
    pub fn prev_pos((x, y): Pos, dir: Direction) -> Pos {
        match dir {
            Direction::Up => (x, y + 1),
            Direction::Right => (x - 1, y),
            Direction::Down => (x, y - 1),
            Direction::Left => (x + 1, y),
        }
    }

    pub fn can_obstruct_ahead_fast(
        &self,
        obs_pos: Pos,
        start_dir: Direction,
        move_table: &HashMap<(Pos, Direction), Pos>,
    ) -> bool {
        let mut visited = HashSet::<(Pos, Direction)>::with_capacity(128);

        // Since we only try to add an obstacle the first time we reach a square,
        // we don't need to reset at the beginning, we can start from the obstacle
        let mut pos = self.find_move_target((Self::prev_pos(obs_pos, start_dir), start_dir));
        let mut dir = start_dir.turn();

        while let Some(&(x, y)) = move_table.get(&(pos, dir)) {
            dir = dir.turn();

            // If the move table jumped across the obstacle, we hit the obstacle instead
            // We have to check obs.{x/y} == {x/y} in case the obstacle is next to a real wall
            // Otherwise (o<a) ^ (o<b) checks that the obstacle o is not between points a and b
            if (x == obs_pos.0
                && (dir == Direction::Up || dir == Direction::Down)
                && (obs_pos.1 == y || ((obs_pos.1 < y) ^ (obs_pos.1 < pos.1))))
                || (y == obs_pos.1
                    && (dir == Direction::Left || dir == Direction::Right)
                    && (obs_pos.0 == x || ((obs_pos.0 < x) ^ (obs_pos.0 < pos.0))))
            {
                // We hit stop just before the obstacle, then turn and move again
                let obs_hit_pos = Self::prev_pos(obs_pos, dir);
                pos = self.find_move_target((obs_hit_pos, dir));
                dir = dir.turn();
            } else {
                pos = (x, y);
            }

            if !visited.insert((pos, dir)) {
                return true;
            }
        }
        false
    }

    pub fn find_move_target(&self, ((x, y), dir): (Pos, Direction)) -> Pos {
        let dir = dir.turn();
        let mut pos = (x, y);
        while let Some((x, y)) = self.next_pos(pos, dir) {
            if self.grid[y][x] == '#' {
                return pos;
            }
            pos = (x, y);
        }
        pos
    }

    pub fn fast_move_table(&self) -> HashMap<(Pos, Direction), Pos> {
        let mut move_map = HashMap::<(Pos, Direction), Pos>::default();
        for (y, l) in self.grid.iter().enumerate() {
            for (x, &c) in l.iter().enumerate() {
                if c != '#' {
                    continue;
                }
                if y > 0 {
                    let start = ((x, y - 1), Direction::Down);
                    move_map.insert(start, self.find_move_target(start));
                }
                if y < self.grid.len() - 1 {
                    let start = ((x, y + 1), Direction::Up);
                    move_map.insert(start, self.find_move_target(start));
                }
                if x > 0 {
                    let start = ((x - 1, y), Direction::Right);
                    move_map.insert(start, self.find_move_target(start));
                }
                if x < self.grid.len() - 1 {
                    let start = ((x + 1, y), Direction::Left);
                    move_map.insert(start, self.find_move_target(start));
                }
            }
        }
        move_map
    }
}
