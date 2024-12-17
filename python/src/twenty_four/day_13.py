from dataclasses import dataclass
import re
from typing import List
import numpy as np

@dataclass
class LinearEquationSystem:
    coefficients: List[List[int]]
    constants: List[int]

def main(debug = False):
    print("Advent of Code 2024 - day 13")
    try:
        input = open("./inputs/day_13.txt").read()
        print(" Part one: %d" % part_one(input))
        print(" Part two: %d" % part_two(input))
    except:
        print(" Input malformed / not found, expected to have `./inputs/day_13.txt`")

def part_one(input: str) -> int:
    systems = parse_input(input, 0)
    return sum(map(solve_system_integer, systems))

def part_two(input: str) -> int:
    systems = parse_input(input, 10000000000000)
    return sum(map(solve_system_integer, systems))

def parse_input(input: str, constant_offset: int) -> List[LinearEquationSystem]:
    blocks = input.strip().split('\n\n')
    systems: List[LinearEquationSystem] = []
    for block in blocks:
        lines = block.splitlines()
        a = re.match(r".*?(\d+).*?(\d+)", lines[0].strip())
        b = re.match(r".*?(\d+).*?(\d+)", lines[1].strip())
        prize = re.match(r".*?(\d+).*?(\d+)", lines[2].strip())

        if a is None or b is None or prize is None:
            raise Exception('Input malformed')
        
        coefficients = [
            [int(a.group(1)), int(b.group(1))],
            [int(a.group(2)), int(b.group(2))],
        ]
        constants = [int(prize.group(1)) + constant_offset, int(prize.group(2)) + constant_offset]
        systems.append(LinearEquationSystem(coefficients=coefficients, constants=constants))

    return systems

# Returns the number of required tokens or None if there is no integer solution
def solve_system_integer(system: LinearEquationSystem) -> int:
    solution = np.linalg.solve(np.array(system.coefficients), np.array(system.constants))

    # count only if solution is in integers
    # magic number is added to convert floats that are slightly under the integer value
    if all((i + 0.001) % 1 < 0.01 for i in solution):
        return int(solution[0] + 0.001) * 3 + int(solution[1] + 0.001)
    

    return 0