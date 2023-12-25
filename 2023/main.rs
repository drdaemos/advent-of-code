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
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day25;

use day03::day03;
use day04::day04;
use day05::day05;
use day06::day06;
use day07::day07;
use day08::day08;
use day09::day09;
use day10::day10;
use day11::day11;
use day12::day12;
use day13::day13;
use day14::day14;
use day15::day15;
use day25::day25;

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
        "day12" => day12(),
        "day13" => day13(),
        "day14" => day14(),
        "day15" => day15(),
        "day25" => day25(),
        &_ => todo!(),
    }
}
