use std::collections::{HashMap, HashSet};

use glam::IVec2;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

advent_of_code::solution!(14);

struct Sentinel {
    pos: IVec2,
    speed: IVec2,
}
pub fn part_one(input: &str) -> Option<usize> {
    part_one_with_params(input, IVec2::new(101, 103))
}

fn part_one_with_params(input: &str, size: IVec2) -> Option<usize> {
    let sentinels = parse_input(input);

    let finals: Vec<_> = sentinels
        .iter()
        .map(|s| {
            let x = (s.pos.x + s.speed.x * 100).rem_euclid(size.x);
            let y = (s.pos.y + s.speed.y * 100).rem_euclid(size.y);
            IVec2::new(x, y)
        })
        .collect();

    let h = size / 2;
    println!("h: {:#?}", h);
    println!("finals: {:#?}", finals);
    print_grid(&finals, size);

    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
    for f in finals.iter() {
        if f.x < h.x {
            if f.y < h.y {
                q1 += 1;
            } else if f.y > h.y {
                q2 += 1;
            }
        } else if f.x > h.x {
            if f.y < h.y {
                q3 += 1;
            } else if f.y > h.y {
                q4 += 1;
            }
        }
    }
    Some(q1 * q2 * q3 * q4)
}

pub fn part_two(input: &str) -> Option<usize> {
    let size = IVec2::new(101, 103);
    let sentinels = parse_input(input);

    let mut step = 1;
    let (result, unique) = loop {
        let mut unique = HashSet::new();
        unique.reserve(sentinels.len());
        for s in sentinels.iter() {
            let x = (s.pos.x + s.speed.x * step).rem_euclid(size.x);
            let y = (s.pos.y + s.speed.y * step).rem_euclid(size.y);
            unique.insert(IVec2::new(x, y));
        }
        if unique.len() == sentinels.len() {
            // all unique !
            break (step, unique);
        }
        step += 1;
    };
    println!("Step {}", step);
    print_grid(&unique.into_iter().collect(), size);
    Some(result as usize)
}

fn print_grid(sentinels: &Vec<IVec2>, size: IVec2) {
    let places = sentinels.iter().fold(HashMap::new(), |mut acc, v| {
        acc.entry(v).and_modify(|x| *x += 1).or_insert(1);
        acc
    });
    for x in 0..size.x {
        for y in 0..size.y {
            if let Some(n) = places.get(&IVec2 { x, y }) {
                print!("{}", n);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn parse_input(input: &str) -> Vec<Sentinel> {
    let (_, sentinels) = separated_list1(line_ending, sentinel)(input).unwrap();
    sentinels
}

fn sentinel(input: &str) -> IResult<&str, Sentinel> {
    let (input, (p, v)) = separated_pair(
        preceded(tag("p="), ivec2),
        space1,
        preceded(tag("v="), ivec2),
    )(input)?;
    Ok((input, Sentinel { pos: p, speed: v }))
}

fn ivec2(input: &str) -> IResult<&str, IVec2> {
    let (input, (x, y)) = separated_pair(complete::i32, tag(","), complete::i32)(input)?;
    Ok((input, IVec2 { x, y }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one_with_params(&input, IVec2::new(11, 7));

        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        // We don't have a test case for part 2.
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }

    #[test]
    fn test_modulo() {
        assert_eq!(-8, -8);
        assert_eq!((-8i32).rem_euclid(10), 2);
    }
}
