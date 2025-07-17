from itertools import product


def parse_input(filename):
    equations = []
    with open(filename) as file:
        for line in file:
            [test_value, args] = line.strip().split(": ")
            args = args.split()
            equations.append((int(test_value), [int(arg) for arg in args]))
    return equations


if __name__ == "__main__":

    equations = parse_input("input.txt")

    sum = 0

    for test_value, args in equations:
        perms = product(["+", "*"], repeat=len(args) - 1)
        for perm in perms:
            result = args[0]
            for i in range(1, len(args)):
                result = result * args[i] if perm[i - 1] == "*" else result + args[i]
            if test_value == result:
                sum += test_value
                break
    print(sum)
