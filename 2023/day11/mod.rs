use std::hash::Hash;
use std::{collections::HashSet, usize};

use advent_of_code::utils::get_file_contents;
use itertools::Itertools;

type Pos = (usize, usize);

pub fn day11() {
    let input = get_file_contents("2023/day11/input.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    return solve_for_expansion_factor(input, 2);
}

fn part_two(input: &str) -> usize {
    return solve_for_expansion_factor(input, 1000000);
}

fn solve_for_expansion_factor(input: &str, factor: usize) -> usize {
    let lines = input.trim().lines().collect_vec();

    // Parse expansions
    let vertical_space = find_vertical_space(&lines);
    let horizontal_space = find_horizontal_space(&lines);

    // Parse map
    let galaxies = parse_galaxies(&lines);

    // Find combinations
    let combinations = create_combinations(&galaxies);

    // Get manhattan distance for each with expansion
    let distances = combinations
        .into_iter()
        .map(|(pos1, pos2)| {
            find_distance_with_expansion(factor, *pos1, *pos2, &horizontal_space, &vertical_space)
        })
        .collect_vec();

    return distances.into_iter().sum();
}

fn find_vertical_space(input: &Vec<&str>) -> Vec<usize> {
    return input
        .into_iter()
        .enumerate()
        .clone()
        .filter(|(_, line)| !line.contains("#"))
        .map(|(y, _)| y)
        .collect_vec();
}

fn find_horizontal_space(input: &Vec<&str>) -> Vec<usize> {
    let chars = input
        .into_iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let row_len = &chars.len();
    let col_len = &chars[0].len();

    let mut column_counters = vec![0; *col_len];

    for r in 0..*row_len {
        for c in 0..*col_len {
            if chars[r][c] != '.' {
                column_counters[c] += 1;
            }
        }
    }

    return column_counters
        .into_iter()
        .enumerate()
        .filter(|(_, counter)| *counter == 0)
        .map(|(x, _)| x)
        .collect_vec();
}

fn parse_galaxies(input: &Vec<&str>) -> HashSet<Pos> {
    return HashSet::from_iter(
        input
            .into_iter()
            .enumerate()
            .flat_map(|(y, line)| line.match_indices('#').map(|(x, _)| (x, y)).collect_vec())
            .collect_vec(),
    );
}

fn create_combinations<T>(set: &HashSet<T>) -> Vec<(&T, &T)>
where
    T: Hash + Eq,
{
    let mut pairs = Vec::new();
    let items: Vec<_> = set.iter().collect();

    for (i, &item1) in items.iter().enumerate() {
        for &item2 in items.iter().skip(i + 1) {
            pairs.push((item1, item2));
        }
    }

    pairs
}

fn find_distance_with_expansion(
    factor: usize,
    (x1, y1): Pos,
    (x2, y2): Pos,
    horizontal: &Vec<usize>,
    vertical: &Vec<usize>,
) -> usize {
    let distance = manhattan_distance((x1, y1), (x2, y2));
    let x_range = match () {
        _ if x1 < x2 => (x1 + 1)..x2,
        _ => (x2 + 1)..x1,
    };
    let y_range = match () {
        _ if y1 < y2 => (y1 + 1)..y2,
        _ => (y2 + 1)..y1,
    };

    let expansion_cols = horizontal
        .into_iter()
        .filter(|col| x_range.contains(col))
        .collect_vec()
        .len();

    let expansion_rows = vertical
        .into_iter()
        .filter(|row| y_range.contains(row))
        .collect_vec()
        .len();

    return distance + expansion_cols * (factor - 1) + expansion_rows * (factor - 1);
}

fn manhattan_distance((x1, y1): Pos, (x2, y2): Pos) -> usize {
    return x1.abs_diff(x2) + y1.abs_diff(y2);
}

#[cfg(test)]
mod tests {
    use crate::day11::{part_one, part_two};

    const INPUT: &str = "
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(INPUT), 374);
    }
}
