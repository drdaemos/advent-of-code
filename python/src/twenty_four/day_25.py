from typing import List


def main(debug = False):
    print("Advent of Code 2024 - day 25")
    try:
        input = open("./inputs/day_25.txt").read()
        print(" Part one: %d" % part_one(input, debug))
        print(" Part two: Go and collect the rest of the stars")
    except (ValueError, FileNotFoundError):
        print(" Input malformed / not found, expected to have `./inputs/day_25.txt`")

def part_one(input: str, debug = False) -> int:
    locks, keys = parse_input(input)
    fitting = 0
    for lock in locks:
        for key in keys:
            if all([x[0]+x[1] <= 7 for x in zip(lock, key)]):
                fitting += 1

    return fitting

# returns a tuple of lock and key heights
def parse_input(input: str) -> tuple[List[List[int]], List[List[int]]]:
    locks = []
    keys = []

    for group in input.strip().split("\n\n"):
        heights: List[int] = list()
        is_lock = True
        for y, row in enumerate(group.splitlines()):
            for x, char in enumerate(row):
                if len(heights) <= x:
                    heights.append(0)

                if x == 0 and y == 0 and char == '.':
                    is_lock = False
                
                if char == "#":
                    heights[x] += 1
        
        if is_lock:
            locks.append(heights)
        else:
            keys.append(heights)
    
    return (locks, keys)
