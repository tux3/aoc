use std::path::Path;
use anyhow::Result;
use itertools::Itertools;

fn parse_input_file(path: impl AsRef<Path>) -> Result<(Vec<i32>, Vec<i32>)> {
    let input_str = std::fs::read_to_string(path)?;
    Ok(input_str.lines().map(|l| {
        let (l, r) = l.split_once(' ').unwrap();
        (l.parse().unwrap(), r.parse().unwrap())
    }).unzip::<i32, i32, _, _>())
}

fn compute_distance(left: &[i32], right: &[i32]) -> i32 {
   left.iter().zip(right.as_ref()).map(|(l, r)|
        (r - l).abs()
    ).sum()
}

fn compute_similarity(left: &[i32], right: &[i32]) -> i32 {
    let freq = right.into_iter().counts();
    left.iter().map(|l|
        l * freq.get(&l).copied().unwrap_or(0) as i32
    ).sum()
}

fn display_scores(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    let (mut left, mut right) = parse_input_file(path)?;
    left.sort();
    right.sort();

    let name = path.file_name().unwrap().to_string_lossy();
    println!("Distance ({name}): {}", compute_distance(&left, &right));
    println!("Similarity ({name}): {}", compute_similarity(&left, &right));
    Ok(())
}

fn main() -> Result<()> {
    display_scores("../input-test")?;
    display_scores("../input")?;
    Ok(())
}
