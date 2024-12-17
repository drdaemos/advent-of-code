from collections import defaultdict
from typing import Dict, List

def main(debug = False):
    print("Advent of Code 2024 - day 5")
    try:
        input = open("./inputs/day_05.txt").read()
        print(" Part one: %d" % part_one(input))
        print(" Part two: %d" % part_two(input))
    except:
        print(" Input not found, expected to have `./inputs/day_05.txt`")

def part_one(input: str) -> int:
    rules = parse_rules(input)
    middles = []
    for update in parse_updates(input):
        if is_valid_update(update, rules):
            middles.append(update[(len(update) - 1)//2])

    return sum(map(int, middles))

def part_two(input: str) -> int:
    rules = parse_rules(input)
    middles = []
    for update in parse_updates(input):
        if not is_valid_update(update, rules):
            fixed = fix_invalid_update(update, rules)
            middles.append(fixed[(len(fixed) - 1)//2])

    return sum(map(int, middles))
    
def parse_rules(input: str) -> Dict[str, List[str]]:
    lines = input.strip().split('\n\n')[0].split('\n')
    hashmap: dict[str, List[str]] = defaultdict(list)
    
    for line in lines:
        key, value = line.split('|')
        hashmap[key].append(value)
    
    return hashmap

def parse_updates(input: str) -> List[List[str]]:
    lines = input.strip().split('\n\n')[-1].split('\n')
    return list(map(lambda line: line.split(','), lines))

def is_valid_update(update: List[str], rules: Dict[str, List[str]]) -> bool:
    reversed = update[::-1]
    # Iterate over pages from the end to start
    for i, page in enumerate(reversed):
        invalid = rules.get(page)
        if invalid is None:
            continue

        rest = reversed[i+1:]
        # If next pages have any incorrect pages based on ruleset - update is invalid
        if set(invalid) & set(rest): 
            return False

    return True
        
def fix_invalid_update(update: List[str], rules: Dict[str, List[str]]) -> List[str]:
    reversed = update[::-1]
    needs_sorting = True

    # Apply a bubble-sort like algorithm
    # Exchange adjacent elements until they don't have any following incorrect pages
    while needs_sorting:
        needs_sorting = False

        for i in range(len(reversed) - 1):
            invalid = rules.get(reversed[i])
            if invalid is None:
                continue

            rest = reversed[i+1:]
            if set(invalid) & set(rest):
                reversed[i], reversed[i + 1] = reversed[i + 1], reversed[i]
                needs_sorting = True

    reversed.reverse()
    return reversed