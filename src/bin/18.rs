use std::collections::{HashMap, VecDeque};

use pathfinding::prelude::bfs;

use glam::IVec2;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<usize> {
    part_one_with_size(input, (71, 71), 1024)
}

fn part_one_with_size(input: &str, size: (usize, usize), fallen: usize) -> Option<usize> {
    let falling = parse_input(input);
    let mut grid: HashMap<IVec2, bool> = HashMap::new();
    grid.reserve(size.0 * size.1);
    for x in 0..size.0 {
        for y in 0..size.0 {
            grid.insert(IVec2::new(x as i32, y as i32), false);
        }
    }
    for f in falling.iter().take(fallen) {
        grid.insert(*f, true);
    }

    let start = IVec2::new(0, 0);
    let end = IVec2::new(size.0 as i32 - 1, size.1 as i32 - 1);

    // mydfs(&grid, start, end)

    let path = bfs(
        &start,
        |pos| {
            let mut next = Vec::new();
            for dir in UPDOWNRIGHTLEFT.iter() {
                let neighbourg = pos + dir;
                if let Some(blocked) = grid.get(&neighbourg) {
                    if !blocked {
                        next.push(neighbourg);
                    }
                }
            }
            next
        },
        |pos| *pos == end,
    );
    path.map(|p| p.len())
}

const UPDOWNRIGHTLEFT: [IVec2; 4] = [IVec2::NEG_Y, IVec2::Y, IVec2::X, IVec2::NEG_X];

fn mydfs(grid: &HashMap<IVec2, bool>, start: IVec2, end: IVec2) -> Option<usize> {
    println!("going from {} to {}", start, end);
    let mut to_visit = VecDeque::new();
    to_visit.push_back((start, 0usize));

    let mut visited = HashMap::new();

    let mut shortest = usize::MAX;
    while let Some((node, path_len)) = to_visit.pop_front() {
        if let Some(n) = visited.get(&node) {
            if *n <= path_len {
                continue;
            }
        }
        // println!("visiting {} at path_len {}", node, path_len);
        visited.insert(node, path_len);
        if node == end {
            shortest = shortest.min(path_len);
            println!("found {} at path_len {}", node, path_len);
            continue;
        }
        if path_len >= shortest {
            continue;
        }

        for dir in UPDOWNRIGHTLEFT.iter() {
            let neighbourg = node + dir;
            if let Some(blocked) = grid.get(&neighbourg) {
                if !blocked {
                    // println!(
                    //     "  adding {} to queue at path_len {}",
                    //     neighbourg,
                    //     path_len + 1
                    // );
                    to_visit.push_front((neighbourg, path_len + 1));
                }
            }
        }
    }
    Some(shortest)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn parse_input(input: &str) -> Vec<IVec2> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split(",");
            let x = it.next().unwrap().parse().unwrap();
            let y = it.next().unwrap().parse().unwrap();
            IVec2::new(x, y)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_with_size(
            &advent_of_code::template::read_file("examples", DAY),
            (7, 7),
            12,
        );
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
