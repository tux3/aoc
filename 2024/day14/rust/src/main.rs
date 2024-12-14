use anyhow::Result;
use colored::{ColoredString, Colorize};
use std::path::Path;

#[derive(Debug)]
pub struct Bot {
    pub px: isize,
    pub py: isize,
    pub vx: isize,
    pub vy: isize,
}

fn parse_line(l: &str) -> Bot {
    let (p, v) = l.split_once(" ").unwrap();
    let (px, py) = p[2..].split_once(',').unwrap();
    let (vx, vy) = v[2..].split_once(',').unwrap();
    Bot {
        px: px.parse().unwrap(),
        py: py.parse().unwrap(),
        vx: vx.parse().unwrap(),
        vy: vy.parse().unwrap(),
    }
}

fn color_num(n: usize) -> ColoredString {
    match n {
        0 => " ".normal(),
        1 => "#".green(),
        _n => "O".red(),
    }
}

fn stddev(data: &[isize]) -> f32 {
    let sum = data.iter().sum::<isize>() as f32;
    let mean = sum / data.len() as f32;
    let variance = data.iter().map(|&v| (mean - v as f32).powi(2)).sum::<f32>() / data.len() as f32;
    variance.sqrt()
}

fn solve_file(path: impl AsRef<Path>, w: isize, h: isize) -> Result<(usize, usize)> {
    let bots = std::fs::read_to_string(path)?
        .lines()
        .map(|l| parse_line(l))
        .collect::<Vec<_>>();

    const P1_STEPS: isize = 100;
    let (ul, ur, dl, dr) = bots
        .iter()
        .map(|bot| {
            let x = (bot.px + bot.vx * P1_STEPS).rem_euclid(w);
            let y = (bot.py + bot.vy * P1_STEPS).rem_euclid(h);
            (x, y)
        })
        .fold((0, 0, 0, 0), |acc, (x, y)| {
            let (l, r, u, d) = (x < w / 2, x > w / 2, y < h / 2, y > h / 2);
            (
                acc.0 + (l && u) as usize,
                acc.1 + (r && u) as usize,
                acc.2 + (l && d) as usize,
                acc.3 + (r && d) as usize,
            )
        });
    let safety = ul * ur * dl * dr;

    for i in 0.. {
        let mut grid = vec![];
        let mut xs = vec![];
        let mut ys = vec![];
        grid.resize(w as usize * h as usize, 0);
        for bot in &bots {
            let x = (bot.px + bot.vx * i).rem_euclid(w);
            let y = (bot.py + bot.vy * i).rem_euclid(h);
            xs.push(x);
            ys.push(y);
            grid[(y * w + x) as usize] += 1;
        }

        let stddev = stddev(&xs) * stddev(&ys);
        if stddev < 400. {
            println!("----- STEP {i}: stddev {stddev} -----");
            for y in 0..h {
                for x in 0..w {
                    print!("{}", color_num(grid[(y * w + x) as usize]))
                }
                println!()
            }
            break;
        }
    }

    Ok((safety, 0))
}

fn main() -> Result<()> {
    println!("Result (input): {:?}", solve_file("../input", 101, 103)?);
    Ok(())
}
