use regex::Regex;

#[derive(Debug)]
pub struct Machine {
    pub xa: f64,
    pub xb: f64,
    pub ya: f64,
    pub yb: f64,
    pub x: f64,
    pub y: f64,
}

impl Machine {
    pub fn from_str(str: &str) -> Self {
        let lines = str.split("\n").collect::<Vec<_>>();
        assert_eq!(lines.len(), 3);

        let btn_reg = Regex::new(r"Button .: X\+(\d+), Y\+(\d+)").unwrap();
        let caps_a = btn_reg.captures(&lines[0]).unwrap();
        let caps_b = btn_reg.captures(&lines[1]).unwrap();
        let prize_reg = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
        let caps_prize = prize_reg.captures(&lines[2]).unwrap();

        Self {
            xa: caps_a[1].parse().unwrap(),
            xb: caps_b[1].parse().unwrap(),
            ya: caps_a[2].parse().unwrap(),
            yb: caps_b[2].parse().unwrap(),
            x: caps_prize[1].parse().unwrap(),
            y: caps_prize[2].parse().unwrap(),
        }
    }

    pub fn solve(&self) -> Option<(f64, f64)> {
        // xa*a + xb*b = x
        // ya*a + yb*b = y
        //
        // a = (x - xb*b)/xa
        // ya*x/xa + b*(yb - ya*xb/xa) = y
        // b = (y - ya*x/xa)/(yb - ya*xb/xa)
        let b = (self.y - self.ya * self.x / self.xa) / (self.yb - self.ya * self.xb / self.xa);
        let a = (self.x - self.xb * b) / self.xa;
        let (a_round, b_round) = (a.round(), b.round());
        if (a_round - a).abs() < 1e-2 && (b_round - b).abs() < 1e-2 {
            Some((a_round, b_round))
        } else {
            None
        }
    }
}
