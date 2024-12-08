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
    map = parse_input(input)

    for key in map:
        antinodes: Set[Point] = set([])
        for antenna in map[key]:
            print("antenna", antenna)
            for neighbour in map[key]:
                print("vec", get_vec_from_points(neighbour, antenna))
                # ??????
                antinode = get_vec_from_points(neighbour, get_vec_from_points(antenna, neighbour))
                if antinode not in map[key] and antinode[0] > -1 and antinode[1] > -1:
                    antinodes.add(antinode)
        
        print(key, antinodes)
        print_visited(antinodes)

    return 0

def part_two(input: str) -> int:
    return 0

def print_visited(visited: Set[Point]):
    for row in range(0, 12):
        for col in range(0, 12):
            if (row, col) in visited:
                print('X', end="")   
            else:
                print('.', end="")
        print("")

def parse_input(input: str) -> Map:
    map: Dict[str, List[Point]] = defaultdict(list)
    lines = input.split('\n')

    for row, line in enumerate(lines):
        for col, char in enumerate(line):
            if char != '.':
                map[char].append((row, col))

    return map

def get_vec_from_points(a: Point, b: Point) -> tuple[int, int]:
    return (a[0] - b[0], a[1] - b[1])

def sum_vec(a: Point, vec: tuple[int, int]) -> Point:
    return (a[0] + vec[0], a[0] + vec[1])
