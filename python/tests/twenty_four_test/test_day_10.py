from twenty_four import day_10

input_1 = """...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9"""

input_2 = """..90..9
...1.98
...2..7
6543456
765.987
876....
987...."""

input_3 = """10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01"""

input_4 = """89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"""

input_5 = """.....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9...."""

input_6 = """..90..9
...1.98
...2..7
6543456
765.987
876....
987...."""

input_7 = """012345
123456
234567
345678
4.6789
56789."""

def test_part_one():
    assert day_10.part_one(input_1) == 2
    assert day_10.part_one(input_2) == 4
    assert day_10.part_one(input_3) == 3
    assert day_10.part_one(input_4) == 36

def test_part_two():
    assert day_10.part_two(input_5) == 3
    assert day_10.part_two(input_6) == 13
    assert day_10.part_two(input_7) == 227
    assert day_10.part_two(input_4) == 81