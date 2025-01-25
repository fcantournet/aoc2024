use std::collections::HashMap;

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<usize> {
    let secrets = parse_input(input);

    let res = secrets
        .into_iter()
        .map(|secret| {
            let mut next = secret;
            for _ in 0..2000 {
                next = next_secret_faster(next);
            }
            next
        })
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<usize> {
    let secrets = parse_input(input);

    let mut seq = HashMap::with_capacity(50000);
    for s in secrets.into_iter() {
        let next = sequence(s);
        for e in next.into_iter() {
            seq.entry(e.0).and_modify(|v| *v += e.1).or_insert(e.1);
        }
    }

    let max = seq.iter().max_by(|a, b| a.1.cmp(b.1));
    println!("sequence of len {} with max {:?}", seq.len(), max);
    let max_value = *max.unwrap().1 as usize;
    Some(max_value)
}

fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

/// Compute next secret number using a
/// [Xorshift LFSR](https://en.wikipedia.org/wiki/Linear-feedback_shift_register#Xorshift_LFSRs).
fn next_secret_faster(mut n: usize) -> usize {
    n = (n ^ (n << 6)) & 0xffffff;
    n = (n ^ (n >> 5)) & 0xffffff;
    (n ^ (n << 11)) & 0xffffff
}

fn next_secret(current: usize) -> usize {
    let a = mix_and_prune(current * 64, current);
    let b = mix_and_prune(a / 32, a);
    let c = mix_and_prune(b * 2048, b);
    c
}

fn mix_and_prune(input: usize, current: usize) -> usize {
    (input ^ current) % 16777216
}

fn sequence(start: usize) -> HashMap<(i64, i64, i64, i64), i64> {
    let mut seq = HashMap::with_capacity(2000);

    let last0 = (start % 10) as i64;
    let mut next = next_secret_faster(start);
    let last1 = (next % 10) as i64;
    let mut a1 = (last1 - last0);

    next = next_secret_faster(next);
    let last2 = (next % 10) as i64;
    let mut a2 = (last2 - last1);

    next = next_secret_faster(next);
    let mut last3 = (next % 10) as i64;
    let mut a3 = (last3 - last2);

    next = next_secret_faster(next);
    let mut last4 = (next % 10) as i64;
    let mut a4 = (last4 - last3);

    seq.entry((a1, a2, a3, a4))
        .and_modify(|v| *v += last4)
        .or_insert(last4);

    for i in 5..2000 {
        a1 = a2;
        a2 = a3;
        a3 = a4;
        last3 = last4;
        next = next_secret_faster(next);
        last4 = (next % 10) as i64;
        a4 = (last4 - last3);
        if !seq.contains_key(&(a1, a2, a3, a4)) {
            seq.insert((a1, a2, a3, a4), last4);
        }
    }
    seq
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use super::*;

    #[test]
    fn basic_next_secret() {
        assert_eq!(42 ^ 15, 37);
        assert_eq!(100000000 % 16777216, 16113920);

        let mut next = 123;
        next = next_secret(next);
        assert_eq!(next, 15887950);
        next = next_secret(next);
        assert_eq!(next, 16495136);
        next = next_secret(next);
        assert_eq!(next, 527345);
        next = next_secret(next);
        assert_eq!(next, 704524);
        next = next_secret(next);
        assert_eq!(next, 1553684);
        next = next_secret(next);
        assert_eq!(next, 12683156);
        next = next_secret(next);
        assert_eq!(next, 11100544);
        next = next_secret(next);
        assert_eq!(next, 12249484);
        next = next_secret(next);
        assert_eq!(next, 7753432);
        next = next_secret(next);
        assert_eq!(next, 5908254);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let input = "1
2
3
2024";
        let result = part_two(input);
        assert_eq!(result, Some(23));
    }
}
