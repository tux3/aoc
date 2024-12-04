mod grid;
use grid::Grid;

use anyhow::Result;

fn wordsearch(grid: Grid) -> usize {
    grid.lines()
        .chain(grid.columns())
        .chain(grid.diagonals())
        .map(|s| s.matches("XMAS").count() + s.matches("SAMX").count())
        .sum()
}

fn crosssearch(grid: Grid) -> usize {
    grid.crosses()
        .map(|(a, b)| (a == "MAS" || a == "SAM") && (b == "MAS" || b == "SAM"))
        .map(usize::from)
        .sum()
}

fn main() -> Result<()> {
    println!(
        "Count (input-test): {}",
        wordsearch(Grid::from_file("../input-test")?)
    );
    println!(
        "Count (input): {}",
        wordsearch(Grid::from_file("../input")?)
    );

    println!(
        "X-Count (input-test): {}",
        crosssearch(Grid::from_file("../input-test")?)
    );
    println!(
        "X-Count (input): {}",
        crosssearch(Grid::from_file("../input")?)
    );
    Ok(())
}
