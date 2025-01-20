use bitvec::prelude::*;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    u8,
};

use rand::Rng;

advent_of_code::solution!(24);

#[derive(Debug, Clone)]
struct Gate {
    a: GateInput,
    b: GateInput,
    out: String,
    op: Op,
}

#[derive(Debug, Clone, Hash)]
enum GateInput {
    GateName(String),
    InputRef(InputRef),
}
#[derive(Debug)]
struct ParseGateInputError;

impl FromStr for GateInput {
    type Err = ParseGateInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Check if it starts with 'x' or 'y' and has exactly 3 characters
        if s.len() == 3 && (s.starts_with('x') || s.starts_with('y')) {
            if let Some(ir) = parse_input_ref(s) {
                return Ok(GateInput::InputRef(ir));
            }
            // Get the first character ('x' or 'y')
            return Err(ParseGateInputError);
        }
        // If it doesn't match InputRef pattern, treat it as a GateName
        Ok(GateInput::GateName(s.to_string()))
    }
}

#[derive(Debug, Clone, Hash)]
struct InputRef {
    c: char,
    nbits: usize,
}

fn parse_input_ref(s: &str) -> Option<InputRef> {
    let c = s.chars().next().unwrap();
    // Parse the two digits
    if let Ok(nbits) = s[1..].parse::<usize>() {
        if nbits < 100 {
            // Ensure it's two digits
            return Some(InputRef { c, nbits });
        }
    }
    return None;
}

struct Inputs {
    x: usize,
    y: usize,
}

impl Inputs {
    fn read(&self, iref: &InputRef) -> bool {
        match iref.c {
            'x' => (self.x & (1 << iref.nbits)) != 0,
            'y' => (self.y & (1 << iref.nbits)) != 0,
            _ => unreachable!("invalid input ref"),
        }
    }

    fn set_bit(&mut self, iref: &InputRef, bit: u8) -> Option<usize> {
        let mask = 1usize << iref.nbits;
        match iref.c {
            'x' => {
                self.x = (self.x & !mask) | ((bit & 1) as usize) << iref.nbits;
                Some(self.x)
            }
            'y' => {
                self.y = (self.y & !mask) | ((bit & 1) as usize) << iref.nbits;
                Some(self.y)
            }
            _ => None,
        }
    }
}

fn execute_gate(
    start: &Gate,
    connectivity: &HashMap<String, Gate>,
    inputs: &Inputs,
    outputs: &mut HashMap<String, bool>,
    depth: usize,
) -> (bool, HashSet<(String, usize)>) {
    let mut visited = HashSet::new();
    if depth > 50 {
        return (false, visited);
    }
    visited.insert((start.out.to_owned(), depth));
    if let Some(val) = outputs.get(&start.out) {
        return (val.clone(), visited);
    }

    let a = match &start.a {
        GateInput::InputRef(ir) => inputs.read(&ir),
        GateInput::GateName(name) => {
            let (val, sub_graph) = execute_gate(
                &connectivity[name],
                connectivity,
                inputs,
                outputs,
                depth + 1,
            );
            visited.extend(sub_graph);
            val
        }
    };

    let b = match &start.b {
        GateInput::InputRef(ir) => inputs.read(&ir),
        GateInput::GateName(name) => {
            let (val, sub_graph) = execute_gate(
                &connectivity[name],
                connectivity,
                inputs,
                outputs,
                depth + 1,
            );
            visited.extend(sub_graph);
            val
        }
    };

    let ret = match start.op {
        Op::AND => a & b,
        Op::OR => a | b,
        Op::XOR => a ^ b,
    };
    outputs.insert(start.out.to_string(), ret);
    (ret, visited)
}

#[derive(Debug, Clone)]
enum Op {
    AND,
    OR,
    XOR,
}

pub fn part_one(input: &str) -> Option<usize> {
    let gates: Vec<_> = (0..=45).map(|i| format!("z{:0>2}", i)).collect();
    part_one_from_gate(input, &gates)
}

