def parse_input(input_file):
    list1 = []
    list2 = []

    with open(input_file) as input:
        for line in input:
            [n1, n2] = line.split("   ")
            list1.append(int(n1))
            list2.append(int(n2))

    return list1, list2


if __name__ == "__main__":

    list1, list2 = parse_input("input.txt")

    print(sum([abs(n1 - n2) for n1, n2 in zip(sorted(list1), sorted(list2))]))
