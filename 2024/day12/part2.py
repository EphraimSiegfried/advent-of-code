import numpy as np
from part1 import explore, parse_input


def pos_to_map(positions):
    max_row = max(pos[0] for pos in positions) + 1
    max_col = max(pos[1] for pos in positions) + 1
    matrix = np.zeros((max_row, max_col), dtype=bool)
    for row, col in positions:
        matrix[row, col] = 1
    return matrix


def count_edges(map: np.ndarray):
    # zenoli idea

    num_edges = 0

    num_rows, num_cols = map.shape
    padded = np.pad(map, 1)
    # sliding window
    for (row, col), _ in np.ndenumerate(padded):
        if row > num_rows or col > num_cols:
            continue
        frame = padded[row : row + 2, col : col + 2]
        if np.all(np.eye(2) == frame) or np.all(np.flip(np.eye(2), 1) == frame):
            num_edges += 2

        num_edges += np.sum(frame) == 1 or np.sum(frame) == 3
    return num_edges


if __name__ == "__main__":

    garden_map = parse_input("input.txt")
    a = np.array([[1, 1, 0], [0, 1, 0], [0, 1, 1]])

    not_visited_pos = set(
        (i, j) for i in range(garden_map.shape[0]) for j in range(garden_map.shape[1])
    )

    sum = 0
    while len(not_visited_pos) > 0:
        area, perimeter, block_pos = explore(not_visited_pos.pop(), garden_map)
        num_sides = count_edges(pos_to_map(block_pos))
        not_visited_pos -= block_pos
        sum += area * num_sides
    print(sum)
