#![feature(let_chains)]

mod grid;
mod grid_wide;
mod moves;

use crate::grid::Grid;
use crate::grid_wide::GridWide;
use crate::moves::Move;
use anyhow::Result;
use std::path::Path;

fn parse_moves(input: &str) -> Vec<Move> {
    input
        .chars()
        .filter(|&c| c != '\n')
        .map(Move::from_char)
        .collect::<Vec<_>>()
}

fn solve_file_p1(path: impl AsRef<Path>) -> Result<usize> {
    let input = std::fs::read_to_string(path)?;
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let mut grid = Grid::from_str(grid)?;

    for dir in parse_moves(moves) {
        grid.do_move(dir);
    }
    println!("{grid}");
    Ok(grid.gps_sum())
}

fn solve_file_p2(path: impl AsRef<Path>) -> Result<usize> {
    let input = std::fs::read_to_string(path)?;
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let mut grid = GridWide::from_str(grid)?;

    for dir in parse_moves(moves) {
        grid.do_move(dir);
    }
    println!("{grid}");
    Ok(grid.gps_sum())
}

fn main() -> Result<()> {
    println!("Result part 1: {:?}", solve_file_p1("../input")?);
    println!("Result part 2: {:?}", solve_file_p2("../input")?);
    Ok(())
}
