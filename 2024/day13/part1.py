import numpy as np


def parse_input(filename):
    with open(filename) as file:
        lines = [line.strip() for line in file]

    lines.append("")
    num_matrices = (len(lines) + 1) // 4
    A = np.zeros((2, 2))
    y = np.zeros(2)
    buttons = np.zeros((2, 2, num_matrices))
    prizes = np.zeros((2, num_matrices))
    i = 0

    for line in lines:
        if ":" in line:
            if "+" in line:
                [left, right] = line.split(": X+")
                [X, Y] = right.split(", Y+")
            elif "=" in line:
                [left, right] = line.split(": X=")
                [X, Y] = right.split(", Y=")
            if left[-1] == "A":
                A[0, 1] = int(X)
                A[1, 1] = int(Y)
            elif left[-1] == "B":
                A[0, 0] = int(X)
                A[1, 0] = int(Y)
            else:
                y = np.array([int(X), int(Y)])
        else:
            buttons[:, :, i] = A
            prizes[:, i] = y
            i += 1
    return buttons, prizes


def is_integer(array):
    return all(
        np.isclose(np.mod(n, 1), 0) or np.isclose(np.mod(n, 1), 1) for n in array
    )


if __name__ == "__main__":

    buttons, prizes = parse_input("input.txt")

    sum = 0
    for i in range(prizes.shape[1]):
        solution = np.linalg.solve(buttons[:, :, i], prizes[:, i])
        if is_integer(solution):
            sum += solution[0] + solution[1] * 3

    print(int(sum))
