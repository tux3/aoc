mod machine;

use crate::machine::Machine;
use anyhow::Result;
use std::path::Path;

fn solve_file(path: impl AsRef<Path>) -> Result<(usize, usize)> {
    let input: Vec<Machine> = std::fs::read_to_string(path)?
        .trim_end()
        .split("\n\n")
        .map(|s| Machine::from_str(s))
        .collect();
    let mut tokens = 0.;
    let mut tokens_p2 = 0.;
    for mut machine in input {
        if let Some((a, b)) = machine.solve() {
            println!("{machine:?} => {a},{b}");
            tokens += 3. * a + b;
        }
        machine.x += 10000000000000.;
        machine.y += 10000000000000.;
        if let Some((a, b)) = machine.solve() {
            println!("{machine:?} => {a},{b}");
            tokens_p2 += 3. * a + b;
        }
    }

    Ok((tokens.round() as usize, tokens_p2.round() as usize))
}

fn main() -> Result<()> {
    println!("Result (test): {:?}", solve_file("../input-test")?);
    println!("Result (input): {:?}", solve_file("../input")?);
    Ok(())
}
