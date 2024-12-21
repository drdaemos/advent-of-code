from twenty_four import day_20

input = """###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"""

def test_part_one():
    assert day_20.part_one(input, 20) == 5

def test_part_two():
    assert day_20.part_two(input, 76) == 3
    assert day_20.part_two(input, 74) == 7
    assert day_20.part_two(input, 68) == 55