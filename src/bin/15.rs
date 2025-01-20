use std::collections::{HashMap, HashSet, VecDeque};

use glam::IVec2;
advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u64> {
    let (mut g, dir) = parse_input(input);
    let mut fish_pos = *g.iter().find(|(_, val)| **val == Object::Fish).unwrap().0;

    for d in dir {
        //println!("Fish at {} moving {}", fish_pos, d);

        if let Some(tomove) = find_all_mover(&g, d, fish_pos) {
            //println!("to_move: {:#?}", tomove);
            let mut to_resinsert: Vec<_> = Vec::default();
            for e in tomove.iter() {
                if let Some(present) = g.remove(e) {
                    to_resinsert.push((e + d, present));
                } else {
                    panic!("moving non-existing object")
                }
            }
            //println!("reinserting: {:#?}", to_resinsert);
            for (pos, o) in to_resinsert {
                g.insert(pos, o);
            }
            fish_pos += d;
        }
    }

    let total = g
        .iter()
        .filter(|(_, value)| **value == Object::Box)
        .fold(0, |acc, (k, _)| acc + k.x + k.y * 100);

    Some(total as u64)
}

fn find_all_mover(g: &HashMap<IVec2, Object>, d: IVec2, fish_pos: IVec2) -> Option<Vec<IVec2>> {
    let mut tomove = HashSet::new();
    tomove.insert(fish_pos);

    let mut tocheck = VecDeque::new();
    tocheck.push_back(fish_pos);

    while let Some(current) = tocheck.pop_front() {
        let next = current + d;
        match g.get(&next) {
            Some(Object::Wall) => return None,
            Some(Object::Fish) => panic!("There is only 1 fish"),
            Some(Object::Box) => {
                if !tomove.contains(&next) {
                    _ = tomove.insert(next);
                    tocheck.push_back(next);
                }
            }
            Some(Object::BoxLeft) => {
                if !tomove.contains(&next) {
                    _ = tomove.insert(next);
                    tocheck.push_back(next);
                }
                let rightbox = next + IVec2 { x: 1, y: 0 };
                if !tomove.contains(&rightbox) {
                    _ = tomove.insert(rightbox);
                    tocheck.push_back(rightbox);
                }
            }
            Some(Object::BoxRight) => {
                if !tomove.contains(&next) {
                    _ = tomove.insert(next);
                    tocheck.push_back(next);
                }
                let leftbox = next + IVec2 { x: -1, y: 0 };
                if !tomove.contains(&leftbox) {
                    _ = tomove.insert(leftbox);
                    tocheck.push_back(leftbox);
                }
            }
            None => continue,
        }
    }

    Some(tomove.into_iter().collect())

    // //let mut tomove: Vec<IVec2> = vec![fish_pos.clone()];
    // let mut next = fish_pos + d;
    // while let Some(obstacle) = g.get(&next) {
    //     println!("  found {:#?} at {}", obstacle, next);
    //     match obstacle {
    //         Object::Wall => return None,
    //         Object::Box => tomove.push(next.clone()),
    //         _ => panic!("nothing else should be here"),
    //     }
    //     next = next + d;
    // }
    // return Some(tomove);
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut g, dir) = parse_input2(input);
    let mut fish_pos = *g.iter().find(|(_, val)| **val == Object::Fish).unwrap().0;

    for d in dir {
        //println!("Fish at {} moving {}", fish_pos, d);

        if let Some(tomove) = find_all_mover(&g, d, fish_pos) {
            //println!("to_move: {:#?}", tomove);
            let mut to_resinsert: Vec<_> = Vec::default();
            for e in tomove.iter() {
                if let Some(present) = g.remove(e) {
                    to_resinsert.push((e + d, present));
                } else {
                    panic!("moving non-existing object")
                }
            }
            //println!("reinserting: {:#?}", to_resinsert);
            for (pos, o) in to_resinsert {
                g.insert(pos, o);
            }
            fish_pos += d;
        }
    }

    let total = g
        .iter()
        .filter(|(_, value)| **value == Object::BoxLeft)
        .fold(0, |acc, (k, _)| acc + k.x + k.y * 100);

    Some(total as u64)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Object {
    Wall,
    Box,
    BoxLeft,
    BoxRight,
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
        .flat_map(|line| line.chars().map(c_to_ivec2))
        .collect();
    //println!("movements: {:#?}", mov);
    (g, mov)
}

fn parse_input2(input: &str) -> (HashMap<IVec2, Object>, Vec<IVec2>) {
    let mut it = input.split("\n\n");
    let grid = it.next().unwrap();
    let movements = it.next().unwrap();

    let mut g: HashMap<IVec2, Object> = HashMap::default();
    for (y, line) in grid.lines().enumerate() {
        let line2: Vec<char> = line
            .chars()
            .flat_map(|c| match c {
                '#' => ['#', '#'],
                '@' => ['@', '.'],
                'O' => ['[', ']'],
                '.' => ['.', '.'],
                _ => panic!("bad char"),
            })
            .collect();
        for (x, c) in line2.iter().enumerate() {
            let object = match c {
                '#' => Object::Wall,
                '@' => Object::Fish,
                '[' => Object::BoxLeft,
                ']' => Object::BoxRight,
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
        .flat_map(|line| line.chars().map(c_to_ivec2))
        .collect();
    //println!("movements: {:#?}", mov);
    (g, mov)
}

fn c_to_ivec2(c: char) -> IVec2 {
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
        let result_large = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result_large, Some(9021));
    }
}
