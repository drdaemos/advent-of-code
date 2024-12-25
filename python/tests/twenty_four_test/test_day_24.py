from twenty_four import day_24

input_short = """x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"""

input_long = """x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"""

input_p2 = """x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00"""

input_carry_adder_working = """x00: 0
x01: 1
x02: 0
y00: 1
y01: 1
y02: 0

x00 XOR y00 -> z00
x00 AND y00 -> c00
x01 XOR y01 -> a01
c00 XOR a01 -> z01
c00 AND a01 -> d01
x01 AND y01 -> e01
d01 OR e01 -> c01
x02 XOR y02 -> a02
c01 XOR a02 -> z02
c01 AND a02 -> d02
x02 AND y02 -> e02
d02 OR e02 -> z03"""


input_carry_adder_faulty = """x00: 0
x01: 1
x02: 0
y00: 1
y01: 1
y02: 0

x00 XOR y00 -> z00
x00 AND y00 -> c00
x01 XOR y01 -> a01
c00 XOR a01 -> c01
c00 AND a01 -> d01
x01 AND y01 -> e01
d01 OR e01 -> z01
x02 XOR y02 -> a02
c01 XOR a02 -> z02
c01 AND a02 -> d02
x02 AND y02 -> e02
d02 OR e02 -> z03"""

def test_part_one():
    assert day_24.part_one(input_short) == 4
    assert day_24.part_one(input_long) == 2024
    assert day_24.part_one(input_carry_adder_working) == 5 # 010 + 011 = 0101
    assert day_24.part_one(input_carry_adder_faulty) == 3 # c01 swapped with z01

def test_part_two():
    # assert day_24.part_two(input_carry_adder_working) == ""
    assert day_24.part_two(input_carry_adder_faulty, True) == "c01,z01"
    # assert day_24.part_two(input_p2) == "z00,z01,z02,z05"
    # assert day_24.part_two(input_long) == ""

    # dkr,ggk,hhh,htp,rhv,z05,z15,z20