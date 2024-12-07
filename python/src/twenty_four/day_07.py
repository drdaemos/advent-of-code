from dataclasses import dataclass
from itertools import product
from typing import List

@dataclass
class Equation:
    left: int
    right: List[int]

def main():
    print("Advent of Code 2024 - day 7")
    try:
        input = open("./inputs/day_07.txt").read()
        print("Part one: %d" % part_one(input))
        print("Part two: %d" % part_two(input))
    except:
        print("Input malformed / not found, expected to have `./inputs/day_07.txt`")
    
def part_one(input: str) -> int:
    valid = filter(lambda eq: is_valid_eq(eq, '+*'), parse_input(input))
    return sum(map(lambda eq: eq.left, list(valid)))

def part_two(input: str) -> int:
    valid = filter(lambda eq: is_valid_eq(eq, '+*|'), parse_input(input))
    return sum(map(lambda eq: eq.left, list(valid)))

def is_valid_eq(eq: Equation, ops: str) -> bool:
    combs = list(product(ops, repeat=(len(eq.right) - 1)))
    for operators in combs:
        sum = eq.right[0]
        for i in range(0, len(eq.right) - 1):
            val = eq.right[i+1]
            match operators[i]:
                case '*':
                    sum *= val
                case '+':
                    sum += val
                case '|':
                    sum = concat(sum, val)
        if sum == eq.left:
            return True

    return False

def concat(a: int, b: int) -> int:
    match b:
        case b if b < 10:
            return a * 10 + b
        case b if b < 100:
            return a * 100 + b
        case b if b < 1000:
            return a * 1000 + b
        case b if b < 10000:
            return a * 10000 + b
        case _:
            raise Exception('Unknown case')

def parse_input(input: str) -> List[Equation]:
    result = []
    for line in input.split('\n'):
        parts = line.split(': ')
        result.append(Equation(int(parts[0]), list(map(lambda x: int(x), parts[1].split(' ')))))

    return result