pub fn part_one_from_gate(input: &str, gates: &[String]) -> Option<usize> {
    let (inputs, connectivity) = parse_input(input);
    let mut outputs = HashMap::new();

    for gate in gates {
        let last_gate = connectivity.get(gate.as_str()).unwrap();
        execute_gate(last_gate, &connectivity, &inputs, &mut outputs, 0);
    }
    let mut result = 0;
    let bits: Vec<_> = outputs
        .keys()
        .filter(|k| k.starts_with("z"))
        .sorted()
        .collect();

    for (i, &b) in bits.iter().enumerate() {
        result += (outputs[b] as usize) << i;
    }
    println!("result: 0b{:b}", result);
    Some(result)
}

fn test_many_ints(input: &str) -> Option<Vec<String>> {
    let (_, connectivity) = parse_input(input);
    let mut freq_bad_gate = HashMap::new();
    for _ in 0..1000 {
        let inputs = Inputs {
            x: rand::thread_rng().gen_range(1000000000000..6000000000000),
            y: rand::thread_rng().gen_range(1000000000000..6000000000000),
        };
        let (result, outputs) = check_add(&inputs, &connectivity);
        let expected = inputs.x + inputs.y;
        // println!("result:   0b{:b}", result);
        // println!("expected: 0b{:b}", expected);

        let bad_gates: Vec<_> = (expected ^ result)
            .view_bits::<Lsb0>()
            .iter_ones()
            .map(|i| format!("z{:0>2}", i))
            .collect();
        // println!("{}", &bad_gates.iter().join(":"));
        for g in bad_gates {
            *freq_bad_gate.entry(g).or_insert(1) += 1;
        }
    }

    let mut it = freq_bad_gate.iter().sorted_by(|a, b| a.1.cmp(b.1));
    let worst = it.next().unwrap();

    return None;
}

pub fn part_two(input: &str) -> Option<String> {
    test_many_ints(input);
    None
}
fn part_two_real(input: &str) -> Option<String> {
    let (_, connectivity) = parse_input(input);

    let inputs = Inputs {
        x: 0b11111111111111111111111111111111111111111111,
        y: 0b10101010101010101010101101010101010101010101,
    };
    let (result, outputs) = check_add(&inputs, &connectivity);
    let expected = inputs.x + inputs.y;
    println!("result:   0b{:b}", result);
    println!("expected: 0b{:b}", expected);

    let bad_gates: Vec<_> = (expected ^ result)
        .view_bits::<Lsb0>()
        .iter_ones()
        .map(|i| format!("z{:0>2}", i))
        .collect();
    dbg!(&bad_gates);

    let mut remaining_candidates = HashSet::new();
    let mut all_fixer = Vec::new();
    for g in bad_gates {
        let z_xx = connectivity.get(g.as_str()).unwrap();
        let mut empty = HashMap::new();
        let (bad_zxx, visited) = execute_gate(z_xx, &connectivity, &inputs, &mut empty, 0);
        let candidates = visited
            .into_iter()
            .filter(|(_, depth)| *depth < 4)
            .collect::<HashSet<_>>();
        dbg!(&candidates);

        let mut fixer = Vec::new();
        for c in candidates
            .clone()
            .into_iter()
            .sorted_by(|(_, a), (_, b)| a.cmp(b))
            .combinations(2)
        {
            let a = c[0].0.as_str();
            let b = c[1].0.as_str();
            let conn = match swap_gates(&connectivity, &[vec![a.to_string(), b.to_string()]]) {
                None => continue,
                Some(conn) => conn,
            };
            let (zxx_again, visited) = execute_gate(z_xx, &conn, &inputs, &mut HashMap::new(), 0);
            if zxx_again != bad_zxx {
                fixer.push(vec![a.to_string(), b.to_string()])
            }
        }
        if fixer.len() == 0 {
            remaining_candidates.extend(candidates)
        } else {
            all_fixer.extend(fixer);
        }
    }

    let swaps: Vec<_> = remaining_candidates
        .iter()
        .map(|(g, _)| g.clone())
        .combinations(2)
        .collect();

    all_fixer.extend(swaps);

    dbg!(&all_fixer);

    let to_check = &all_fixer
        .into_iter()
        .combinations(4)
        .filter(|swaps| {
            let mut seen = HashSet::new();
            for s in swaps.clone().into_iter().flatten() {
                seen.insert(s);
            }
            seen.len() == 4 * 2
        })
        .collect::<Vec<_>>();

    for c in to_check.clone() {
        println!("Checking {:#?}", &c);
        let conn = match swap_gates(&connectivity, &c) {
            None => continue,
            Some(conn) => conn,
        };

        let (result, outputs) = check_add(&inputs, &conn);
        let expected = inputs.x + inputs.y;
        if expected == result {
            println!("Found them {:#?}", &c);
            return Some(c.iter().flatten().sorted().join(","));
        }
    }
    return None;
}

