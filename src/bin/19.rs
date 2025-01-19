use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<usize> {
    let (towels, patterns) = parse_input(input);

    let mut memo = HashSet::new();
    let mut memo_neg = HashSet::new();
    let mut solved = 0;
    for pattern in patterns.iter() {
        if solves(pattern, &towels, &mut memo, &mut memo_neg) {
            solved += 1;
        }
        println!("{}", solved);
    }
    // println!("{:#?}", memo);
    Some(solved)
}

fn solves(
    pattern: &str,
    towels: &Vec<&str>,
    memo: &mut HashSet<String>,
    memoneg: &mut HashSet<String>,
) -> bool {
    if memo.contains(pattern) || pattern.len() == 0 {
        return true;
    }
    if memoneg.contains(pattern) {
        return false;
    }

    for t in towels.into_iter() {
        if let Some(remaining) = pattern.strip_suffix(t) {
            println!("{} matched {}: remaing {}", pattern, t, remaining);
            let solves = solves(remaining, towels, memo, memoneg);
            if solves {
                memo.insert(pattern.to_string());
                return true;
            }
        }
    }
    memoneg.insert(pattern.to_string());
    false
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
