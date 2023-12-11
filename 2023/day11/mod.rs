use advent_of_code::utils::get_file_contents;

pub fn day11() {
    let input = get_file_contents("2023/day11/input.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &str) -> isize {
    return 0;
}

fn part_two(input: &str) -> isize {
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::day11::{part_one, part_two};

    const INPUT: &str = "
";

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(INPUT), 0);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(INPUT), 0);
    }
}
