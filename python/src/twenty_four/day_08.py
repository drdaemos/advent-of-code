from collections import defaultdict
from typing import Dict, List, Set

# coordinate within map as (row, col)
type Point = tuple[int, int]

# dict of different freq types and all antennas belonging to this type
type Map = Dict[str, List[Point]]

def main():
    print("Advent of Code 2024 - day 8")
    try:
        input = open("./inputs/day_08.txt").read()
        print(" Part one: %d" % part_one(input))
        print(" Part two: %d" % part_two(input))
    except:
        print(" Input malformed / not found, expected to have `./inputs/day_08.txt`")
    
def part_one(input: str) -> int:
    map, size = parse_input(input)
    points = find_antinodes(map, size, False)

    return len(points)

def part_two(input: str) -> int:
    map, size = parse_input(input)
    points = find_antinodes(map, size, True)

    return len(points)

def find_antinodes(map: Map, size: int, part_two_mode: bool) -> Set[Point]:
    visited: Set[Point] = set([])

    for key in map:
        antinodes: Set[Point] = set([])
        for antenna in map[key]:
            for neighbour in map[key]:
                vec_diff = get_vec_from_points(antenna, neighbour)

                if vec_diff == (0,0):
                    continue

                if part_two_mode is False:
                    antinode = get_vec_from_points(neighbour, vec_diff)
                    if antinode not in map[key] and antinode[0] > -1 and antinode[1] > -1 and antinode[0] < size and antinode[1] < size:
                        antinodes.add(antinode)
                else:
                    within_bounds = True
                    current = neighbour
                    antinodes.add(neighbour)
                    while within_bounds:
                        antinode = get_vec_from_points(current, vec_diff)
                        if antinode[0] > -1 and antinode[1] > -1 and antinode[0] < size and antinode[1] < size:
                            current = antinode
                            antinodes.add(antinode)
                        else:
                            within_bounds = False

        visited.update(antinodes)

    return visited

def print_visited(visited: Set[Point]):
    for row in range(0, 12):
        for col in range(0, 12):
            if (row, col) in visited:
                print('X', end="")   
            else:
                print('.', end="")
        print("")

def parse_input(input: str) -> tuple[Map, int]:
    map: Dict[str, List[Point]] = defaultdict(list)
    lines = input.split('\n')

    for row, line in enumerate(lines):
        for col, char in enumerate(line):
            if char != '.':
                map[char].append((row, col))

    return (map, len(lines))

def get_vec_from_points(a: Point, b: Point) -> tuple[int, int]:
    return (a[0] - b[0], a[1] - b[1])