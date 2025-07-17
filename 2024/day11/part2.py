from part1 import parse_input, evolve


if __name__ == "__main__":
    stones = parse_input("input.txt")
    num_stones = sum(evolve(stone, 75) for stone in stones)
    print(num_stones)
