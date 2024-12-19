from collections import defaultdict
from twenty_four import day_19

input = """r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"""

input_test = """r, wr, b, g, bwu, rb, gb, br

brwrr
rgb
brwrrrwr
brwrrgb
brwrrgbbrwrr"""

def test_part_one():
    assert day_19.part_one(input) == 6

def test_part_two():
    assert day_19.part_two(input) == 16
    assert day_19.part_two(input_test) == 18