#![feature(iter_map_windows)]

mod rules;
mod update;

use crate::rules::Rules;
use crate::update::Update;
use anyhow::Result;
use std::path::Path;

pub fn parse_file(path: impl AsRef<Path>) -> Result<(Rules, Vec<Update>)> {
    let input = std::fs::read_to_string(path.as_ref())?;
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules: Vec<(u32, u32)> = rules
        .lines()
        .map(|s| {
            let (a, b) = s.split_once('|').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();
    let updates = updates
        .lines()
        .map(|line| Update {
            pages: line.split(',').map(|e| e.parse().unwrap()).collect(),
        })
        .collect::<Vec<Update>>();
    Ok((Rules { rules }, updates))
}

fn sum_middle_pages_for_file(path: impl AsRef<Path>) -> Result<(u32, u32)> {
    let (rules, updates) = parse_file(path)?;
    let reach = rules.direct_reachability();
    let valid_sum = updates
        .iter()
        .filter(|u| u.is_valid(&reach))
        .map(Update::middle_page)
        .sum();

    let invalid_sum = updates
        .iter()
        .filter(|u| !u.is_valid(&reach))
        .map(|u| u.sorted(&reach))
        .map(|u| u.middle_page())
        .sum();
    Ok((valid_sum, invalid_sum))
}

fn main() -> Result<()> {
    println!(
        "Middle page sums (test): {:?}",
        sum_middle_pages_for_file("../input-test")?
    );
    println!(
        "Middle page sum (input): {:?}",
        sum_middle_pages_for_file("../input")?
    );

    Ok(())
}
