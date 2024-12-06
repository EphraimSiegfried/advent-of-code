from part1 import find_start, parse_input, move, pretty_print
import copy


def is_loopy(map):
    map = copy.deepcopy(map)
    num_cols = len(map[0])
    num_rows = len(map)
    x, y = find_start(map)
    direction = 0
    in_bounds = lambda x, y: 0 <= x < num_cols and 0 <= y < num_rows

    # simulate
    while True:
        # pretty_print(map)
        map[y][x] = str(direction)
        x_prev, y_prev = x, y
        x, y = move(direction, x, y)
        if not in_bounds(x, y):
            break
        if map[y][x] == "#":
            x, y = x_prev, y_prev
            direction = (direction + 1) % 4
        elif map[y][x] == str(direction):
            return True
        map[y][x] = "^"
    return False


if __name__ == "__main__":

    map = parse_input("input.txt")
    count = 0
    # brute force hehe :)
    for y in range(len(map)):
        for x in range(len(map[0])):
            if map[x][y] == ".":
                new_map = copy.deepcopy(map)
                new_map[x][y] = "#"
                if is_loopy(new_map):
                    count += 1
    print(count)
