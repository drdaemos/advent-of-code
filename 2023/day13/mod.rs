use std::str::Chars;

use advent_of_code::utils::get_file_contents;
use itertools::Itertools;

pub fn day13() {
    let input = get_file_contents("2023/day13/input.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    return analyze_patterns(input, false);
}

fn part_two(input: &str) -> usize {
    return analyze_patterns(input, true);
}

fn analyze_patterns(input: &str, smudged: bool) -> usize {
    let patterns = input
        .trim()
        .split("\n\n")
        .map(|pattern| {
            pattern
                .lines()
                .map(|line| line.chars().collect_vec())
                .collect_vec()
        })
        .collect_vec();

    return patterns
        .into_iter()
        .map(|pattern| find_reflection_score(pattern, &smudged))
        .sum();
}

fn find_reflection_score(pattern: Vec<Vec<char>>, smudged: &bool) -> usize {
    return find_horizontal_reflection(&pattern, &smudged) * 100
        + find_vertical_reflection(&pattern, &smudged);
}

fn find_vertical_reflection(pattern: &Vec<Vec<char>>, smudged: &bool) -> usize {
    return find_horizontal_reflection(&transpose_pattern(&pattern), smudged);
}

fn transpose_pattern(pattern: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let matrix = pattern.clone();
    let num_cols = matrix.first().unwrap().len();
    let mut row_iters: Vec<_> = matrix.into_iter().map(Vec::into_iter).collect();
    let mut out: Vec<Vec<_>> = (0..num_cols).map(|_| Vec::new()).collect();

    for out_row in out.iter_mut() {
        for it in row_iters.iter_mut() {
            out_row.push(it.next().unwrap());
        }
    }

    return out;
}

fn find_horizontal_reflection(pattern: &Vec<Vec<char>>, smudged: &bool) -> usize {
    for y in 1..pattern.len() {
        if is_perfect_reflection(pattern, (y - 1) as isize, y, smudged, 0) {
            return y;
        }
    }

    return 0;
}

fn is_perfect_reflection(
    pattern: &Vec<Vec<char>>,
    back: isize,
    forward: usize,
    smudged: &bool,
    smudges_found: usize,
) -> bool {
    // We checked all inner rows already
    if back < 0 || forward >= pattern.len() {
        return if !smudged || smudges_found == 1 {
            true
        } else {
            false
        };
    }

    let line1 = &pattern[back as usize];
    let line2 = &pattern[forward];

    // Lines are not a reflection
    let diff = find_different_elements(line1, line2);

    if !smudged && line1 != line2 {
        return false;
    }

    if *smudged && (smudges_found + diff) > 1 {
        return false;
    }

    return is_perfect_reflection(
        pattern,
        back - 1,
        forward + 1,
        smudged,
        smudges_found + diff,
    );
}

fn find_different_elements<T: PartialEq>(vec1: &Vec<T>, vec2: &Vec<T>) -> usize {
    vec1.iter().zip(vec2.iter()).filter(|(a, b)| a != b).count()
}

#[cfg(test)]
mod tests {
    use crate::day13::{part_one, part_two};

    const INPUT: &str = "
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(INPUT), 405);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(INPUT), 400);
    }
}
