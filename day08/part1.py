type point = tuple[int, int]
from itertools import combinations, starmap


def parse_input(filename):
    frequencies = {}
    with open(filename) as file:
        lines = [line.strip() for line in file]
    num_rows, num_cols = len(lines), len(lines[0])
    for y, line in enumerate(lines):
        for x, char in enumerate(line):
            if char != ".":
                frequencies.setdefault(char, set()).add((x, y))
    return frequencies, num_rows, num_cols


def compute_antinodes(p1: point, p2: point):
    (x1, y1), (x2, y2) = p1, p2
    dx, dy = x2 - x1, y2 - y1
    return (x1 - dx, y1 - dy), (x2 + dx, y2 + dy)


def flatten(nested_list):
    return [inner for outer in nested_list for inner in outer]


if __name__ == "__main__":
    frequencies, num_rows, num_cols = parse_input("input.txt")

    in_bounds = lambda x, y: 0 <= x < num_cols and 0 <= y < num_rows

    all_antinodes = set()
    for freq in frequencies:
        pairs = combinations(frequencies[freq], 2)
        freq_antinodes = flatten(starmap(compute_antinodes, pairs))
        all_antinodes.update(set(freq_antinodes))

    all_antinodes = {(x, y) for (x, y) in all_antinodes if in_bounds(x, y)}

    print(len(all_antinodes))
