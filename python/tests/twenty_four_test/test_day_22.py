from twenty_four import day_22

input = """1
10
100
2024"""

input_2 = """1
2
3
2024"""

def test_part_one():
    assert day_22.part_one(input) == 37327623

def test_part_two():
    assert day_22.part_two(input) == 24
    assert day_22.part_two(input_2) == 23

def test_get_next_number():
    assert day_22.get_next_number(123) == 15887950
    assert day_22.get_next_number(15887950) == 16495136
    assert day_22.get_next_number(16495136) == 527345
    assert day_22.get_next_number(527345) == 704524
    assert day_22.get_next_number(704524) == 1553684
    assert day_22.get_next_number(1553684) == 12683156
    assert day_22.get_next_number(12683156) == 11100544
    assert day_22.get_next_number(11100544) == 12249484
    assert day_22.get_next_number(12249484) == 7753432
    assert day_22.get_next_number(7753432) == 5908254

def test_rotate_number():
    assert day_22.rotate_number(123, 10) == 5908254