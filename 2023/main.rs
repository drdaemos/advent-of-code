use std::env;

pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;

use day03::day03;
use day04::day04;
use day05::day05;
use day06::day06;

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
        &_ => todo!(),
    }
}
