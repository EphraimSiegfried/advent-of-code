import numpy as np
from part1 import is_safe


if __name__ == "__main__":

    count = 0

    with open("input.txt") as input:
        for report in input:
            levels = np.fromstring(report, dtype=int, sep=" ")
            diff = np.diff(levels)
            if is_safe(levels):
                count += 1
            else:
                mask = np.ones(levels.shape, dtype=bool)
                for i in range(levels.size):
                    mask[i] = 0
                    if is_safe(levels[mask]):
                        count += 1
                        break
                    mask[i] = 1
    print(count)
