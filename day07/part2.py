from part1 import parse_input
from itertools import product

if __name__ == "__main__":

    equations = parse_input("input.txt")

    sum = 0

    for test_value, args in equations:
        perms = product(["+", "*", "||"], repeat=len(args) - 1)
        for perm in perms:
            result = args[0]
            for i in range(1, len(args)):
                match perm[i - 1]:
                    case "*":
                        result *= args[i]
                    case "+":
                        result += args[i]
                    case "||":
                        result = int(str(result) + str(args[i]))
            if test_value == result:
                sum += test_value
                break
    print(sum)
