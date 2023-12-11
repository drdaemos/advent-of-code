use std::collections::{HashMap, HashSet, VecDeque};

use advent_of_code::utils::get_file_contents;
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
    let (start, graph, _) = parse_graph(input);
    let (walk, _) = longest_path_bfs(&graph, start);
    let half_walk = (walk + 1) / 2;

    return half_walk;
}

fn part_two(input: &str) -> usize {
    // Get graph
    let (start, graph, width) = parse_graph(input);

    // Get loop polygon
    let (_, polygon) = longest_path_bfs(&graph, start);

    // Get parsed version of map because graph omits some unlinked nodes
    let mut map = parse_map(input);

    // Replace 'S' with appropriate pipe type
    map.insert(start, get_start_sub(graph.get(&start).unwrap().pos, &graph));

    // Run search for inner points
    return find_inner_points(width, &polygon, &map);
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

fn parse_graph(input: &str) -> (Pos, Graph, usize) {
    let lines = input.trim().split("\n").enumerate().collect_vec();
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

    return (start, graph, width);
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

fn find_inner_points(width: usize, polygon: &HashSet<Pos>, map: &HashMap<Pos, char>) -> usize {
    let outer_points = map
        .into_iter()
        .filter(|(pos, _)| !polygon.contains(pos))
        .map(|(pos, _)| pos)
        .collect_vec();

    let inner = outer_points
        .into_iter()
        .filter(|point| raycast_line(point, width, &polygon, map))
        .collect_vec();

    return inner.len();
}

fn raycast_line(
    point: &Pos,
    max_x: usize,
    polygon_set: &HashSet<Pos>,
    map: &HashMap<Pos, char>,
) -> bool {
    let mut crossings = 0;
    let mut prev: Option<char> = None;
    let y = point.1;
    if point.0 + 1 == max_x {
        return false;
    }

    for x in (point.0 + 1)..max_x {
        let current = &(x, y);
        let char = map.get(current);
        match polygon_set.contains(current) {
            true => match char {
                Some('|') => crossings += 1,
                Some('L') => prev = Some('L'),
                Some('F') => prev = Some('F'),
                Some('J') if prev.is_some_and(|ch| ch == 'F') => {
                    crossings += 1;
                    prev = None;
                }
                Some('7') if prev.is_some_and(|ch| ch == 'L') => {
                    crossings += 1;
                    prev = None;
                }
                Some('J') | Some('7') => prev = None,
                Some('-') => {}
                _ => todo!(),
            },
            false => {}
        }
    }

    return crossings % 2 != 0;
}

fn get_char((x, y): Pos, graph: &Graph) -> Option<char> {
    graph.get(&(x, y)).and_then(|f| Some(f.char))
}

fn from_top(ch: Option<char>) -> bool {
    match ch {
        Some('|') | Some('F') | Some('7') => true,
        _ => false,
    }
}

fn from_left(ch: Option<char>) -> bool {
    match ch {
        Some('-') | Some('F') | Some('L') => true,
        _ => false,
    }
}

fn from_bottom(ch: Option<char>) -> bool {
    match ch {
        Some('|') | Some('J') | Some('L') => true,
        _ => false,
    }
}

fn from_right(ch: Option<char>) -> bool {
    match ch {
        Some('-') | Some('J') | Some('7') => true,
        _ => false,
    }
}

fn get_start_sub((x, y): Pos, graph: &Graph) -> char {
    let top = if y > 0 {
        get_char((x, y - 1), graph)
    } else {
        None
    };
    let left = if x > 0 {
        get_char((x - 1, y), graph)
    } else {
        None
    };
    let bottom = get_char((x, y + 1), graph);
    let right = get_char((x + 1, y), graph);

    return match (top, left, bottom, right) {
        (top, left, _, _) if from_top(top) && from_left(left) => 'J',
        (top, _, _, right) if from_top(top) && from_right(right) => 'L',
        (_, _, bottom, right) if from_bottom(bottom) && from_right(right) => 'F',
        (_, left, bottom, _) if from_bottom(bottom) && from_left(left) => '7',
        (top, _, bottom, _) if from_top(top) && from_bottom(bottom) => '|',
        (_, left, _, right) if from_left(left) && from_right(right) => '-',
        _ => todo!(),
    };
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
