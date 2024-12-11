pub fn split_even_digits(n: u64) -> Option<(u64, u64)> {
    let log = n.checked_ilog10().unwrap_or(0) + 1;
    if log % 2 == 0 {
        let div = 10u64.pow(log / 2);
        Some((n / div, n % div))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_num_digits() {
        assert_eq!(split_even_digits(0), None);
        assert_eq!(split_even_digits(1), None);
        assert_eq!(split_even_digits(9), None);
        assert_eq!(split_even_digits(10), Some((1, 0)));
        assert_eq!(split_even_digits(11), Some((1, 1)));
        assert_eq!(split_even_digits(100), None);
        assert_eq!(split_even_digits(101), None);
        assert_eq!(split_even_digits(1234), Some((12, 34)));
    }
}
