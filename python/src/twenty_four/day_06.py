from collections import defaultdict
from copy import deepcopy
from dataclasses import dataclass
from enum import Enum, auto
from typing import Dict, List, Set

from tqdm import tqdm

# from tqdm import tqdm

# coordinate within map as (row, col)
type Point = tuple[int, int]

# movement direction
class Direction(Enum):
    NORTH = auto()
    EAST = auto()
    SOUTH = auto()
    WEST = auto()

@dataclass
class Map:
    # Walls index by rows, contains list of cols with a wall
    walls_by_rows: Dict[int, List[int]]
    # Walls index by cols, contains list of rows with a wall
    walls_by_cols: Dict[int, List[int]]
    start: Point
    direction: Direction
    size: int

def main(debug = False):
    print("Advent of Code 2024 - day 6")
    try:
        input = open("./inputs/day_06.txt").read()
        print(" Part one: %d" % part_one(input))
        print(" Part two: %d" % part_two(input))
    except:
        print(" Input not found, expected to have `./inputs/day_06.txt`")

# Analyzes the 2d matrix (map)
# Traces the programmed path across the map
# Returns the number of visited positions before leaving the map
def part_one(input: str) -> int:
    map = parse_map(input)
    visited = simulate_path(map)
    return len(visited)

def part_two(input: str) -> int:
    working_blocks = 0
    map = parse_map(input)
    visited = simulate_path(map)
    for point in tqdm(visited):
        map_with_block = get_map_with_new_wall(map, point)
        if detect_loop(map_with_block, False):
            working_blocks += 1

    return working_blocks

# Returns dicts of walls in map, starting position and direction
def parse_map(input: str) -> Map:
    start: Point
    walls_by_rows: Dict[int, List[int]] = defaultdict(list)
    walls_by_cols: Dict[int, List[int]] = defaultdict(list)
    lines = input.split('\n')

    for row, line in enumerate(lines):
        for col, char in enumerate(line):
            if char == '#':
                walls_by_rows[row].append(col)
                walls_by_cols[col].append(row)
            if char == '^':
                start = (row, col)

    for key in walls_by_rows:
        walls_by_rows[key].sort()

    for key in walls_by_cols:
        walls_by_cols[key].sort()

    return Map(walls_by_rows=walls_by_rows, walls_by_cols=walls_by_cols, start=start, direction=Direction.NORTH, size=len(lines))

def get_map_with_new_wall(map: Map, wall: Point) -> Map:
    updated_map = deepcopy(map)
    updated_map.walls_by_cols[wall[1]].append(wall[0])
    updated_map.walls_by_rows[wall[0]].append(wall[1])

    for key in updated_map.walls_by_rows:
        updated_map.walls_by_rows[key].sort()

    for key in updated_map.walls_by_cols:
        updated_map.walls_by_cols[key].sort()

    return updated_map

# Simulates a path based on rules and returns the set of visited positions
def simulate_path(map: Map) -> Set[Point]:
    visited: Set[Point] = set([map.start])
    current = map.start
    direction = map.direction
    within_bounds = True

    while within_bounds:
        wall = get_wall_in_direction(map, current, direction)
        if wall is not None:
            visited.update(get_visited_set(current, direction, wall))
            current = update_current(direction, wall)
            direction = update_direction(direction)
        else:
            visited.update(get_visited_set(current, direction, get_bounding_wall(current, direction, map.size)))
            within_bounds = False

    return visited

# Simulates a path based on rules and tries to detect a loop
def detect_loop(map: Map, debug: bool) -> bool:
    visited_walls: Set[tuple[Point, Direction]] = set([])
    current = map.start
    direction = map.direction
    within_bounds = True
    if debug: print(map)

    while within_bounds:
        wall = get_wall_in_direction(map, current, direction)
        if debug: print(wall, direction)
        if wall is not None:
            if (wall, direction) in visited_walls:
                return True
            
            visited_walls.add((wall, direction))
            current = update_current(direction, wall)
            direction = update_direction(direction)
        else:
            within_bounds = False

    return False

def get_bounding_wall(current: Point, direction: Direction, size: int) -> Point:
    match direction:
        case Direction.NORTH:
            return (-1, current[1])
        case Direction.EAST:
            return (current[0], size)
        case Direction.SOUTH:
            return (size, current[1])
        case Direction.WEST:
            return (current[0], -1)

def get_wall_in_direction(map: Map, pos: Point, direction: Direction) -> Point | None:
    row = pos[0]
    col = pos[1]

    match direction:
        case Direction.NORTH:
            wall_row = next((x for x in reversed(map.walls_by_cols[col]) if x < row), -1)
            return (wall_row, col) if wall_row >= 0 else None
    
        case Direction.SOUTH:
            wall_row = next((x for x in map.walls_by_cols[col] if x > row), -1)
            return (wall_row, col) if wall_row >= 0 else None
    
        case Direction.EAST:
            wall_col = next((x for x in map.walls_by_rows[row] if x > col), -1)
            return (row, wall_col) if wall_col >= 0 else None
    
        case Direction.WEST:
            wall_col = next((x for x in reversed(map.walls_by_rows[row]) if x < col), -1)
            return (row, wall_col) if wall_col >= 0 else None


    return None

def print_visited(visited: Set[Point]):
    for row in range(0, 130):
        for col in range(0, 130):
            if (row, col) in visited:
                print('X', end="")   
            else:
                print('.', end="")
        print("")


def get_visited_set(current: Point, direction: Direction, wall: Point) -> Set:
    visited: Set[Point] = set([current])

    match direction:
        case Direction.NORTH | Direction.SOUTH:
            for row in range(current[0], wall[0]) if current[0] <= wall[0] else range(wall[0], current[0]):
                visited.add((row, current[1]))
        case Direction.EAST | Direction.WEST:
            for col in range(current[1], wall[1]) if current[1] <= wall[1] else range(wall[1], current[1]):
                visited.add((current[0], col))

    if wall in visited:
        visited.remove(wall)

    return visited

def update_current(direction: Direction, wall: Point) -> Point:
    match direction:
        case Direction.NORTH:
            return (wall[0]+1, wall[1])
        case Direction.EAST:
            return (wall[0], wall[1]-1)
        case Direction.SOUTH:
            return (wall[0]-1, wall[1])
        case Direction.WEST:
            return (wall[0], wall[1]+1)


def update_direction(current: Direction) -> Direction:
    match current:
        case Direction.NORTH:
            return Direction.EAST
        case Direction.EAST:
            return Direction.SOUTH
        case Direction.SOUTH:
            return Direction.WEST
        case Direction.WEST:
            return Direction.NORTH

def distance(a: Point, b: Point) -> int:
    return abs(a[0] - b[0]) + abs(a[1] - b[1])