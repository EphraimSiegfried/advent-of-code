from itertools import combinations, starmap
from part1 import parse_input, point, flatten


def compute_antinodes(p1: point, p2: point, num_cols: int, num_rows: int):
    def in_bounds(x, y):
        return 0 <= x < num_cols and 0 <= y < num_rows

    (x1, y1), (x2, y2) = p1, p2
    dx, dy = x2 - x1, y2 - y1
    antinodes = set()

    for direction in [-1, 1]:
        scalar = 0
        while True:
            x, y = x1 + direction * scalar * dx, y1 + direction * scalar * dy
            if in_bounds(x, y):
                antinodes.add((x, y))
            else:
                break
            scalar += 1

    return antinodes


if __name__ == "__main__":
    frequencies, num_rows, num_cols = parse_input("input.txt")

    all_antinodes = set()
    for freq, points in frequencies.items():
        pairs = combinations(points, 2)
        freq_antinodes = flatten(
            starmap(lambda p1, p2: compute_antinodes(p1, p2, num_cols, num_rows), pairs)
        )
        all_antinodes.update(freq_antinodes)

    print(len(all_antinodes))
