pub type Key = Vec<usize>;
pub type Lock = Vec<usize>;

fn parse_lock(schema: &str) -> Key {
    let mut heights = vec![0, 0, 0, 0, 0];
    for l in schema.lines().skip(1) {
        for (i, c) in l.chars().enumerate() {
            if c == '#' {
                heights[i] += 1
            }
        }
    }
    heights
}

fn parse_key(schema: &str) -> Key {
    let mut heights = vec![0, 0, 0, 0, 0];
    for l in schema.lines().rev().skip(1) {
        for (i, c) in l.chars().enumerate() {
            if c == '#' {
                heights[i] += 1
            }
        }
    }
    heights
}

pub fn parse(input: &str) -> (Vec<Key>, Vec<Lock>) {
    let mut keys = vec![];
    let mut locks = vec![];
    for schema in input.split("\n\n") {
        if schema.starts_with("#####") {
            locks.push(parse_lock(schema))
        } else  {
            keys.push(parse_key(schema))
        }
    }
    (keys, locks)
}
