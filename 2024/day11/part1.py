from math import floor, log10
from functools import cache


def parse_input(filename):
    with open(filename) as file:
        first_line = file.readline()
    return [int(num) for num in first_line.split()]


def count_digits(num):
    return floor(log10(num) + 1)


def split_digits(num):
    n = count_digits(num) / 2
    left = num // 10**n
    right = num % 10**n
    return left, right


@cache
def evolve(stone, num_blinks):
    if num_blinks == 0:
        return 1
    if stone == 0:
        return evolve(1, num_blinks - 1)
    if count_digits(stone) % 2 == 0:
        left, right = split_digits(stone)
        return evolve(left, num_blinks - 1) + evolve(right, num_blinks - 1)
    return evolve(stone * 2024, num_blinks - 1)


if __name__ == "__main__":

    stones = parse_input("input.txt")
    num_stones = sum(evolve(stone, 25) for stone in stones)
    print(num_stones)
