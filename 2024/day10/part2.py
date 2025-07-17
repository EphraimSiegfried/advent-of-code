import numpy as np


def parse_input(filename):
    return np.genfromtxt(filename, delimiter=1, dtype=int)


if __name__ == "__main__":

    top_map = parse_input("input_simple.txt")

    dx = np.diff(top_map)
    dy = np.diff(top_map, axis=0)
    trailheads = np.argwhere(top_map == 0)
    num_rows, num_cols = np.shape(top_map)

    def walk(position, visited=[]):
        (y, x) = position
        visited.append(position)

        if top_map[y, x] == 9:
            return 1

        sum_paths = 0
        # right
        if x < num_cols - 1 and dx[y, x] == 1 and (y, x + 1) not in visited:
            sum_paths += walk((y, x + 1), visited[:])

        # left
        if x > 0 and dx[y, x - 1] == -1 and (y, x - 1) not in visited:
            sum_paths += walk((y, x - 1), visited[:])

        # down
        if y < num_rows - 1 and dy[y, x] == 1 and (y + 1, x) not in visited:
            sum_paths += walk((y + 1, x), visited[:])

        # up
        if y > 0 and dy[y - 1, x] == -1 and (y - 1, x) not in visited:
            sum_paths += walk((y - 1, x), visited[:])

        return sum_paths

    # print(, []))
    # print(walk(tuple(trailheads[4])))
    print(sum(walk(tuple(trailhead)) for trailhead in trailheads))
