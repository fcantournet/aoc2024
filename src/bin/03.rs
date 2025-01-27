use std::ops::RangeFrom;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::{map_res, value},
    multi::{many1, many_till},
    sequence::{separated_pair, tuple},
    IResult, Parser,
};

advent_of_code::solution!(3);

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Mul(u64, u64),
    Do,
    Dont,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(
        many_till(
            anychar,
            alt((
                value(Instruction::Dont, tag("don't()")),
                value(Instruction::Do, tag("do()")),
                mul,
            )),
        )
        .map(|(_discard, ins)| ins),
    )(input)
}

fn mul(input: &str) -> IResult<&str, Instruction> {
    let res = tuple((
        tag("mul("),
        separated_pair(complete::u64, tag(","), complete::u64),
        tag(")"),
    ))(input);

    match res {
        Err(e) => Err(e),
        Ok((input, (_, (a, b), _))) => Ok((input, Instruction::Mul(a, b))),
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, instructions) = parse_input(input).unwrap();
    // println!("{:?}", instructions);
    Some(
        instructions
            .iter()
            .fold(0u64, |acc, instruction| match instruction {
                Instruction::Mul(x, y) => acc + (x * y),
                Instruction::Do => acc,
                Instruction::Dont => acc,
            }),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, instructions) = parse_input(input).unwrap();
    let mut active = true;
    let mut total = 0;
    for i in instructions {
        match i {
            Instruction::Mul(a, b) => {
                if active {
                    total += a * b;
                }
            }
            Instruction::Do => active = true,
            Instruction::Dont => active = false,
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
