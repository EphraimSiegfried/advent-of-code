def parse_input(filename):
    with open(filename) as file:
        seeds = [int(line) for line in file]
    return seeds


def pseudorandom(secret_val, num_iter=1):
    mix = lambda x1, x2: x1 ^ x2
    prune = lambda x: x % 16777216
    for _ in range(num_iter):
        secret_val = prune(mix(secret_val * 64, secret_val))
        secret_val = mix(secret_val // 32, secret_val)
        secret_val = prune(mix(secret_val * 2048, secret_val))
    return secret_val


if __name__ == "__main__":
    seeds = parse_input("input.txt")
    print(sum(pseudorandom(seed, 2000) for seed in seeds))
