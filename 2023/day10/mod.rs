use advent_of_code::utils::get_file_contents;

pub fn day10() {
    let input = get_file_contents("2023/day10/input.txt");
    println!("Steps: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    return 0;
}

fn part_two(input: &str) -> usize {
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::day10::{part_one, part_two};

    const INPUT: &str = "
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(INPUT), 6);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(INPUT), 0);
    }
}
