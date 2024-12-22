from part1 import pseudorandom, parse_input
from itertools import combinations, tee, islice
from operator import itemgetter


def get_prices(secret_val, num_iter=1):
    prices = []
    for _ in range(num_iter):
        prices.append(secret_val % 10)
        secret_val = pseudorandom(secret_val)
    return prices


def diff(lst):
    return [x2 - x1 for x1, x2 in zip(lst, lst[1:])]


def sliding_window(lst, n):
    iterators = tee(lst, n)
    return zip(*(islice(it, i, None) for i, it in enumerate(iterators)))


if __name__ == "__main__":
    seeds = parse_input("input.txt")

    buyer_prices = [get_prices(seed, 2000) for seed in seeds]
    buyer_changes = [diff(prices) for prices in buyer_prices]

    window_size = 4
    sliding_window_sets = []
    sliding_window_dicts = []
    for changes in buyer_changes:
        windows = list(sliding_window(changes, window_size))
        sliding_window_sets.append(set(windows))

        seq_positions = {}
        for idx, w in enumerate(windows):
            seq_positions.setdefault(w, []).append(idx)
        sliding_window_dicts.append(seq_positions)

    max_num_bananas = -1

    n = len(sliding_window_sets)
    for combo_size in range(n, max(n - 3, 0), -1):
        for comb in combinations(range(n), combo_size):
            common_sequences = set.intersection(*itemgetter(*comb)(sliding_window_sets))
            if not common_sequences:
                continue

            for sequence in common_sequences:
                total_bananas = 0
                for buyer_idx in comb:
                    pos = sliding_window_dicts[buyer_idx][sequence][0]
                    total_bananas += buyer_prices[buyer_idx][pos + window_size]

                if total_bananas > max_num_bananas:
                    print(total_bananas)
                    max_num_bananas = total_bananas

    print(max_num_bananas)
