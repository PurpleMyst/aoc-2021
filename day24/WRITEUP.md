This problem is very interesting. When decompiled, we can see that the program is
equivalent to the following Python code:
```python
def _step(z: int, w: int, d: int, n: int, m: int) -> int:
    x = (z % 26 + n) != w
    z //= d
    z *= (25 * x) + 1
    z += (w + m) * x
    return z


def checksum(values: list[int]) -> int:
    assert len(values) == len(PARAMS)
    z = 0
    for w, (d, n, m) in zip(values, PARAMS):
        z = _step(z, w, d, n, m)
    return z
```
Where `PARAMS` is dependent upon each users' input. 

By analyzing `PARAMS` some properties emerge:
* There are exactly as many ones as there are twentysixes for the `d` parameter.
* Every time `d = 1`, `n >= 10`. This means that, in `_step`, `x` can never be zero. Therefore, we can specialize the code as:
```python
def _step1(z: int, w: int n: int, m: int) -> int:
    return 26 * z + (w + m)
```
* Every time `d = 26`, `n <= 0`. This means that we can find a value of `w` such that
  `x` becomes zero. We can specialize the code this time as:
```python
def _step26(z: int, w: int, n: int) -> int:
    assert w == (z % 26 + n)
    return z // 26
```

We can therefore treat the `z` variable as if it were a stack: In the `d = 1` case, we
push a value to the stack. In the `d = 26`, given the right `w`, we pop a value off the
stack. The problem therefore boils down to making sure we have an empty stack at the end
of the program.

We can therefore iterate through `PARAMS` and keep track of the stack: Any time we push,
we push `m` and the index `j` of the corresponding `w`. For each pop, we calculate all
pairs of `w` values in the current position and in the top-of-stack's position and find
the two that maximize and minimize the value of the code.
