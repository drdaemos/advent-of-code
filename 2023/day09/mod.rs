use advent_of_code::{math::lagrange_interpolation, utils::get_file_contents};
use itertools::Itertools;

pub fn day09() {
    let input = get_file_contents("2023/day09/input.txt");
    println!("Trend sum: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &str) -> isize {
    let sequences = parse_input(input);

    return find_interpolation_sum(sequences, false);
}

fn part_two(input: &str) -> isize {
    let sequences = parse_input(input);

    return find_interpolation_sum(sequences, true);
}

fn find_interpolation_sum(sequences: Vec<Vec<isize>>, backwards: bool) -> isize {
    let interpolated = sequences
        .into_iter()
        .map(|seq| {
            lagrange_interpolation(&seq[..], if backwards { -1 } else { seq.len() as isize })
        })
        .collect_vec();

    return interpolated.into_iter().sum();
}

fn parse_input(input: &str) -> Vec<Vec<isize>> {
    return input
        .trim()
        .split("\n")
        .map(|line| {
            line.split(" ")
                .map(|val| val.parse().unwrap())
                .collect_vec()
        })
        .collect_vec();
}

#[cfg(test)]
mod tests {
    use crate::day09::{part_one, part_two};

    const INPUT: &str = "
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(INPUT), 114);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(INPUT), 2);
    }
}
