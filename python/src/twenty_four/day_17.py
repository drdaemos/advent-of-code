
from copy import deepcopy
from dataclasses import dataclass, field
import re
from typing import List

@dataclass
class State:
    reg_a = 0
    reg_b = 0
    reg_c = 0
    std_out: List[int] = field(default_factory=list)

    def __hash__(self) -> int:
        return hash(str(self))

    def __repr__(self):
        return f'State(A: {self.reg_a}, B: {self.reg_b}, C: {self.reg_c})\nOutput: \n{self.std_out}'
    
    def flush(self):
        return ",".join(map(str, self.std_out))
    
    def out(self, operand):
        self.std_out.append(self.combo(operand) % 8)
    
    def combo(self, operand: int) -> int:
        match operand:
            case op if 0 <= op <= 3:
                return op
            case 4:
                return self.reg_a
            case 5:
                return self.reg_b
            case 6:
                return self.reg_c
            case 7:
                raise Exception('Combo operand 7 unsupported')
            case _:
                raise Exception('Unknown combo operand')

def main(debug = False):
    print("Advent of Code 2024 - day 17")
    try:
        input = open("./inputs/day_17.txt").read()
        print(" Part one: %s" % part_one(input))
        print(" Part two: %d" % part_two(input, debug))
    except (ValueError, FileNotFoundError):
        print(" Input malformed / not found, expected to have `./inputs/day_17.txt`")

def part_one(input: str, debug = False) -> str:
    state, program = parse_input(input)
    if debug: print(state)
    if debug: print(program)
    final = execute_program(state, program, debug)

    return final.flush()

def part_two(input: str, debug = False) -> int:
    state, program = parse_input(input)
    candidates = [0]

    # for each digit in a program (3bit) from the end
    for length in range(1, len(program) + 1):
        out = []

        # out of all numbers that satisfy lower bits
        for num in candidates:

            # iterate through all possible options
            for offset in range(2**3):
                a = (2**3) * num + offset
                state_copy = deepcopy(state)
                state_copy.reg_a = a

                # compare if program output is the one we seek (comparing last N digits)
                # add to be used in the next iterations
                if execute_program(state_copy, program, debug).std_out == program[-length:]:
                    out.append(a)

        candidates = out

    return min(candidates)

def execute_program(initial: State, program: List[int], debug: bool) -> State:
    state = deepcopy(initial)
    length = len(program)
    pointer = 0
    while pointer < length:
        operand = program[pointer+1]
        if debug: print("ptr", pointer, "op", operand)
        match program[pointer]:
            # adv
            case 0:
                if debug: print("adv", operand)
                state.reg_a = state.reg_a // pow(2, state.combo(operand))
                pointer += 2

            # bxl
            case 1:
                if debug: print("bxl", operand)
                state.reg_b = state.reg_b ^ operand
                pointer += 2

            # bst
            case 2:
                if debug: print("bst", operand)
                state.reg_b = state.combo(operand) % 8
                pointer += 2

            # jnz
            case 3:
                if debug: print("jnz", operand)
                if state.reg_a != 0:
                    pointer = operand
                else:
                    pointer += 2

            # bxc
            case 4:
                if debug: print("bxc", operand)
                state.reg_b = state.reg_b ^ state.reg_c
                pointer += 2

            # out
            case 5:
                if debug: print("out", operand)
                state.out(operand)
                pointer += 2

            # bdv
            case 6:
                if debug: print("bdv", operand)
                state.reg_b = state.reg_a // pow(2, state.combo(operand))
                pointer += 2

            # cdv
            case 7: 
                if debug: print("cdv", operand)
                state.reg_c = state.reg_a // pow(2, state.combo(operand))
                pointer += 2

        if debug: print(state)

    return state

def parse_input(input: str) -> tuple[State, List[int]]:
    sections = input.strip().split('\n\n')
    state = State()
    match = re.match(r".*A: (\d+)\n.*B: (\d+)\n.*C: (\d+).*", sections[0].strip())
    if match is None or any(i is None for i in match.groups()):
        raise Exception('Malformed input')

    state.reg_a = int(match.group(1))
    state.reg_b = int(match.group(2))
    state.reg_c = int(match.group(3))
    
    # extract ints
    program = list(map(int, sections[1].strip().split(" ")[1].split(',')))

    return (state, program)

