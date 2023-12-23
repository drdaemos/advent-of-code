use std::{
    collections::{hash_map::DefaultHasher, HashSet},
    hash::{Hash, Hasher},
};

use advent_of_code::utils::get_file_contents;
use itertools::Itertools;

trait Board {
    fn run_cycle(&self) -> Self;
    fn rotate(&self, clockwise: bool) -> Self;
    fn tilt_left(&self) -> Self;
    fn count_load(&self) -> usize;
    fn get_hash(&self) -> u64;
}

impl Board for Vec<Vec<char>> {
    // Assumes that board rotated counterclockwise before running, so the North is pointing left
    fn run_cycle(&self) -> Self {
        return self
            .tilt_left() // north
            .rotate(true)
            .tilt_left() // west
            .rotate(true)
            .tilt_left() // south
            .rotate(true)
            .tilt_left() // east
            .rotate(true); // restore orientation
    }

    fn rotate(&self, clockwise: bool) -> Self {
        let size = self.len();

        let mut rotated = vec![vec![' '; size]; size];

        for (i, row) in self.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                if clockwise {
                    // Set the new position of the element by transposing and reversing the row.
                    rotated[j][size - i - 1] = val;
                } else {
                    // Just transpose the element
                    rotated[size - j - 1][i] = val;
                }
            }
        }

        return rotated;
    }

    fn tilt_left(&self) -> Self {
        let size = self.len();
        let mut tilted = vec![vec![' '; size]; size];

        for (i, row) in self.iter().enumerate() {
            tilted[i] = row
                .split(|ch| *ch == '#')
                .map(|group| {
                    group
                        .to_owned()
                        .into_iter()
                        .sorted()
                        .rev()
                        .chain(std::iter::once('#'))
                })
                .flatten()
                .take(size)
                .collect_vec();
        }

        return tilted;
    }

    fn count_load(&self) -> usize {
        let columns = self.first().unwrap().len();
        let mut counts = vec![0; columns];
        for row in 0..self.len() {
            for col in 0..columns {
                let char = self[row][col];
                if char == 'O' {
                    counts[col] += self.len() - row;
                }
            }
        }

        return counts.into_iter().sum();
    }

    fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        return hasher.finish();
    }
}

pub fn day14() {
    let input = get_file_contents("2023/day14/input.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    return parse_grid(input)
        .rotate(false)
        .tilt_left()
        .rotate(true)
        .count_load();
}

fn part_two(input: &str) -> usize {
    let desired_cycles = 1000000000;
    let mut board = parse_grid(input).rotate(false); // pre-rotate so North is pointing left

    let mut met_hashes = HashSet::<u64>::new();
    let mut cycle_start = -1;
    let mut cycle_start_hash = 0;
    let mut cycle_length = -1;
    let mut cycle_index = 0;

    // seeking for cycle
    while cycle_length < 0 {
        board = board.run_cycle();

        // we have seen the whole cycle
        if cycle_start_hash == board.get_hash() {
            cycle_length = cycle_index - cycle_start;
        }

        // we met the start of cycle
        if met_hashes.contains(&board.get_hash()) && cycle_start < 0 {
            cycle_start = cycle_index;
            cycle_start_hash = board.get_hash();
        }

        // increment cycle counter
        cycle_index += 1;
        met_hashes.insert(board.get_hash());
    }

    println!("found cycle: {:?}, start: {:?}", cycle_length, cycle_start);

    // Cycle the board to desired position
    while cycle_index % cycle_length != desired_cycles % cycle_length {
        board = board.run_cycle();
        cycle_index += 1;
    }

    // revert pre-rotation and calculate North load
    return board.rotate(true).count_load();
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    return input
        .trim()
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
}

#[cfg(test)]
mod tests {
    use crate::day14::{parse_grid, part_one, part_two, Board};

    const INPUT: &str = "
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    const MINI_BOARD: &str = "
..0..
..0..
..000
.....
.....    
";

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(INPUT), 136);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(INPUT), 64);
    }

    #[test]
    fn tilt_left_test() {
        let expected = "
O....#....
OOO.#....#
.....##...
OO.#OO....
OO......#.
O.#O...#.#
O....#OO..
O.........
#....###..
#OO..#....
";
        assert_eq!(parse_grid(INPUT).tilt_left(), parse_grid(expected));
    }

    #[test]
    fn rotate_test() {
        let ccw = "
..0..
..0..
000..
.....
.....   
";
        assert_eq!(parse_grid(MINI_BOARD).rotate(false), parse_grid(ccw));

        let cw = "
.....
.....
..000
..0..
..0..   
";
        assert_eq!(parse_grid(MINI_BOARD).rotate(true), parse_grid(cw));
    }

    #[test]
    fn hash_test() {
        assert_eq!(parse_grid(INPUT).get_hash(), 9680173050119588196);
        assert_eq!(parse_grid(INPUT).get_hash(), 9680173050119588196);
        assert_eq!(parse_grid(MINI_BOARD).get_hash(), 8538414628197951262);
    }
}
