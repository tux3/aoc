use KeyPadKey::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::iter::repeat_n;
/* A KeyPad:
x,y +---+---+
0,0 | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+
 */

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum KeyPadKey {
    L,
    D,
    R,
    U,
    A,
}

impl KeyPadKey {
    pub fn coord(self) -> (usize, usize) {
        match self {
            L => (0, 1),
            D => (1, 1),
            R => (2, 1),
            U => (1, 0),
            A => (2, 0),
        }
    }
}

pub struct KeyPad {
    // Map (from, to) to the cost of pressing `to` from `from`
    // The "parent" keypad always rests on A after each press of this one,
    // but this one could be trying to go from anywhere to anywhere
    pub costs: HashMap<(KeyPadKey, KeyPadKey), usize>,
}

impl KeyPad {
    pub fn key_dist(a: KeyPadKey, b: KeyPadKey) -> usize {
        let ((ax, ay), (bx, by)) = (a.coord(), b.coord());
        usize::abs_diff(ax, bx) + usize::abs_diff(ay, by)
    }
}

// This is the taxicab distance of a key to another key, plus one when we press A
pub fn precompute_costs_kp1() -> KeyPad {
    let kpk = [L, D, R, U, A];
    let costs = kpk
        .into_iter()
        .cartesian_product(kpk)
        .map(|(a, b)| ((a, b), 1 + KeyPad::key_dist(a, b)))
        .collect();
    KeyPad { costs }
}

// Cost to go from a to b on a nested keypad
fn kp2_cost(a: KeyPadKey, b: KeyPadKey, kp1: &KeyPad) -> usize {
    let ((ax, ay), (bx, by)) = (a.coord(), b.coord());
    let (xdist, ydist) = (usize::abs_diff(ax, bx), usize::abs_diff(ay, by));
    // There's really two possible paths, move on the X axis first, or the Y axis first
    // The "mixed" paths that alternate back and forth clearly require more movement
    // Although the hole at (0, 0) is a forbidden path
    let xkey = if bx >= ax { R } else { L };
    let ykey = if by >= ay { D } else { U };
    let pathx = repeat_n(xkey, xdist)
        .chain(repeat_n(ykey, ydist))
        .chain(std::iter::once(A));
    let pathy = repeat_n(ykey, ydist)
        .chain(repeat_n(xkey, xdist))
        .chain(std::iter::once(A));
    let (_, costx) = pathx.fold((A, 0), |(from, cost), to| {
        (to, cost + kp1.costs[&(from, to)])
    });
    let (_, costy) = pathy.fold((A, 0), |(from, cost), to| {
        (to, cost + kp1.costs[&(from, to)])
    });

    // It turns out the two paths are NOT always equal cost in part 2! So we must avoid the hole.
    // If we'd move left into the hole, we must move up first
    if bx == 0 && ay == 0 {
        return costy;
    }
    // If we'd move up into the hole, we must move right first
    if by == 0 && ax == 0 {
        return costx;
    }

    usize::min(costx, costy)
}

pub fn precompute_costs_kp2(kp1: &KeyPad) -> KeyPad {
    let kpk = [L, D, R, U, A];
    let costs = kpk
        .into_iter()
        .cartesian_product(kpk)
        .map(|(a, b)| ((a, b), kp2_cost(a, b, kp1)))
        .collect();
    KeyPad { costs }
}
