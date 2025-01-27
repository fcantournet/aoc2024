use std::collections::{HashMap, HashSet};

use glam::IVec2;
use itertools::Itertools;

advent_of_code::solution!(8);

fn debug(input: &str, antinodes: &HashSet<IVec2>) {
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if antinodes.contains(&IVec2::new(x as i32, y as i32)) {
                print!("#")
            } else {
                print!("{c}")
            }
        }
        println!();
    }
}

fn solve(input: &str, short_circuit: bool) -> Option<usize> {
    let (m, (x_size, y_size)) = parse_input(input);
    // println!("Grid of size {} * {}", x_size, y_size);

    let mut antinodes: HashSet<IVec2> = HashSet::new();

    for (k, v) in m {
        for (a, b) in v.iter().tuple_combinations() {
            let an = gen_antinodes(a, b, x_size, y_size, short_circuit);
            // println!("{} Nodes ( {:?} , {:?} ) => {:?}", k, a, b, an);
            antinodes.extend(an.iter());
        }
    }
    // debug(input, &antinodes);
    Some(antinodes.len())
}

fn gen_antinodes(
    a: &IVec2,
    b: &IVec2,
    x_size: i32,
    y_size: i32,
    short_circuit: bool,
) -> Vec<IVec2> {
    let mut res = Vec::new();
    if !short_circuit {
        res.push(*a);
        res.push(*b);
    }
    let dir = b - a;

    let mut after_b = b + dir;
    while in_grid(after_b, x_size, y_size) {
        res.push(after_b);
        if short_circuit {
            break;
        }
        after_b += dir;
    }

    let mut before_a = a - dir;
    while in_grid(before_a, x_size, y_size) {
        res.push(before_a);
        if short_circuit {
            break;
        }
        before_a -= dir;
    }

    res
}

fn in_grid(p: IVec2, x_size: i32, y_size: i32) -> bool {
    if (p.x >= 0) && (p.x < x_size) && (p.y >= 0) && (p.y < y_size) {
        // println!("{:?} in {} * {}", p, x_size, y_size);
        return true;
    }
    false
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, true)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, false)
}

fn parse_input(input: &str) -> (HashMap<char, Vec<IVec2>>, (i32, i32)) {
    let mut x_size = 0;
    let mut y_size = 0;

    let mut m: HashMap<char, Vec<IVec2>> = HashMap::new();

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            x_size = x_size.max(x + 1);
            if c == '.' {
                continue;
            }
            m.entry(c).or_default().push(IVec2 {
                x: x as i32,
                y: y as i32,
            });
        }
        y_size += 1;
    }
    (m, (x_size as i32, y_size))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
