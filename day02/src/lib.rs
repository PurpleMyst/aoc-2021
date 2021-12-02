use std::fmt::Display;

#[derive(Default, Debug, Clone, Copy)]
struct State {
    x: i64,
    y_p1: i64,
    y_p2: i64,
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let state = include_str!("input.txt")
        .trim()
        .lines()
        .fold(State::default(), |state, line| {
                let (instr, amount) = line.split_once(' ').unwrap();
                let n: i64 = amount.parse().unwrap();
                match instr.as_bytes()[0] {
                    b'f' => State {
                        x: state.x + n,
                        y_p2: state.y_p2 + state.y_p1 * n,
                        ..state
                    },
                    b'd' => State {
                        y_p1: state.y_p1 + n,
                        ..state
                    },
                    b'u' => State {
                        y_p1: state.y_p1 - n,
                        ..state
                    },
                    _ => unreachable!(),
                }
            });

    (state.x * state.y_p1, state.x * state.y_p2)
}
