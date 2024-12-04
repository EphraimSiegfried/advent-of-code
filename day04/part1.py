def parse_input(input_file):

    with open(input_file) as file:
        return [line.strip() for line in file]


def get_diagonals(mat):
    num_cols = len(mat[0])
    num_rows = len(mat)
    fdiag = [""] * (num_cols + num_rows - 1)
    bdiag = [""] * len(fdiag)
    min_bdiag = -num_rows + 1
    for x in range(num_cols):
        for y in range(num_rows):
            fdiag[x + y] += mat[y][x]
            bdiag[x - y - min_bdiag] += mat[y][x]

    return fdiag, bdiag


def reverse(mat):
    return [line[::-1] for line in mat]


def get_vertical(mat):
    num_cols = len(mat[0])
    num_rows = len(mat)
    l = [""] * num_cols
    for x in range(num_cols):
        for y in range(num_rows):
            l[x] += mat[y][x]
    return l


if __name__ == "__main__":

    input = parse_input("input.txt")

    fdiag, bdiag = get_diagonals(input)

    l = input + fdiag + bdiag + get_vertical(input)
    l += reverse(l)

    print(sum(e.count("XMAS") for e in l))
