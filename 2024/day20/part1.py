import math

DIRS = [(0, 1), (0, -1), (1, 0), (-1, 0)]


def parse_input(filename):
    with open(filename) as file:
        content = file.read()
    wall = set()
    start = (0, 0)
    end = (0, 0)
    rows = content.split("\n")
    for i, row in enumerate(rows):
        for j, char in enumerate(row):
            coord = (j, i)
            match char:
                case "#":
                    wall.add(coord)
                case "S":
                    start = coord
                case "E":
                    end = coord
    return wall, start, end, len(rows) - 1, len(rows[0])


def add(p1, p2):
    return p1[0] + p2[0], p1[1] + p2[1]


def get_adjacent(pos, wall):
    for dir in DIRS:
        adjacent = add(pos, dir)
        if adjacent not in wall:
            yield adjacent


def dijkstra(start, end, wall):
    dist = {}
    dist[start] = 0
    q = set()
    q.add(start)
    visited = set()
    while q:
        min_node = min(q, key=lambda node: dist[node])
        q.remove(min_node)
        visited.add(min_node)
        if min_node == end:
            break
        for neighbour in get_adjacent(min_node, wall):
            if neighbour not in visited:
                if neighbour not in q:
                    q.add(neighbour)
                    dist[neighbour] = math.inf
                new_dist = dist[min_node] + 1
                if new_dist < dist[neighbour]:
                    dist[neighbour] = new_dist
    return dist


if __name__ == "__main__":

    wall, start, end, num_cols, num_rows = parse_input("input.txt")
    dist = dijkstra(start, end, wall)
    sum = 0
    in_bounds = lambda x: 0 < x[0] < num_cols and 0 < x[1] < num_rows
    for coord, distance in dist.items():
        for dir in DIRS:
            adj_wall = add(coord, dir)
            shortcut = add(adj_wall, dir)
            if (
                in_bounds(adj_wall)
                and in_bounds(shortcut)
                and adj_wall in wall
                and shortcut not in wall
                and dist[shortcut] - dist[coord] - 2 >= 100
            ):
                sum += 1
    print(sum)
