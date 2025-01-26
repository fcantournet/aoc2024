use std::collections::HashMap;

use itertools::Itertools;
use std::ops::Range;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let (mut blocks, _, _) = parse_input(input);

    let mut pointer = blocks.len() - 1;
    let mut free_head = 0;

    while pointer > free_head {
        if blocks[free_head].is_none() {
            blocks[free_head] = blocks[pointer];
            blocks[pointer] = None;
            pointer -= 1;
        } else {
            free_head += 1;
        }
    }
    checksum(blocks)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut blocks, mut free_segmentsv, mut file_segments) = parse_input(input);

    let mut free_segments = free_segmentsv.as_mut_slice();

    // dbg_blocks(&blocks);
    // println!("{:?}", free_segments);
    // println!("{:?}", file_segments);
    for (id, range) in file_segments.into_iter().rev() {
        let size_to_move = range.len();
        // println!(
        //     "Trying to compact id {} of size {} ({:?})",
        //     id, size_to_move, range
        // );

        let cut_off = free_segments
            .partition_point(|fs| fs.is_none() || fs.clone().unwrap().start < range.start);

        free_segments = &mut free_segments[..cut_off];

        for (index, mut segment) in free_segments.iter_mut().enumerate() {
            if let Some(free_range) = segment {
                if range.len() <= free_range.len() && free_range.start < range.start {
                    // println!(
                    //     "moving id {} of size {} from ({:?}) to ({:?})",
                    //     id, size_to_move, &range, free_range
                    // );
                    match free_range.len() - range.len() {
                        0 => {
                            blocks[free_range.clone()].fill(Some(id));
                            blocks[range.clone()].fill(None);
                            // I really need to accept that dereferncing like this is The Right Way.
                            *segment = None;
                        }
                        i if i > 0 => {
                            blocks[free_range.start..free_range.start + range.len()].fill(Some(id));
                            blocks[range.clone()].fill(None);
                            free_range.start += range.len();
                            // println!("resized free segment to {:#?}", segment);
                        }
                        _ => unreachable!(),
                    }
                    break;
                }
            }
        }

        // println!("remainging free_space {:#?}", free_segments);
        // dbg_blocks(&blocks);
    }

    // dbg_blocks(&blocks);
    checksum(blocks)
}

fn dbg_blocks(blocks: &[Option<usize>]) {
    for b in blocks {
        match b {
            Some(v) => print!("{}", v),
            None => print!("."),
        }
    }
    print!("\n");
}

fn checksum(blocks: Vec<Option<usize>>) -> Option<usize> {
    blocks
        .iter()
        .enumerate()
        .filter_map(|(i, b)| b.map(|id| id * i))
        .reduce(|acc, e| acc + e)
}

type FreeSegments = Vec<Option<Range<usize>>>;
type FileSegments = Vec<(usize, Range<usize>)>;

fn parse_input(input: &str) -> (Vec<Option<usize>>, FreeSegments, FileSegments) {
    let mut it = input.trim().chars();
    // let mut id = 0;
    let mut blocks = Vec::with_capacity(1 << 14);
    let mut free_segments = FreeSegments::new();
    let mut file_segments = FileSegments::new();

    let mut id = 0;
    while let Some(file) = it.next() {
        let size = file.to_digit(10).unwrap() as usize;
        for _ in 0..size {
            blocks.push(Some(id));
        }
        file_segments.push((id, blocks.len() - size..blocks.len()));

        if let Some(free) = it.next() {
            let size = free.to_digit(10).unwrap() as usize;
            for _ in 0..size {
                blocks.push(None);
            }
            free_segments.push(Some(blocks.len() - size..blocks.len()));
        }
        id += 1;
    }
    (blocks, free_segments, file_segments)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
