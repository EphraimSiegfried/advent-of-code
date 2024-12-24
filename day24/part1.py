type program = list[list[str]]
type assignments = dict[str, int]


def parse_input(filename) -> tuple[assignments, program]:
    with open(filename) as file:
        content = file.read()
    [p1, p2] = content.split("\n\n")

    vars = [line.strip().split(": ") for line in p1.split("\n")]
    vars = {var: int(value) for [var, value] in vars}

    prog = []
    for line in p2.strip().split("\n"):
        [a1, op, a2, _, out] = line.split(" ")
        prog.append((op, a1, a2, out))
    return vars, prog


def topological_sort(vars: assignments, prog: program) -> program:
    unordered = prog.copy()
    visited = set(vars.keys())
    ordered = []
    while unordered:
        for stmt in unordered:
            [_, a1, a2, out] = stmt
            if a1 in visited and a2 in visited:
                visited.add(out)
                ordered.append(stmt)
                unordered.remove(stmt)
    return ordered


if __name__ == "__main__":

    vars, prog = parse_input("input.txt")

    prog = topological_sort(vars, prog)

    for [op, var_a1, var_a2, var_out] in prog:
        a1 = vars[var_a1]
        a2 = vars[var_a2]
        match op:
            case "AND":
                vars[var_out] = a1 & a2
            case "OR":
                vars[var_out] = a1 | a2
            case "XOR":
                vars[var_out] = a1 ^ a2

    out_vars = sorted([var for var in vars if var.startswith("z")], reverse=True)
    bitstring = "".join([str(vars[out_var]) for out_var in out_vars])
    print(int(bitstring, 2))
