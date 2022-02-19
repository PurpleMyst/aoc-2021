from itertools import groupby
from pathlib import Path


def main() -> None:
    code = Path("src", "input.txt").read_text()

    instructions = map(str.split, code.splitlines())

    # Condense instructions into an easier-to-parse form
    condensed = []
    for var, ops in groupby(instructions, lambda kv: kv[1]):
        ops = list(ops)

        if ops == [["inp", "w"]]:
            condensed.extend(ops)
            continue

        new_ops = []
        while ops:
            match ops:
                case [["mul", _, "0"], ["add", _, src], *_]:
                    ops[:2] = [["set", var, src]]
                case [["eql", _, target], ["eql", _, "0"], *_]:
                    ops[:2] = [["neq", var, target]]

            if ops:
                new_ops.append(ops.pop(0))

        condensed.extend(new_ops)

    # Parse out parameters
    params = []
    while condensed:
        match condensed:
            case [
                ["inp", "w"],
                ["set", "x", "z"],
                ["mod", "x", "26"],
                ["div", "z", d],
                ["add", "x", n],
                ["neq", "x", "w"],
                ["set", "y", "25"],
                ["mul", "y", "x"],
                ["add", "y", "1"],
                ["mul", "z", "y"],
                ["set", "y", "w"],
                ["add", "y", m],
                ["mul", "y", "x"],
                ["add", "z", "y"],
                *_,
            ]:
                del condensed[:14]
                params.append(" ".join((d,n,m)))
            case _:
                raise RuntimeError

    # Write params
    Path("src", "input_params.txt").write_text("\n".join(params))


if __name__ == "__main__":
    main()
