mod map;
mod move_table;

use crate::map::Map;
use anyhow::Result;

fn main() -> Result<()> {
    println!(
        "Walk results (test): {:?}",
        Map::from_file("../input-test")?.walk_results()
    );
    println!(
        "Walk results (input): {:?}",
        Map::from_file("../input")?.walk_results()
    );
    Ok(())
}
