/* A numpad:
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+
*/

use crate::keypad::KeyPad;
use crate::keypad::KeyPadKey::{A, D, L, R, U};
use NumPadKey::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::iter::repeat_n;

#[repr(u8)]
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum NumPadKey {
    KA,
    K0,
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
    K9,
}

impl NumPadKey {
    pub fn from_char(c: char) -> Self {
        match c {
            'A' => KA,
            '0' => K0,
            '1' => K1,
            '2' => K2,
            '3' => K3,
            '4' => K4,
            '5' => K5,
            '6' => K6,
            '7' => K7,
            '8' => K8,
            '9' => K9,
            _ => panic!("Invalid numpad char {c}"),
        }
    }

    pub fn coord(self) -> (usize, usize) {
        match self {
            KA => (2, 3),
            K0 => (1, 3),
            K1 => (0, 2),
            K2 => (1, 2),
            K3 => (2, 2),
            K4 => (0, 1),
            K5 => (1, 1),
            K6 => (2, 1),
            K7 => (0, 0),
            K8 => (1, 0),
            K9 => (2, 0),
        }
    }
}

pub struct NumPad {
    // Map (from, to) to the cost of pressing `to` from `from`
    // The "parent" keypad always rests on A after each press of this one,
    // but this one could be trying to go from anywhere to anywhere
    pub costs: HashMap<(NumPadKey, NumPadKey), usize>,
}

impl NumPad {
    pub fn code_cost(&self, code: &str) -> usize {
        let code_keys = code.chars().map(NumPadKey::from_char);
        let (_, cost) = code_keys.fold((KA, 0), |(from, cost), to| {
            (to, cost + self.costs[&(from, to)])
        });
        cost
    }
}

// Cost to go from a to b on NP
fn np_cost(a: NumPadKey, b: NumPadKey, kp2: &KeyPad) -> usize {
    let ((ax, ay), (bx, by)) = (a.coord(), b.coord());
    let (xdist, ydist) = (usize::abs_diff(ax, bx), usize::abs_diff(ay, by));
    // There's really two possible paths, move on the X axis first, or the Y axis first
    // The "mixed" paths that alternate back and forth clearly require more movement
    // Although the hole at (0, 3) is a forbidden path
    let xkey = if bx >= ax { R } else { L };
    let ykey = if by >= ay { D } else { U };
    let pathx = repeat_n(xkey, xdist)
        .chain(repeat_n(ykey, ydist))
        .chain(std::iter::once(A));
    let pathy = repeat_n(ykey, ydist)
        .chain(repeat_n(xkey, xdist))
        .chain(std::iter::once(A));
    let (_, costx) = pathx.fold((A, 0), |(from, cost), to| {
        (to, cost + kp2.costs[&(from, to)])
    });
    let (_, costy) = pathy.fold((A, 0), |(from, cost), to| {
        (to, cost + kp2.costs[&(from, to)])
    });

    // This time two paths are not always equal cost. We must avoid the hole.
    // If we'd move left into the hole, we must move up first
    if bx == 0 && ay == 3 {
        return costy;
    }
    // If we'd move down into the hole, we must move right first
    if by == 3 && ax == 0 {
        return costx;
    }

    usize::min(costx, costy)
}

pub fn precompute_costs_np(kp2: &KeyPad) -> NumPad {
    let npk = [KA, K0, K1, K2, K3, K4, K5, K6, K7, K8, K9];
    let costs = npk
        .into_iter()
        .cartesian_product(npk)
        .map(|(a, b)| ((a, b), np_cost(a, b, kp2)))
        .collect();
    NumPad { costs }
}
