use anyhow::Result;
use genawaiter::{rc::r#gen, yield_};
use std::path::Path;

/// Assumed to be a square grid
pub struct Grid {
    lines: Vec<String>,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let lines = input
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect::<Vec<_>>();
        assert_eq!(lines.len(), lines[0].len());
        Self { lines }
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self::new(&std::fs::read_to_string(path)?))
    }

    fn build_string(&self, indexes: impl IntoIterator<Item = (usize, usize)>) -> String {
        indexes
            .into_iter()
            .map(|(i, j)| char::from(self.lines[i].as_bytes()[j]))
            .collect()
    }

    pub fn lines(&self) -> impl Iterator<Item = String> {
        self.lines.clone().into_iter()
    }

    pub fn columns(&self) -> impl Iterator<Item = String> {
        r#gen!({
            let n = self.lines.len(); // width == height
            for i in 0..n {
                yield_!(self.build_string((0..n).map(|j| (j, i))));
            }
        })
        .into_iter()
    }

    pub fn diagonals(&self) -> impl Iterator<Item = String> {
        r#gen!({
            let n = self.lines.len(); // width == height
            for i in 0..(n - 3) {
                let it = (0..(n - i));
                yield_!(self.build_string(it.clone().map(|j| (i + j, j))));
                yield_!(self.build_string(it.clone().map(|j| (n - 1 - (i + j), j))));
                if i > 0 {
                    yield_!(self.build_string(it.clone().map(|j| (j, i + j))));
                    yield_!(self.build_string(it.clone().map(|j| (n - 1 - j, i + j))));
                    continue;
                }
            }
        })
        .into_iter()
    }

    // Returns (a, b) strings for each 3x3 cross in the grid:
    // a  .  b
    // . a/b .
    // b  .  a
    pub fn crosses(&self) -> impl Iterator<Item = (String, String)> {
        r#gen!({
            let n = self.lines.len(); // width == height
            for i in 0..(n - 2) {
                for j in 0..(n - 2) {
                    let downward = self.build_string((0..3).map(|k| (i + k, j + k)));
                    let upward = self.build_string((0..3).map(|k| (i + 2 - k, j + k)));
                    yield_!((downward, upward));
                }
            }
        })
        .into_iter()
    }
}
