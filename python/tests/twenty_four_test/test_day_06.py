from twenty_four import day_06


input = """....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."""

def test_day_05_part_one():
    assert day_06.part_one(input) == 41
