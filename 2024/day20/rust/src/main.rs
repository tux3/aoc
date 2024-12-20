#![feature(let_chains)]

mod direction;
mod grid;

use crate::grid::Grid;
use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("../input")?;
    let mut grid = Grid::from_str(&input)?;
    println!("Map:\n{grid}");
    println!("Shortcuts under 2ps saving 50ps: {:?}", grid.solve());
    println!(
        "Shortcuts under 20ns saving 100ps: {:?}",
        grid.solve_p2(20, 100)
    );
    Ok(())
}
