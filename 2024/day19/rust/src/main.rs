use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

fn parse_input(path: impl AsRef<Path>) -> Result<(Vec<String>, Vec<String>)> {
    let input = std::fs::read_to_string(path)?;
    let (patterns, designs) = input.split_once("\n\n").unwrap();
    let patterns = patterns.split(", ").map(|s| s.to_owned()).collect();
    let designs = designs.lines().map(|s| s.to_owned()).collect();
    Ok((patterns, designs))
}

fn solve_rec(
    patterns: &[String],
    design: &str,
    pos: usize,
    solutions_at: &mut HashMap<usize, usize>,
) -> usize {
    if pos == design.len() {
        return 1;
    }
    if let Some(&n) = solutions_at.get(&pos) {
        return n;
    }
    let mut solutions = 0;
    for pattern in patterns {
        if design[pos..].starts_with(pattern) {
            solutions += solve_rec(patterns, &design, pos + pattern.len(), solutions_at);
        }
    }
    solutions_at.insert(pos, solutions);
    solutions
}

fn solve(patterns: &[String], design: &str) -> usize {
    let mut solutions_at = HashMap::<usize, usize>::new();
    solve_rec(patterns, design, 0, &mut solutions_at)
}

fn main() -> Result<()> {
    let (mut patterns, designs) = parse_input("../input")?;
    patterns.sort_by(|a, b| b.len().cmp(&a.len()));
    let num_solvable = designs
        .iter()
        .map(|d| (solve(&patterns, d) > 0) as usize)
        .sum::<usize>();
    println!("Num solvable: {num_solvable}");

    let num_solutions = designs.iter().map(|d| solve(&patterns, d)).sum::<usize>();
    println!("Num solutions: {num_solutions}");
    Ok(())
}
