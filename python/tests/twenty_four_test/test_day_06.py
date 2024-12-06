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

input_with_loop = """.#...
....#
.^...
#....
...#."""

def test_part_one():
    assert day_06.part_one(input) == 41

def test_part_two():
    assert day_06.part_two(input) == 6

def test_detect_loop():
    map_no_loop = day_06.parse_map(input)
    map_loop = day_06.parse_map(input_with_loop)
    assert day_06.detect_loop(map_no_loop, False) == False
    assert day_06.detect_loop(map_loop, False) == True