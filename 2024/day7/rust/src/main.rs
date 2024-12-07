#![feature(unboxed_closures, fn_traits)]

mod equation;
mod op;

use crate::equation::Equation;
use crate::op::Op;
use anyhow::Result;
use std::path::Path;

pub fn sum_solvable_by(eqs: &[Equation], ops: &[Op]) -> usize {
    eqs.iter()
        .filter(|eq| eq.solvable(ops))
        .map(|eq| eq.target)
        .sum()
}

pub fn calibrate(path: impl AsRef<Path>) -> Result<(usize, usize)> {
    let eqs: Vec<Equation> = std::fs::read_to_string(path)?
        .lines()
        .map(Equation::from_line)
        .collect();
    Ok((
        sum_solvable_by(&eqs, &[Op::Add, Op::Mul]),
        sum_solvable_by(&eqs, &[Op::Add, Op::Mul, Op::Cat]),
    ))
}

fn main() -> Result<()> {
    println!("Results (test): {:?}", calibrate("../input-test")?);
    println!("Results (input): {:?}", calibrate("../input")?);
    Ok(())
}
