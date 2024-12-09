def parse_input(filename):
    with open(filename) as file:
        line = file.readline().strip()
    return [int(digit) for digit in line]


def get_explicit_diskmap(disk_map):
    compact = []
    for i in range(len(disk_map)):
        id_value = i // 2 if i % 2 == 0 else "."
        compact.extend([id_value] * disk_map[i])
    return compact


if __name__ == "__main__":

    disk_map = parse_input("input.txt")

    explicit_map = get_explicit_diskmap(disk_map)
    i = 0
    while i < len(explicit_map):
        if explicit_map[i] == ".":
            id = "."
            while id == ".":
                id = explicit_map.pop()
            explicit_map[i] = id
        i += 1

    checksum = sum([i * id for i, id in enumerate(explicit_map)])
    print(checksum)
