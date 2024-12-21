mod keypad;
mod numpad;

use crate::keypad::{precompute_costs_kp1, precompute_costs_kp2};
use crate::numpad::{NumPad, precompute_costs_np};
use anyhow::Result;
use itertools::Itertools;

fn code_complexity(np: &NumPad, code: &str) -> usize {
    let code_num = code[..code.len() - 1].parse::<usize>().unwrap();
    let cost = np.code_cost(code);
    code_num * cost
}

fn solve(num_keypads: usize, codes: &[&str]) -> usize {
    let mut kp = precompute_costs_kp1();
    for _ in 1..num_keypads {
        kp = precompute_costs_kp2(&kp);
    }
    let np = precompute_costs_np(&kp);
    codes.iter().map(|l| code_complexity(&np, l)).sum()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("../input")?;
    let codes = input.lines().collect_vec();

    println!("Complexity (part 1): {}", solve(2, &codes));
    println!("Complexity (part 2): {}", solve(25, &codes));
    Ok(())
}
