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
    path = find_path_least_turns(walls, start, end, complex(1, 0), start)
    if path is None:
        raise Exception('Path not found')
    return calculate_score(path)

def part_two(input: str, debug = False) -> int:
    walls, start, end = parse_input(input)
    path = find_path_least_turns(walls, start, end, complex(1, 0), start)
    if path is None:
        raise Exception('Best path not found')

    alternatives = find_alternatives(walls, start, end, path)

    # calculate set of points
    points = set(path)
    for item in alternatives:
        points.update(item)

    if debug: print_map(walls, start, end, list(points))

    return len(points)

def find_alternatives(walls: Walls, start: Point, end: Point, best_path: List[Point]) -> List[List[Point]]:
    best_score = calculate_score(best_path)
    # reversed_path = list(reversed(best_path))
    reversed_path = best_path
    alternatives = []
    # walk back and spawn pathfinding on each possible alternative route
    for i, node in enumerate(reversed_path):
        # fully traced
        if node == end:
            break
    
        path_before = reversed_path[:i+1]
        # ignore actual path and try look for alternatives
        for d in [1, -1, 1j, -1j]:
            next_point = node + d
            if next_point not in walls and next_point not in reversed_path:
                alternative = find_path_least_turns(walls, next_point, end, d, node)
                if alternative is not None:
                    alternatives.append(list(path_before + alternative))
    
    return list(filter(lambda path: calculate_score(path) == best_score, alternatives))

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

def find_path_least_turns(walls: Walls, start: Point, end: Point, direction: complex, ban_start: Point) -> List[Point] | None:
    # Directions represented as complex unit vectors
    directions = [1, -1, 1j, -1j]
    
    tiebreak = 0
    open_set: List[tuple[int, int, Point, List[Point], complex, int]] = []
    heapq.heappush(open_set, (0, tiebreak, start, [start], complex(1, 0), 0))  # (priority, tiebreak, current, path, direction, turns)
    best_turn_count = {start: 0}
    
    while open_set:
        _, _, current, path, prev_direction, turns = heapq.heappop(open_set)

        if current == end:
            return path
        
        for d in directions:
            next_point = current + d
            if next_point not in walls and next_point not in path and next_point != ban_start:

                new_turns = turns
                if d != prev_direction:
                    new_turns += 1

                # Check if this path is better (less turns) than previously found path
                if next_point not in best_turn_count or new_turns <= best_turn_count[next_point]:
                    best_turn_count[next_point] = new_turns
                    tiebreak += 1
                    heapq.heappush(open_set, (calculate_score(path), tiebreak, next_point, path + [next_point], d, new_turns))

    return None

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