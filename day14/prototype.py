from pathlib import Path

import numpy as np


def main() -> None:
    # Parse the input
    inp = Path("src/input.txt").read_text()
    [template, rules] = inp.split("\n\n")
    rules = dict(rule.split(" -> ") for rule in rules.splitlines())

    # Make an order for all possible pairs
    order = list(rules.keys())

    # Construct a matrix that maps each rule AB -> C to the pairs AC and CB that it produces.
    # Each column corresponds to one pair.
    step_matrix = np.zeros((len(order), len(order)), dtype=np.int64)
    for i, pair in enumerate(order):
        new = rules[pair]
        products = pair[0] + new, new + pair[1]
        for product in products:
            assert product in order
            step_matrix[order.index(product), i] += 1

    # Construct the vector corresponding to the pairs in our template
    template_matrix = np.zeros(len(order), dtype=np.int64)
    for pair in zip(template, template[1:]):
        template_matrix[order.index("".join(pair))] += 1

    print(step_matrix)
    print( np.linalg.matrix_power(step_matrix, 2))
    print( np.linalg.matrix_power(step_matrix, 3))

    def solve(steps: int) -> int:
        # Exponentiate our matrix to the number of steps required.
        steps_matrix = np.linalg.matrix_power(step_matrix, steps)

        # Apply our matrix to the template.
        state = steps_matrix @ template_matrix

        # Calculate the frequency of each element.
        # We only count the first item of each pair to avoid double-counting: this means
        # we'd skip over the last element in the polymer, so we handle that edge case
        # specially.
        freq = {template[-1]: 1}
        for pair, count in zip(order, state):
            freq[pair[0]] = freq.get(pair[0], 0) + count

        # Get the answer
        return max(freq.values()) - min(freq.values())

    print(solve(10))
    print(solve(40))

if __name__ == "__main__":
    main()
