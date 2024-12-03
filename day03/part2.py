import re

if __name__ == "__main__":

    with open("input.txt") as input:
        input = input.read()

    captures = re.findall(r"mul\((\d{1,3}),(\d{1,3})\)|(don't\(\))|(do\(\))", input)

    should_count = True
    sum = 0
    for capture in captures:
        if "don't()" in capture:
            should_count = False
        elif "do()" in capture:
            should_count = True
        elif should_count:
            sum += int(capture[0]) * int(capture[1])

    print(sum)
