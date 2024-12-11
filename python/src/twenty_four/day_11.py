from functools import lru_cache
import math
from typing import Iterable, List

def main():
    print("Advent of Code 2024 - day 11")
    try:
        input = open("./inputs/day_11.txt").read()
        print(" Part one: %d" % part_one(input))
        print(" Part two: %d" % part_two(input))
    except:
        print(" Input malformed / not found, expected to have `./inputs/day_11.txt`")

def part_one(input: str) -> int:
    return count_stones(parse_input(input), 25)

def part_two(input: str) -> int:
    return count_stones(parse_input(input), 75)

def count_stones(stones: Iterable[int], times: int) -> int:
    return sum(map(lambda stone: flip_stone_times(stone, times), stones))

def parse_input(input: str) -> Iterable[int]:
    return map(int, input.split(' '))

@lru_cache(maxsize=None)
def flip_stone_times(stone: int, times: int) -> int:
    if times > 0:
        return sum(map(lambda stone: flip_stone_times(stone, times - 1), flip_stone_once(stone)))
    else:
        return 1

@lru_cache(maxsize=None)
def flip_stone_once(stone: int) -> List[int]:
    match stone:
        case 0:
            return [1]
        case stone if (int(math.log10(stone)) + 1) % 2 == 0:
            as_str = str(stone)
            half = len(as_str) // 2
            return list(map(int, [as_str[:half], as_str[half:]]))
        case _:
            return [stone * 2024]