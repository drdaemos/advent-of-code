use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    sync::{mpsc, Arc},
    thread::{self, JoinHandle},
};

use advent_of_code::utils::get_file_contents;

type AlmanacMaps = Vec<Vec<MapRule>>;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct MapRule {
    dest_start: u64,
    range_start: u64,
    length: u64,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct SeedRange {
    start: u64,
    length: u64,
}

pub fn day05() {
    let input = get_file_contents("2023/day05/input.txt");
    println!("Lowest location: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &str) -> u64 {
    let seeds = parse_seeds(input);
    let almanac = parse_almanac(input);

    let locations = seeds
        .into_iter()
        .map(|seed| map_seed_to_location(seed, &almanac))
        .collect::<Vec<u64>>();

    println!("{:?}", locations);

    return locations
        .into_iter()
        .min()
        .expect("There should be locations");
}

fn part_two(input: &str) -> u64 {
    let mut seed_ranges = parse_ranges(input);
    seed_ranges.sort_unstable_by(|a, b| a.start.cmp(&b.start));
    let almanac = parse_almanac(input);

    let mut threads = Vec::<JoinHandle<()>>::new();
    let mut locations = Vec::<u64>::new();
    let thread_limit = seed_ranges.len();
    let (transmitter, receiver) = mpsc::channel::<Option<u64>>();
    let data_arc = Arc::new((seed_ranges, almanac));

    for part in 0..thread_limit {
        let data = data_arc.clone();
        let tx = transmitter.clone();
        threads.push(thread::spawn(move || {
            let range = &data.0[part];
            println!("{:?}", range);
            let maps = &data.1;
            let lowest = (range.start..(range.start + range.length))
                .map(|seed| map_seed_to_location(seed, maps))
                .min();
            tx.send(lowest).unwrap();
        }));
    }

    loop {
        // Receive values
        let value = receiver.recv().unwrap();
        locations.push(value.expect("should be lowest"));

        // Check if all threads are finished
        let num_left = threads.iter().filter(|th| !th.is_finished()).count();
        if num_left == 0 {
            break;
        }
    }

    for thread in threads {
        thread.join().unwrap();
    }

    println!("{:?}", locations);

    return locations
        .into_iter()
        .min()
        .expect("There should be locations");
}

fn map_seed_to_location(seed: u64, maps: &AlmanacMaps) -> u64 {
    let mut mapped = seed;
    for map in maps {
        let rule = find_nearest_less_than(map, mapped);
        if rule.is_some_and(|x| x.range_start + x.length > mapped) {
            let map_rule = rule.expect("should be here");
            mapped = mapped + map_rule.dest_start - map_rule.range_start;
        }
    }

    return mapped;
}

fn find_nearest_less_than(sorted_numbers: &Vec<MapRule>, target: u64) -> Option<MapRule> {
    // Check if the smallest value is already greater or equal to the target
    // In that case, no value in the vector is less than the target
    if let Some(&first) = sorted_numbers.first() {
        if first.range_start >= target {
            return None;
        }
    }

    // Perform binary search
    match sorted_numbers.binary_search_by(|probe| probe.range_start.cmp(&target)) {
        Err(0) => None,                                // All elements are greater
        Ok(index) => Some(sorted_numbers[index]),      // Element exactly equal
        Err(index) => Some(sorted_numbers[index - 1]), // One less than the greater element's index
    }
}

fn parse_seeds(input: &str) -> Vec<u64> {
    return input
        .trim()
        .split("\n")
        .next()
        .expect("First line should be there")
        .split("seeds: ")
        .last()
        .expect("Seed numbers should be there")
        .split(" ")
        .map(to_int)
        .collect();
}

fn parse_ranges(input: &str) -> Vec<SeedRange> {
    return input
        .trim()
        .split("\n")
        .next()
        .expect("First line should be there")
        .split("seeds: ")
        .last()
        .expect("Seed numbers should be there")
        .split(" ")
        .map(to_int)
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|chunk| SeedRange {
            start: chunk[0],
            length: chunk[1],
        })
        .collect::<Vec<SeedRange>>();
}

fn parse_almanac(input: &str) -> AlmanacMaps {
    return input
        .trim()
        .split("\n\n")
        .skip(1)
        .filter(|group| !group.is_empty())
        .map(parse_map_rule)
        .collect();
}

fn parse_map_rule(input: &str) -> Vec<MapRule> {
    let mut rules = input
        .split("\n")
        .skip(1)
        .map(|line| line.split(" ").map(to_int).collect::<Vec<u64>>())
        .map(|values| MapRule {
            dest_start: values[0],
            range_start: values[1],
            length: values[2],
        })
        .collect::<Vec<MapRule>>();

    rules.sort_unstable_by(|a, b| a.range_start.cmp(&b.range_start));

    return rules;
}

fn to_int(input: &str) -> u64 {
    return input.parse().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::day05::{part_one, part_two};

    const INPUT: &str = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn part_one_test() {
        let result = part_one(INPUT);
        assert_eq!(result, 35);
    }

    #[test]
    fn part_two_test() {
        let result = part_two(INPUT);
        assert_eq!(result, 46);
    }
}
