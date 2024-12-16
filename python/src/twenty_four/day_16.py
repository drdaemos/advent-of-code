import heapq
import os
from random import randint
from typing import Deque, List, Sequence, Set
from collections import deque

# walls on map
type Walls = Set[Point]

# .real is x, .imag is y
type Point = complex

def main():
    print("Advent of Code 2024 - day 16")
    try:
        input = open("./inputs/day_16.txt").read()
        print(" Part one: %d" % part_one(input))
        print(" Part two: %d" % part_two(input))
    except (ValueError, FileNotFoundError):
        print(" Input malformed / not found, expected to have `./inputs/day_16.txt`")

def part_one(input: str, debug = False) -> int:
    walls, start, end = parse_input(input)
    path = a_star_least_turns(walls, start, end)
    print(start, "->", end)
    print_map(walls, start, end, path)
    return calculate_score(path)

def part_two(input: str, debug = False) -> int:
    return 0

def print_map(walls: Walls, start: Point, end: Point, path: List[Point]):
    print()
    bottom_right = list(walls)[-1]
    for y in range(0, int(bottom_right.real) + 1):
        for x in range(0, int(bottom_right.real) + 1):
            key = complex(x, y)

            if key == start:
                print('S', end="")
            elif key == end:
                print('E', end="")
            elif key in path:
                print('X', end="")
            elif key not in walls:
                print('.', end="")
            else:
                print('#', end="")
        print()

def manhattan_distance(a, b):
    # Calculate the Manhattan distance between two points in complex form
    return int(abs(a.real - b.real) + abs(a.imag - b.imag))

def a_star_least_turns(walls, start, end) -> List[Point]:
    # Directions represented as complex unit vectors
    directions = [1, -1, 1j, -1j]
    
    best_paths = []
    open_set = []
    heapq.heappush(open_set, (0, hash(start), start, [start], None, 0))  # (priority, current, path, direction, turns)
    best_turn_count = {start: 0}
    best_path_length = {start: 0}
    
    while open_set:
        _, point_hash, current, path, prev_direction, turns = heapq.heappop(open_set)

        if current == end:
            best_paths.append(path)
            if len(best_paths) > 10:
                break
        
        for d in directions:
            next_point = current + d
            if next_point not in walls:
                new_turns = turns
                if prev_direction is not None and d != prev_direction:
                    new_turns += 1

                new_path_length = len(path) + 1

                # Check if this path is better (less turns or shorter) than previously found path
                if (next_point not in best_turn_count or
                    new_turns < best_turn_count[next_point] or
                    (new_turns == best_turn_count[next_point] and new_path_length < best_path_length[next_point])):
                    
                    best_turn_count[next_point] = new_turns
                    best_path_length[next_point] = new_path_length
                    
                    priority = new_turns * 1000 + new_path_length + manhattan_distance(next_point, end)
                    heapq.heappush(open_set, (priority, hash(next_point), next_point, path + [next_point], d, new_turns))

    for i, path in enumerate(best_paths):
        print('Path #', i)
        print(path)

    return best_paths[0]

# Calculate number of turns in a path
def calculate_turns(path) -> int:
    if len(path) < 2:
        return 0
    
    turns = 0
    last_direction = path[1] - path[0]
    # Initial direction is EAST, we need to account for that
    if last_direction != complex(1, 0):
        turns =+ 1

    for i in range(2, len(path)):
        new_direction = path[i] - path[i-1]
        if new_direction != last_direction:
            turns += 1
            last_direction = new_direction
    return turns

def calculate_score(path: List[Point]) -> int:
    # remove initial point in path
    return len(path) - 1 + calculate_turns(path) * 1000

# returns set of walls, starting point and end point in maze
def parse_input(input: str) -> tuple[Walls, Point, Point]:
    start = complex(-1, -1)
    end = complex(-1, -1)
    walls: Set[Point] = set()
    for y, line in enumerate(input.strip().splitlines()):
        for x, char in enumerate(line.strip()):
            pos = complex(x, y)
            match char:
                case '#':
                    walls.add(pos)
                case 'S':
                    start = pos
                case 'E':
                    end = pos

    return (walls, start, end)