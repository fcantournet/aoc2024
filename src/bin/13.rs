use nom::{
    bytes::complete::tag,
    character::{complete, complete::line_ending},
    error::Error,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};
advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<usize> {
    let machines = parse_input(input).unwrap().1;
    // for machine in machines.iter() {
    //     if let Some(cost) = cost(machine) {
    //         println!("machine {:#?} has cost: {}", machine, cost);
    //     }
    // }
    Some(machines.iter().filter_map(cost).sum())
}

// This is a coordinate transform, to a non-orthogonal grid,
// but because target coordinates can only be positive, I think the
// solution is guaranteed to be unique.
// In any case we can solve as a system of 2 equation.
fn cost(m: &Machine) -> Option<usize> {
    let ax = m.a.0 as i64;
    let bx = m.b.0 as i64;
    let ay = m.a.1 as i64;
    let by = m.b.1 as i64;
    let px = m.prize.0 as i64;
    let py = m.prize.1 as i64;

    // (1) a * ax + b * bx = px
    // (2) a * ay + b * by = py
    // (1) a = (px - b * bx) / ax
    // (2) b * by = py - a * ay
    // (2) b * by = py - ((px - b * bx) / ax) * ay
    // (2) b * by * ax = py * ax - (px - b * bx) * ay
    // (2) b * by * ax = py * ax - px * ay + b * ay * bx
    // (2) b (by * ax - bx * ay) = py * ax - px * ay

    let b = (py * ax - px * ay) / (by * ax - bx * ay);
    let a = (px - b * bx) / ax;

    if a * ax + b * bx == px && a * ay + b * by == py {
        return Some(a as usize * 3 + b as usize);
    }
    None
}

const BUMP: usize = 10_000_000_000_000;
pub fn part_two(input: &str) -> Option<usize> {
    let mut machines = parse_input(input).unwrap().1;
    Some(
        machines
            .iter_mut()
            .filter_map(|machine| {
                machine.prize.0 += BUMP;
                machine.prize.1 += BUMP;
                cost(machine)
            })
            .sum(),
    )
}

#[derive(Debug, Clone)]
struct Machine {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
}

// Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400
fn machine(input: &str) -> IResult<&str, Machine> {
    let (rest, ((ax, ay), (bx, by), (rx, ry))) = tuple((
        terminated(
            preceded(
                tag("Button A: X+"),
                separated_pair(
                    complete::u64::<&str, Error<&str>>,
                    tag(", Y+"),
                    complete::u64,
                ),
            ),
            line_ending,
        ),
        terminated(
            preceded(
                tag("Button B: X+"),
                separated_pair(complete::u64, tag(", Y+"), complete::u64),
            ),
            line_ending,
        ),
        preceded(
            tag("Prize: X="),
            separated_pair(complete::u64, tag(", Y="), complete::u64),
        ),
    ))(input)
    .unwrap();

    let machine = Machine {
        a: (ax as usize, ay as usize),
        b: (bx as usize, by as usize),
        prize: (rx as usize, ry as usize),
    };
    Ok((rest, machine))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(tuple((line_ending, line_ending)), machine)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(280));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
