from collections import defaultdict
from itertools import combinations
from operator import xor, and_, or_
import operator
from typing import Any, Callable, Dict, List, Set
from networkx import DiGraph, NetworkXNoPath, NodeNotFound, draw, draw_networkx, draw_networkx_edges, draw_networkx_labels, draw_networkx_nodes, get_node_attributes, multipartite_layout, neighbors, set_node_attributes, shortest_path, shortest_path_length, spring_layout, topological_generations
import matplotlib.pyplot as plt
import networkx as nx

type Operator = Callable[[Any, Any], Any]

def main(debug = False):
    print("Advent of Code 2024 - day 24")
    try:
        input = open("./inputs/day_24.txt").read()
        print(" Part one: %d" % part_one(input, debug))
        print(" Part two: %s" % part_two(input, debug))
    except (ValueError, FileNotFoundError):
        print(" Input malformed / not found, expected to have `./inputs/day_24.txt`")

def part_one(input: str, debug = False) -> int:
    graph, values = parse_input(input)

    return get_decimal_value(graph, values, 'z')

def part_two(input: str, debug = False) -> str:
    graph, values = parse_input(input)
    output_bits = sorted(list(filter(lambda node: node.startswith('z'), graph.nodes())))

    problems: Set[str] = set()
    # previous level carry
    carry: str = ""
    bit_index = 0

    # circuit is expected to be a ripple-carry adder
    while bit_index < len(output_bits):
        operations: Dict[str, Operator] = get_node_attributes(graph, 'op')
        bit = output_bits[bit_index]
        layer = bit[1:]
        # check first layer of adder
        if int(layer) == 0:
            if sorted(graph.successors(bit)) != ['x00', 'y00']:
                print('z00 has a problem')
                problems.add(bit)
        
            # no try-catch here, if no carry found we cannot trace the circuit fully
            # find (x00 AND y00) gate - it is the first carry
            carry = next(x for x in graph.predecessors('x00') if operations[x] == and_ and x != bit)
            if sorted(graph.successors(carry)) != sorted(['x00', 'y00']):
                print('carry problem on level 0')
                problems.add(carry)

        elif int(layer) < len(output_bits) - 1:
            # check other layers of adder
            # Gates schematic:
            # xor_x_y = x** XOR y**
            # z** = xor_x_y XOR carry
            # and_carry = xor_x_y AND carry
            # and_x_y = x** AND y**
            # new carry = and_carry OR and_x_y

            # check xor_x_y
            # there X** node connected with something that is a XOR gate
            check = check_gate(graph, {'x'+layer, 'y'+layer}, xor, problems)
            if check[1] != "":
                swap_gates(graph, problems, check)
                continue
            xor_x_y = check[0]

            # check Z**
            if sorted(graph.successors(bit)) != sorted([xor_x_y, carry]) or operations[bit] != xor:
                if carry not in problems and xor_x_y not in problems:
                    print(bit,' misconnected', operations[bit], list(graph.successors(bit)))
                    # find another pair of a problem
                    faulty_pair = find_shared_predecessor(graph, {xor_x_y, carry}, xor)
                    # if z** needs to be swapped
                    if faulty_pair is not None and faulty_pair != bit:
                        print(faulty_pair,' misconnected')
                        problems.add(bit)
                        problems.add(faulty_pair)
                        swap_nodes(graph, bit, faulty_pair)
                        continue
                    # if connections need to be swapped
                    if xor_x_y not in graph.successors(bit):
                        faulty = next((x for x in graph.successors(bit) if x != carry), None)
                        if faulty is not None:
                            problems.add(xor_x_y)
                            problems.add(faulty)
                            swap_nodes(graph, xor_x_y, faulty)
                            continue
                    
                    if carry not in graph.successors(bit):
                        faulty = next((x for x in graph.successors(bit) if x != xor_x_y), None)
                        if faulty is not None:
                            problems.add(carry)
                            problems.add(faulty)
                            swap_nodes(graph, carry, faulty)
                            continue

            # check and_x_y
            check = check_gate(graph, {'x'+layer, 'y'+layer}, and_, problems)
            if check[1] != "":
                swap_gates(graph, problems, check)
                continue
            and_x_y = check[0]

            # check and_carry
            check = check_gate(graph, {xor_x_y, carry}, and_, problems)
            if check[1] != "":
                swap_gates(graph, problems, check)
                continue
            and_carry = check[0]
                
            # check new_carry
            check = check_gate(graph, {and_carry, and_x_y}, or_, problems)
            if check[1] != "":
                swap_gates(graph, problems, check)
                continue
            new_carry = check[0]

            # ready for next level
            carry = new_carry

        # move to next bit
        bit_index += 1

    # Try summing after fixing problems
    sum = get_decimal_value(graph, values, 'z')
    x = get_decimal_value(graph, values, 'x')
    y = get_decimal_value(graph, values, 'y')

    if debug: print("x", x, "y", y, "sum", sum, "is same", sum == x+y)
    if debug: print("miswired", list(sorted(problems)))
    if debug: draw_graph(graph)
    
    return ",".join(sorted(problems))

