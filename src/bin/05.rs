use std::cmp::Ordering::*;
use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<usize> {
    let (rules, updates) = parse_input(input);
    let mut order = [[Greater; 100]; 100];
    for (from, to) in rules.iter() {
        order[*from][*to] = Less;
    }

    let mut answer = 0;
    for update in updates {
        let middle = update.len() / 2;

        if update.is_sorted_by(|&from, &to| order[from][to] == Less) {
            answer += update[middle];
        }

        // let mut correct = true;
        // for (p1, p2) in update.iter().tuple_windows() {
        //     // Assuming the existence of a total order, there is an explicit rule explaining each position (except last one).
        //     // Basically any (p1, p2) without an explicit precedence rule could be swapped in place and we wouldn't have
        //     // a total ordering, except the "last/first" element which could be deterministically ordered with any rule vs a non-contiguous
        //     // other page (that would be enough to determine if it's the last or first).
        //     // if !applicable.contains(&(*p1, *p2)) {
        //     if order[*p1][*p2] == Greater {
        //         correct = false;
        //         // println!("{:?} violates {:?}", update, (p1, p2));
        //         break;
        //     }
        // }
        // if correct {
        //     // println!("corect: {:?}", update);
        //     answer += update[update.len() / 2];
        // }
    }

    Some(answer)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (rules, updates) = parse_input(input);
    let mut order = [[Greater; 100]; 100];
    for (from, to) in rules.iter() {
        order[*from][*to] = Less;
    }

    let mut answer = 0;
    for mut update in updates {
        let middle = update.len() / 2;
        if !update.is_sorted_by(|&from, &to| order[from][to] == Less) {
            // We only need the middle index so this is slightly faster than "sort_unstable_by"
            update.select_nth_unstable_by(middle, |&from, &to| order[from][to]);
            answer += update[middle];
        }

        // let mut correct = true;
        // for (p1, p2) in update.iter().tuple_windows() {
        //     // if !applicable.contains(&(*p1, *p2)) {
        //     if order[*p1][*p2] == Greater {
        //         correct = false;
        //         //
        //         // println!("{:?} violates {:?}", update, (p1, p2));
        //         break;
        //     }
        // }

        // if !correct {
        //     // Working with our confirmed total ordering assumption:
        //     // We swap each (p1, p2) which doesn't have an explicit ordering rule, or has a (p2, p1) rules.
        //     // In both case the current relative ordering of p1 and p2 is known to be incorrect.
        //     // We only need to check the absence of the (p1,p2) rules, which is nice.
        //     let mut changed = true;
        //     let mut iters = 0;
        //     let mut swaps = 0;
        //     while changed {
        //         iters += 1;
        //         changed = false;
        //         for (i1, i2) in (0..update.len()).tuple_windows() {
        //             let p1 = update[i1];
        //             let p2 = update[i2];
        //             if order[p1][p2] == Greater {
        //                 // if !applicable.contains(&(p1, p2)) {
        //                 changed = true;
        //                 // println!("swapping/ {:?}", (p1, p2));
        //                 swaps += 1;
        //                 update[i2] = p1;
        //                 update[i1] = p2;
        //             }
        //         }
        //     }
        //     // println!(
        //     //     "corrected: {:?} in {} iterations and {} swaps",
        //     //     update, iters, swaps
        //     // );
        //     answer += update[middle];
        // }
    }

    Some(answer)
}

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();
    let rules: Vec<_> = rules_str
        .lines()
        .map(|line| {
            let (astr, bstr) = line.trim().split_once("|").unwrap();
            let a = astr.parse().unwrap();
            let b = bstr.parse().unwrap();
            (a, b)
        })
        .collect();

    let updates = updates_str
        .lines()
        .map(|l| l.split(",").map(|c| c.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
