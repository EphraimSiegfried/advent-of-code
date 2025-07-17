from part1 import parse_input

if __name__ == "__main__":

    list1, list2 = parse_input("input.txt")
    print(sum([num * list2.count(num) for num in list1]))
