use crate::utils::get_file_contents;
use std::collections::{HashSet};
use regex::{Regex, Captures};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Sensor {
    x: isize,
    y: isize,
    nearest: isize
}

pub fn main() {
    println!("Part one: {}", part_one()); // 
    println!("Part two: {}", part_two()); // 
}

fn part_one() -> usize {
    let sensors = parse_input("2022/day15/input_test.txt");

    print!("       ");
    for x in 0..30 {
        if i32::abs(x) >= 10 && i32::abs(x) % 5 == 0 {
            print!("{}", x / 10)
        } else {
            print!(" ")
        }
    }
    println!("");

    print!("       ");
    for x in 0..30 {
        print!("{}", x % 10);
    }
    println!("");
    for y in -4..30 {
        if y >= 0 && y < 10 {
            print!(" ")
        }
        print!("{} ", y);
        for x in -4..30 {
            if sensors.iter().any(|sensor| sensor.x == x && sensor.y == y) {
                print!("S");
                continue;
            }
            if is_reachable(&sensors, x, y) {
                // unavailable += 1;
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    return unavailable_hor(&sensors, 10);
}

fn part_two() -> usize {
    let sensors = parse_input("2022/day15/input_test.txt");
    return 0;
}

fn unavailable_hor(sensors: &HashSet<Sensor>, y: isize) -> usize {
    return (-4..30).map(|x| is_reachable(sensors, x, y)).filter(|val| val.to_owned() == true).count()
}

fn is_reachable(sensors: &HashSet<Sensor>, x: isize, y: isize) -> bool {
    for sensor in sensors.iter() {
        let dist = get_dist(sensor.x, sensor.y, x, y);
        if dist <= sensor.nearest {
            return true;
        }
    }

    return false;
}

fn parse_input(file_path: &str) -> HashSet<Sensor> {
    let file_contents = get_file_contents(file_path);
    let input_re = Regex::new(r"x=(-?\d+), y=(-?\d+):.*x=(-?\d+), y=(-?\d+)").unwrap();
    let input_iter: _ = file_contents
        .split("\n")
        .map(|line| input_re.captures(line).unwrap())
        .map(|coords|
            Sensor {
                x: get_match(&coords, 1), 
                y: get_match(&coords, 2), 
                nearest: get_dist(
                    get_match(&coords, 1), 
                    get_match(&coords, 2),
                    get_match(&coords, 3),
                    get_match(&coords, 4)
                )
            }
        );

    return HashSet::from_iter(input_iter);
}

fn get_match(matches: &Captures, index: usize) -> isize {
    return to_int(matches
        .get(index)
        .map_or("", |m| m.as_str())
    )
}

fn to_int(input: &str) -> isize {
    return input.parse().unwrap();
}

fn get_dist(x1: isize, y1: isize, x2: isize, y2: isize) -> isize {
    return (x2 - x1).abs() + (y2 - y1).abs();
}