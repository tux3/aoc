#![feature(let_chains)]

mod direction;
mod grid;

use crate::grid::Grid;
use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("../input")?;
    let mut grid = Grid::from_str(&input, 71)?;

    let nwalls = 1024;
    grid.add_walls(nwalls);
    println!("Part 1 result: {:?}", grid.solve());

    let mut max_good = 0;
    let mut min_bad = input.lines().count();

    while min_bad - max_good > 1 {
        let num_walls = max_good + (min_bad - max_good) / 2;
        grid.clear_walls();
        grid.add_walls(num_walls);
        if grid.solvable() {
            max_good = num_walls;
        } else {
            min_bad = num_walls;
        }
    }

    // We reversed the walls, so this looks a bit silly
    let first_bad = grid
        .orig_walls
        .iter()
        .rev()
        .skip(min_bad - 1)
        .next()
        .unwrap();
    println!("First unsolvable wall: {first_bad:?}");

    Ok(())
}
