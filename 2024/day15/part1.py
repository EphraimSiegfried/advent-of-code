import numpy as np


def parse_input(filename):
    with open(filename) as file:
        content = [line.strip() for line in file]
    directions = ""
    wh_map = np.full((content.index(""), len(content[0])), ".", dtype=str)
    content.index("")
    for row, line in enumerate(content):
        if line == "":
            directions = "".join(content[row + 1 :])
            break
        for col, char in enumerate(line):
            wh_map[row, col] = char
    return wh_map, directions


def add(t1, t2):
    return t1[0] + t2[0], t1[1] + t2[1]


def sub(t1, t2):
    return t1[0] - t2[0], t1[1] - t2[1]


def move(pos, dir, wh_map):
    next_pos = add(pos, dir)

    if wh_map[next_pos] == "#":
        return pos, wh_map
    if wh_map[next_pos] == ".":
        wh_map[pos] = "."
        wh_map[next_pos] = "@"
        return next_pos, wh_map

    while wh_map[next_pos := add(next_pos, dir)] == "O":
        pass
    if wh_map[next_pos] == "#":
        return pos, wh_map

    while True:
        prev_pos = sub(next_pos, dir)
        wh_map[next_pos], wh_map[prev_pos] = wh_map[prev_pos], wh_map[next_pos]
        if wh_map[next_pos] == "@":
            break
        next_pos = prev_pos
    return next_pos, wh_map


def simulate(wh_map, directions):
    dir_dict = {"^": (-1, 0), "v": (1, 0), ">": (0, 1), "<": (0, -1)}
    pos = tuple(np.argwhere(wh_map == "@")[0])
    for dir in directions:
        pos, wh_map = move(pos, dir_dict[dir], wh_map)
    return wh_map


def print_map(wh_map):
    for row in wh_map:
        print("".join(row))


if __name__ == "__main__":

    wh_map, directions = parse_input("input.txt")

    wh_map = simulate(wh_map, directions)

    box_pos = np.argwhere(wh_map == "O")
    print(np.sum(box_pos @ np.array([100, 1])))
