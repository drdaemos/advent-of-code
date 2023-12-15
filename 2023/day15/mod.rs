use std::collections::HashMap;
use std::hash::Hash;
use std::{collections::HashSet, usize};

use advent_of_code::utils::get_file_contents;
use indexmap::{indexmap, IndexMap};
use itertools::Either;
use itertools::Either::*;
use itertools::Itertools;

type RemoveOp = String;
type AddOp = (String, usize);
type Op = Either<RemoveOp, AddOp>;

pub fn day15() {
    let input = get_file_contents("2023/day15/input.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    return input.trim().split(",").map(calc_hash).sum();
}

fn part_two(input: &str) -> usize {
    let mut boxes = HashMap::<usize, IndexMap<String, usize>>::new();

    for operation in input.trim().split(",") {
        let op = parse_op(operation);

        match op {
            Left(label) => {
                boxes.entry(calc_hash(&label)).and_modify(|e| {
                    e.shift_remove(&label);
                });
            }
            Right((label, focal)) => {
                boxes
                    .entry(calc_hash(&label))
                    .and_modify(|e| upsert_map(e, &label, focal))
                    .or_insert(indexmap! { label => focal });
            }
        }
    }

    let mut sum: usize = 0;

    for (box_i, contents) in boxes.iter() {
        sum += contents
            .into_iter()
            .enumerate()
            .map(|(lens_i, (_, focal))| (lens_i + 1) * focal)
            .sum::<usize>()
            * (box_i + 1);
    }

    return sum;
}

fn upsert_map(map: &mut IndexMap<String, usize>, label: &String, value: usize) {
    map.entry(label.to_string())
        .and_modify(|val| *val = value)
        .or_insert(value);
}

fn parse_op(input: &str) -> Op {
    if input.contains('=') {
        let split = input.split("=").collect_vec();
        return Right((split[0].to_string(), split[1].parse().unwrap()));
    } else {
        return Left(input[..(input.len() - 1)].to_string());
    }
}

fn calc_hash(input: &str) -> usize {
    return input
        .chars()
        .fold(0, |acc, ch| ((acc + ch as usize) * 17) % 256);
}

#[cfg(test)]
mod tests {
    use crate::day15::{calc_hash, parse_op, part_one, part_two};

    const INPUT: &str = "
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(INPUT), 1320);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(INPUT), 145);
    }

    #[test]
    fn calc_hash_test() {
        assert_eq!(calc_hash("HASH"), 52);
        assert_eq!(calc_hash("rn"), 0);
        assert_eq!(calc_hash("pc"), 3);
        assert_eq!(calc_hash("cm"), 0);
    }

    #[test]
    fn parse_op_test() {
        let val = parse_op("rn=2");
        let unwrapped = val.unwrap_right();
        assert_eq!(unwrapped.0, "rn");
        assert_eq!(unwrapped.1, 2);

        let val = parse_op("casu-");
        let unwrapped = val.unwrap_left();
        assert_eq!(unwrapped, "casu");
    }
}
