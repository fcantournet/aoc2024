// use petgraph::graph::UnGraph;
// use petgraph::matrix_graph::{NodeIndex, UnMatrix};
use petgraph::prelude::UnGraphMap;
use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<usize> {
    let (_, edges) = parse_input(input).unwrap();
    // let adj = adjacencies(edges);
    // let long: HashSet<String> = adj
    //     .into_iter()
    //     .filter_map(|(node, neighbourgs)| {
    //         if neighbourgs.len() > 2 {
    //             if node.starts_with("t") || neighbourgs.iter().any(|n| n.starts_with("t")) {
    //                 let mut next = neighbourgs.clone();
    //                 next.push(node);
    //                 next.sort();
    //                 return Some((next.join("-")));
    //             }
    //         }
    //         None
    //     })
    //     .collect();

    // println!("{:?}", long);
    // let to_usize = |b: u8| (b - b'a') as usize;
    // let to_index = |b: &[u8]| 26 * to_usize(b[0]) + to_usize(b[1]);
    // let pg = UnGraph::<&str, &str>::from_edges(edges);
    // .into_iter()
    // .map(|(a, b)| (to_index(a.as_bytes()), to_index(a.as_bytes()))),
    //);

    let pg = UnGraphMap::<&str, ()>::from_edges(edges);

    Some(1)
}

// fn nodify(node: &str) -> usize {
//     node[0] as byte * 26 + node[1] as byte
// }

fn adjacencies<'a>(edges: Vec<(&'a str, &'a str)>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut adj = HashMap::new();
    for edge in edges {
        adj.entry(edge.0)
            .and_modify(|neighbourgs: &mut Vec<&str>| neighbourgs.push(edge.1))
            .or_insert(vec![edge.1]);
        adj.entry(edge.1)
            .and_modify(|neighbourgs: &mut Vec<&str>| neighbourgs.push(edge.0))
            .or_insert(vec![edge.0]);
    }

    println!("{:#?}", adj);
    adj
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn parse_input(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list1(line_ending, separated_pair(alpha1, tag("-"), alpha1))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
