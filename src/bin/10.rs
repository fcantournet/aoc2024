use std::collections::{HashMap, HashSet, VecDeque};

use glam::IVec2;

advent_of_code::solution!(10);

const AROUND: [IVec2; 4] = [
    IVec2 { x: -1, y: 0 },
    IVec2 { x: 1, y: 0 },
    IVec2 { x: 0, y: 1 },
    IVec2 { x: 0, y: -1 },
];

pub fn part_one(input: &str) -> Option<usize> {
    walk_and_shit(input).0
}

fn walk_and_shit(input: &str) -> (Option<usize>, Option<usize>) {
    let grid = parse_input(input);
    let heads: Vec<_> = grid.iter().filter(|(_, &v)| v == 0).collect();

    let mut part1 = 0;
    let mut part2 = 0;
    for head in heads {
        // We just walk around following permissible paths.
        let mut to_visit = VecDeque::new();
        to_visit.push_back(head);
        let mut found_paths = 0;
        let mut found = HashSet::new();
        while let Some(current) = to_visit.pop_front() {
            if *current.1 == 9 {
                found_paths += 1;
                found.insert(current);
                continue;
            }
            for dir in AROUND {
                if let Some(next) = grid.get_key_value(&(current.0 + dir)) {
                    if *next.1 == (current.1 + 1) {
                        to_visit.push_back(next);
                    }
                }
            }
        }
        part1 += found.len();
        part2 += found_paths;
    }

    return (Some(part1), Some(part2));
}

pub fn part_two(input: &str) -> Option<usize> {
    walk_and_shit(input).1
}

fn parse_input(input: &str) -> HashMap<IVec2, usize> {
    let mut grid = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let height = c.to_digit(10).unwrap() as usize;
            grid.insert(
                IVec2 {
                    x: x as i32,
                    y: y as i32,
                },
                height,
            );
        }
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "0123
1234
8765
9876";
        let result = part_one(input);
        assert_eq!(result, Some(1));

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
