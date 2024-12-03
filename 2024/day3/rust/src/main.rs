#![feature(substr_range)]

use anyhow::Result;

fn read_num_and_advance(slice: &mut &str, until: char) -> Option<usize> {
    let Some(until_pos) = slice.find(until) else {
        *slice = &slice[1..];
        return None;
    };
    let Some(num) = slice[..until_pos].parse().ok() else {
        *slice = &slice[1..];
        return None;
    };
    *slice = &slice[until_pos + 1..];
    Some(num)
}

fn is_enabled(start: &str, cur: &str) -> bool {
    let prev = &start[..start.substr_range(cur).unwrap().start];
    let Some(last_dont) = prev.rfind("don't()") else {
        return true;
    };
    let Some(last_do) = prev.rfind("do()") else {
        return false;
    };
    last_do > last_dont
}

fn compute_mul(mut input: &str, parse_ctrl: bool) -> usize {
    let orig_input = input;
    let mut count = 0;
    loop {
        let Some(next_mul) = input.find("mul(") else {
            break;
        };
        if input[next_mul..].len() < "mul(0,0)".len() {
            break;
        }
        input = &input[next_mul + 4..];

        let Some(n1) = read_num_and_advance(&mut input, ',') else {
            continue;
        };
        let Some(n2) = read_num_and_advance(&mut input, ')') else {
            continue;
        };
        if !parse_ctrl || is_enabled(orig_input, input) {
            count += n1 * n2;
        }
    }
    count
}

fn main() -> Result<()> {
    println!(
        "Mul (input-test): {}",
        compute_mul(&std::fs::read_to_string("../input-test")?, false)
    );
    println!(
        "Mul (input): {}",
        compute_mul(&std::fs::read_to_string("../input")?, false)
    );

    println!(
        "Mul with control flow (input-test2): {}",
        compute_mul(&std::fs::read_to_string("../input-test2")?, true)
    );
    println!(
        "Mul with control flow (input-test2): {}",
        compute_mul(&std::fs::read_to_string("../input")?, true)
    );
    Ok(())
}
