from typing import Dict, List, Set
from cachetools import cached

def main(debug = False):
    print("Advent of Code 2024 - day 19")
    try:
        input = open("./inputs/day_19.txt").read()
        print(" Part one: %d" % part_one(input, debug))
        print(" Part two: %d" % part_two(input, debug))
    except (ValueError, FileNotFoundError):
        print(" Input malformed / not found, expected to have `./inputs/day_19.txt`")

def part_one(input: str, debug = False) -> int:
    tokens, queries = parse_input(input)
    valid_queries = sum([1 for i in queries if valid_combinations(tokens, i)])
    return valid_queries

def part_two(input: str, debug = False) -> int:
    tokens, queries = parse_input(input)
    valid_queries = sum([valid_combinations(tokens, i, debug) for i in queries])
    return valid_queries

combination_cache: Dict[str, int] = {}

def valid_combinations(tokens: Set[str], query: str, debug = False) -> int:

    @cached(combination_cache)
    def search(part: str):
        if part == '':
            return 1
        
        total = 0
        
        for token in tokens:
            if part.startswith(token):
                total += search(part.removeprefix(token))

        return total
    
    valid = search(query)
    if debug: print(query, valid)

    return valid

def parse_input(input: str) -> tuple[Set[str], List[str]]:
    sections = input.strip().split('\n\n')
    tokens = set(sections[0].split(', '))
    queries = sections[1].splitlines()

    return (tokens, queries)