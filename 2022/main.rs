use std::env;

use advent_of_code::day18;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: &str = &args[1];
    match input {
        "day18" => day18::main(),
        &_ => todo!()
    }
}