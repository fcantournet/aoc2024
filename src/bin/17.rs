use itertools::Itertools;
advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    let (m, instructions) = parse_input(input);
    println!("State: {:#?}, instructions: {:#?}", m, instructions);

    let mut machine = m.clone();
    let mut output = Vec::new();
    while machine.pc < instructions.len() {
        if let Some(out) = machine.execute(instructions[machine.pc], instructions[machine.pc + 1]) {
            output.push(out);
        }
    }
    Some(output.iter().join(","))
}

// For part 2 we just cannot brute force the 2^45+ possible values of A.
// We need to reverse engineer the actual program from the input to figure out how to solve this.
// Some guy on youtube uses a SAT solver but honestly the code is INSANE, I don't know how to use those
// and the analysis he uses for coding it (shown below) is enough to write a simple exhaustive tree search.
//
// Program:             2,4     1,5     7,5     1,6     0,3     4,3     5,5     3,0
// What does this do ?  00      02      04      06      08      10      12      14
//      00: B <- A % 8
//      02: B <- B ^ 5
//      04: C <- A // 2 ** B
//      06: B <- B ^ 6
//      08: A <- A // 8
//      10: B <- B ^ C
//      12: print B % 8
//
// Every loop we divide A by 8 a.k.a bitshift >> 3.
// And B & C only depend on the value of A at the beginning of the loop.
// Additionally we know that at the last iteration (A >> 3) == 0 because this is how the program stops.
// So at the last iteration A is in 0..=7, so we can test all possible values to find which outputs instructions[-1].
// Then we set thos first highest bits, and try all A with those 3 top bits and the next 3 in 0..=7.
// Rince, repeat.
// Multiple values can work at each layer, and some branches of the tress will be dead ends.
pub fn part_two(input: &str) -> Option<usize> {
    let (m, instructions) = parse_input(input);

    let mut a_known_bits = vec![0usize];

    for _ in 0..instructions.len() {
        // println!("{:#?}", a_known_bits);
        let mut next_possibles: Vec<usize> = Vec::new();
        for possible in a_known_bits {
            // println!("known bits: {:#b}", possible);
            let mut matching = Vec::new();
            for new_bits in 0..=7 {
                let a = new_bits + (possible << 3);
                // println!("trying a: {:#b}", a);
                let output = try_a(m.clone(), a, &instructions);
                if match_up_until_know(&output, &instructions) {
                    // println!("matching {:#?}", output);
                    matching.push(a);
                }
            }
            // println!("those work {:#?}", matching);
            next_possibles.extend_from_slice(&matching);
        }
        a_known_bits = next_possibles;
    }

    a_known_bits.into_iter().min()
}

fn match_up_until_know(output: &[usize], instructions: &[usize]) -> bool {
    for (x, y) in output.iter().rev().zip(instructions.iter().rev()) {
        if *x != *y {
            return false;
        }
    }
    true
}

fn try_a(mut m: Machine, a: usize, instructions: &[usize]) -> Vec<usize> {
    m.a = a;
    let mut output = Vec::new();
    while m.pc < instructions.len() {
        if let Some(out) = m.execute(instructions[m.pc], instructions[m.pc + 1]) {
            output.push(out);
        }
    }
    output
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Machine {
    a: usize,
    b: usize,
    c: usize,
    pc: usize,
}

impl Machine {
    fn execute(&mut self, code: usize, operand: usize) -> Option<usize> {
        // println!("State: {:#?} code: {} operand: {}", self, code, operand);

        self.pc += 2; // the only time we don't incr PC by 2 is with jump which will reset the PC.
        match code {
            0 => {
                //adv
                self.a /= (1 << self.combo(operand));
                None
            }
            1 => {
                //bxl
                self.b ^= operand;
                None
            }
            2 => {
                // bst
                self.b = self.combo(operand) % 8;
                None
            }
            3 => {
                // jnz
                if self.a != 0 {
                    self.pc = operand;
                }
                None
            }
            4 => {
                // bxc
                self.b ^= self.c;
                None
            }
            5 => {
                // out
                Some(self.combo(operand) % 8)
            }
            6 => {
                // bdv
                self.b = self.a / (1 << self.combo(operand));
                None
            }
            7 => {
                // cdv
                self.c = self.a / (1 << self.combo(operand));
                None
            }
            _ => panic!("invalid op code"),
        }
    }

    fn combo(&self, operand: usize) -> usize {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("combo cannot be more than 6"),
        }
    }
}

fn parse_input(input: &str) -> (Machine, Vec<usize>) {
    let mut it = input.split("\n\n");
    let regs: Vec<_> = it
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split(": ").last().unwrap().trim().parse().unwrap())
        .collect();
    assert!(regs.len() == 3);
    let instructions: Vec<_> = it
        .next()
        .unwrap()
        .trim()
        .split(": ")
        .last()
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    (
        Machine {
            a: regs[0],
            b: regs[1],
            c: regs[2],
            pc: 0,
        },
        instructions,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".into()));

        let result2 = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result2, Some("4,2,5,6,7,7,7,7,3,1,0".into()));

        let actual = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(actual, Some("1,2,3,1,3,2,5,3,1".into()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }
}
