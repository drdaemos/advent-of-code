use std::collections::HashMap;

use advent_of_code::utils::get_file_contents;
use itertools::Itertools;

type Pos = (usize, usize);
type Edges = Vec<Pos>;
type Graph = (Pos, HashMap<Pos, Edges>);

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Node {
    pos: Pos,
    edges: Edges,
    start: bool,
}

pub fn day10() {
    let input = get_file_contents("2023/day10/input.txt");
    println!("Steps: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    let (start, graph) = parse_graph(input);

    // TODO: BFS to find longest path in graph from start

    println!("{:?}", graph);
    println!("Start: {:?}", start);
    return 0;
}

fn part_two(input: &str) -> usize {
    return 0;
}

fn parse_graph(input: &str) -> Graph {
    let lines = input.trim().split("\n").enumerate().collect_vec();

    let nodes = lines
        .iter()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| parse_node((x, *y), char, &lines))
        })
        .flatten()
        .filter_map(|node| node)
        .collect_vec();

    let start = nodes.iter().find(|node| node.start).unwrap().pos;
    let graph = HashMap::from_iter(
        nodes
            .into_iter()
            .map(|node| (node.pos, node.edges))
            .collect_vec(),
    );

    return (start, graph);
}

fn parse_node(pos: Pos, char: char, lines: &Vec<(usize, &str)>) -> Option<Node> {
    if char == '.' {
        return None;
    }

    let x = pos.0 as isize;
    let y = pos.1 as isize;

    let edges = match char {
        '|' => vec![(x, y - 1), (x, y + 1)],
        '-' => vec![(x - 1, y), (x + 1, y)],
        'L' => vec![(x, y - 1), (x + 1, y)],
        'J' => vec![(x, y - 1), (x - 1, y)],
        '7' => vec![(x, y + 1), (x - 1, y)],
        'F' => vec![(x, y + 1), (x + 1, y)],
        'S' => vec![(x, y - 1), (x, y + 1), (x + 1, y), (x - 1, y)],
        _ => panic!("unknown node type"),
    };

    return Some(Node {
        pos: (x as usize, y as usize),
        edges: filter_edges(edges, lines),
        start: char == 'S',
    });
}

fn filter_edges(edges: Vec<(isize, isize)>, lines: &Vec<(usize, &str)>) -> Vec<Pos> {
    return edges
        .into_iter()
        .filter(|edge| edge.0 >= 0 && edge.1 >= 0)
        .map(|edge| (edge.0 as usize, edge.1 as usize))
        .filter(|edge| {
            lines[edge.1]
                .1
                .chars()
                .nth(edge.0)
                .is_some_and(|char| char != '.')
        })
        .collect_vec();
}

#[cfg(test)]
mod tests {
    use crate::day10::{part_one, part_two};

    const INPUT: &str = "
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

    const INPUT_2: &str = "
.....
.S-7.
.|.|.
.L-J.
.....
";

    #[test]
    fn part_one_test() {
        // assert_eq!(part_one(INPUT), 8);
        assert_eq!(part_one(INPUT_2), 4);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(INPUT), 0);
    }
}
