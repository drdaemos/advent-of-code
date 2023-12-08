use std::collections::HashMap;

use advent_of_code::utils::get_file_contents;
use itertools::Itertools;

const CARD_POSITION: [usize; 5] = [160000, 8000, 400, 20, 1];

#[derive(PartialEq, Debug)]
enum HandType {
    AllFive = 80000000,
    Quads = 70000000,
    FullHouse = 60000000,
    Trips = 50000000,
    TwoPair = 40000000,
    OnePair = 30000000,
    HighCard = 1,
}

pub fn day07() {
    let input = get_file_contents("2023/day07/input.txt");
    println!("Total winnings: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    return calc_total_winnings(input, false);
}

fn part_two(input: &str) -> usize {
    return calc_total_winnings(input, true);
}

fn calc_total_winnings(input: &str, joker_mode: bool) -> usize {
    let mut hands = parse_input(input)
        .into_iter()
        .map(|(hand, bid)| (calc_hand_strength(hand, joker_mode), bid))
        .collect_vec();

    hands.sort_by(|(a, _), (b, _)| a.cmp(&b));

    return hands
        .into_iter()
        .enumerate()
        .map(|(index, (_, bid))| bid * (index + 1))
        .sum();
}

fn parse_input(input: &str) -> Vec<(&str, usize)> {
    return input
        .trim()
        .split("\n")
        .map(|line| line.split(" ").collect_vec())
        .map(line_to_tuple)
        .collect_vec();
}

fn line_to_tuple(line: Vec<&str>) -> (&str, usize) {
    if line.len() < 2 {
        panic!("Line should have hand and bid");
    }

    return (line[0], line[1].parse().unwrap());
}

fn calc_hand_strength(hand: &str, joker_mode: bool) -> usize {
    let mut card_value = 0;

    for (i, c) in hand.chars().enumerate() {
        let card_rank = if joker_mode && c == 'J' {
            1
        } else {
            get_card_value(c).expect("card value")
        };
        card_value += card_rank * CARD_POSITION[i];
    }

    return card_value + match_hand_type(hand, joker_mode) as usize;
}

fn match_hand_type(hand: &str, joker_mode: bool) -> HandType {
    let mut groups = hand.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    let jokers = if joker_mode {
        groups.remove(&'J').unwrap_or(0)
    } else {
        0
    };

    let mut frequencies = groups.values().collect_vec();

    frequencies.sort();
    frequencies.reverse();

    return match () {
        _ if jokers == 5 => HandType::AllFive,
        _ if frequencies[0] + jokers == 5 => HandType::AllFive,
        _ if frequencies[0] + jokers == 4 => HandType::Quads,
        _ if frequencies[0] + jokers == 3 && frequencies[1] == &2 => HandType::FullHouse,
        _ if frequencies[0] == &3 && frequencies[1] + jokers == 2 => HandType::FullHouse,
        _ if frequencies[0] + jokers == 3 => HandType::Trips,
        _ if frequencies[0] + jokers == 2 && frequencies[1] == &2 => HandType::TwoPair,
        _ if frequencies[0] == &2 && frequencies[1] + jokers == 2 => HandType::TwoPair,
        _ if frequencies[0] + jokers == 2 => HandType::OnePair,
        _ => HandType::HighCard,
    };
}

fn get_card_value(c: char) -> Result<usize, ()> {
    match c {
        'A' => Ok(14),
        'K' => Ok(13),
        'Q' => Ok(12),
        'J' => Ok(11),
        'T' => Ok(10),
        '9' => Ok(9),
        '8' => Ok(8),
        '7' => Ok(7),
        '6' => Ok(6),
        '5' => Ok(5),
        '4' => Ok(4),
        '3' => Ok(3),
        '2' => Ok(2),
        _ => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use crate::day07::{calc_hand_strength, match_hand_type, part_one, part_two, HandType};

    const INPUT: &str = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn part_one_test() {
        let result = part_one(INPUT);
        assert_eq!(result, 6440);
    }

    #[test]
    fn part_two_test() {
        let result = part_two(INPUT);
        assert_eq!(result, 5905);
    }

    #[test]
    fn calc_hand_strength_test() {
        assert_eq!(calc_hand_strength("32T3K", false), 3500073);
        assert_eq!(calc_hand_strength("T55J5", false), 6642225);
        assert_eq!(calc_hand_strength("KK677", false), 6186547);
    }

    #[test]
    fn match_hand_type_test() {
        assert_eq!(match_hand_type("32T3K", false), HandType::OnePair);
        assert_eq!(match_hand_type("T55J5", false), HandType::Trips);
        assert_eq!(match_hand_type("KK677", false), HandType::TwoPair);
    }

    #[test]
    fn match_hand_type_joker_test() {
        assert_eq!(match_hand_type("32J3K", true), HandType::Trips);
        assert_eq!(match_hand_type("T55J5", true), HandType::Quads);
        assert_eq!(match_hand_type("KKQJQ", true), HandType::FullHouse);
        assert_eq!(match_hand_type("JJJ5Q", true), HandType::Quads);
        assert_eq!(match_hand_type("JJJJJ", true), HandType::AllFive);
    }
}
