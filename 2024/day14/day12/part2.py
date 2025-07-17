import numpy as np


def pos_to_map(positions):
    max_row = max(pos[0] for pos in positions) + 1
    max_col = max(pos[1] for pos in positions) + 1
    matrix = np.zeros((max_row, max_col), dtype=bool)
    for row, col in positions:
        matrix[row, col] = 1
    return matrix
