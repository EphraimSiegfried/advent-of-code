import re
from queue import SimpleQueue

DIRS = [(0, 1), (1, 0), (-1, 0), (0, -1)]


def parse_input(filename):
    with open(filename) as file:
        content = file.read()
    coordinates = re.findall(r"(\d+),(\d+)", content)
    return [(int(x), int(y)) for x, y in coordinates]


def add(p1, p2):
    return p1[0] + p2[0], p1[1] + p2[1]


def in_bounds(pos, bound):
    return all(0 <= c < bound for c in pos)


def get_adjacent(pos, bound, wall):
    for dir in DIRS:
        adjacent = add(pos, dir)
        if in_bounds(adjacent, bound) and adjacent not in wall:
            yield adjacent


def bfs(start, end, bound, wall):
    Q = SimpleQueue()
    visited = set()
    distances = {}
    visited.add(start)
    Q.put(start)
    distances[start] = 0
    i = 0
    while not Q.empty():
        v = Q.get()
        i += 1
        if v == end:
            return distances[v]
        for adj in get_adjacent(v, bound, wall):
            if adj not in visited:
                visited.add(adj)
                distances[adj] = distances[v] + 1
                Q.put(adj)
    return -1


if __name__ == "__main__":
    wall = parse_input("input.txt")

    num_bytes = 1024
    bound = 71
    wall = wall[:num_bytes]
    start = (0, 0)
    end = (bound - 1, bound - 1)

    print(bfs(start, end, bound, wall))
