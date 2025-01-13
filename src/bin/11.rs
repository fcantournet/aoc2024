use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let mut start = parse_input(input);
    start = naive(start, 25);
    return Some(start.len());
}

fn recursive(n: usize, remaining_iters: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if remaining_iters == 0 {
        return 1;
    }
    if let Some(res) = memo.get(&(n, remaining_iters)) {
        return *res;
    }

    let mut res = 0;
    if n == 0 {
        res = recursive(1, remaining_iters - 1, memo);
    } else if let Some((a, b)) = split_if_even_digits(&n) {
        res = recursive(a, remaining_iters - 1, memo) + recursive(b, remaining_iters - 1, memo);
    } else {
        res = recursive(n * 2024, remaining_iters - 1, memo);
    }

    memo.insert((n, remaining_iters), res);
    return res;
}

fn naive(mut start: Vec<usize>, iters: usize) -> Vec<usize> {
    for i in 0..iters {
        println!("iter {}: len(start): {}", i, start.len());
        let mut next: Vec<usize> = Vec::with_capacity(start.capacity());
        for elem in start.iter() {
            if *elem == 0 {
                next.push(1);
            } else if let Some((a, b)) = split_if_even_digits(elem) {
                next.push(a);
                next.push(b);
            } else {
                next.push(*elem * 2024);
            }
        }
        start = next;
    }
    start
}

fn split_if_even_digits(n: &usize) -> Option<(usize, usize)> {
    let ns = n.to_string();
    if ns.len() % 2 == 0 {
        let (a, b) = ns.split_at(ns.len() / 2);
        return Some((a.parse().unwrap(), b.parse().unwrap()));
    }
    None
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut start = parse_input(input);

    let mut memo = HashMap::new();
    let mut res = 0;
    for n in start {
        res += recursive(n, 75, &mut memo);
    }
    return Some(res);
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
