use anyhow::Result;
use std::path::Path;

pub struct Report {
    levels: Vec<i32>,
}

impl Report {
    pub fn new(levels: Vec<i32>) -> Self {
        Self { levels }
    }

    // If equally balanced, we return false (either would work)
    fn is_mainly_ascending(&self) -> bool {
        let mut num_inc = 0;
        let mut num_dec = 0;

        for [prev, cur] in self.levels.array_windows() {
            if cur > prev {
                num_inc += 1
            } else if prev > cur {
                num_dec += 1
            }
        }
        num_inc > num_dec
    }

    fn is_safe_step(ascending: bool, a: i32, b: i32) -> bool {
        if a == b || (ascending && a > b) || (!ascending && b > a) {
            false
        } else if (b - a).abs() > 3 {
            false
        } else {
            true
        }
    }

    // If unsafety is > 1, we always return 2
    // This lets us compute the unsafety in a single iteration,
    // with O(1) storage and without mutating or copying the report
    fn unsafe_levels(&self) -> u32 {
        let levels = &self.levels;
        let is_asc = self.is_mainly_ascending();

        // We "removed" the current level
        let mut skip_cur = false;
        // "Removing" the previous level would be OK
        let mut prev_next_was_safe = true;
        // We only have enough lookahead to track unsafety < 2,
        // if it would be >= 2 we just return early, it doesn't make a difference for us
        let mut unsafety = 0;

        for [prev, cur, next] in levels.array_windows().copied() {
            if skip_cur {
                skip_cur = false;
                prev_next_was_safe = Self::is_safe_step(is_asc, prev, next);
                continue;
            }

            if prev == cur {
                // Simple case, it doesn't matter which one we skip
                unsafety += 1
            } else if !Self::is_safe_step(is_asc, prev, cur) {
                if Self::is_safe_step(is_asc, prev, next) {
                    // Removing cur leaves prev/next OK, so we can do it and skip next iteration
                    unsafety += 1;
                    skip_cur = true;
                } else if Self::is_safe_step(is_asc, cur, next) {
                    // I can't remove cur, and removing prev is fine ONLY if:
                    // - We are at the start,
                    // - We saw prev/next would have been safe last iteration (now prevprev/cur)
                    if !prev_next_was_safe {
                        return 2;
                    }
                    unsafety += 1;
                } else {
                    // Removing cur or prev isn't enough, badness > 1
                    return 2;
                }
            }

            prev_next_was_safe = Self::is_safe_step(is_asc, prev, next);
        }

        // The last element doesn't have a full window, so handle it manually
        if skip_cur {
            // We "removed" -2 (the second to last level), so we must have checked this already
            assert!(Self::is_safe_step(
                is_asc,
                levels[levels.len() - 3],
                levels[levels.len() - 1]
            ))
        } else if !Self::is_safe_step(is_asc, levels[levels.len() - 2], levels[levels.len() - 1]) {
            // We did not skip -2, so if -2/-1 are unsafe, we must account for it now
            unsafety += 1
        }

        unsafety
    }

    pub fn is_safe(&self) -> bool {
        self.unsafe_levels() == 0
    }

    pub fn is_safe_dampened(&self) -> bool {
        self.unsafe_levels() <= 1
    }
}

pub fn parse_reports_file(path: impl AsRef<Path>) -> Result<Vec<Report>> {
    parse_reports_str(&std::fs::read_to_string(path)?)
}

fn parse_reports_str(input: &str) -> Result<Vec<Report>> {
    Ok(input
        .lines()
        .filter(|&l| !l.is_empty())
        .map(|line| Report::new(line.split(' ').map(|s| s.parse().unwrap()).collect()))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe() -> Result<()> {
        assert_eq!(parse_reports_str("1 2 3")?[0].unsafe_levels(), 0);
        assert_eq!(parse_reports_str("3 2 1")?[0].unsafe_levels(), 0);
        assert_eq!(parse_reports_str("1 3 4 6")?[0].unsafe_levels(), 0);
        assert_eq!(parse_reports_str("6 4 3 1")?[0].unsafe_levels(), 0);

        assert_eq!(parse_reports_str("0 3 6")?[0].unsafe_levels(), 0);
        assert_eq!(parse_reports_str("6 3 0")?[0].unsafe_levels(), 0);
        Ok(())
    }

    #[test]
    fn damp_safe() -> Result<()> {
        assert_eq!(parse_reports_str("1 2 3 3")?[0].unsafe_levels(), 1);
        assert_eq!(parse_reports_str("1 2 2 3")?[0].unsafe_levels(), 1);
        assert_eq!(parse_reports_str("1 1 2 3")?[0].unsafe_levels(), 1);

        assert_eq!(parse_reports_str("1 2 3 99")?[0].unsafe_levels(), 1);
        assert_eq!(parse_reports_str("1 2 99 3")?[0].unsafe_levels(), 1);
        assert_eq!(parse_reports_str("99 1 2 3")?[0].unsafe_levels(), 1);

        assert_eq!(parse_reports_str("1 4 7 5 7 10")?[0].unsafe_levels(), 1);
        assert_eq!(parse_reports_str("1 4 7 5 6 9")?[0].unsafe_levels(), 1);
        Ok(())
    }

    #[test]
    fn not_safe() -> Result<()> {
        assert_eq!(parse_reports_str("1 2 3 2 1")?[0].unsafe_levels(), 2);
        assert_eq!(parse_reports_str("1 2 3 10 11 12")?[0].unsafe_levels(), 2);
        assert_eq!(parse_reports_str("0 4 8")?[0].unsafe_levels(), 2);
        Ok(())
    }
}
