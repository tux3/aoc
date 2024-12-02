#![feature(array_windows)]

use anyhow::Result;

mod report;
use report::*;

pub fn num_safe(reports: Vec<Report>) -> u32 {
    reports
        .iter()
        .map(|r| if r.is_safe() { 1 } else { 0 })
        .sum()
}

pub fn num_safe_dampened(reports: Vec<Report>) -> u32 {
    reports
        .iter()
        .map(|r| if r.is_safe_dampened() { 1 } else { 0 })
        .sum()
}

fn main() -> Result<()> {
    println!(
        "Safe count (test): {}",
        num_safe(parse_reports_file("../input-test")?)
    );
    println!(
        "Safe count (input): {}",
        num_safe(parse_reports_file("../input")?)
    );

    println!(
        "Dampened safe count (test): {}",
        num_safe_dampened(parse_reports_file("../input-test")?)
    );
    println!(
        "Dampened safe count (input): {}",
        num_safe_dampened(parse_reports_file("../input")?)
    );

    Ok(())
}
