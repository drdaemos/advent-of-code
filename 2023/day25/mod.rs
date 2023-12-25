use std::collections::HashSet;

use advent_of_code::utils::get_file_contents;
use itertools::Itertools;
use rand::{seq::SliceRandom, thread_rng};

struct Graph {
    nodes: HashSet<String>,
    edges: Vec<Edge>,
}

trait GraphTrait {
    fn new() -> Self;
    fn find_min_cut(&self, size: usize) -> Vec<Vec<String>>;
    fn contract_edge(&mut self, edge: &Edge);
    fn insert(&mut self, edge: Edge);
}

impl GraphTrait for Graph {
    fn new() -> Self {
        return Graph {
            edges: vec![],
            nodes: HashSet::new(),
        };
    }

    // Will return groups of nodes
    fn find_min_cut(&self, size: usize) -> Vec<Vec<String>> {
        let mut rng = thread_rng();
        let mut min_cut = self.edges.len();
        let mut node_groups: Vec<String> = vec![];

        while min_cut > size {
            // execute Karger's algorithm and randomly contract edges
            let mut copy: Graph = Graph {
                nodes: self.nodes.to_owned(),
                edges: self.edges.to_owned(),
            };

            while copy.nodes.len() > 2 {
                let edge = copy.edges.choose(&mut rng).unwrap().clone();
                copy.contract_edge(&edge);
            }

            if min_cut > copy.edges.len() {
                min_cut = copy.edges.len();
                node_groups = Vec::from_iter(copy.nodes);
            }

            println!("Found cut of {:?}", copy.edges.len());
        }

        return node_groups
            .into_iter()
            .map(|group| group.split(":").map(|part| part.to_owned()).collect_vec())
            .collect_vec();
    }

    fn contract_edge(&mut self, edge: &Edge) {
        // create new node
        let new_node = format!("{}:{}", edge.0, edge.1);

        // remove edge
        self.edges.retain(|item| !item.equals(&edge));

        // rewire connected edges
        for node in [&edge.0, &edge.1] {
            let connected = self
                .edges
                .iter_mut()
                .filter(|item| item.contains_node(&node))
                .collect_vec();

            for edge in connected {
                edge.replace_node(&node, &new_node);
            }
        }

        // replace nodes with merged node
        self.nodes.retain(|node| !edge.contains_node(node));
        self.nodes.insert(new_node);
    }

    fn insert(&mut self, edge: Edge) {
        let (src, dest) = edge.clone();
        self.edges.push(edge);
        self.nodes.insert(src);
        self.nodes.insert(dest);
    }
}

type Edge = (String, String);

trait EdgeTrait {
    fn contains_node(&self, node: &String) -> bool;
    fn equals(&self, other: &Edge) -> bool;
    fn replace_node(&mut self, old: &String, new: &String);
}

impl EdgeTrait for Edge {
    fn contains_node(&self, node: &String) -> bool {
        return self.0 == *node || self.1 == *node;
    }
    fn equals(&self, other: &Edge) -> bool {
        return self.contains_node(&other.0) && self.contains_node(&other.1);
    }
    fn replace_node(&mut self, old: &String, new: &String) {
        if self.0 == *old {
            self.0 = new.to_string();
        }
        if self.1 == *old {
            self.1 = new.to_string();
        }
    }
}

pub fn day25() {
    let input = get_file_contents("2023/day25/input.txt");
    println!("Part one: {}", part_one(&input));
}

fn part_one(input: &str) -> usize {
    let graph = parse_graph(input);

    let node_groups = graph.find_min_cut(3);

    println!("{:?}", node_groups);
    return multiply_vec(
        node_groups
            .into_iter()
            .map(|group| group.len())
            .collect_vec(),
    );
}

fn multiply_vec(vec: Vec<usize>) -> usize {
    return vec.into_iter().reduce(|acc, i| acc * i).unwrap();
}

fn parse_graph(input: &str) -> Graph {
    let mut graph = Graph::new();

    let records = input.trim().lines();

    for line in records {
        let parts = line.split(":").collect_vec();
        let src = parts[0].trim();
        let dest = parts[1].trim().split(" ");
        for node in dest {
            graph.insert((src.to_owned(), node.to_owned()));
        }
    }

    return graph;
}

#[cfg(test)]
mod tests {
    use crate::day25::{part_one, Graph, GraphTrait};

    const INPUT: &str = "
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
";

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(INPUT), 54);
    }

    #[test]
    fn graph_contract_edge() {
        let mut graph = Graph {
            nodes: vec!["abc".to_string(), "def".to_string(), "hij".to_string()]
                .into_iter()
                .collect(),
            edges: vec![
                ("abc".to_string(), "def".to_string()),
                ("abc".to_string(), "hij".to_string()),
                ("hij".to_string(), "def".to_string()),
            ],
        };
        graph.contract_edge(&("hij".to_string(), "def".to_string()));

        assert_eq!(
            graph.nodes,
            vec!["abc".to_string(), "hij:def".to_string()]
                .into_iter()
                .collect()
        );
        assert_eq!(
            graph.edges,
            vec![
                ("abc".to_string(), "hij:def".to_string()),
                ("abc".to_string(), "hij:def".to_string())
            ]
        );
    }
}
