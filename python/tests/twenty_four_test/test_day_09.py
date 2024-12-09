from twenty_four import day_09

input = "2333133121414131402"
input_2 = "228993736146253756574365439219143024328579513043515929473982105837116657267443959698474894465011455010224"
input_3 = "228993736146253756574365439219143024328"

def test_part_one():
    assert day_09.part_one(input) == 1928

def test_part_two():
    assert day_09.part_two(input) == 2858
    assert day_09.part_two(input_3) == 44609
    assert day_09.part_two(input_2) == 890695