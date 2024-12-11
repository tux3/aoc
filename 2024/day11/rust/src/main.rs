mod digits;

use crate::digits::split_even_digits;
use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

fn sim_num(cache: &mut HashMap<(u64, usize), usize>, n: u64, steps: usize) -> usize {
    if steps == 0 {
        return 1;
    }
    if let Some(&r) = cache.get(&(n, steps)) {
        return r;
    }
    let r = if n == 0 {
        sim_num(cache, 1, steps - 1)
    } else if let Some((l, r)) = split_even_digits(n) {
        sim_num(cache, l, steps - 1) + sim_num(cache, r, steps - 1)
    } else {
        sim_num(cache, n * 2024, steps - 1)
    };
    cache.insert((n, steps), r);
    r
}

fn solve_file(path: impl AsRef<Path>) -> Result<(usize, usize)> {
    let nums: Vec<u64> = std::fs::read_to_string(path)?
        .trim_end()
        .split(' ')
        .map(|e| e.parse().unwrap())
        .collect();
    let cache = &mut HashMap::new();
    let sum1 = nums.iter().map(|&n| sim_num(cache, n, 25)).sum();
    let sum2 = nums.iter().map(|&n| sim_num(cache, n, 75)).sum();

    Ok((sum1, sum2))
}

fn main() -> Result<()> {
    println!("Result (test): {:?}", solve_file("../input-test")?);
    println!("Result (input): {:?}", solve_file("../input")?);
    Ok(())
}
