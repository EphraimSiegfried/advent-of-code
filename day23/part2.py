from part1 import parse_input, construct_graph


def find_all_maximal_cliques(graph):

    maximal_cliques = set()

    # https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
    def bron_kerbosch(P: set, R=set(), X=set()):
        if not P and not X:
            maximal_cliques.add(frozenset(R))
        for v in list(P):
            bron_kerbosch(P & graph[v], R | {v}, X & graph[v])
            P -= {v}
            X |= {v}

    bron_kerbosch(set(graph.keys()))
    return maximal_cliques


if __name__ == "__main__":

    edges = parse_input("input.txt")
    graph = construct_graph(edges)

    maximal_cliques = find_all_maximal_cliques(graph)
    maximum_clique = max(maximal_cliques, key=lambda x: len(x))
    print(",".join(sorted(maximum_clique)))
