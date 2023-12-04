use std::env;

pub mod day03;
pub mod day04;

use day03::day03;
use day04::day04;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please specify a solution name, e.g. 'day03'")
    }
    let input: &str = &args[1];
    match input {
        "day03" => day03(),
        "day04" => day04(),
        &_ => todo!(),
    }
}
