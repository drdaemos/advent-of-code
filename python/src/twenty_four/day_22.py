from collections import defaultdict
from functools import cache
from typing import List, Dict
from itertools import islice
from collections import deque
from tqdm import tqdm

def main(debug = False):
    print("Advent of Code 2024 - day 22")
    try:
        input = open("./inputs/day_22.txt").read()
        print(" Part one: %d" % part_one(input, debug))
        print(" Part two: %d" % part_two(input, debug))
    except (ValueError, FileNotFoundError):
        print(" Input malformed / not found, expected to have `./inputs/day_22.txt`")

def part_one(input: str, debug = False) -> int:
    sum = 0
    for buyer in [int(i) for i in input.strip().splitlines()]:
        sum += rotate_number(buyer, 2000)

    return sum

def part_two(input: str, debug = False) -> int:
    buyers = [int(i) for i in input.strip().splitlines()]

    return find_most_common_set(buyers, 2000)

def find_most_common_set(buyers: List[int], rotations: int) -> int:
    bananas: Dict[tuple[int, int, int, int], int] = defaultdict(int)
    for buyer in tqdm(buyers):
        seen_in_buyer: Dict[tuple[int, int, int, int], int] = defaultdict(int)
        numbers = generate_numbers(buyer, rotations)
        ones = list(map(lambda x: x % 10, numbers))
        changes = calculate_changes([buyer % 10] + ones)

        for seq_change, seq_ones in zip(groupwise(changes, 4), groupwise(ones, 4)):
            if seq_change not in seen_in_buyer:
                seen_in_buyer[seq_change] = seq_ones[-1]

        for sequence, val in seen_in_buyer.items():
            bananas[sequence] = bananas[sequence] + val

    return max(bananas.values())

def calculate_changes(numbers):    
    if len(numbers) < 2:
        return []
    
    changes = []
    for i in range(1, len(numbers)):
        diff = numbers[i] - numbers[i - 1]
        changes.append(diff)
    
    return changes

@cache
def get_next_number(initial: int) -> int:
    first = ((initial * 64) ^ initial) % 16777216
    second = ((first // 32) ^ first) % 16777216
    third = ((second * 2048) ^ second) % 16777216

    return third

def rotate_number(initial: int, times: int) -> int:
    current = initial
    for i in range(times):
        current = get_next_number(current)

    return current

def generate_numbers(initial: int, count: int) -> List[int]:
    numbers = []
    current = initial
    for i in range(count):
        current = get_next_number(current)
        numbers.append(current)

    return numbers

def groupwise(iterable, n): 
    if n < 1:
        return None

    it = iter(iterable) 
    accum = deque(islice(it, n-1), maxlen=n)
        
    for _ in map(accum.append, it):
        yield tuple(accum)