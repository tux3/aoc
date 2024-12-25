mod parse;

use anyhow::Result;
use crate::parse::{parse, Key, Lock};

fn matches(key: &Key, lock: &Lock) -> bool {
    for (kx, lx) in key.iter().zip(lock) {
        if kx + lx > 5 {
            return false
        }
    }
    true
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("../input")?;
    let (keys, locks) = parse(&input);

    let mut num_matches = 0usize;
    for k in &keys {
        for l in &locks {
            if matches(&k, &l) {
                num_matches += 1
            }
        }
    }
    println!("{num_matches}");

    Ok(())
}
