from itertools import combinations
from networkx import Graph, find_cliques

def main(debug = False):
    print("Advent of Code 2024 - day 23")
    try:
        input = open("./inputs/day_23.txt").read()
        print(" Part one: %d" % part_one(input, debug))
        print(" Part two: %s" % part_two(input, debug))
    except (ValueError, FileNotFoundError):
        print(" Input malformed / not found, expected to have `./inputs/day_23.txt`")

def part_one(input: str, debug = False) -> int:
    graph = parse_input(input)
    t_nodes = list(filter(lambda node: node.startswith('t'), graph.nodes()))
    triangles = set()
    for node in t_nodes:
        for nbr1, nbr2 in combinations(graph.neighbors(node), 2):
            if graph.has_edge(nbr1, nbr2):
                triangle = sorted([node, nbr1, nbr2])
                triangles.add(tuple(triangle))

    if debug: print(triangles)
    return len(triangles)

def part_two(input: str, debug = False) -> str:
    graph = parse_input(input)
    cliques = find_cliques(graph)
    biggest = max(cliques, key=len)
    return ",".join(sorted(biggest))

def parse_input(input: str) -> Graph:
    graph: Graph[str] = Graph()
    for edge in input.strip().splitlines():
        split = edge.split('-')
        graph.add_edge(split[0], split[1])

    return graph