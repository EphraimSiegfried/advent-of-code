import numpy as np
from functools import cache
import sys

clockwise = np.array([[0, 1], [-1, 0]])


def parse_input(filename):
    with open(filename) as file:
        lines = file.readlines()
    return np.array([list(line.strip()) for line in lines])


def print_path(visited, maze):
    for entry in visited:
        maze[tuple(entry)] = "X"
    for row in maze:
        print("".join(row))


def dfs2(
    pos: np.ndarray, dir: np.ndarray, maze: np.ndarray, visited: list, min_score, score
):
    if maze[tuple(pos)] == "E":
        return min(score, min_score)
    next_pos = pos + dir
    visited.append(tuple(pos))
    if maze[tuple(next_pos)] != "#" and tuple(next_pos) not in visited:
        min_score = dfs2(next_pos, dir, maze, visited, min_score, score + 1)

    for sign in [-1, 1]:
        next_dir = (sign * clockwise) @ dir
        next_pos = pos + next_dir
        if maze[tuple(next_pos)] != "#" and tuple(next_pos) not in visited:
            min_score = dfs2(next_pos, next_dir, maze, visited, min_score, score + 1001)

    visited.remove(tuple(pos))
    return min_score


if __name__ == "__main__":

    maze = parse_input("input.txt")

    start_pos = np.argwhere(maze == "S")[0]
    start_dir = np.array([0, 1])
    sys.setrecursionlimit(2000)
    print(sys.getrecursionlimit())
    print()
    visited = []
    print(dfs2(start_pos, start_dir, maze, visited, np.inf, 0))
