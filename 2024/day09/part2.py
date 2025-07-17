from part1 import parse_input, get_explicit_diskmap

if __name__ == "__main__":
    disk_map = parse_input("input.txt")

    file_lengths = disk_map[::2]
    free_space_lengths = disk_map[1::2]
    explicit_map = get_explicit_diskmap(disk_map)

    max_id = len(file_lengths) - 1

    for id in range(max_id, 0, -1):
        s = explicit_map.index(id)
        for i in range(s):
            if explicit_map[i : i + file_lengths[id]] == ["."] * file_lengths[id]:
                # swap
                (
                    explicit_map[i : i + file_lengths[id]],
                    explicit_map[s : s + file_lengths[id]],
                ) = (
                    explicit_map[s : s + file_lengths[id]],
                    explicit_map[i : i + file_lengths[id]],
                )
                break
    checksum = sum([i * id if id != "." else 0 for i, id in enumerate(explicit_map)])
    print(checksum)
