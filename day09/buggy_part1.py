def parse_input(filename):
    with open(filename) as file:
        line = file.readline().strip()
    return [int(digit) for digit in line]


if __name__ == "__main__":

    # disk_map = parse_input("input.txt")
    disk_map = [1, 1, 2, 0, 1]

    file_lengths = disk_map[::2]
    free_space_lengths = disk_map[1::2]
    total_free_space = sum(free_space_lengths)

    blocks_to_move = []

    id = len(file_lengths) - 1
    while total_free_space - 1 > 0:
        num_blocks = file_lengths[id]
        if total_free_space < num_blocks:
            num_blocks = total_free_space
        blocks_to_move += [id] * num_blocks
        file_lengths[id] -= num_blocks
        total_free_space -= num_blocks
        id -= 1

    filesystem = []
    for id, num_blocks in enumerate(file_lengths):
        filesystem += [id] * num_blocks
        if id < len(free_space_lengths):
            filesystem += blocks_to_move[: free_space_lengths[id]]
            blocks_to_move = blocks_to_move[free_space_lengths[id] :]

    checksum = sum([i * id for i, id in enumerate(filesystem)])
    print(filesystem)
    print(checksum)
