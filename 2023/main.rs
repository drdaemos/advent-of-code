use std::env;

pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day15;

use day03::day03;
use day04::day04;
use day05::day05;
use day06::day06;
use day07::day07;
use day08::day08;
use day09::day09;
use day10::day10;
use day11::day11;
use day15::day15;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please specify a solution name, e.g. 'day03'")
    }
    let input: &str = &args[1];
    match input {
        "day03" => day03(),
        "day04" => day04(),
        "day05" => day05(),
        "day06" => day06(),
        "day07" => day07(),
        "day08" => day08(),
        "day09" => day09(),
        "day10" => day10(),
        "day11" => day11(),
        "day15" => day15(),
        &_ => todo!(),
    }
}
