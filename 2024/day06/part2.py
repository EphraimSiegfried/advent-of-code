from part1 import find_start, parse_input, move, pretty_print
import copy


def is_loopy(map, x, y):
    map = copy.deepcopy(map)
    map[x][y] = "#"
    num_cols = len(map[0])
    num_rows = len(map)
    x, y = find_start(map)
    direction = 0
    in_bounds = lambda x, y: 0 <= x < num_cols and 0 <= y < num_rows
    visited = set()

    # simulate
    while True:
        x_prev, y_prev = x, y
        x, y = move(direction, x, y)
        if not in_bounds(x, y):
            break
        if map[y][x] == "#":
            node = (x,y,direction)
            if node in visited:
                return True
            visited.add((x,y,direction))
            x, y = x_prev, y_prev
            direction = (direction + 1) % 4
    return False


if __name__ == "__main__":

    map = parse_input("input.txt")
    count = 0

    # brute force hehe :)
    # https://preview.redd.it/swmwroeby8171.png?auto=webp&s=7c18bca81baefa11330a821ce78305445ac8556d
    i = 0
    for y in range(len(map)):
        for x in range(len(map[0])):
            if map[x][y] == ".":
                if is_loopy(map,x,y):
                    count += 1
    print(count)
