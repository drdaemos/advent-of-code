use advent_of_code::utils::get_file_contents;
use std::{collections::{HashSet}, thread, sync::{mpsc::{self, Sender}, Arc}};
use regex::{Regex, Captures};


#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Sensor {
    x: isize,
    y: isize,
    nearest: isize
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Beacon {
    x: isize,
    y: isize
}

pub fn day15() {
    // println!("Part one: {}", part_one()); // 5461729
    println!("Part two: {}", part_two()); // 
}

fn part_one() -> usize {
    let sensors = parse_sensors("2022/day15/input.txt");
    let beacons = parse_beacons("2022/day15/input.txt");

    return unavailable_hor(&sensors, &beacons, 2000000, -1000000, 5000000);
}

fn print_axes(min: isize, max: isize) {
    print!("       ");
    for x in min..max {
        if isize::abs(x) >= 10 && isize::abs(x) % 5 == 0 {
            print!("{}", x / 10)
        } else {
            print!(" ")
        }
    }
    println!("");

    print!("       ");
    for x in min..max {
        print!("{}", x % 10);
    }
    println!("");
}

fn print_map(sensors: &HashSet<Sensor>, beacons: &HashSet<Beacon>, min: isize, max: isize) {
    for y in min..max {
        if y >= 0 && y < 10 {
            print!(" ")
        }
        print!("{} ", y);
        for x in min..max {
            if sensors.iter().any(|it| it.x == x && it.y == y) {
                print!("S");
                continue;
            }
            if beacons.iter().any(|it| it.x == x && it.y == y) {
                print!("B");
                continue;
            }
            if is_reachable(&sensors, beacons, x, y) {
                // unavailable += 1;
                print!("▓");
            } else {
                print!("░");
            }
        }
        println!("");
    }
}

fn part_two() -> usize {
    let sensors = parse_sensors("2022/day15/input.txt");
    let beacons = parse_beacons("2022/day15/input.txt");
    let coords = find_distress_beacon(sensors, beacons, 0, 4000000);

    println!("Beacon: {}, {}", coords.x, coords.y);

    return coords.x as usize * 4000000 + coords.y as usize;
}

fn unavailable_hor(sensors: &HashSet<Sensor>, beacons: &HashSet<Beacon>, y: isize, min: isize, max: isize) -> usize {
    return (min..max)
        .map(|x| is_reachable(sensors, beacons, x, y))
        .filter(|val| *val)
        .count()
}

fn find_distress_beacon(sensors: HashSet<Sensor>, beacons: HashSet<Beacon>, min: usize, max: usize) -> Beacon {
    let threads = 10;
    let (transmitter, receiver) = mpsc::channel::<Option<(usize, usize)>>();
    let data_arc = Arc::new((sensors, beacons));
    
    for part in 0..threads {
        let data = data_arc.clone();
        let tx = transmitter.clone();
        let size = (max - min) / threads;
        let offset = min + size * part;
        println!("Spawning thread #{} for range {}-{}", part, min + size * part, min + offset + size);
        thread::spawn(move || {
            let candidate = find_unreachable_pos(&data.0, &data.1, min + offset, min + offset + size, min, max);
            tx.send(candidate).unwrap();
        });
    }

    loop {
        let beacon = receiver.recv().unwrap();
        println!("Recv: {:?}", beacon);
    }

    return Beacon { x: 0, y: 0 };
}

fn find_unreachable_pos(sensors: &HashSet<Sensor>, beacons: &HashSet<Beacon>, min_x: usize, max_x: usize, min_y: usize, max_y: usize) -> Option<(usize, usize)> {
    for y in min_y..max_y {
        for x in min_x..max_x {
            if !is_reachable(sensors, beacons, x as isize, y as isize) &&
                !is_object(sensors, beacons, x as isize, y as isize) &&
                is_reachable(sensors, beacons, (x-1) as isize, y as isize) &&
                is_reachable(sensors, beacons, x as isize, (y-1) as isize) &&
                is_reachable(sensors, beacons, (x+1) as isize, y as isize) &&
                is_reachable(sensors, beacons, x as isize, (y+1) as isize) {
                return Some((x, y));
            }
        }
    }

    return None;
}

fn is_object(sensors: &HashSet<Sensor>, beacons: &HashSet<Beacon>, x: isize, y: isize) -> bool {
    if beacons.contains(&Beacon { x, y }) {
        return true;
    }

    if sensors.iter().any(|it| it.x == x && it.y == y) {
        return true;
    } 

    return false;
}

fn is_reachable(sensors: &HashSet<Sensor>, beacons: &HashSet<Beacon>, x: isize, y: isize) -> bool {
    if beacons.contains(&Beacon { x, y }) {
        return false;
    }

    for sensor in sensors.iter() {
        let dist = get_dist(sensor.x, sensor.y, x, y);
        if dist <= sensor.nearest {
            return true;
        }
    }

    return false;
}

fn parse_sensors(file_path: &str) -> HashSet<Sensor> {
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

fn parse_beacons(file_path: &str) -> HashSet<Beacon> {
    let file_contents = get_file_contents(file_path);
    let input_re = Regex::new(r"x=(-?\d+), y=(-?\d+):.*x=(-?\d+), y=(-?\d+)").unwrap();
    let input_iter: _ = file_contents
        .split("\n")
        .map(|line| input_re.captures(line).unwrap())
        .map(|coords|
            Beacon {
                x: get_match(&coords, 3), 
                y: get_match(&coords, 4), 
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