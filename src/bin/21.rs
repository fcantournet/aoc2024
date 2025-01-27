use cached::proc_macro::cached;
use std::collections::HashMap;

use glam::IVec2;
use itertools::Itertools;
use num::abs;

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 2)
}
pub fn part_two(input: &str) -> Option<usize> {
    solve(input, 25)
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn solve(input: &str, robot_count: usize) -> Option<usize> {
    let codes = parse_input(input);

    let pad = NumPad {
        possible_path: all_possible_paths(),
    };
    let mut total = 0;

    // println!("{:#?}", pad.possible_path);

    for code in codes.into_iter() {
        let mut travel = format!("A{}", code);
        let mut input_lenght = 0;
        for (from, to) in travel.chars().tuple_windows() {
            let short_test = complexity(from, to, robot_count, &pad);
            input_lenght += short_test;
        }
        let value = code
            .chars()
            .filter(|c| c.is_ascii_digit())
            .join("")
            .parse::<usize>()
            .unwrap();
        let res = input_lenght * value;
        println!("{} * {} = {}", input_lenght, value, res);
        total += res;
    }
    Some(total)
}

#[cached(
    key = "String",
    convert = r#"{ format!("{}-{}-{}", from, to, robot_count) }"#
)]
fn complexity(from: char, to: char, robot_count: usize, pad: &NumPad) -> usize {
    let paths = pad.paths(from, to);
    // we recurse until out of robots in the chain.
    let min_inputs = if robot_count > 0 {
        paths
            .iter()
            .map(|path| {
                let path = format!("A{}", path); // All moves start with the robot on 'A'
                path.chars()
                    .tuple_windows()
                    .map(|(a, b)| complexity(a, b, robot_count - 1, pad))
                    .sum::<usize>()
            })
            .min()
            .expect("couldn't get min ??")
    } else {
        paths
            .iter()
            .map(|path| path.len())
            .min()
            .expect("couldn't calculate min")
    };
    println!("{} -> {} @{}: {}", from, to, robot_count, min_inputs);
    min_inputs
}

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
struct ArrowPad {}

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
struct NumPad {
    possible_path: HashMap<IVec2, Vec<String>>,
}

// This is the dead zone coordinate in both pads.
const VERBOTTEN: IVec2 = IVec2 { x: 2, y: 0 };

// We can a a single implementation for both types of pads.
// 'A' has the same coordinate on both pads: we choose (0,0).
// The forbidden key has the same coordinate on both pads too (2,0).
// All other keys are unique to each pad type.
impl NumPad {
    fn char_to_pos(&self, c: char) -> IVec2 {
        match c {
            'A' => IVec2::new(0, 0),
            '0' => IVec2::new(1, 0),
            '1' => IVec2::new(2, 1),
            '2' => IVec2::new(1, 1),
            '3' => IVec2::new(0, 1),
            '4' => IVec2::new(2, 2),
            '5' => IVec2::new(1, 2),
            '6' => IVec2::new(0, 2),
            '7' => IVec2::new(2, 3),
            '8' => IVec2::new(1, 3),
            '9' => IVec2::new(0, 3),
            // arrowpad
            '^' => IVec2::new(1, 0),
            '<' => IVec2::new(2, -1),
            '>' => IVec2::new(0, -1),
            'v' => IVec2::new(1, -1),
            _ => unreachable!(),
        }
    }
    fn paths(&self, from: char, to: char) -> Vec<String> {
        let from_vec = self.char_to_pos(from);
        let to_vec = self.char_to_pos(to);
        let dir = to_vec - from_vec;
        let mut paths = self.possible_path[&dir].clone();
        if paths.len() == 1 {
            return paths;
        }
        if (to_vec.x == VERBOTTEN.x && from_vec.y == VERBOTTEN.y)
            || (to_vec.y == VERBOTTEN.y && from_vec.x == VERBOTTEN.x)
        {
            // We risk going through verbotten case
            paths.retain(|p| NumPad::valid_path(from_vec, p));
        }

        paths
    }

    fn valid_path(mut pos: IVec2, path: &str) -> bool {
        for c in path.chars() {
            match c {
                '>' => pos.x -= 1,
                '<' => pos.x += 1,
                '^' => pos.y += 1,
                'v' => pos.y -= 1,
                'A' => (),
                _ => unreachable!("bad direction"),
            };
            if pos == VERBOTTEN {
                return false;
            }
        }
        true
    }
}

// We can pre-compute all possible input sequences for a given move (as an IVec2).
// From char X to Y, we can compute a IVec2 representing the movement over the pad.
// Computing the sequence of inputs from that movement is agnostic of the pad on which we move
// and only depends on the pad we use to control the move which is always a ArrowPad.
fn all_possible_paths() -> HashMap<IVec2, Vec<String>> {
    let mut pos = HashMap::new();
    for x in -2..=2 {
        for y in -3..=3 {
            let vert = match y {
                0 => None,
                1.. => Some("^".repeat(y as usize)),
                ..0 => Some("v".repeat(abs(y) as usize)),
            };

            let horizontal = match x {
                0 => None,
                1.. => Some("<".repeat(x as usize)),
                ..0 => Some(">".repeat(abs(x) as usize)),
            };
            let combo = match (horizontal, vert) {
                (None, None) => vec!["A".to_string()],
                (None, Some(vert)) => vec![vert + "A"],
                (Some(horizontal), None) => vec![horizontal + "A"],
                (Some(horizontal), Some(vert)) => {
                    vec![horizontal.clone() + &vert + "A", vert + &horizontal + "A"]
                }
            };
            pos.insert(IVec2::new(x, y), combo);
        }
    }
    pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        //         let solutions_text = "029A: <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
        // 980A: <v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A
        // 179A: <v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
        // 456A: <v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A

        // optimal:
        // to go A -> 3 input: <v<A>>^AvA^A
        // to go 3 -> 7 input: <vA<AA>>^AAvA<^A>AAvA^A
        // to go 7 -> 9 input: <vA>^AA<A>A
        // to go 9 -> A input: <v<A>A>^AAAvA<^A>A

        // mine:
        // to go A -> 3 input: v<<A>>^AvA^A
        // to go 3 -> 7 input: v<<A>>^AAv<A<A>>^AAvAA^<A>A
        // to go 7 -> 9 input: v<A>^AA<A>A
        // to go 9 -> A input: v<A<A>>^AAAvA^<A>A
        // 68 * 379 = 25772
        //
        //
        //
        // Good (1) vs bad (2)
        // (1) to go 3 -> 7 input:  <vA<AA>>^AAvA<^A>AAvA^A
        //                          v<<AA>^AA>A
        //                          <<^^A
        // (2) to go 3 -> 7 input:  v<<A>>^AAv<A<A>>^AAvAA^<A>A
        //                          <AAv<AA>>^A
        //                          ^^<<A
        // The shortest sequence at level N is not obvious from each sub-level's shortests paths
        // We need to check all possible path at each level, recurse and take min.

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
