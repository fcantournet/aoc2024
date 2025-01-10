use std::{collections::HashMap, u8};

use itertools::Itertools;

advent_of_code::solution!(24);

#[derive(Debug, Clone)]
struct Gate<'a> {
    a: &'a str,
    b: &'a str,
    out: &'a str,
    op: Op,
}

fn execute_gate<'a>(
    start: &'a Gate,
    connectivity: &'a HashMap<&'a str, Gate<'a>>,
    inputs: &HashMap<&str, bool>,
    outputs: &mut HashMap<&'a str, bool>,
) -> bool {
    if let Some(val) = outputs.get(start.out) {
        return val.clone();
    }
    let a = if let Some(val) = inputs.get(start.a) {
        val.clone()
    } else {
        execute_gate(&connectivity[start.a], connectivity, inputs, outputs)
    };
    let b = if let Some(val) = inputs.get(start.b) {
        val.clone()
    } else {
        execute_gate(&connectivity[start.b], connectivity, inputs, outputs)
    };

    let ret = match start.op {
        Op::AND => a & b,
        Op::OR => a | b,
        Op::XOR => a ^ b,
    };
    outputs.insert(start.out, ret);
    dbg!(outputs);
    ret
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

    dbg!(gates);
    for gate in gates {
        let last_gate = connectivity.get(gate.as_str()).unwrap();
        execute_gate(last_gate, &connectivity, &inputs, &mut outputs);
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
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn parse_input(input: &str) -> (HashMap<&str, bool>, HashMap<&str, Gate>) {
    let mut it = input.split("\n\n");
    let inits: HashMap<_, _> = it
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut i = line.trim().split(": ");
            let k = i.next().unwrap();
            let v = match i.next().unwrap().parse::<u8>().unwrap() {
                0 => false,
                1 => true,
                _ => unreachable!(),
            };
            (k, v)
        })
        .collect();

    let connectivity = it
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let elems: Vec<_> = line.trim().split_whitespace().collect();
            // x04 AND y04 -> ppw
            (
                elems[4],
                Gate {
                    a: elems[0],
                    b: elems[2],
                    out: elems[4],
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
    return (inits, connectivity);
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
