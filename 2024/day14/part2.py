from numpy import argmax
from part1 import parse_input, simulate
from PIL import Image
from random import randint
from day12.part1 import explore
from day12.part2 import pos_to_map


def create_image(positions, filename):
    img = Image.new("RGB", (num_cols, num_rows), color="white")
    pixels = img.load()
    for x, y in positions:
        pixels[x, y] = (0, 0, 0)
    img.save(filename)


if __name__ == "__main__":

    positions, velocities = parse_input("input.txt")
    num_cols = 101
    num_rows = 103

    cluster_sizes = []
    for i in range(50000):
        positions = simulate(positions, velocities, 1)
        index = randint(0, len(positions) - 1)
        area, _, _ = explore(positions[index], pos_to_map(positions))
        cluster_sizes.append(area)
    i = argmax(cluster_sizes)

    positions, velocities = parse_input("input.txt")
    create_image(simulate(positions, velocities, i + 1), f"frame-{i+1}.png")
