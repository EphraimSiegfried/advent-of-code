import numpy as np

UP = np.array([-1, 0])
DOWN = np.array([1, 0])
LEFT = np.array([0, -1])
RIGHT = np.array([0, 1])


def parse_input(filename):
    return np.genfromtxt(filename, delimiter=1, dtype=str)


def explore(pos, garden_map):
    num_rows, num_cols = np.shape(garden_map)

    def in_bounds(pos):
        y, x = tuple(pos)
        return 0 <= y < num_rows and 0 <= x < num_cols

    def walk(pos, block, sides=4):
        current_plant = garden_map[tuple(pos)]
        block.add(tuple(pos))
        area = 0
        for dir in [UP, DOWN, LEFT, RIGHT]:
            next_pos = dir + pos
            if in_bounds(next_pos):
                next_plant = garden_map[tuple(next_pos)]
                if tuple(next_pos) in block:
                    sides -= 1
                elif current_plant == next_plant:
                    sides -= 1
                    num_plots, perimeter, _ = walk(next_pos, block)
                    sides += perimeter
                    area += num_plots
        return 1 + area, sides, block

    return walk(pos, set())


if __name__ == "__main__":
    garden_map = parse_input("input.txt")

    not_visited_pos = set(
        (i, j) for i in range(garden_map.shape[0]) for j in range(garden_map.shape[1])
    )

    sum = 0
    while len(not_visited_pos) > 0:
        area, perimeter, block_pos = explore(not_visited_pos.pop(), garden_map)
        not_visited_pos -= block_pos
        sum += area * perimeter
    print(sum)
