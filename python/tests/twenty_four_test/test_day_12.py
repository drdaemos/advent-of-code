from twenty_four import day_12

input_1 = """AAAA
BBCD
BBCC
EEEC"""

input_2 = """OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"""

input_3 = """RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"""

input_4 = """EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"""

input_5 = """AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"""

def test_part_one():
    assert day_12.part_one(input_1) == 140
    assert day_12.part_one(input_2) == 772
    assert day_12.part_one(input_3) == 1930

def test_part_two():
    # assert day_12.part_two(input_1) == 80
    assert day_12.part_two(input_2) == 436
    assert day_12.part_two(input_3) == 1206
    assert day_12.part_two(input_4) == 236
    assert day_12.part_two(input_5) == 368