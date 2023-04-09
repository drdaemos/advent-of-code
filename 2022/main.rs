use std::env;

use advent_of_code::day15;
use advent_of_code::day18;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: &str = &args[1];
    match input {
        "day15" => day15::main(),
        "day18" => day18::main(),
        &_ => todo!()
    }
}