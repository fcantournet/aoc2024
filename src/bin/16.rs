use pathfinding::prelude::*;
use std::collections::{HashSet, VecDeque};

use glam::IVec2;

advent_of_code::solution!(16);

const DIRS: [IVec2; 4] = [
    // N,E,S,W
    IVec2 { x: 0, y: -1 },
    IVec2 { x: 1, y: 0 },
    IVec2 { x: 0, y: 1 },
    IVec2 { x: -1, y: 0 },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: IVec2,
    dir: usize,
}

// returns next possible tiles + associated cost to move there.
fn next(grid: &HashSet<IVec2>, s: &State, base_cost: usize, back: i32) -> Vec<(State, usize)> {
    let mut next = Vec::new();
    for i in -1i32..3 {
        let dir = (s.dir as i32 + i).rem_euclid(4) as usize;
        let pos = s.pos + back * DIRS[dir];
        let cost = i.abs() as usize * 1000 + 1 + base_cost;
        if grid.contains(&pos) {
            next.push((State { pos, dir }, cost));
        }
    }
    next
}

fn next_symetrical(grid: &HashSet<IVec2>, s: &State, back: i32) -> Vec<(State, usize)> {
    let mut next = Vec::new();
    // try going forward
    let pos = s.pos + back * DIRS[s.dir];
    if grid.contains(&pos) {
        next.push((State { pos, dir: s.dir }, 1));
    }
    // Test if rotating will allow to move forward next
    for i in [-1i32, 1, 2] {
        let dir = (s.dir as i32 + i).rem_euclid(4) as usize;
        let next_pos = s.pos + back * DIRS[dir];
        let cost = i.abs() as usize * 1000;
        if grid.contains(&next_pos) {
            // We don't skip directly to the pos after rotate + walk
            // We store the state when we only rotated, so that the path can be walked back.
            next.push((State { pos: s.pos, dir }, cost));
        }
    }
    next
}

pub fn part_one(input: &str) -> Option<usize> {
    let (grid, start, end, (_, _)) = parse_input(input);

    println!("From {:?} to {:?}", start, end);
    let paths = dijkstra(
        &State { pos: start, dir: 1 },
        |s| next(&grid, s, 0, 1),
        |s| s.pos == end,
    );
    Some(paths.unwrap().1)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (grid, start, end, (xmax, ymax)) = parse_input(input);

    println!("From {:?} to {:?}", start, end);
    let path = dijkstra(
        &State { pos: start, dir: 1 },
        |s| next(&grid, s, 0, 1),
        |s| s.pos == end,
    )?;

    let from_start = dijkstra_all(&State { pos: start, dir: 1 }, |s| {
        next_symetrical(&grid, s, 1)
    });
    let from_end = dijkstra_all(path.0.last().unwrap(), |s| next_symetrical(&grid, s, -1));

    assert_eq!(from_start[&path.0[17]].1 + from_end[&path.0[17]].1, path.1);

    // Just walk from the start with NEXT and check each new state with the assert above,
    // and push those that match in a queue (and a set I guess)
    //
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    let mut good = HashSet::new();
    q.push_back(State { pos: start, dir: 1 });
    while let Some(s) = q.pop_front() {
        let nexts = next(&grid, &s, 0, 1);
        nexts.iter().for_each(|(ns, _)| {
            if !seen.contains(ns) {
                let fs = from_start.get(ns);
                let fe = from_end.get(ns);
                match (fs, fe) {
                    // (None, None) => unreachable!("{:?} neither in fs or fe", ns),
                    // (None, Some(_)) => unreachable!("{:?} not in fs", ns),
                    // (Some(_), None) => unreachable!("{:?} not in fe", ns),
                    (Some(fs), Some(fe)) => {
                        if fs.1 + fe.1 == path.1 {
                            // this is also part of a best path
                            good.insert(ns.pos);
                            q.push_back(*ns);
                        }
                    }
                    (_, _) => {}
                }
                seen.insert(*ns);
            }
        });
    }
    good.insert(start);
    good.insert(end);

    println!(
        "Visited {} states and found {} tiles in good paths",
        seen.len(),
        good.len()
    );
    debug_paths(&grid, &good, xmax, ymax);

    Some(good.len())
}

fn debug_paths(grid: &HashSet<IVec2>, paths: &HashSet<IVec2>, xmax: i32, ymax: i32) {
    for y in 0..=ymax {
        for x in 0..=xmax {
            let coords = IVec2::new(x, y);
            if paths.contains(&coords) {
                print!("0");
            } else if grid.contains(&coords) {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
}

fn parse_input(input: &str) -> (HashSet<IVec2>, IVec2, IVec2, (i32, i32)) {
    let mut grid = HashSet::new();
    let mut start = IVec2::default();
    let mut end = IVec2::default();
    let mut max_y = 0;
    let mut max_x = 0;
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            max_x = max_x.max(x);
            match c {
                '.' => _ = grid.insert(IVec2::new(x as i32, y as i32)),
                'S' => {
                    start = IVec2::new(x as i32, y as i32);
                    grid.insert(start);
                }
                'E' => {
                    end = IVec2::new(x as i32, y as i32);
                    grid.insert(end);
                }
                _ => continue,
            }
        }
        max_y = max_y.max(y);
    }
    (grid, start, end, (max_x as i32, max_y as i32))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
