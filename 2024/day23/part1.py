from collections import defaultdict
from itertools import combinations


def parse_input(filename):
    with open(filename) as file:
        edges = [tuple(l.strip().split("-")) for l in file]
    return edges


def construct_graph(edges):
    graph = defaultdict(set)
    for edge in edges:
        (a, b) = edge
        graph[a].add(b)
        graph[b].add(a)
    return graph


if __name__ == "__main__":
    edges = parse_input("input.txt")
    graph = construct_graph(edges)

    cliques = set()
    for vertex, neighbors in graph.items():
        if len(neighbors) >= 2:
            cliques.update(
                frozenset((a, b, vertex))
                for a, b in combinations(neighbors, 2)
                if a in graph[b] and any(v.startswith("t") for v in (a, b, vertex))
            )
    print(len(cliques))
