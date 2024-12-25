use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, Clone)]
enum Net {
    Value(bool),
    Gate(Gate),
}

impl Display for Net {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Net::Value(b) => write!(f, "{}", *b as usize),
            Net::Gate(g) => write!(f, "{} {} {}", g.l, g.op, g.r),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Op {
    And,
    Or,
    Xor
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::And => f.write_str("AND"),
            Op::Or => f.write_str("OR"),
            Op::Xor => f.write_str("XOR"),
        }
    }
}

#[derive(Debug, Clone)]
struct Gate {
    l: String,
    r: String,
    op: Op,
}

fn eval_net(netlist: &mut HashMap<String, Net>, name: &str) -> bool {
    let gate: Gate = match &netlist[name] {
        Net::Value(b) => return *b,
        Net::Gate(g) => g.clone(),
    };
    let l = eval_net(netlist, &gate.l);
    let r = eval_net(netlist, &gate.r);
    let val = match gate.op {
        Op::And => l && r,
        Op::Or => l || r,
        Op::Xor => l ^ r,
    };
    netlist.insert(name.to_owned(), Net::Value(val));
    val
}

fn eval_output(netlist: &mut HashMap<String, Net>) -> u64 {
    let mut z_num = 0;
    let mut z_gates = netlist.keys().filter(|k| k.starts_with('z')).cloned().collect::<Vec<_>>();
    z_gates.sort();
    for (i, z) in z_gates.iter().enumerate() {
        let val = eval_net(netlist, &z) as u64;
        z_num |= val << i;
    }
    z_num
}


fn find_gate(netlist: &HashMap<String, Net>, l: &str, op: Op, r: &str) -> Option<String> {
    for (name, net) in netlist {
        let Net::Gate(gate) = net else {
            continue
        };
        if gate.op == op && ((gate.l == l && gate.r == r) || (gate.l == r && gate.r == l)) {
            return Some(name.clone())
        }
    }
    println!("Net {l} {op} {r} not found!");
    None
}

fn netlist_swap(netlist: &mut HashMap<String, Net>, a: String, b: String) {
    let net_a = netlist.remove(&a).unwrap();
    let net_b = netlist.remove(&b).unwrap();
    netlist.insert(a, net_b);
    netlist.insert(b, net_a);
}

fn find_adder_swapped_nets(mut netlist: HashMap<String, Net>) -> Vec<String> {
    use Op::*;

    let mut swapped_nets = vec![];

    // Setup base case of z00
    let z00 = find_gate(&netlist, "y00", Xor, "x00").unwrap();
    assert_eq!(z00, "z00");
    let mut ci_prev = find_gate(&netlist, "y00", And, "x00").unwrap();

    for i in 1..45 {
        // Output bit i
        let mut zi_axb = find_gate(&netlist, &format!("y{i:02}"), Xor, &format!("x{i:02}")).unwrap();
        let zi_name = format!("z{i:02}");
        let zi = match find_gate(&netlist, &ci_prev, Xor, &zi_axb) {
            Some(zi) => zi,
            None => {
                // Whatever is on wire zi doesn't match what we expected. Let's compare and fix it.
                let Net::Gate(bad_zi) = netlist.get(&zi_name).unwrap() else {
                    panic!("net {zi_name} is not a gate!")
                };
                assert_eq!(bad_zi.op, Xor);
                let (bad_arg, good_arg) = if ci_prev == bad_zi.l {
                    (bad_zi.r.clone(), &mut zi_axb)
                } else if zi_axb == bad_zi.l {
                    (bad_zi.r.clone(), &mut ci_prev)
                } else if ci_prev == bad_zi.r {
                    (bad_zi.l.clone(), &mut zi_axb)
                } else if  zi_axb == bad_zi.r {
                    (bad_zi.l.clone(), &mut ci_prev)
                } else {
                    panic!("Both input nets of {zi_name} are mismatched!")
                };
                println!("Nets {bad_arg} and {good_arg} are swapped, in arguments of {zi_name}");
                swapped_nets.push(bad_arg.clone());
                swapped_nets.push(good_arg.clone());
                netlist_swap(&mut netlist, bad_arg.clone(), good_arg.to_owned());
                *good_arg = bad_arg;
                find_gate(&netlist, &ci_prev, Xor, &zi_axb).unwrap()
            }
        };
        if zi != zi_name {
            println!("Gate {zi_name} swapped with {zi}, fixing");
            swapped_nets.push(zi_name.clone());
            swapped_nets.push(zi.clone());
            netlist_swap(&mut netlist, zi_name.clone(), zi);
        }

        // Output carry bit i
        let ci_cab = find_gate(&netlist, &ci_prev, And, &zi_axb).unwrap();
        let ci_anb = find_gate(&netlist, &format!("y{i:02}"), And, &format!("x{i:02}")).unwrap();
        let ci = find_gate(&netlist, &ci_cab, Or, &ci_anb).unwrap();

        ci_prev = ci;
    }

    // We didn't need to handle the special case of output bit 45, as it turns out!
    // It's just more of the same. Leaving it as an exercise to the reader =)

    swapped_nets.sort();
    swapped_nets
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("../input")?;
    let (drivers, gates) = input.split_once("\n\n").unwrap();
    let mut netlist: HashMap<String, Net> = drivers.lines().map(|s| {
        let (name, val) = s.split_once(": ").unwrap();
        (name.to_owned(), Net::Value(val.parse::<usize>().unwrap() == 1))
    }).collect();
    for gate_str in gates.lines() {
        let (l, op, r, _, name) = gate_str.split(' ').collect_tuple().unwrap();
        let (l, r) = (l.to_owned(), r.to_owned());
        let gate = match op {
            "AND" => Gate {l, r, op: Op::And},
            "OR" => Gate {l, r, op: Op::Or},
            "XOR" => Gate {l, r, op: Op::Xor},
            op => panic!("Unknown op {op}"),
        };
        netlist.insert(name.to_owned(), Net::Gate(gate));
    }
    println!("Part 1 output: {}", eval_output(&mut netlist.clone()));

    let swapped_nets = find_adder_swapped_nets(netlist.clone());
    println!("Part 2 wwapped nets: {}", swapped_nets.join(","));

    Ok(())
}
