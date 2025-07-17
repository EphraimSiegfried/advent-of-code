type order = list[tuple[int, int]]
type path = list[int]
type graph = dict[int, set[int]]


def parse_input(filename) -> tuple[order, path]:

    page_ordering: order = []
    page_to_produce: list[path] = []
    with open(filename) as file:
        for line in file:
            if "|" in line:
                [n1, n2] = line.strip().split("|")
                page_ordering.append((int(n1), int(n2)))
            elif "," in line:
                page_to_produce.append(list(map(int, line.split(","))))
    return page_ordering, page_to_produce


class DirectedGraph:

    def __init__(self, arcs: order):
        self._graph: graph = {}
        self.add_arcs(arcs)

    def add_arcs(self, arcs: order) -> None:
        for node1, node2 in arcs:
            if node1 in self._graph:
                self._graph[node1].add(node2)
            else:
                self._graph[node1] = {node2}
            if not node2 in self._graph:
                self._graph[node2] = set()

    def is_walkable(self, p: path) -> bool:
        successors = {p[0]}
        for node in p:
            if not node in successors:
                return False
            successors = self._graph[node]
        return True


if __name__ == "__main__":

    page_order, paths = parse_input("input.txt")

    dgraph = DirectedGraph(page_order)

    sum = 0
    for p in paths:
        if dgraph.is_walkable(p):
            sum += p[len(p) // 2]

    print(sum)
