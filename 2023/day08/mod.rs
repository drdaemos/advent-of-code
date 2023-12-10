use std::collections::HashMap;

use advent_of_code::{math::lcm_of_vec, utils::get_file_contents};
use itertools::Itertools;
use regex::Regex;

type NodeMap = HashMap<String, (String, String)>;

pub fn day08() {
    let input = get_file_contents("2023/day08/input.txt");
    println!("Steps: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    let instructions = parse_instructions(input);
    let nodes = parse_nodes(input);
    return follow_instructions(instructions, &nodes, "AAA", "ZZZ");
}

fn part_two(input: &str) -> usize {
    let instructions = parse_instructions(input);
    let nodes = parse_nodes(input);
    let starts = nodes.keys().filter(|key| key.ends_with("A")).collect_vec();
    let mut steps = vec![];
    for start in starts {
        steps.push(follow_instructions(instructions, &nodes, start, "Z"))
    }
    return lcm_of_vec(&steps);
}

fn follow_instructions(instructions: &str, nodes: &NodeMap, start: &str, ends_with: &str) -> usize {
    let mut current = start;
    let mut steps_count = 0;

    for step in instructions.chars().cycle() {
        if current.ends_with(ends_with) {
            return steps_count;
        }

        current = match step {
            'L' => &nodes.get(current).expect("map node").0,
            'R' => &nodes.get(current).expect("map node").1,
            _ => panic!("Unknown move"),
        };
        steps_count += 1;
    }

    return steps_count;
}

fn parse_instructions(input: &str) -> &str {
    return input.trim().split("\n\n").next().expect("first line");
}

fn parse_nodes(input: &str) -> NodeMap {
    return input
        .trim()
        .split("\n\n")
        .last()
        .expect("map block")
        .split("\n")
        .map(line_to_node)
        .collect::<NodeMap>();
}

fn line_to_node(line: &str) -> (String, (String, String)) {
    let re = Regex::new(r"(?P<key>\w+) = \((?P<left>\w+), (?P<right>\w+)\)").unwrap();
    let caps = re.captures_iter(line).next().unwrap();

    return (
        caps["key"].to_string(),
        (caps["left"].to_string(), caps["right"].to_string()),
    );
}

#[cfg(test)]
mod tests {
    use crate::day08::{part_one, part_two};

    const INPUT: &str = "
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    const INPUT_2: &str = "
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

    const INPUT_3: &str = "
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(INPUT), 6);
        assert_eq!(part_one(INPUT_2), 2);
    }

    #[test]
    fn part_two_test() {
        let result = part_two(INPUT_3);
        assert_eq!(result, 6);
    }
}
