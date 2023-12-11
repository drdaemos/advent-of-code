use std::collections::{HashMap, HashSet, VecDeque};

use advent_of_code::{
    terminal::{clear_screen, render_map, sub_pipes},
    utils::get_file_contents,
};
use itertools::Itertools;

type Pos = (usize, usize);
type Edges = Vec<Pos>;
type Graph = HashMap<Pos, Node>;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Node {
    pos: Pos,
    edges: Edges,
    start: bool,
    char: char,
}

pub fn day10() {
    let input = get_file_contents("2023/day10/input.txt");
    println!("Steps to furthest point: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    let (start, graph, _, _) = parse_graph(input);
    let (walk, _) = longest_path_bfs(&graph, start);
    let half_walk = (walk + 1) / 2;

    return half_walk;
}

fn part_two(input: &str) -> usize {
    let (start, graph, height, width) = parse_graph(input);

    let (_, loop_set) = longest_path_bfs(&graph, start);

    return (0..width)
        .map(|y| find_points_inside(&loop_set, &parse_map(input), width, y))
        .sum();
}

fn parse_map(input: &str) -> HashMap<Pos, char> {
    return HashMap::from_iter(
        input
            .trim()
            .split("\n")
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, char)| ((x, y), char))
            })
            .flatten(),
    );
}

fn parse_graph(input: &str) -> (Pos, Graph, usize, usize) {
    let lines = input.trim().split("\n").enumerate().collect_vec();
    let height = lines.len();
    let width = lines[0].1.len();

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
    let graph = HashMap::from_iter(nodes.into_iter().map(|node| (node.pos, node)));

    return (start, graph, height, width);
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
        char,
    });
}

fn filter_edges(edges: Vec<(isize, isize)>, lines: &Vec<(usize, &str)>) -> Vec<Pos> {
    let hor_length = lines[0].1.len() as isize;

    return edges
        .into_iter()
        .filter(|edge| {
            edge.0 >= 0 && edge.1 >= 0 && edge.0 < hor_length && edge.1 < lines.len() as isize
        })
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

fn longest_path_bfs(graph: &Graph, start: Pos) -> (usize, HashSet<Pos>) {
    let mut max_walk = 0;
    let mut max_visited: HashSet<Pos> = HashSet::from([start]);

    // Queue will store a tuple of (current_node, path_length, visited_set)
    let mut queue = VecDeque::new();

    // Initialize the queue
    queue.push_back((start, 0, max_visited.clone()));

    while let Some((current_node, path_length, path_visited)) = queue.pop_front() {
        // Check all neighbors
        for &neighbor in &graph[&current_node].edges {
            // Only continue if we haven't visited this neighbor before
            if !path_visited.contains(&neighbor) {
                let mut new_visited = path_visited.clone();
                new_visited.insert(neighbor);

                // Update max distance if this path is farther
                if path_length + 1 > max_walk {
                    max_walk = path_length + 1;
                    max_visited = new_visited.clone();
                }

                // Add this path to the queue
                queue.push_back((neighbor, path_length + 1, new_visited));
            }
        }
    }

    // Return the length of the longest path found
    (max_walk, max_visited)
}

fn find_inner_points(
    width: usize,
    height: usize,
    loop_set: &HashSet<Pos>,
    map: &HashMap<Pos, char>,
) -> usize {
    let mut inner: HashSet<Pos> = HashSet::new();

    for y in 0..height {
        let new_points = raycast_line(y, width, loop_set, map);
        println!("line {} - {:?}", y, new_points);
        inner.extend(new_points);
    }

    return inner.len();
}

fn raycast_line<'a>(
    y: usize,
    max_x: usize,
    polygon_set: &HashSet<Pos>,
    map: &HashMap<Pos, char>,
) -> HashSet<Pos> {
    let mut left = 0;
    let mut right = 0;
    let mut inner_points = HashSet::new();

    for x in 0..max_x {
        let current = &(x, y);
        let char = map.get(current);
        match polygon_set.contains(current) {
            true => match char {
                Some('-') => {
                    left += 1;
                    right += 1;
                }
                Some('L') => right += 1,
                Some('F') => right += 1,
                Some('J') => left += 1,
                Some('7') => left += 1,
                _ => (),
            },
            false => {
                if left.min(right) % 2 == 1 {
                    inner_points.insert(current.clone());
                }
            }
        }
    }

    return inner_points;
}

fn find_points_inside(
    polygon: &HashSet<Pos>,
    map: &HashMap<Pos, char>,
    max_x: usize,
    y: usize,
) -> usize {
    // Note that it's possible for the 'S' to be a corner in some inputs but it wasn't in mine.
    // If it is, it needs to be added to these sets.
    // Ideally I should have replaced it when parsing the input.
    let walls: HashSet<char> = ['L', 'J', '7', 'F', '|'].iter().cloned().collect();
    let corners: HashSet<char> = ['L', 'J', '7', 'F'].iter().cloned().collect();

    let mut count = 0;
    let mut inside = false;
    let mut prev_corner: Option<char> = None;
    for x in 0..max_x {
        if polygon.contains(&(x, y)) {
            let current = &(x, y);
            let cell = map.get(current).unwrap();
            if walls.contains(&cell) {
                if corners.contains(&cell) {
                    if let Some(prev) = prev_corner {
                        // These corners extend the vertical edge we've already accounted for.
                        if (prev == 'L' && *cell == '7') || (prev == 'F' && *cell == 'J') {
                            prev_corner = None;
                            continue;
                        }
                    }
                    prev_corner = Some(*cell);
                }
                inside = !inside;
            }
            continue;
        }
        if inside {
            count += 1;
        }
    }
    count
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

    const INPUT_3: &str = "
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

    const INPUT_4: &str = "
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(INPUT), 8);
        assert_eq!(part_one(INPUT_2), 4);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(INPUT), 1);
        assert_eq!(part_two(INPUT_2), 1);
        assert_eq!(part_two(INPUT_3), 8);
        assert_eq!(part_two(INPUT_4), 10);
    }
}
