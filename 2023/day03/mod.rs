use std::collections::{HashMap, HashSet};

use advent_of_code::utils::get_file_contents;
use regex::Regex;

type EngineSymbols = std::collections::HashSet<(usize, usize)>;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct EnginePart {
    line: usize,
    col: usize,
    value: isize,
    length: usize,
}

pub fn day03() {
    let input = get_file_contents("2023/day03/input.txt");
    println!("Sum of part numbers: {}", part_one(&input));
    println!("Sum of gear ratios: {}", part_two(&input));
}

fn part_one(input: &str) -> isize {
    let parts = parse_parts(&input);
    let symbols = parse_symbols(&input);
    let actual_parts = filter_parts(&parts, &symbols);
    return actual_parts.into_iter().map(|part| part.value).sum();
}

fn part_two(input: &str) -> isize {
    let parts = parse_parts(&input);
    let gears = parse_gears(&input);
    let gear_ratios = find_gear_ratios(&parts, &gears);
    return gear_ratios.into_iter().sum();
}

fn parse_parts(input: &str) -> Vec<EnginePart> {
    let part_re = Regex::new(r"(\d+)").unwrap();
    return Vec::from_iter(
        input
            .trim()
            .split("\n")
            .map(|line| part_re.find_iter(line))
            .enumerate()
            .map(|(line, parts)| {
                parts.map(move |part| EnginePart {
                    line,
                    col: part.start(),
                    value: to_int(part.as_str()),
                    length: part.range().len(),
                })
            })
            .flatten(),
    );
}

fn parse_symbols(input: &str) -> EngineSymbols {
    let symbol_re = Regex::new(r"([^0-9.\s])").unwrap();

    return HashSet::from_iter(
        input
            .trim()
            .split("\n")
            .map(|line| symbol_re.find_iter(line))
            .enumerate()
            .map(|(line, symbols)| symbols.map(move |x| (x.start(), line)))
            .flatten(),
    );
}

fn parse_gears(input: &str) -> EngineSymbols {
    let symbol_re = Regex::new(r"([*])").unwrap();

    return HashSet::from_iter(
        input
            .trim()
            .split("\n")
            .map(|line| symbol_re.find_iter(line))
            .enumerate()
            .map(|(line, symbols)| symbols.map(move |x| (x.start(), line)))
            .flatten(),
    );
}

fn filter_parts<'a>(parts: &'a Vec<EnginePart>, symbols: &'a EngineSymbols) -> Vec<&'a EnginePart> {
    return parts
        .iter()
        .filter(|&part| is_adjacent_to(part, &symbols).is_ok())
        .collect();
}

fn find_gear_ratios(parts: &Vec<EnginePart>, gears: &EngineSymbols) -> Vec<isize> {
    let mut gear_ratios_map: HashMap<(usize, usize), Vec<isize>> = HashMap::new();

    for part in parts.iter() {
        let gear_check = is_adjacent_to(part, &gears);
        if gear_check.is_ok() {
            let gear = &gear_check.unwrap();
            if gear_ratios_map.contains_key(&gear) {
                gear_ratios_map
                    .entry(*gear)
                    .and_modify(|e| e.push(part.value));
            } else {
                gear_ratios_map.insert(*gear, vec![part.value]);
            }
        }
    }

    return gear_ratios_map
        .into_iter()
        .filter(|(_, parts)| parts.len() == 2)
        .map(|(_, parts)| multiply_vec(parts))
        .collect::<Vec<isize>>();
}

fn is_adjacent_to(part: &EnginePart, symbols: &EngineSymbols) -> Result<(usize, usize), bool> {
    let top = part.line as isize - 1;
    let bottom = part.line as isize + 1;
    let left = part.col as isize - 1;
    let right = part.col as isize + part.length as isize;

    for y in top..=bottom {
        for x in left..=right {
            if x < 0 || y < 0 {
                continue;
            }

            if symbols.contains(&(x as usize, y as usize)) {
                return Ok((x as usize, y as usize));
            }
        }
    }

    return Err(false);
}

fn to_int(input: &str) -> isize {
    return input.parse().unwrap();
}

fn multiply_vec(vec: Vec<isize>) -> isize {
    return vec.into_iter().reduce(|acc, i| acc * i).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::day03::part_one;
    use crate::day03::part_two;

    const INPUT: &str = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn part_one_test() {
        let result = part_one(INPUT);
        assert_eq!(result, 4361);
    }

    #[test]
    fn part_two_test() {
        let result = part_two(INPUT);
        assert_eq!(result, 467835);
    }
}
