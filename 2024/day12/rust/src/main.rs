#![feature(let_chains)]

mod grid;

use crate::grid::Grid;
use anyhow::Result;
use std::path::Path;

fn solve_file(path: impl AsRef<Path>) -> Result<(usize, usize)> {
    let grid = Grid::from_file(path)?;
    Ok(grid.fence_prices())
}

fn main() -> Result<()> {
    println!("Result (test): {:?}", solve_file("../input-test")?);
    println!("Result (input): {:?}", solve_file("../input")?);
    Ok(())
}
