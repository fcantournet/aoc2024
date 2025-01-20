use std::collections::{HashMap, HashSet, VecDeque};

use glam::IVec2;

advent_of_code::solution!(12);

const AROUND: [IVec2; 4] = [
    IVec2 { x: 0, y: 1 },
    IVec2 { x: 0, y: -1 },
    IVec2 { x: 1, y: 0 },
    IVec2 { x: -1, y: 0 },
];

#[derive(Debug, Clone)]
struct Region {
    name: char,
    id: usize,
    tiles: HashSet<IVec2>,
}

fn find_regions(grid: &HashMap<IVec2, char>) -> HashMap<usize, Region> {
    // map from pos to region ID
    let mut pos_to_region: HashMap<IVec2, usize> = HashMap::new();
    let mut regions: HashMap<usize, Region> = HashMap::new();
    let mut next_region_id = 0;

    for tile in grid.iter() {
        if pos_to_region.get(tile.0).is_some() {
            continue; // We already mapped this to a region.
        }
        // New region ! Let's map it !
        next_region_id += 1;
        let mut region = Region {
            name: *tile.1,
            id: next_region_id,
            tiles: [*tile.0].into(),
        };
        pos_to_region.insert(*tile.0, region.id);

        let mut to_visit = VecDeque::new();
        to_visit.push_back(*tile.0);
        while let Some(visiting) = to_visit.pop_front() {
            for dir in AROUND {
                let next = visiting + dir;
                if let Some(n_char) = grid.get(&next) {
                    if n_char == tile.1 {
                        // this next tile is in region.
                        if let Some(id) = pos_to_region.get(&next) {
                            assert!(*id == region.id);
                            continue;
                        }
                        to_visit.push_back(next); // add it to the stuff to check for further neighbourgs is region
                        region.tiles.insert(next);
                        pos_to_region.insert(next, region.id);
                    }
                }
            }
        }
        regions.insert(region.id, region);
    }
    regions
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse_input(input);
    let regions = find_regions(&grid);

    let mut result = 0;
    for region in regions.values() {
        let area = region.tiles.len();
        let peri = perimeter(region);
        println!(
            "region {} has area {} and perimeter {}",
            region.name,
            &area,
            &peri.len()
        );
        result += area * peri.len();
    }
    Some(result)
}

fn perimeter(region: &Region) -> HashSet<(IVec2, IVec2)> {
    let mut perimeter = HashSet::new();
    for tile in region.tiles.iter() {
        for dir in AROUND {
            if region.tiles.get(&(tile + dir)).is_none() {
                perimeter.insert((*tile, dir));
            }
        }
    }
    perimeter
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse_input(input);
    let regions = find_regions(&grid);

    let mut result = 0;
    for region in regions.values() {
        let area = region.tiles.len();
        let sides = sides(region);
        println!(
            "region {} has area {} and # of sides {}",
            region.name, &area, &sides
        );
        result += area * sides;
    }
    Some(result)
}

// Sides computes the number of conituous segments of the perimeter.
// For each element of the perimeter, we look to the perpendicular direction
// of the normale to see if the perimeter contains another element with
// the same normale, if not we have reached a corner.
// number of corner == number of segments.
fn sides(region: &Region) -> usize {
    let perimeter = perimeter(region);
    let mut sides = 0;
    for (tile, normale) in perimeter.iter() {
        let dir = normale.perp();
        let next = tile + dir;
        if perimeter.get(&(next, *normale)).is_none() {
            sides += 1;
        }
    }
    sides
}

fn parse_input(input: &str) -> HashMap<IVec2, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                (
                    IVec2 {
                        x: x as i32,
                        y: y as i32,
                    },
                    c,
                )
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
