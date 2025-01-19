use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<usize> {
    let (towels, patterns) = parse_input(input);

    let mut memo_neg = HashSet::new();
    let mut solved = 0;
    for pattern in patterns.iter() {
        if solves(pattern, &towels, &mut memo_neg) {
            solved += 1;
        }
    }
    Some(solved)
    // part_one_factored(input)
}

fn part_one_factored(input: &str) -> Option<usize> {
    let (towels, patterns) = parse_input(input);

    let mut memo = HashMap::new();
    let mut solved = 0;
    for pattern in patterns.iter() {
        if counts(pattern, &towels, &mut memo, true) > 0 {
            solved += 1;
        }
    }
    Some(solved)
}

fn solves(pattern: &str, towels: &Vec<&str>, memoneg: &mut HashSet<String>) -> bool {
    if pattern.len() == 0 {
        return true;
    }

    if memoneg.contains(pattern) {
        return false;
    }

    for t in towels.into_iter() {
        if let Some(remaining) = pattern.strip_suffix(t) {
            // println!("{} matched {}: remaing {}", pattern, t, remaining);
            let solves = solves(remaining, towels, memoneg);
            if solves {
                // memo.insert(pattern.to_string());
                return true;
            }
        }
    }
    memoneg.insert(pattern.to_string());
    false
}

fn counts(
    pattern: &str,
    towels: &Vec<&str>,
    memo: &mut HashMap<String, usize>,
    short_circuit: bool,
) -> usize {
    if pattern.len() == 0 {
        return 1;
    }
    if let Some(count) = memo.get(pattern) {
        return *count;
    }

    let mut count = 0;
    for t in towels.into_iter() {
        if let Some(remaining) = pattern.strip_suffix(t) {
            count += counts(remaining, towels, memo, short_circuit);
            if short_circuit && count > 0 {
                return count;
            }
        }
    }
    memo.insert(pattern.to_string(), count);
    count
}

pub fn part_two(input: &str) -> Option<usize> {
    let (towels, patterns) = parse_input(input);

    let mut memo = HashMap::new();
    let mut solved = 0;
    for pattern in patterns.iter() {
        solved += counts(pattern, &towels, &mut memo, false);
    }
    Some(solved)
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut it = input.split("\n\n");
    let towels = it
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.trim())
        .sorted_by(|a, b| Ord::cmp(&b.len(), &a.len()))
        .collect();
    let patterns = it.next().unwrap().lines().map(|s| s.trim()).collect();
    (towels, patterns)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