# Returns a tuple of nodes to be swapped if gate is incorrect or gate if everything looks good
def check_gate(graph: DiGraph, nodes: Set[str], op: Operator, problems: Set[str]) -> tuple[str, str]:
    try:
        operations: Dict[str, Operator] = get_node_attributes(graph, 'op')
        for successor in nodes:
            gate_candidate = next(x for x in graph.predecessors(successor) if operations[x] == op)
            if sorted(graph.successors(gate_candidate)) != sorted(nodes):
                if nodes & problems == set():
                    print(gate_candidate, 'misconnected')
                    # find another pair of a problem
                    faulty_pair = find_shared_predecessor(graph, nodes, op)
                    if faulty_pair is not None and faulty_pair != gate_candidate:
                        print(faulty_pair, 'misconnected')
                        return (gate_candidate, faulty_pair)
    except StopIteration:
        print("Failed to check gate", nodes, op)
    
    return (gate_candidate, "")

def swap_gates(graph: DiGraph, problems: Set[str], gates: tuple[str, str]):
    problems.add(gates[0])
    problems.add(gates[1])
    swap_nodes(graph, gates[0], gates[1])

def get_decimal_value(graph: DiGraph, values: Dict[str, int], type: str) -> int:
    bits = sorted(list(filter(lambda node: node.startswith(type), graph.nodes())), reverse=True)
    bit_values = [str(get_value(bit, values, graph)) for bit in bits]

    return int("".join(bit_values), 2)

def find_shared_predecessor(graph: DiGraph, connected_nodes: Set[str], op: Operator) -> str | None:
    operations: Dict[str, Operator] = get_node_attributes(graph, 'op')
    seen = set()
    for node in connected_nodes:
        for predecessor in [x for x in graph.predecessors(node) if operations[x] == op]:
            if predecessor in seen:
                return predecessor
            
            seen.add(predecessor)
    
    return None

def swap_nodes(graph: DiGraph, node_a: str, node_b: str):
    print("swapping " + node_a + " and " + node_b)
    edges_to_add: Set[tuple[str, str]] = set()
    edges_to_remove: Set[tuple[str, str]] = set()
    swap_node_attrs(graph, node_a, node_b)

    for successor_node in graph.successors(node_a):
        edges_to_remove.add((node_a, successor_node))
        edges_to_add.add((node_b, successor_node))

    for successor_node in graph.successors(node_b):
        edges_to_remove.add((node_b, successor_node))
        edges_to_add.add((node_a, successor_node))

    for edge in edges_to_remove:
        graph.remove_edge(edge[0], edge[1])

    for edge in edges_to_add:
        graph.add_edge(edge[0], edge[1])

def swap_node_attrs(graph: DiGraph, node_a: str, node_b: str):
    print("swapping attrs " + node_a + " and " + node_b)
    operations: Dict[str, Operator] = get_node_attributes(graph, 'op')
    set_node_attributes(graph, {node_a: {"op": operations[node_b]}, node_b: {"op": operations[node_a]}})

def get_value(node: str, values: Dict[str, int], graph: DiGraph) -> int:
    if node in values:
        return values[node]
    
    operations: Dict[str, Operator] = get_node_attributes(graph, 'op')
    inputs = [get_value(x, values, graph) for x in tuple(graph.successors(node))]

    values[node] = operations[node](inputs[0], inputs[1])

    return values[node]
    
def parse_input(input: str) -> tuple[DiGraph, Dict[str, int]]:
    values: Dict[str, int] = defaultdict(int)
    graph: DiGraph[str] = DiGraph()
    sections = input.strip().split('\n\n')

    for value in [x.split(': ') for x in sections[0].splitlines()]:
        graph.add_node(value[0], value=value[1])
        values[value[0]] = int(value[1])

    for connection in [x.split(' ') for x in sections[1].splitlines()]:
        graph.add_node(connection[4], op=parse_op(connection[1]))
        graph.add_edge(connection[4], connection[0])
        graph.add_edge(connection[4], connection[2])

    return (graph, values)

def parse_op(input: str) -> Operator:
    match input:
        case "AND": return and_
        case "XOR": return xor
        case "OR": return or_
        case _: raise Exception("Unknown operation")

def map_op(input: Operator) -> str:
    match input:
        case operator.and_: return "and"
        case operator.xor: return "xor"
        case operator.or_: return "or"
        case _: raise Exception("Unknown operation")

def draw_graph(graph: DiGraph):
    operations: Dict[str, Operator] = get_node_attributes(graph, 'op')
    color_map = ['red' if node.startswith('z') else 'green' for node in graph]
    op_map = [map_op(operations[x]) if x in operations else '' for x in graph]
    pos = nx.nx_pydot.graphviz_layout(graph, prog="C:\\Program Files\\Graphviz\\bin\\dot.exe")
    draw_networkx(graph, pos, node_color=color_map)
    for i, node in enumerate(pos):
        plt.text(pos[node][0]+10,pos[node][1],s=op_map[i], bbox=dict(facecolor='yellow', alpha=0.6),horizontalalignment='center')
    plt.show()

    