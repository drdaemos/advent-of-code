from dataclasses import dataclass
from functools import reduce
from operator import mul
import re
from typing import List

@dataclass
class Robot:
    start: complex
    pos: complex
    velocity: complex

    def __init__(self, start: complex, velocity: complex):
        self.start = start
        self.pos = start
        self.velocity = velocity

def main():
    print("Advent of Code 2024 - day 14")
    try:
        input = open("./inputs/day_14.txt").read()
        print(" Part one: %d" % part_one(input, 101, 103))
        print(" Part two: %d" % part_two(input, 101, 103))
    except:
        print(" Input malformed / not found, expected to have `./inputs/day_14.txt`")

def part_one(input: str, width: int, height: int) -> int:
    robots = parse_input(input)
    for i in range(0, 100):
        simulate_tick(robots, width, height)
    return calculate_safety_factor(robots, width, height)

def part_two(input: str, width: int, height: int) -> int:
    robots = parse_input(input)
    easter_egg_found = False
    ticks = 0
    while not easter_egg_found:
        print(ticks)
        simulate_tick(robots, width, height)
        positions = [r.pos for r in robots]
        if easter_egg_picture(positions, width, height):
            print_map(robots, width, height)
            easter_egg_found = True

        ticks += 1

    return ticks

def simulate_tick(robots: List[Robot], width: int, height: int):
    for robot in robots:
        robot.pos = overlap_pos(robot.pos + robot.velocity, width, height)

def easter_egg_picture(robot_positions: List[complex], width: int, height: int) -> bool:
    for y in range(0, height):
        consecutive_hor = 0
        prev_x = -1
        for x in range(0, width):
            if complex(x, y) in robot_positions:
                if x - prev_x == 1:
                    consecutive_hor += 1
                else: 
                    consecutive_hor = 0
            else:
                consecutive_hor = 0

            prev_x = x
            if consecutive_hor > 10:
                return True
                
    return False

def overlap_pos(pos: complex, width: int, height: int) -> complex:
    return complex(pos.real % width, pos.imag % height)

def calculate_safety_factor(robots: List[Robot], width: int, height: int) -> int:
    quadrants = [0,0,0,0]
    for y in range(0, height):
        for x in range(0, width):
            count = sum(1 for i in robots if i.pos == complex(x,y))
            match (x, y):
                case (x, y) if x < width // 2 and y < height // 2:
                    quadrants[0] += count
                case (x, y) if x > width // 2 and y < height // 2:
                    quadrants[1] += count
                case (x, y) if x > width // 2 and y > height // 2:
                    quadrants[2] += count
                case (x, y) if x < width // 2 and y > height // 2:
                    quadrants[3] += count

    return reduce(mul, (i for i in quadrants))

def print_map(robots: List[Robot], width: int, height: int):
    for y in range(0, height):
        for x in range(0, width):
            count = sum(1 for i in robots if i.pos == complex(x,y))
            print(count if count > 0 else '.', end="")
        print()
        
def parse_input(input: str) -> List[Robot]:
    robots: List[Robot] = []
    for line in input.strip().splitlines():
        definition = re.match(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)", line.strip())
        if definition is None or any(i is None for i in definition.groups()):
            raise Exception('Malformed input')
        
        robots.append(Robot(
            start=complex(int(definition.group(1)), int(definition.group(2))), 
            velocity=complex(int(definition.group(3)), int(definition.group(4)))
        ))

    return robots
        