import numpy as np


if __name__ == "__main__":

    count = 0

    with open("input.txt") as input:
        for report in input:
            levels = np.fromstring(report, dtype=int, sep=" ")
            diff = np.diff(levels)
            count += np.all((1 <= np.abs(diff)) & (np.abs(diff) <= 3)) and (
                np.all(diff > 0) or np.all(diff < 0)
            )
    print(count)
