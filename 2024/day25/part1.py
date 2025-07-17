import numpy as np


def parse_input(filename):
    with open(filename) as file:
        content = file.read()
    schematics = content.split("\n\n")
    keys = []
    locks = []
    for schema in schematics:
        lines = schema.strip().split("\n")
        array = np.array([[1 if char == "#" else 0 for char in line] for line in lines])
        if array[0, 0]:
            locks.append(array)
        else:
            keys.append(array)

    return np.stack(locks), np.stack(keys)


if __name__ == "__main__":
    locks, keys = parse_input("input.txt")
    num_rows = keys.shape[1]
    lock_heights, key_heights = locks.sum(axis=1), keys.sum(axis=1)
    num_pairs = sum(
        np.all(kh + lh <= num_rows) for kh in key_heights for lh in lock_heights
    )
    print(num_pairs)
