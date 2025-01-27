use core::fmt;
use std::{ops::Add, path::Display};

use itertools::Itertools;
use num::CheckedAdd;

advent_of_code::solution!(7);

#[derive(Debug)]
struct Problem {
    target: usize,
    operands: Vec<usize>,
}

#[derive(Debug, Clone)]
enum Ops {
    Mul,
    Add,
    Concat,
}

impl Ops {
    fn call(&self, a: usize, b: usize) -> usize {
        match self {
            Ops::Mul => a * b,
            Ops::Add => a + b,
            Ops::Concat => (10usize.pow(b.ilog10() + 1)) * a + b,
        }
    }
}

impl fmt::Display for Ops {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ops::Mul => write!(f, "*"),
            Ops::Add => write!(f, "+"),
            Ops::Concat => write!(f, "|"),
        }
    }
}

fn dbg_equations(p: &Problem, ops: Vec<&Ops>) {
    print!("testing {} = {}", p.target, p.operands[0]);
    for (i, operand) in p.operands.iter().skip(1).enumerate() {
        print!(" {} {}", ops[i], operand);
    }
    println!()
}

fn solve_fn(input: &str, ops: &[fn(usize, usize) -> usize]) -> Option<usize> {
    let problems = parse_input(input);

    let mut result = 0;
    for p in problems {
        let ops_combinations = (0..p.operands.len() - 1)
            .map(|_| ops)
            .multi_cartesian_product();
        for ops in ops_combinations {
            let mut sum = p.operands[0];
            for (i, operand) in p.operands.iter().skip(1).enumerate() {
                sum = ops[i](sum, *operand);
            }

            if sum == p.target {
                // dbg_equations(&p, ops);
                result += p.target;
                break;
            }
        }
    }
    Some(result)
}

fn solve(input: &str, ops: &[Ops]) -> Option<usize> {
    let problems = parse_input(input);

    let mut result = 0;
    for p in problems {
        let ops_combinations = (0..p.operands.len() - 1)
            .map(|_| ops)
            .multi_cartesian_product();
        for ops in ops_combinations {
            let mut sum = p.operands[0];
            for (i, operand) in p.operands.iter().skip(1).enumerate() {
                sum = ops[i].call(sum, *operand);
            }

            if sum == p.target {
                // dbg_equations(&p, ops);
                result += p.target;
                break;
            }
        }
    }
    Some(result)
}

pub fn part_one(input: &str) -> Option<usize> {
    let ops = [usize::wrapping_mul, usize::wrapping_add];
    solve_fn(input, &ops) //&[Ops::Add, Ops::Mul])
}

pub fn part_two(input: &str) -> Option<usize> {
    // let ops = [usize::wrapping_mul, usize::wrapping_add, concat];
    // solve(input, &ops)
    solve(input, &[Ops::Add, Ops::Mul, Ops::Concat])
}

fn concat(a: usize, b: usize) -> usize {
    (10usize.pow(b.ilog10() + 1)) * a + b
}

fn parse_input(input: &str) -> Vec<Problem> {
    input
        .lines()
        .map(|l| {
            let (result, operands) = l.split_once(":").unwrap();
            Problem {
                target: result.parse().unwrap(),
                operands: operands
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat() {
        let res = Ops::Concat.call(15, 6);
        assert_eq!(res, 156);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
