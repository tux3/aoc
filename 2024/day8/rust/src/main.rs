mod map;

use crate::map::Map;
use anyhow::Result;
use std::path::Path;

fn antennas(path: impl AsRef<Path>) -> anyhow::Result<(usize, usize)> {
    let map = Map::from_file(path)?;
    let antinodes_simple = map.antinodes(false);
    let hantinodes_harmonic = map.antinodes(true);

    Ok((antinodes_simple.len(), hantinodes_harmonic.len()))
}

fn main() -> Result<()> {
    println!("Results (test): {:?}", antennas("../input-test")?);
    println!("Results (input): {:?}", antennas("../input")?);
    Ok(())
}
