from itertools import product
from collections import Counter

from tqdm import trange

T_SCORE = 21

states: Counter[tuple[int, int, int, int]] = Counter({(8, 10, 0, 0): 1})

dice_triplets: Counter[int] = Counter(map(sum, product(range(1, 4), repeat=3)))
print(list(dice_triplets.items()))

p1_wins = 0
p2_wins = 0

for _ in trange(10):
    next_states = Counter()
    updated = 0
    for state, v in states.items():
        if v == 0:
            continue
        updated += 1
        for triplet1, count1 in dice_triplets.items():
            pos1 = (state[0] + triplet1) % 10
            score1 = state[2] + (pos1 if pos1 else 10)

            if score1 >= T_SCORE:
                p1_wins += v * count1
                continue

            for triplet2, count2 in dice_triplets.items():
                pos2 = (state[1] + triplet2) % 10
                score2 = state[3] + (pos2 if pos2 else 10)

                if score2 >= T_SCORE:
                    p2_wins += v * count1 * count2
                    continue

                next_states[(pos1, pos2, score1, score2)] += v * count1 * count2

    states = next_states

print(max(p1_wins, p2_wins))
