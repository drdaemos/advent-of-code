from copy import deepcopy
from enum import Enum, auto
from typing import Dict, List

class Direction(Enum):
    NORTH = complex(0, -1)
    EAST = complex(1, 0)
    SOUTH = complex(0, 1)
    WEST = complex(-1, 0)

class Kind(Enum):
    WALL = auto()
    BOX = auto()
    BOX_RIGHT = auto()

type Map = Dict[Point, Kind]

# .real is x, .imag is y
type Point = complex

def main(debug = False):
    print("Advent of Code 2024 - day 15")
    try:
        input = open("./inputs/day_15.txt").read()
        print(" Part one: %d" % part_one(input))
        print(" Part two: %d" % part_two(input))
    except:
        print(" Input malformed / not found, expected to have `./inputs/day_15.txt`")

def part_one(input: str, debug = False) -> int:
    map, moves, start = parse_input(input)
    return run_simulation(map, moves, start, debug)

def part_two(input: str, debug = False) -> int:
    map, moves, start = parse_input(input, True)
    return run_simulation(map, moves, start, debug)

def run_simulation(map: Map, moves: List[Direction], start: Point, debug: bool) -> int:
    current_pos = start
    bottom_right = list(map)[-1]

    for move in moves:
        current_pos = simulate_move(map, move, current_pos)

    if debug: print_map(map, current_pos, int(bottom_right.real) + 1)

    return calculate_box_coordinates(map)

def calculate_box_coordinates(map: Map) -> int:
    sum = 0
    for key in map.keys():
        if map[key] == Kind.BOX:
            sum += int(key.real + key.imag * 100)
    
    return sum
    
def print_map(map: Map, cursor: Point, size: int):
    for y in range(0, size //2):
        for x in range(0, size):
            key = complex(x, y)
            if key == cursor:
                print('R' , end="")
                continue

            if key not in map:
                print('.', end="")
                continue

            match map[key]:
                case Kind.WALL:
                    print('#', end="")
                case Kind.BOX:
                    print('[', end="")
                case Kind.BOX_RIGHT:
                    print(']', end="")
                case _:
                    print('.', end="")
        print()

def simulate_move(map: Map, move: Direction, pos: Point) -> Point:
    next_pos = pos + move.value
    if next_pos not in map:
        return next_pos
    
    if map[next_pos] == Kind.WALL:
        return pos
    
    # else == Kind.BOX:
    map_copy = deepcopy(map)
    result = move_box(map_copy, next_pos, move, [])

    # if move is possible - copy changes to original map
    if result: 
        map.clear()
        map.update(map_copy)

    return next_pos if result else pos
    
def move_box(map: Map, start: Point, move: Direction, dirty: List[Point]) -> bool:
    pos_cursor = start + move.value
    visited: List[Point] = [start]
    while True:
        # Free space available
        if pos_cursor not in map:

            # pull to free space in reverse order
            for i in range(len(visited)):
                cursor = visited.pop()
                cursor_value = map[cursor]
                
                dirty.append(cursor)
                # If left box and right is a box and movement is vertical
                # move horizontal neighbours
                if abs(move.value.imag) > 0:
                    right = cursor+Direction.EAST.value
                    left = cursor+Direction.WEST.value
                    if cursor_value == Kind.BOX and right not in dirty and right in map and map[right] == Kind.BOX_RIGHT:
                        if not move_box(map, cursor + Direction.EAST.value, move, dirty):
                            return False

                    elif cursor_value == Kind.BOX_RIGHT and left not in dirty and left in map and map[left] == Kind.BOX:
                        if not move_box(map, cursor + Direction.WEST.value, move, dirty):
                            return False
                        
                # move this block
                map[pos_cursor] = cursor_value
                pos_cursor = cursor

            del map[start]
            return True

        # Unable to move
        if map[pos_cursor] == Kind.WALL:
            return False
        
        visited.append(pos_cursor)
        pos_cursor += move.value

# Returns game map, list of next moves and a starting point
def parse_input(input: str, wider = False) -> tuple[Map, List[Direction], Point]:
    start = complex(-1, -1)
    map = {}
    sections = input.strip().split('\n\n')
    for y, line in enumerate(sections[0].splitlines()):
        for x, char in enumerate(line):
            pos = complex(x * 2, y) if wider else complex(x, y)

            match char:
                case 'O':
                    map[pos] = Kind.BOX
                    if wider: map[pos + Direction.EAST.value] = Kind.BOX_RIGHT
                case '#':
                    map[pos] = Kind.WALL
                    if wider: map[pos + Direction.EAST.value] = Kind.WALL
                case '@':
                    start = pos

    moves = []
    for char in sections[1].strip().replace("\n", ""):
        match char:
            case '^':
                moves.append(Direction.NORTH)
            case '>':
                moves.append(Direction.EAST)
            case 'v':
                moves.append(Direction.SOUTH)
            case '<':
                moves.append(Direction.WEST)

    return (map, moves, start)