#![feature(array_windows)]

use anyhow::Result;
use hashbrown::HashMap;
use rayon::prelude::*;

fn step(s: usize) -> usize {
    let p = 16777216;
    let s = (s * 64 ^ s) % p;
    let s = (s / 32 ^ s) % p;
    let s = (s * 2048 ^ s) % p;
    s
}

fn forward(mut s: usize, steps: usize) -> usize {
    for _ in 0..steps {
        s = step(s);
    }
    s
}

fn forward_prices(mut s: usize, steps: usize) -> Vec<usize> {
    let mut prices = vec![s % 10];
    for _ in 0..steps {
        s = step(s);
        prices.push(s % 10);
    }
    prices
}

// Iterating over windows of 4 isizes is kind slow,
// let's just map the sequences of 4 diffs into a single "sequence number"
type Seqnum = u32;

fn seq_to_seqnum(seq: [isize; 4]) -> Seqnum {
    let seq = seq.map(|n| (n + 10) as Seqnum);
    seq[0] * 1 + seq[1] * 20 + seq[2] * 400 + seq[3] * 8000
}

fn seqnum_to_seq(seqnum: Seqnum) -> [isize; 4] {
    let seq = [
        seqnum / 1 % 20,
        (seqnum / 20) % 20,
        (seqnum / 400) % 20,
        (seqnum / 8000) % 20,
    ];
    seq.map(|n| n as isize - 10)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("../input")?
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let p1sum = input.iter().map(|&s| forward(s, 2000)).sum::<usize>();
    println!("Part 1 sum: {p1sum}");

    let prices = input
        .iter()
        .map(|&s| forward_prices(s, 2000))
        .collect::<Vec<_>>();
    let price_diffs = prices
        .iter()
        .map(|prices| {
            prices
                .array_windows()
                .map(|[a, b]| *b as isize - *a as isize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let price_diffs_seqnums = price_diffs
        .iter()
        .map(|diffs| {
            diffs
                .array_windows::<4>()
                .map(|&s| seq_to_seqnum(s))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let partial_scores = prices
        .par_iter()
        .zip(&price_diffs_seqnums)
        .map(|(prices, seqnums)| {
            let mut partial_score = HashMap::<Seqnum, usize>::new();
            for (i, seqnum) in seqnums.iter().enumerate() {
                partial_score
                    .entry(*seqnum)
                    .or_insert_with(|| prices[i + 4]);
            }
            partial_score
        })
        .collect::<Vec<_>>();

    let mut unique_seqnums = price_diffs_seqnums
        .iter()
        .flatten()
        .copied()
        .collect::<Vec<Seqnum>>();
    unique_seqnums.sort_unstable();
    unique_seqnums.dedup();

    let (&best_seqnum, best_score) = unique_seqnums
        .par_iter()
        .map(|seqnum| {
            (
                seqnum,
                partial_scores
                    .iter()
                    .map(|pscores| pscores.get(seqnum).copied().unwrap_or(0))
                    .sum::<usize>(),
            )
        })
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();

    println!(
        "Best seq: {:?}, best score: {best_score}",
        seqnum_to_seq(best_seqnum)
    );
    Ok(())
}