// We know we never call this with sets of swaps that touch the same gate twice.
fn swap_gates(old: &HashMap<String, Gate>, swaps: &[Vec<String>]) -> Option<HashMap<String, Gate>> {
    let mut conn = old.clone();
    for swap in swaps {
        let a = swap[0].clone();
        let b = swap[1].clone();
        let mut ga = conn.remove(a.as_str()).unwrap();
        let mut gb = conn.remove(b.as_str()).unwrap();

        ga.out = b.to_string();
        if let GateInput::GateName(some) = ga.a.clone() {
            if some == ga.out {
                return None;
            }
        }
        if let GateInput::GateName(some) = ga.b.clone() {
            if some == ga.out {
                return None;
            }
        }
        gb.out = a.to_string();
        if let GateInput::GateName(some) = gb.a.clone() {
            if some == gb.out {
                return None;
            }
        }
        if let GateInput::GateName(some) = gb.b.clone() {
            if some == gb.out {
                return None;
            }
        }

        conn.insert(b.to_string(), ga);
        conn.insert(a.to_string(), gb);
    }
    Some(conn)
}

fn check_add(
    inputs: &Inputs,
    connectivity: &HashMap<String, Gate>,
) -> (usize, HashMap<String, bool>) {
    let mut gates: Vec<_> = connectivity
        .clone()
        .into_iter()
        .filter(|(k, _)| k.starts_with("z"))
        .map(|(_, v)| v)
        .collect();

    gates.sort_by(|a, b| a.out.cmp(&b.out));

    let mut outputs = HashMap::new();

    for gate in gates.iter() {
        execute_gate(gate, &connectivity, &inputs, &mut outputs, 0);
    }

    let mut result = 0;
    let bits: Vec<_> = outputs
        .keys()
        .filter(|k| k.starts_with("z"))
        .sorted()
        .collect();

    for (i, &b) in bits.iter().enumerate() {
        result += (outputs[b] as usize) << i;
    }

    return (result, outputs);
}

fn parse_input(input: &str) -> (Inputs, HashMap<String, Gate>) {
    let mut it = input.split("\n\n");
    let inits: Vec<_> = it
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut i = line.trim().split(": ");
            let k = parse_input_ref(i.next().unwrap()).unwrap();
            let v = i.next().unwrap().parse::<u8>().unwrap();
            (k, v)
        })
        .collect();
    let mut inputs = Inputs { x: 0, y: 0 };
    for (iref, bit) in inits {
        let _ = inputs.set_bit(&iref, bit).unwrap();
        // println!(
        //     "setting {}{:0>2} to {} : {}",
        //     iref.c, iref.nbits, bit, new_val
        // );
    }
    let connectivity = it
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let elems: Vec<_> = line.trim().split_whitespace().collect();
            // x04 AND y04 -> ppw
            (
                elems[4].to_string(),
                Gate {
                    a: GateInput::from_str(elems[0]).unwrap(),
                    b: GateInput::from_str(elems[2]).unwrap(),
                    out: elems[4].to_string(),
                    op: match elems[1] {
                        "AND" => Op::AND,
                        "OR" => Op::OR,
                        "XOR" => Op::XOR,
                        _ => unreachable!("invalid input for OP"),
                    },
                },
            )
        })
        .collect();
    return (inputs, connectivity);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_from_gate(
            &advent_of_code::template::read_file("examples", DAY),
            &["z00".to_string(), "z01".to_string(), "z02".to_string()],
        );
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
        test_many_ints(&advent_of_code::template::read_file("inputs", DAY));
    }
}
