use advent_of_code::utils::get_file_contents;
use itertools::Itertools;
use memoize::memoize;

pub fn day12() {
    let input = get_file_contents("2023/day12/input.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    return input
        .trim()
        .lines()
        .map(parse_line)
        .map(|line| memoized_calc(line.0, &line.1))
        .sum();
}

fn part_two(input: &str) -> usize {
    return input
        .trim()
        .lines()
        .map(parse_line)
        .map(|line| unfold_springs(line, 5))
        .map(|line| memoized_calc(&line.0, &line.1))
        .sum();
}

fn parse_line(input: &str) -> (&str, Vec<usize>) {
    let (springs, groups) = input.split(" ").take(2).collect_tuple().unwrap();

    return (
        &springs,
        groups.split(",").map(|x| x.parse().unwrap()).collect_vec(),
    );
}

fn unfold_springs(line: (&str, Vec<usize>), factor: usize) -> (String, Vec<usize>) {
    let springs = vec![line.0; factor].join("?");
    let groups = line.1.repeat(factor);
    return (springs, groups);
}

fn memoized_calc(springs: &str, groups: &[usize]) -> usize {
    memoized_flush_calc_arrangements();
    return calc_arrangements(springs.to_string(), groups.to_vec());
}

#[memoize]
fn calc_arrangements(springs: String, groups: Vec<usize>) -> usize {
    // If all groups have been matched
    if groups.len() == 0 {
        // and there are no left-over springs, the branch is valid
        return if springs.contains('#') { 0 } else { 1 };
    }

    // trim leading dots
    if springs.starts_with(".") {
        return calc_arrangements(springs[1..].to_string(), groups);
    }

    // get next group size
    let (first, rest) = groups.split_first().unwrap();

    // if group is longer than available record, branch is invalid
    if first > &springs.len() {
        return 0;
    }

    // slide window by 1 and check other possible arrangements
    let siblings = match springs.starts_with('#') {
        true => 0,
        false => calc_arrangements(springs[1..].to_string(), groups.clone()),
    };

    // if next `first` chars match only spring chars ('?', '#')
    let group_match = springs
        .chars()
        .take(*first)
        .into_iter()
        .all(|ch| ['?', '#'].contains(&ch));

    // check other groups in this arrangement
    let after_group = *first + 1;
    let descendants = match group_match && springs.chars().nth(*first) != Some('#') {
        true => calc_arrangements(
            springs.get(after_group..).unwrap_or_default().to_string(),
            rest.to_vec(),
        ),
        false => 0,
    };

    return siblings + descendants;
}

#[cfg(test)]
mod tests {
    use crate::day12::{memoized_calc, part_one, part_two};

    const INPUT: &str = "
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(INPUT), 21);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(INPUT), 525152);
    }

    #[test]
    fn memoized_calc_test() {
        assert_eq!(memoized_calc("???.###", &[1, 1, 3]), 1);
        assert_eq!(memoized_calc(".??..??...?##.", &[1, 1, 3]), 4);
        assert_eq!(memoized_calc("?#?#?#?#?#?#?#?", &[1, 3, 1, 6]), 1);
        assert_eq!(memoized_calc("????.#...#...", &[4, 1, 1]), 1);
        assert_eq!(memoized_calc("????.######..#####.", &[1, 6, 5]), 4);
        assert_eq!(memoized_calc("?###????????", &[3, 2, 1]), 10);
    }
}
