from part1 import parse_input, DirectedGraph
from functools import cmp_to_key


class DirectedGraph(DirectedGraph):
    def find_path(self, p):
        def compare(x, y):
            return -1 if y in self._graph[x] else 1

        return sorted(p, key=cmp_to_key(compare))


if __name__ == "__main__":

    page_order, paths = parse_input("input.txt")

    dgraph = DirectedGraph(page_order)

    sum = 0
    for p in paths:
        if not dgraph.is_walkable(p):
            p = dgraph.find_path(p)
            sum += p[len(p) // 2]

    print(sum)
