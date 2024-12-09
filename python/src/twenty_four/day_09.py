from collections import defaultdict
from typing import Dict, List

def main():
    print("Advent of Code 2024 - day 9")
    try:
        input = open("./inputs/day_09.txt").read()
        print(" Part one: %d" % part_one(input))
        print(" Part two: %d" % part_two(input))
    except:
        print(" Input malformed / not found, expected to have `./inputs/day_09.txt`")
    
def part_one(input: str) -> int:
    block_sizes = parse_input(input)
    busy = block_sizes[::2]
    len_busy = len(busy)
    reversed_unpacked = flatten(list(map(lambda item: [len_busy - item[0] - 1] * item[1], enumerate(reversed(busy)))))
    defragmented: List[str] = list()

    right = 0
    for index, size in enumerate(block_sizes):
        if (index % 2 - 1) == 0:
            defragmented += map(str, reversed_unpacked[right:size+right])
            right += size
        else:
            defragmented += [str(index // 2)] * size

    defragmented = defragmented[:-right] + ['.'] * right

    return get_checksum(defragmented)

def part_two(input: str) -> int:
    block_sizes = parse_input_with_pos(input)
    busy = block_sizes[::2]
    total_blocks = len(busy)
    free_spaces = block_sizes[1::2]
    moved_free: Dict[int, List[str]] = defaultdict(list)
    moved_ids: List[str] = list()

    for index, block in enumerate(reversed(busy)):
        file_id = str(total_blocks - index - 1)
        for free_i, free_block in enumerate(free_spaces):
            if block[0] <= free_block[0] and block[1] > free_block[1]:
                moved_free[free_i] += [file_id] * block[0]
                free_spaces[free_i] = (free_block[0] - block[0], free_block[1])
                moved_ids.append(file_id)
                break

    defragmented: List[str] = list()

    for index in range(total_blocks):
        if str(index) not in moved_ids:
            defragmented += [str(index)] * busy[index][0]
        else:
            defragmented += ['.'] * busy[index][0]
        defragmented += moved_free[index]
        if index < len(free_spaces):
            defragmented += ['.'] * free_spaces[index][0]

    return get_checksum(defragmented)

def parse_input(input: str) -> List[int]:
    return list(map(int, input))

# Returns list with a tuple of block size and block pos
def parse_input_with_pos(input: str) -> List[tuple[int, int]]:
    blocks: List[tuple[int, int]] = list()
    cursor = 0

    for char in input:
        blocks.append((int(char), cursor))
        cursor += int(char)

    return blocks

def flatten(xss: List[List[int]]):
    return [x for xs in xss for x in xs]

def get_checksum(input: List[str]) -> int:
    checksum = 0

    for pos, id in enumerate(input):
        if id == '.':
            continue

        checksum += pos * int(id)

    return checksum