import re
from math import prod

num_cols = 101
num_rows = 103


def parse_input(filename):
    with open(filename) as file:
        content = file.read()
    captured = re.findall(r"(-*\d+),(-*\d+)", content)
    captured = [(int(e1), int(e2)) for (e1, e2) in captured]
    positions = captured[::2]
    velocities = captured[1::2]
    return positions, velocities


def move(pos, vel):
    x = (pos[0] + vel[0]) % num_cols
    y = (pos[1] + vel[1]) % num_rows
    return x, y


def simulate(positions, velocities, seconds):
    for i, (pos, vel) in enumerate(zip(positions, velocities)):
        for _ in range(seconds):
            pos = move(pos, vel)
        positions[i] = pos
    return positions


def count_robots_in_quadrants(positions):
    q1, q2, q3, q4 = 0, 0, 0, 0
    ver = num_cols // 2
    hor = num_rows // 2
    for x, y in positions:
        q1 += x < ver and y < hor
        q2 += x > ver and y < hor
        q3 += x < ver and y > hor
        q4 += x > ver and y > hor
    return q1, q2, q3, q4


def print_map(positions):
    map = [list("." * num_cols) for _ in range(num_rows)]
    for x, y in positions:
        if map[y][x] == ".":
            map[y][x] = "1"
        else:
            map[y][x] = str(int(map[y][x]) + 1)
    for line in map:
        print("".join(line))


if __name__ == "__main__":
    # positions, velocities = parse_input("input_simple.txt")
    # num_cols = 11
    # num_rows = 7
    positions, velocities = parse_input("input.txt")
    num_cols = 101
    num_rows = 103
    seconds = 100

    positions = simulate(positions, velocities, seconds)

    print(prod(count_robots_in_quadrants(positions)))
