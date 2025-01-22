use std::collections::HashMap;

use glam::IVec2;
use itertools::Itertools;

advent_of_code::solution!(21);

// let NumPad: HashMap<char, IVec2> = HashMap::from([('A', IVec2::new(0, 0))]);

pub fn part_one(input: &str) -> Option<usize> {
    let codes = parse_input(input);
    // let NumPad: HashMap<char, IVec2> = HashMap::from([('A', IVec2::new(0, 0))]);

    let mut position = IVec2::new(0, 0);

    let layers = 3;
    let pad = NumpPad { layers: 3 };
    let mut total = 0;
    for code in codes.iter() {
        let mut travel = "A".to_string();
        travel.extend(code);
        let mut res = 0;
        let mut input_lenght = 0;
        for (from, to) in travel.chars().tuple_windows() {
            let input = pad.solve(from, to, 0);
            println!("to go {} -> {} input: {}", from, to, input);
            input_lenght += input.len()
        }
        // Empty block to fix syntax error
        let value = code
            .iter()
            .filter(|c| c.is_digit(10))
            .join("")
            .parse::<usize>()
            .unwrap();
        res = input_lenght * value;
        println!("{} * {} = {}", input_lenght, value, res);
        total += res;
    }
    Some(total)
}

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
struct NumpPad {
    layers: usize,
}

impl NumpPad {
    fn solve(&self, from: char, to: char, level: usize) -> String {
        let mid = self.shortest_arrow_pad_for_key_from_pos(from, to);
        // println!("      {} -> {} ({}) : {}", from, to, level, mid);
        if level == self.layers - 1 {
            return mid.to_string();
        }
        let mut res = String::with_capacity(mid.len() * 4);
        let travel = "A".to_string() + mid;
        for (a, b) in travel.chars().tuple_windows() {
            res.extend(self.solve(a, b, level + 1).chars());
        }
        // println!("  {} -> {} at layer {}: {}", from, to, level, res);
        res
    }

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
    fn shortest_arrow_pad_for_key_from_pos(&self, from: char, to: char) -> &str {
        let dir = self.char_to_pos(to) - self.char_to_pos(from);
        match dir {
            // IVec2{x: 0, y} if y > 0 => "^".repeat(y as usize),
            // IVec2{x: 0, y} if y < 0 => "v".repeat(y as usize),
            // IVec2{x, y: 0} => "<".repeat(y as usize),
            // IVec2{x, y: 0} => "<".repeat(y as usize),
            IVec2 { x: 0, y: 0 } => JUSTA,
            IVec2 { x: 0, y: 1 } => UP,
            IVec2 { x: 0, y: 2 } => UPUP,
            IVec2 { x: 0, y: 3 } => UPUPUP,
            IVec2 { x: 0, y: -1 } => DOWN,
            IVec2 { x: 0, y: -2 } => DOWNDOWN,
            IVec2 { x: 0, y: -3 } => DOWNDOWNDOWN,
            IVec2 { x: 1, y: 0 } => LEFT,
            IVec2 { x: 2, y: 0 } => LEFTLEFT,
            IVec2 { x: -1, y: 0 } => RIGHT,
            IVec2 { x: -2, y: 0 } => RIGHTRIGHT,
            IVec2 { x: 1, y: 1 } => UPLEFT,
            IVec2 { x: 2, y: 1 } => UPLEFTLEFT,
            IVec2 { x: 1, y: 2 } => UPUPLEFT,
            IVec2 { x: 2, y: 2 } => UPUPLEFTLEFT,
            IVec2 { x: 1, y: 3 } => UPUPUPLEFT,
            IVec2 { x: 2, y: 3 } => UPUPUPLEFTLEFT,
            IVec2 { x: -1, y: 1 } => RIGHTUP,
            IVec2 { x: -1, y: 2 } => RIGHTUPUP,
            IVec2 { x: -1, y: 3 } => RIGHTUPUPUP,
            IVec2 { x: -2, y: 1 } => RIGHTRIGHTUP,
            IVec2 { x: -2, y: 2 } => RIGHTRIGHTUPUP,
            IVec2 { x: -1, y: -1 } => RIGHTDOWN,
            IVec2 { x: -1, y: -2 } => RIGHTDOWNDOWN,
            IVec2 { x: -1, y: -3 } => RIGHTDOWNDOWNDOWN,
            IVec2 { x: -2, y: -1 } => RIGHTRIGHTDOWN,
            IVec2 { x: -2, y: -2 } => RIGHTRIGHTDOWNDOWN,
            IVec2 { x: -2, y: -3 } => RIGHTRIGHTDOWNDOWNDOWN,
            IVec2 { x: 1, y: -1 } => LEFTDOWN,
            IVec2 { x: 1, y: -2 } => LEFTDOWNDOWN,
            IVec2 { x: 1, y: -3 } => LEFTDOWNDOWNDOWN,
            IVec2 { x: 2, y: -1 } => LEFTLEFTDOWN,
            IVec2 { x: 2, y: -2 } => LEFTLEFTDOWNDOWN,
            _ => unreachable!("invalid movement vector {:#?}", dir),
        }
    }
}

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
struct ArrowPad {}

static JUSTA: &str = "A";

static UP: &str = "^A";
static UPUP: &str = "^^A";
static UPUPUP: &str = "^^^A";
static DOWN: &str = "vA";
static DOWNDOWN: &str = "vvA";
static DOWNDOWNDOWN: &str = "vvvA";
static LEFT: &str = "<A";
static LEFTLEFT: &str = "<<A";
static RIGHT: &str = ">A";
static RIGHTRIGHT: &str = ">>A";

static UPLEFT: &str = "^<A";
static UPLEFTLEFT: &str = "^<<A";
static UPUPLEFT: &str = "^^<A";
static UPUPLEFTLEFT: &str = "^^<<A";
static UPUPUPLEFT: &str = "^^^<A";
static UPUPUPLEFTLEFT: &str = "^^^<<A";

static RIGHTUP: &str = ">^A";
static RIGHTUPUP: &str = ">^^A";
static RIGHTUPUPUP: &str = ">^^^A";
static RIGHTRIGHTUP: &str = ">>^A";
static RIGHTRIGHTUPUP: &str = ">>^^A";

static RIGHTDOWN: &str = ">vA";
static RIGHTDOWNDOWN: &str = ">vvA";
static RIGHTDOWNDOWNDOWN: &str = ">vvvA";
static RIGHTRIGHTDOWN: &str = ">>vA";
static RIGHTRIGHTDOWNDOWN: &str = ">>vvA";
static RIGHTRIGHTDOWNDOWNDOWN: &str = ">>vvvA";

static LEFTDOWN: &str = "v<A";
static LEFTDOWNDOWN: &str = "vv<A";
static LEFTDOWNDOWNDOWN: &str = "vvv<A";
static LEFTLEFTDOWN: &str = "v<<A";
static LEFTLEFTDOWNDOWN: &str = "vv<<A";

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn part_two(input: &str) -> Option<usize> {
    None
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
        //
        // to go 3 -> 7 input: <vA<AA>>^AAvA<^A>AAvA^A
        //  is better than
        // to go 3 -> 7 input: v<<A>>^AAv<A<A>>^AAvAA^<A>A

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
