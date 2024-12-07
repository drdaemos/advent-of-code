from twenty_four import day_07

input = """190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"""

def test_part_one():
    assert day_07.part_one(input) == 3749

def test_part_two():
    assert day_07.part_two(input) == 11387