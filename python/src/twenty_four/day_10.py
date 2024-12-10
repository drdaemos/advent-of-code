from collections import defaultdict
from typing import Dict, List, Set

def main():
    print("Advent of Code 2024 - day 10")
    try:
        input = open("./inputs/day_10.txt").read()
        print(" Part one: %d" % part_one(input))
        print(" Part two: %d" % part_two(input))
    except:
        print(" Input malformed / not found, expected to have `./inputs/day_10.txt`")

def part_one(input: str) -> int:
    map = parse_input(input)
    paths = depth_first_search(map)
    sum = 0
    for trailhead in paths:
        ends = set([x[-1] for x in paths[trailhead] if x])
        sum += len(ends)

    return sum

def part_two(input: str) -> int:
    map = parse_input(input)
    paths = depth_first_search(map)
    sum = 0
    for trailhead in paths:
        sum += len(paths[trailhead])

    return sum

def parse_input(input:str) -> List[List[int]]:
    map = list()
    lines = input.split('\n')

    for line in lines:
        row = list()
        for x, char in enumerate(line):
            row.append(int(char) if char != '.' else -1)
        
        map.append(row)

    return map

def depth_first_search(matrix: List[List[int]]) -> Dict[tuple[int, int], List[tuple[int, int]]]:
    directions = [(-1, 0), (1, 0), (0, -1), (0, 1)]
    rows = len(matrix)
    cols = len(matrix[0])
    paths: Dict[tuple[int, int], List[tuple[int, int]]] = defaultdict(list)

    def dfs(x, y, path):
        if matrix[x][y] == 9:
            paths[path[0]].append(path + [(x, y)])
            return

        for dx, dy in directions:
            new_x, new_y = x + dx, y + dy

            if 0 <= new_x < rows and 0 <= new_y < cols:
                if matrix[new_x][new_y] == matrix[x][y] + 1:
                    dfs(new_x, new_y, path + [(new_x, new_y)])

    # Find all starting points (cells with 0)
    start_points = [(i, j) for i in range(rows) for j in range(cols) if matrix[i][j] == 0]

    # Start DFS from each starting point
    for x, y in start_points:
        dfs(x, y, [(x, y)])

    return paths