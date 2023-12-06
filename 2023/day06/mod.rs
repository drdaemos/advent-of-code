use advent_of_code::utils::get_file_contents;
use itertools::Itertools;
use regex::Regex;

pub fn day06() {
    let input = get_file_contents("2023/day06/input.txt");
    println!("Ways multiple: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &str) -> u64 {
    let (times, distances) = parse_input(input);

    return times
        .iter()
        .zip(distances.iter())
        .map(|(time, record)| calc_ways_count(time, record))
        .product();
}

fn part_two(input: &str) -> u64 {
    let (times, distances) = parse_input(input);

    let time: u64 = times.into_iter().join("").parse().unwrap();
    let distance: u64 = distances.into_iter().join("").parse().unwrap();

    return calc_ways_count(&time, &distance);
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut lines = input.trim().split("\n");

    let times = parse_numbers(lines.next().expect("First line should be there"));
    let distances = parse_numbers(lines.next().expect("First line should be there"));

    return (times, distances);
}

fn parse_numbers(input: &str) -> Vec<u64> {
    let number_re = Regex::new(r"\d+").unwrap();

    return number_re
        .find_iter(input)
        .filter_map(|m| m.as_str().parse().ok())
        .collect_vec();
}

fn calc_ways_count(time: &u64, record: &u64) -> u64 {
    let halftime = *time as f64 / 2.0;
    let root = (halftime.powf(2.0) - *record as f64 - 1.0).sqrt();
    let upper = (halftime + root).floor() as i64;
    let lower = (halftime - root).ceil() as i64;
    return (upper - lower + 1) as u64;
}

#[cfg(test)]
mod tests {
    use crate::day06::{calc_ways_count, part_one, part_two};

    const INPUT: &str = "
Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn part_one_test() {
        let result = part_one(INPUT);
        assert_eq!(result, 288);
    }

    #[test]
    fn part_two_test() {
        let result = part_two(INPUT);
        assert_eq!(result, 71503);
    }

    #[test]
    fn calc_ways_count_test() {
        assert_eq!(calc_ways_count(&7, &9), 4);
        assert_eq!(calc_ways_count(&15, &40), 8);
        assert_eq!(calc_ways_count(&30, &200), 9);
    }
}
