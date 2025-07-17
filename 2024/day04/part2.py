from part1 import parse_input


if __name__ == "__main__":

    input = parse_input("input.txt")
    num_cols = len(input[0])
    num_rows = len(input)

    count = 0
    for i in range(1, num_cols - 1):
        for j in range(1, num_rows - 1):
            diag1 = input[j - 1][i - 1] + input[j][i] + input[j + 1][i + 1]
            diag2 = input[j + 1][i - 1] + input[j][i] + input[j - 1][i + 1]
            diags = [diag1, diag2]
            count += all(d == "MAS" or d == "SAM" for d in diags)

    print(count)
