from part1 import dijkstra, parse_input
import math

DIRS = [(0, 1), (0, -1), (1, 0), (-1, 0)]


def add(p1, p2):
    return p1[0] + p2[0], p1[1] + p2[1]


if __name__ == "__main__":

    wall, start, end, num_cols, num_rows = parse_input("input_simple.txt")
    dist = dijkstra(start, end, wall)
    num_cheats = 20
    min_ps = 50
    sum = 0
    in_bounds = lambda p: 0 < p[0] < num_cols and 0 < p[1] < num_rows
    is_reachable = lambda p1, p2: abs(p2[0] - p1[0]) + abs(p2[1] - p1[1]) <= num_cheats
    sorted_dist = sorted(dist.items(), key=lambda x: x[1], reverse=True)

    for coord, coord_dist in dist.items():
        for i, (goal_coord, _) in enumerate(sorted_dist):
            num_steps = abs(goal_coord[0] - coord[0]) + abs(goal_coord[1] - coord[1])
            if (
                num_steps <= num_cheats
                and dist[goal_coord] - (coord_dist + num_steps) >= min_ps
            ):
                print(
                    coord,
                    dist[goal_coord] - (coord_dist + num_steps),
                    coord_dist + num_steps + i,
                )
                sum += 1
                break
    print(sum)
