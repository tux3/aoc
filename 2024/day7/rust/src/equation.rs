use crate::op::Op;

pub struct Equation {
    pub target: usize,
    pub terms: Vec<usize>,
}

impl Equation {
    pub fn from_line(l: &str) -> Self {
        let (target, terms) = l.split_once(':').unwrap();
        Self {
            target: target.parse().unwrap(),
            terms: terms
                .trim()
                .split(' ')
                .map(|s| s.parse().unwrap())
                .collect(),
        }
    }

    fn solve_rec(&self, cur: usize, terms: &[usize], ops: &[Op]) -> bool {
        let Some((&term, terms)) = terms.split_first() else {
            return cur == self.target;
        };
        ops.iter().any(|op| {
            let val = op(cur, term);
            val <= self.target && self.solve_rec(val, terms, ops)
        })
    }

    pub fn solvable(&self, ops: &[Op]) -> bool {
        self.solve_rec(self.terms[0], &self.terms[1..], ops)
    }
}
