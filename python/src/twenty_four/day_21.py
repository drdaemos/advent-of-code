from enum import Enum, auto
from functools import cache
from typing import List
from networkx import Graph, all_shortest_paths

# +---+---+---+
# | 7 | 8 | 9 |
# +---+---+---+
# | 4 | 5 | 6 |
# +---+---+---+
# | 1 | 2 | 3 |
# +---+---+---+
#     | 0 | A |
#     +---+---+
numeric_keypad_nodes = {
    '7': complex(0,0),
    '8': complex(1,0),
    '9': complex(2,0),
    '4': complex(0,1),
    '5': complex(1,1),
    '6': complex(2,1),
    '1': complex(0,2),
    '2': complex(1,2),
    '3': complex(2,2),
    '0': complex(1,3),
    'A': complex(2,3)
}

#     +---+---+
#     | ^ | A |
# +---+---+---+
# | < | v | > |
# +---+---+---+
arrow_keypad_nodes = {
    '^': complex(1,0),
    'A': complex(2,0),
    '<': complex(0,1),
    'v': complex(1,1),
    '>': complex(2,1),
}

# movement direction
class KeypadType(Enum):
    NUMERIC = auto()
    ARROW = auto()
    
# .real = X, .imag = Y
type Point = complex

def main(debug = False):
    print("Advent of Code 2024 - day 21")
    try:
        input = open("./inputs/day_21.txt").read()
        print(" Part one: %d" % part_one(input, debug))
        print(" Part two: %d" % part_two(input, debug))
    except (ValueError, FileNotFoundError):
        print(" Input malformed / not found, expected to have `./inputs/day_21.txt`")

def part_one(input: str, debug = False) -> int:
    sum = 0
    for code in input.strip().splitlines():
        sum += find_minimal_path(code, 2, KeypadType.NUMERIC) * int(code[:3])

    return sum

def part_two(input: str, debug = False) -> int:
    sum = 0
    for code in input.strip().splitlines():
        sum += find_minimal_path(code, 25, KeypadType.NUMERIC) * int(code[:3])

    return sum

@cache
def get_keypad(type: KeypadType) -> Graph:
    nodes = arrow_keypad_nodes if type == KeypadType.ARROW else numeric_keypad_nodes
    keypad: Graph[Point] = Graph()
    keypad.add_nodes_from(nodes.values())
    for node in nodes.values():
        for direction in [1, -1, 1j, -1j]:
            if (node + direction) in nodes.values():
                keypad.add_edge(node, node+direction)

    return keypad

def find_paths(start: str, end: str, type: KeypadType) -> List[str]:  
    graph = get_keypad(type)
    nodes = arrow_keypad_nodes if type == KeypadType.ARROW else numeric_keypad_nodes
    paths = all_shortest_paths(graph, nodes[start], nodes[end])

    return list(map(encode_path, paths))

def encode_path(path: List[Point]) -> str:
    presses = ""
    for i, point in enumerate(path[1:]):
        match point - path[i]:
            case 1: presses += ">"
            case -1: presses += "<"
            case 1j: presses += "v"
            case -1j: presses += "^"
            case 0: presses += "A"

    return presses + 'A'

@cache
def find_minimal_path(code: str, depth: int, type: KeypadType) -> int:
    length = 0
    # For each movement in code
    for i, node in enumerate(code):
        prev = "A" if i == 0 else code[i-1]
        # Find all possible shortest paths
        paths = find_paths(prev, node, type)
        if depth == 0:
            # On lowest depth = just count shortest path
            length += min(map(len, paths))
        else:
            # On higher depths -> generate all paths from higher-level paths and count shortest
            length += min(find_minimal_path(path, depth - 1, KeypadType.ARROW) for path in paths)

    return length