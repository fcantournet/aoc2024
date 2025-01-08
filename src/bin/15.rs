use std::collections::HashMap;

use glam::IVec2;
advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u64> {
    let (mut g, dir) = parse_input(input);
    let mut fish_pos = g
        .iter()
        .filter(|(_, val)| **val == Object::Fish)
        .next()
        .unwrap()
        .0
        .clone();

    for d in dir {
        println!("Fish at {} moving {}", fish_pos, d);

        if let Some(tomove) = find_all_mover(&g, d, fish_pos.clone()) {
            println!("to_move: {:#?}", tomove);
            let mut to_resinsert: Vec<_> = Vec::default();
            for e in tomove.iter() {
                if let Some(present) = g.remove(e) {
                    to_resinsert.push((e + d, present));
                } else {
                    panic!("moving non-existing object")
                }
            }
            println!("reinserting: {:#?}", to_resinsert);
            for (pos, o) in to_resinsert {
                g.insert(pos, o);
            }
            fish_pos = fish_pos + d;
        }
    }

    let total = g
        .iter()
        .filter(|(_, value)| **value == Object::Box)
        .fold(0, |acc, (k, _)| acc + k.x + k.y * 100);

    Some(total as u64)
}

fn find_all_mover(g: &HashMap<IVec2, Object>, d: IVec2, fish_pos: IVec2) -> Option<Vec<IVec2>> {
    let mut tomove: Vec<IVec2> = vec![fish_pos.clone()];
    let mut next = fish_pos + d;
    while let Some(obstacle) = g.get(&next) {
        println!("  found {:#?} at {}", obstacle, next);
        match obstacle {
            Object::Wall => return None,
            Object::Box => tomove.push(next.clone()),
            _ => panic!("nothing else should be here"),
        }
        next = next + d;
    }
    return Some(tomove);
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Object {
    Wall,
    Box,
    Fish,
}

fn parse_input(input: &str) -> (HashMap<IVec2, Object>, Vec<IVec2>) {
    let mut it = input.split("\n\n");
    let grid = it.next().unwrap();
    let movements = it.next().unwrap();

    let mut g: HashMap<IVec2, Object> = HashMap::default();
    for (y, line) in grid.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let object = match c {
                '#' => Object::Wall,
                '@' => Object::Fish,
                'O' => Object::Box,
                '.' => continue,
                _ => panic!("bad char"),
            };
            g.insert(
                IVec2 {
                    x: x as i32,
                    y: y as i32,
                },
                object,
            );
        }
    }
    let mov: Vec<_> = movements
        .lines()
        .flat_map(|line| line.chars().into_iter().map(c_to_IVec2))
        .collect();
    println!("movements: {:#?}", mov);
    return (g, mov);
}

fn c_to_IVec2(c: char) -> IVec2 {
    match c {
        '^' => IVec2 { x: 0, y: -1 },
        '>' => IVec2 { x: 1, y: 0 },
        '<' => IVec2 { x: -1, y: 0 },
        'v' => IVec2 { x: 0, y: 1 },
        _ => panic!("invalid direction"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2028));

        let result_large = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result_large, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
