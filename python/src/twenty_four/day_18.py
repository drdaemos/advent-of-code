from collections import defaultdict
import heapq
from multiprocessing import Pool
from typing import Dict, List

type Walls = List[Point]

# .real is x, .imag is y
type Point = complex

def main(debug = False):
    print("Advent of Code 2024 - day 18")
    try:
        input = open("./inputs/day_18.txt").read()
        print(" Part one: %d" % part_one(input, 71, 1024, debug))
        print(" Part two: %s" % part_two(input, 71, 1024, debug))
    except (ValueError, FileNotFoundError):
        print(" Input malformed / not found, expected to have `./inputs/day_18.txt`")

def part_one(input: str, grid_size: int, after: int, debug = False) -> int:
    walls = parse_input(input)
    start = complex(0, 0)
    end = complex(grid_size-1, grid_size-1)
    result = find_path(walls, start, end, grid_size, after, debug)
    if result is None:
        raise Exception("No path found")

    if debug: print_map(walls[:after], grid_size, start, end, result)
    return len(result[1]) - 1

def part_two(input: str, grid_size: int, after: int, debug = False) -> str:
    walls = parse_input(input)
    start = complex(0, 0)
    end = complex(grid_size-1, grid_size-1)

    with Pool(10) as p:
        data = map(lambda i: (walls, start, end, grid_size, i) ,range(after, len(walls)))
        results = p.map(run_search, data)

    min_index = min(list(filter(lambda x: x is not None, results))) # type: ignore
    point = walls[min_index]
    return str(int(point.real)) + ',' + str(int(point.imag))

def run_search(data: tuple[Walls, Point, Point, int, int]):
    result = find_path(data[0], data[1], data[2], data[3], data[4] + 1)
    if result is None:
        return data[4]
    return None

def find_path(walls: Walls, start: Point, end: Point, grid_size: int, after: int, debug = False) -> tuple[List[Point], List[Point]] | None:    
    tiebreak = 0
    open_set: List[tuple[int, int, Point, List[Point]]] = []
    closed_set: Dict[Point, int] = defaultdict()
    heapq.heappush(open_set, (0, tiebreak, start, [start]))  # (priority, tiebreak, current, path)
    
    while open_set:
        priority, _, current, path = heapq.heappop(open_set)

        # similar path has been found already - skip
        if current in closed_set and priority >= closed_set[current]:
            continue

        # add as a candidate for best path if length is lower
        if current == end:
            return (list(closed_set.keys()), path)
        
        closed_set[current] = priority
        
        for d in [1, -1, 1j, -1j]:
            next_point = current + d
            active_walls = after
            if next_point not in walls[:active_walls] and next_point not in path and 0 <= next_point.real < grid_size and 0 <= next_point.imag < grid_size:
                tiebreak += 1
                priority = manhattan_distance(next_point, end) + len(path)
                heapq.heappush(open_set, (priority, tiebreak, next_point, path + [next_point]))

    return None

def manhattan_distance(a: Point, b: Point) -> int:
    return int(abs(b.real - a.real)) + int(abs(b.imag - a.imag))

# returns set of walls in a maze
def parse_input(input: str) -> Walls:
    return [complex(int(i.split(',')[0]), int(i.split(',')[1])) for i in input.strip().splitlines()]

def print_map(walls: Walls, grid_size: int, start: Point, end: Point, path: tuple[List[Point], List[Point]]):
    print()
    for y in range(0, grid_size):
        for x in range(0, grid_size):
            key = complex(x, y)

            if key == start:
                print('S', end="")
            elif key == end:
                print('E', end="")
            elif key in path[1]:
                print('¤', end="")
            elif key in path[0]:
                print('¿', end="")
            elif key in walls:
                print('#', end="")
            else:
                print('.', end="")
        print()