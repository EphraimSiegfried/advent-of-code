import re


def parse_input(filename):
    with open(filename) as file:
        content = file.read().strip()
    [p1, p2] = content.split("\n\n")
    reg_content = re.findall(r"(\d+)", p1)
    reg_content = [int(c) for c in reg_content]
    regs = dict(zip(["A", "B", "C"], reg_content))
    program = re.findall(r"(\d+),(\d+)", p2)
    program = [(int(s1), int(s2)) for s1, s2 in program]
    return regs, program


def interpret(program, regs):
    output = []

    def combo(num):
        if 0 <= num <= 3:
            return num
        elif 4 <= num <= 6:
            return regs[chr((61+num))]
        else:
            raise ValueError(f"Unexpected combo num: {num}")

    instruction_pointer = 0
    while instruction_pointer < len(program) * 2:
        instruction, operand = program[instruction_pointer // 2]
        match instruction:
            case 0:
                regs["A"] = regs["A"] // (2 ** combo(operand))
            case 1:
                regs["B"] = regs["B"] ^ operand
            case 2:
                regs["B"] = combo(operand) % 8
            case 3:
                if regs["A"] != 0:
                    assert operand % 2 == 0
                    instruction_pointer = operand - 2
            case 4:
                regs["B"] = regs["B"] ^ regs["C"]
            case 5:
                output.append(combo(operand) % 8)
            case 6:
                regs["B"] = regs["A"] // (2 ** combo(operand))
            case 7:
                regs["C"] = regs["A"] // (2 ** combo(operand))
            case _:
                raise ValueError(f"Unexpected instruction: {instruction}")
        instruction_pointer += 2
    return output


if __name__ == "__main__":

    regs, program = parse_input("input.txt")
    output = interpret(program, regs)
    print(",".join(map(str, output)))
