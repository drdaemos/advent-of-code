from typing import List
from networkx import Graph, shortest_path
from tqdm import tqdm

# .real = X, .imag = Y
type Point = complex

def main(debug = False):
    print("Advent of Code 2024 - day 20")
    try:
        input = open("./inputs/day_20.txt").read()
        print(" Part one: %d" % part_one(input, 100, debug))
        print(" Part two: %d" % part_two(input, 100, debug))
    except (ValueError, FileNotFoundError):
        print(" Input malformed / not found, expected to have `./inputs/day_20.txt`")

def part_one(input: str, threshold: int, debug = False) -> int:
    graph, start, end = parse_input(input)
    # I'm not writing another pathfinding algorithm
    path = shortest_path(graph, start, end)

    cheats_count = 0
    for i, node in enumerate(path):
        for direction in [2, -2, 2j, -2j]:
            # if candidate node shortcuts the path
            if is_good_shortcut(node, node + direction, path[i+threshold:]):
                cheats_count += 1

    return cheats_count

# Essentially a bruteforce, may run for a couple of minutes
def part_two(input: str, threshold: int, debug = False) -> int:
    graph, start, end = parse_input(input)
    path = shortest_path(graph, start, end)

    cheats_count = 0
    for i, node in enumerate(tqdm(path)):
        future_path = path[i+threshold:]
        for candidate in [x for x in path if manhattan_distance(node, x) <= 20 and node != x]:
            # if candidate node shortcuts the path
            if is_good_shortcut(node, candidate, future_path):
                if debug: print_map(graph, 15, node, candidate, [])
                cheats_count += 1

    return cheats_count

def is_good_shortcut(entry: Point, exit: Point, path: List[Point]) -> bool:
    if exit in path:
        skipped_length = path.index(exit)
        cheat_distance = manhattan_distance(entry, exit)
        if (skipped_length - cheat_distance) >= 0:
            return True

    return False
    
def manhattan_distance(a: Point, b: Point) -> int:
    return int(abs(b.real - a.real)) + int(abs(b.imag - a.imag))

def parse_input(input: str) -> tuple[Graph, Point, Point]:
    graph: Graph[Point] = Graph()
    nodes = set()
    start = complex(-1,-1)
    end = complex(-1,-1)
    for y, line in enumerate(input.strip().splitlines()):
        for x, char in enumerate(line):
            pos = complex(x,y)
            if char == '.':
                nodes.add(pos)
            elif char == 'S':
                nodes.add(pos)
                start = pos
            elif char == 'E':
                nodes.add(pos)
                end = pos

    graph.add_nodes_from(nodes)

    for node in nodes:
        for direction in [1, -1, 1j, -1j]:
            if (node + direction) in nodes:
                graph.add_edge(node, node+direction)
    
    return (graph, start, end)
    
def print_map(graph: Graph, size: int, start: Point, end: Point, path: List[Point]):
    print()
    for y in range(0, size):
        for x in range(0, size):
            key = complex(x, y)

            if key == start:
                print('S', end="")
            elif key == end:
                print('E', end="")
            elif key in path:
                print('⌖', end="")
            elif key in graph.nodes():
                print('.', end="")
            else:
                print('#', end="")
        print()