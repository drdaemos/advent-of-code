pub fn day03() {
    let cubes = parse_input("2023/day03/input.txt");
}

fn part_one(input: &str) -> usize {
    return 0;
}

fn parse_input(file_path: &str) -> HashSet<Cube> {
    let file_contents = get_file_contents(file_path);
    let cube_iter: _ = file_contents
        .split("\n")
        .map(|line| line.split(","))
        .map(|parts| parts.map(|x| x.parse().unwrap()).collect::<Vec<_>>())
        .map(|coords| Cube {x: coords[0], y: coords[1], z: coords[2]});

    return HashSet::from_iter(cube_iter);
}

#[cfg(test)]
mod tests {
    use crate::day03::part_one;

    const INPUT: &str = 
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part_one_test() {
        let result = part_one(INPUT);
        assert_eq!(result, 4361);
    }

    // #[test]
    // fn part_two_test() {
    //     let result = part_two();
    //     assert_eq!(result, "test");
    // }
}