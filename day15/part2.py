from part1 import parse_input, print_map, add, sub
from functools import cache
import numpy as np


def transform_map(wh_map: np.ndarray):
    wh_map = np.repeat(wh_map, 2, axis=1)
    wh_map = np.array([list("".join(row).replace("OO", "[]")) for row in wh_map])
    wh_map[tuple(np.argwhere(wh_map == "@")[1])] = "."
    return wh_map


def move(pos, dir, wh_map):
    next_pos = add(pos, dir)

    if wh_map[next_pos] == "#":
        return pos, wh_map
    if wh_map[next_pos] == ".":
        wh_map[pos] = "."
        wh_map[next_pos] = "@"
        return next_pos, wh_map

    if dir == (0, 1) or dir == (0, -1):
        return push_horizontal(pos, dir, wh_map)
    else:
        return push_vertical(pos, dir, wh_map)


def push_horizontal(pos, dir, wh_map):
    next_pos = pos
    while True:
        next_pos = add(next_pos, dir)
        entry = wh_map[next_pos]
        if not (entry == "[" or entry == "]"):
            break
    if wh_map[next_pos] == "#":
        return pos, wh_map

    while True:
        prev_pos = sub(next_pos, dir)
        wh_map[next_pos], wh_map[prev_pos] = wh_map[prev_pos], wh_map[next_pos]
        if wh_map[next_pos] == "@":
            break
        next_pos = prev_pos
    return next_pos, wh_map


def push_vertical(pos, dir, wh_map):

    @cache
    def is_movable(pos_lb, pos_rb, dir):
        pos_next_lb = add(pos_lb, dir)
        pos_next_rb = add(pos_rb, dir)
        next_lb = wh_map[pos_next_lb]
        next_rb = wh_map[pos_next_rb]

        if next_lb == "." and next_rb == ".":
            return True
        if next_lb == "#" or next_rb == "#":
            return False
        if next_lb == "[":
            return is_movable(pos_next_lb, pos_next_rb, dir)
        if next_lb == "]" and next_rb == ".":
            return is_movable(add(pos_next_lb, (0, -1)), pos_next_lb, dir)
        if next_rb == "[" and next_lb == ".":
            return is_movable(pos_next_rb, add(pos_next_rb, (0, 1)), dir)
        assert next_lb == "]" and next_rb == "["
        return is_movable(add(pos_next_lb, (0, -1)), pos_next_lb, dir) and is_movable(
            pos_next_rb, add(pos_next_rb, (0, 1)), dir
        )

    def switch(pos_b, pos_next_b):
        wh_map[pos_next_b], wh_map[pos_b] = wh_map[pos_b], wh_map[pos_next_b]

    def push(pos_lb, pos_rb, dir):
        pos_next_lb = add(pos_lb, dir)
        pos_next_rb = add(pos_rb, dir)
        next_lb = wh_map[pos_next_lb]
        next_rb = wh_map[pos_next_rb]
        if next_lb == "[":
            push(pos_next_lb, pos_next_rb, dir)
        if next_lb == "]":
            push(add(pos_next_lb, (0, -1)), pos_next_lb, dir)
        if next_rb == "[":
            push(pos_next_rb, add(pos_next_rb, (0, 1)), dir)
        switch(pos_lb, pos_next_lb)
        switch(pos_rb, pos_next_rb)

    next_pos = add(pos, dir)
    if wh_map[next_pos] == "]":
        pos_lb = add(next_pos, (0, -1))
        pos_rb = next_pos
    else:
        pos_lb = next_pos
        pos_rb = add(next_pos, (0, 1))

    if is_movable(pos_lb, pos_rb, dir):
        push(pos_lb, pos_rb, dir)
        switch(pos, next_pos)
        pos = next_pos
    return pos, wh_map


def simulate(wh_map, directions):
    dir_dict = {"^": (-1, 0), "v": (1, 0), ">": (0, 1), "<": (0, -1)}
    pos = tuple(np.argwhere(wh_map == "@")[0])
    for dir in directions:
        pos, wh_map = move(pos, dir_dict[dir], wh_map)
        # print("DIR", dir)
        # print_map(wh_map)
    return wh_map


if __name__ == "__main__":
    wh_map, directions = parse_input("input.txt")
    wh_map = transform_map(wh_map)
    # print_map(wh_map)
    simulate(wh_map, directions)
    box_pos = np.argwhere(wh_map == "[")
    print(np.sum(box_pos @ np.array([100, 1])))
