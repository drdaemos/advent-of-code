use std::collections::{HashMap, HashSet};

use advent_of_code::utils::get_file_contents;

pub fn day04() {
    let input = get_file_contents("2023/day04/input.txt");
    println!("Scratchcards value: {}", part_one(&input));
    println!("Scratchcards count: {}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    return input
        .trim()
        .split("\n")
        .map(parse_card)
        .map(|(winning, card_numbers)| get_card_value(card_numbers, winning))
        .sum();
}

fn part_two(input: &str) -> usize {
    let match_counts = input
        .trim()
        .split("\n")
        .map(parse_card)
        .map(|(winning, card_numbers)| match_count(winning, card_numbers))
        .collect::<Vec<usize>>();

    let mut card_counts: Vec<usize> = vec![1; match_counts.len()];

    for i in 0..match_counts.len() {
        let matches = match_counts.get(i).expect("Match count should be there");
        for j in 1..matches + 1 {
            card_counts[i + j] += card_counts[i]
        }
    }

    return card_counts.into_iter().sum();
}

fn parse_card(line: &str) -> (HashSet<&str>, HashSet<&str>) {
    let mut parts = line
        .split(":")
        .last()
        .expect("Unexpected input")
        .split(" | ");

    let winning = parts
        .next()
        .expect("Winning numbers")
        .trim()
        .split(" ")
        .into_iter()
        .filter(|number| !number.is_empty())
        .map(|number| number.trim())
        .collect::<HashSet<_>>();

    let card_numbers = parts
        .next()
        .expect("Card belonging numbers")
        .trim()
        .split(" ")
        .into_iter()
        .filter(|number| !number.is_empty())
        .map(|number| number.trim())
        .collect::<HashSet<_>>();

    return (winning, card_numbers);
}

fn get_card_value(card_numbers: HashSet<&str>, winning: HashSet<&str>) -> usize {
    let matches = match_count(winning, card_numbers);
    return match matches {
        0 => 0,
        _ => 1 << (matches - 1),
    };
}

fn match_count(a: HashSet<&str>, b: HashSet<&str>) -> usize {
    return a.intersection(&b).count();
}

#[cfg(test)]
mod tests {
    use crate::day04::{part_one, part_two};

    const INPUT: &str = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn part_one_test() {
        let result = part_one(INPUT);
        assert_eq!(result, 13);
    }

    #[test]
    fn part_two_test() {
        let result = part_two(INPUT);
        assert_eq!(result, 30);
    }
}
