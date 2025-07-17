import re

if __name__ == "__main__":

    with open("input.txt") as input:
        input = input.read()

    mul_args = re.findall(r"mul\((\d{1,3}),(\d{1,3})\)", input)

    print(sum(int(a1) * int(a2) for a1, a2 in mul_args))
