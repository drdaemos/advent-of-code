use crate::utils::get_file_contents;
use std::collections::{HashSet, VecDeque};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Cube {
    x: isize,
    y: isize,
    z: isize
}

#[derive(Debug, EnumIter)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    Front,
    Back
}

pub fn main() {
    println!("Part one: {}", part_one()); // 3396
    println!("Part two: {}", part_two()); // 2044
}

fn part_one() -> usize {
    let cubes = parse_input("2022/day18/input.txt");
    return total_surface_area(&cubes);
}

fn part_two() -> usize {
    let cubes = parse_input("2022/day18/input.txt");
    return exterior_surface_area(&cubes);
}

fn parse_input(file_path: &str) -> HashSet<Cube> {
    let file_contents = get_file_contents(file_path);
    let cube_iter: _ = file_contents
        .split("\n")
        .map(|line| line.split(","))
        .map(|parts| parts.map(|x| x.parse().unwrap()).collect::<Vec<_>>())
        .map(|coords| Cube {x: coords[0], y: coords[1], z: coords[2]});

    return HashSet::from_iter(cube_iter);
}

fn max_coords(set: &HashSet<Cube>) -> Cube {
    Cube {
        x: set.into_iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x + 2,
        y: set.into_iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y + 2,
        z: set.into_iter().max_by(|a, b| a.z.cmp(&b.z)).unwrap().z + 2
    }
}

fn min_coords(set: &HashSet<Cube>) -> Cube {
    Cube {
        x: set.into_iter().min_by(|a, b| a.x.cmp(&b.x)).unwrap().x - 2,
        y: set.into_iter().min_by(|a, b| a.y.cmp(&b.y)).unwrap().y - 2,
        z: set.into_iter().min_by(|a, b| a.z.cmp(&b.z)).unwrap().z - 2
    }
}

fn total_surface_area(cubes: &HashSet<Cube>) -> usize {
    let sides = cubes.len() * 6;
    let covered = cubes
        .into_iter()
        .map(|cube| cube_covered(cube, cubes))
        .sum::<usize>();
    return sides - covered;
}

fn neighbors(cube: &Cube) -> HashSet<Cube> {
    Direction::iter()
        .map(|dir| next_cube(cube, dir))
        .collect::<HashSet<Cube>>()
}

fn exterior_surface_area(cubes: &HashSet<Cube>) -> usize {
    let min = min_coords(cubes);
    let max = max_coords(cubes);

    let mut area: usize = 0;
    let mut queue = VecDeque::from([min]);
    let mut visited: HashSet<Cube> = HashSet::from([min]);
    while let Some(current) = queue.pop_front() {
        for direction in Direction::iter() {
            let neighbor = next_cube(&current, direction);
            if !cube_in_bounds(&neighbor, min, max) {
                continue;
            }

            if cubes.contains(&neighbor) {
                area += 1
            } else if !visited.contains(&neighbor) {
                queue.push_back(neighbor);
                visited.insert(neighbor);
            }
        }
    }

    return area
}

fn next_cube(cube: &Cube, direction: Direction) -> Cube {
    match direction {
        Direction::Left  => Cube {x: cube.x-1, y: cube.y,   z: cube.z},
        Direction::Right => Cube {x: cube.x+1, y: cube.y,   z: cube.z},
        Direction::Up    => Cube {x: cube.x,   y: cube.y-1, z: cube.z},
        Direction::Down  => Cube {x: cube.x,   y: cube.y+1, z: cube.z},
        Direction::Front => Cube {x: cube.x,   y: cube.y,   z: cube.z-1},
        Direction::Back  => Cube {x: cube.x,   y: cube.y,   z: cube.z+1}
    }
}

fn cube_covered(cube: &Cube, set: &HashSet<Cube>) -> usize {
    return set.into_iter()
        .filter(|item| neighbors(cube).contains(item))
        .count();
}

fn cube_in_bounds(cube: &Cube, min: Cube, max: Cube) -> bool {
    (min.x..max.x).contains(&cube.x) && (min.y..max.y).contains(&cube.y) && (min.z..max.z).contains(&cube.z)
}