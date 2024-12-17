from typing import List, Dict, Set
from collections import defaultdict, deque

def main(debug = False):
    print("Advent of Code 2024 - day 12")
    try:
        input = open("./inputs/day_12.txt").read()
        print(" Part one: %d" % part_one(input))
        print(" Part two: %d" % part_two(input))
    except:
        print(" Input malformed / not found, expected to have `./inputs/day_12.txt`")

def part_one(input: str) -> int:
    map = parse_input(input)
    return bfs_find_regions_cost(map, False)

def part_two(input: str) -> int:
    map = parse_input(input)
    return bfs_find_regions_cost(map, True)

def parse_input(input: str) -> Dict[complex, str]:
    matrix_dict = {}
    lines = input.strip().splitlines()
    
    for y, line in enumerate(lines):
        for x, char in enumerate(line):
            position = complex(x, y)
            matrix_dict[position] = char
            
    return matrix_dict

def bfs_find_regions_cost(matrix: Dict[complex, str], is_shared_boundary: bool) -> int:
    visited = set()
    
    # Returns a list of connected points + perimeter of the region
    def bfs(start: complex) -> tuple[List[complex], int]:
        queue = deque([start])
        positions = []
        perimeter = 0
        sides: Dict[tuple[complex, int], Set[int]] = defaultdict(set)
        initial_value = matrix[start]
        
        while queue:
            current = queue.popleft()
            if current in visited:
                continue

            visited.add(current)
            positions.append(current)
            
            for delta in [1, -1, 1j, -1j]:
                neighbor = current + delta

                if neighbor not in matrix or matrix[neighbor] != initial_value:
                    # cound all different walls from different directions
                    if is_shared_boundary:
                        vec = delta
                        # if vertical check - add walls on different horizontal positions to set
                        if vec.imag:
                            sides[(vec, int(current.imag - start.imag))].add(int(current.real - start.real))
                        # same with horizontal direction
                        else:
                            sides[(vec, int(current.real - start.real))].add(int(current.imag - start.imag))
                            
                    perimeter += 1
                
                if neighbor in matrix and not neighbor in visited and matrix[neighbor] == initial_value:
                    queue.append(neighbor)

        # count all consecutive walls and separate if there is space
        if is_shared_boundary:
            perimeter = sum(map(lambda key: count_separate_edges(sides[key]), sides))
                
        return (positions, perimeter)
    
    cost = 0

    for position in matrix:
        if position not in visited:
            region, perimeter = bfs(position)
            if region:
                cost += len(region) * perimeter
    
    return cost

def count_separate_edges(edges: Set[int]) -> int:
    separate = 0
    sorted_edges = sorted(edges)
    prev = sorted_edges[0]

    for edge in sorted_edges[1:]:
        if edge - prev > 1:
            separate += 1

        prev = edge

    if True:
        separate += 1

    return separate