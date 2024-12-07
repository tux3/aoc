#[derive(Debug, Copy, Clone)]
pub enum Op {
    Add,
    Mul,
    Cat,
}

impl FnOnce<(usize, usize)> for Op {
    type Output = usize;
    extern "rust-call" fn call_once(self, (a, b): (usize, usize)) -> Self::Output {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
            Op::Cat => b + a * 10usize.pow(b.checked_ilog10().unwrap_or(0) + 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_cat() {
        assert_eq!(Op::Cat(0, 1), 1);
        assert_eq!(Op::Cat(2, 0), 20);
        assert_eq!(Op::Cat(1, 2), 12);
        assert_eq!(Op::Cat(12, 345), 12345);
    }
}
