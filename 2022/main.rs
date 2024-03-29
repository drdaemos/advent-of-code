use std::env;

pub mod day15;
pub mod day18;

use day15::day15;
use day18::day18;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please specify a solution name, e.g. 'day15'")
    }
    let input: &str = &args[1];
    match input {
        "day15" => day15(),
        "day18" => day18(),
        &_ => todo!()
    }
}