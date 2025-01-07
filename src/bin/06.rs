use rayon::prelude::*;
use std::collections::HashSet;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let (grid, position, orientation) = parse(input);
    let visited = basic_path(&grid, position, orientation);
    Some(visited.len() as u64)
}

fn basic_path(
    grid: &Grid,
    mut position: Position,
    mut orientation: Orientation,
) -> HashSet<Position> {
    let mut visited: HashSet<Position> = HashSet::default();
    while grid.is_not_out(&position) {
        visited.insert(position.clone());
        (position, orientation) = grid.step(&position, &orientation);
    }
    return visited;
}

pub fn part_two(input: &str) -> Option<u64> {
    let (grid, starting_position, starting_orientation) = parse(input);

    let paths = basic_path(
        &grid,
        starting_position.clone(),
        starting_orientation.clone(),
    );

    let loop_makers: HashSet<Position> = paths
        .par_iter()
        .cloned()
        .filter(|pos| *pos != starting_position) // cannot insert at starting position
        .filter(|pos| {
            loops_if_we_add_a_box(
                grid.clone(),
                starting_position.clone(),
                starting_orientation.clone(),
                pos.clone(),
            )
        })
        .collect();

    Some(loop_makers.len() as u64)
}

fn loops_if_we_add_a_box(
    mut grid: Grid,
    mut position: Position,
    mut orientation: Orientation,
    box_position: Position,
) -> bool {
    // add the box durably to the grid for this loop test.
    grid.obstacles.insert(box_position);
    let mut visited: HashSet<(Position, Orientation)> = HashSet::default();

    let mut steps = 0;
    while grid.is_not_out(&position) {
        if visited.contains(&(position.clone(), orientation.clone())) {
            // println!(
            //     "Already visited this: {:#?} {:#?} {} steps: {:#?}",
            //     position, orientation, steps, visited
            // );
            return true;
        }
        visited.insert((position.clone(), orientation.clone()));
        (position, orientation) = grid.step(&position, &orientation);
        steps += 1;
        if steps > 100000 {
            panic!("megaloop1");
        }
    }
    return false;
}

fn parse(input: &str) -> (Grid, Position, Orientation) {
    let mut y = 0;
    let mut grid = Grid {
        obstacles: HashSet::default(),
        size: (0, 0),
    };
    let mut start = Position::default();

    for line in input.lines() {
        let mut x = 0;
        for c in line.chars() {
            match c {
                '#' => _ = grid.obstacles.insert(Position { x, y }),
                '^' => start = Position { x, y },
                _ => (),
            }
            x += 1;
        }
        y += 1;
        grid.size = (x, y);
    }

    //println!("grid: {:#?} start: {:#?}", grid, start);
    return (grid, start, Orientation::Up);
}

#[derive(Debug, Clone)]
struct Grid {
    size: (isize, isize),
    obstacles: HashSet<Position>,
}

impl Grid {
    fn step(&self, position: &Position, orientation: &Orientation) -> (Position, Orientation) {
        let next_position = match orientation {
            Orientation::Up => Position {
                x: position.x,
                y: position.y - 1,
            },
            Orientation::Down => Position {
                x: position.x,
                y: position.y + 1,
            },
            Orientation::Right => Position {
                x: position.x + 1,
                y: position.y,
            },
            Orientation::Left => Position {
                x: position.x - 1,
                y: position.y,
            },
        };
        if self.obstacles.contains(&next_position) {
            return (position.clone(), orientation.rotate90());
        }
        return (next_position, orientation.clone());
    }

    fn is_not_out(&self, position: &Position) -> bool {
        //println!("walking: {:#?}", position);
        position.x < self.size.0 && position.x >= 0 && position.y >= 0 && position.y < self.size.1
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Default, Debug)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

impl Orientation {
    fn rotate90(&self) -> Orientation {
        match self {
            Orientation::Up => Orientation::Right,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
