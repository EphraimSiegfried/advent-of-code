from part1 import parse_input, bfs

if __name__ == "__main__":
    wall = parse_input("input.txt")
    bound = 71
    start = (0, 0)
    end = (bound - 1, bound - 1)

    def binary_search(lower, upper):
        mid = lower + (upper - lower) // 2
        if upper - lower <= 1:
            return lower if bfs(start, end, bound, wall[:lower]) == -1 else upper
        if bfs(start, end, bound, wall[:mid]) != -1:
            return binary_search(mid, upper)
        else:
            return binary_search(lower, mid)

    index = binary_search(0, len(wall))
    print(wall[index - 1])
