#![feature(let_chains)]

mod direction;
mod grid;

use crate::grid::Grid;
use anyhow::Result;
use std::path::Path;

fn solve_file(path: impl AsRef<Path>) -> Result<(usize, usize)> {
    let input = std::fs::read_to_string(path)?;
    let grid = Grid::from_str(&input)?;
    Ok(grid.solve())
}

fn main() -> Result<()> {
    println!("Result: {:?}", solve_file("../input")?);
    Ok(())
}
