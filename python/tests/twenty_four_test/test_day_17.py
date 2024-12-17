from twenty_four import day_17

input = """Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"""

input_2 = """Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"""

def test_part_one():
    assert day_17.part_one(input) == '4,6,3,5,6,3,5,2,1,0'
    assert day_17.part_one(input_2) == '0,3,5,4,3,0'

def test_part_two():
    assert day_17.part_two(input_2) == 117440