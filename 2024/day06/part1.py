def parse_input(filename):
    with open(filename) as file:
        return [list(line.strip()) for line in file]


def find_start(map):
    for y, row in enumerate(map):
        if "^" in row:
            return row.index("^"), y


def move(direction, x, y):
    match direction:
        case 0:
            # move up
            return x, y - 1
        case 1:
            # move right
            return x + 1, y
        case 2:
            # move down
            return x, y + 1
        case 3:
            # move left
            return x - 1, y


def pretty_print(map):
    print()
    for row in map:
        print("".join(row))


if __name__ == "__main__":

    map = parse_input("input_simple.txt")

    num_cols = len(map[0])
    num_rows = len(map)
    x, y = find_start(map)
    direction = 0
    num_distinct_pos = 1
    in_bounds = lambda x, y: 0 <= x < num_cols and 0 <= y < num_rows

    # simulate
    while True:
        pretty_print(map)
        map[y][x] = "X"
        x_prev, y_prev = x, y
        x, y = move(direction, x, y)
        if not in_bounds(x, y):
            break
        match map[y][x]:
            case "#":
                x, y = x_prev, y_prev
                direction = (direction + 1) % 4
            case ".":
                num_distinct_pos += 1
        map[y][x] = "^"

    print(num_distinct_pos)
