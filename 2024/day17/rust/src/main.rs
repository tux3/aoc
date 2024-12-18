#![feature(let_chains)]

mod op;

use crate::op::State;
use anyhow::Result;

fn parse_state(input: String) -> State {
    let (regs_str, program_str) = input.split_once("\n\n").unwrap();
    let regs: Vec<usize> = regs_str
        .lines()
        .map(|s| s.split_once(": ").unwrap().1.parse::<usize>().unwrap())
        .collect();
    let code: Vec<u8> = program_str
        .trim_end()
        .split_once(": ")
        .unwrap()
        .1
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();
    State {
        code,
        regs,
        ip: 0,
        out: vec![],
    }
}

fn solve_p1(mut state: State) -> String {
    state.regs[0] = 117440;

    while let Some((instr, op)) = state.fetch() {
        state.exec(instr, op);
    }

    itertools::join(state.out, ",")
}

// This is a bit of an ugly bruteforce, didn't bother solving any equations this time! ^^'
fn solve_p2_rec(
    state: &mut State,
    pos: usize,
    mut best: Option<usize>,
    input: usize,
) -> Option<usize> {
    if (pos + 1) * 4 > state.code.len() {
        return Some(input);
    }
    for i in 0..65536 {
        let new_input = input + (i << (pos * 12));
        if let Some(b) = best
            && new_input > b
        {
            break;
        }

        state.reset();
        state.regs[0] = new_input;
        while let Some((instr, op)) = state.fetch() {
            state.exec(instr, op);
        }
        if state.out.len() < 4 * (pos + 1) {
            continue;
        }
        if &state.out[..4 * (pos + 1)] == &state.code[..4 * (pos + 1)] {
            if let Some(solution) = solve_p2_rec(state, pos + 1, best, new_input) {
                best = Some(best.unwrap_or(solution).min(solution));
            }
        }
    }
    best
}

fn solve_p2(mut state: State) -> usize {
    let input = solve_p2_rec(&mut state.clone(), 0, None, 0).unwrap();

    state.reset();
    state.regs[0] = input;
    while let Some((instr, op)) = state.fetch() {
        state.exec(instr, op);
    }
    println!("Program code:    {}", itertools::join(state.code, ","));
    println!("Program output:  {}", itertools::join(state.out, ","));

    input
}

fn main() -> Result<()> {
    let state = parse_state(std::fs::read_to_string("../input")?);
    println!("Result for part 1: {}", solve_p1(state.clone()));
    println!("Result for part 2: {}", solve_p2(state.clone()));
    Ok(())
}
