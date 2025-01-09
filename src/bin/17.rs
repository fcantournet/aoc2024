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

pub fn part_two(input: &str) -> Option<u64> {
    None
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
        println!("State: {:#?} code: {} operand: {}", self, code, operand);

        self.pc += 2; // the only time we don't incr PC by 2 is with jump which will reset the PC.
        match code {
            0 => {
                //adv
                self.a = self.a / (1 << self.combo(operand));
                None
            }
            1 => {
                //bxl
                self.b = self.b ^ operand;
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
                self.b = self.b ^ self.c;
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
                // bdv
                self.b = self.a / (1 << self.combo(operand));
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

    return (
        Machine {
            a: regs[0],
            b: regs[1],
            c: regs[2],
            pc: 0,
        },
        instructions,
    );
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
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
