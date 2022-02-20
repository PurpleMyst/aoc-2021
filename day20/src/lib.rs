use ahash::{AHashMap, AHashSet};
use bitvec::prelude::*;

use std::{fmt::Display, mem::swap};

type Point = (isize, isize);
type State = AHashMap<Point, bool>;

fn shell((x, y): Point) -> [Point; 9] {
    [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let (algorithm_data, start_state) = include_str!("input.txt").split_once("\n\n").unwrap();
    let mut algorithm = bitarr![0; 512];
    algorithm_data
        .bytes()
        .zip(algorithm.iter_mut())
        .for_each(|(b, mut dest)| *dest = b == b'#');

    debug_assert!(algorithm[0]);
    debug_assert!(!algorithm[511]);

    let mut state: State = start_state
        .split('\n')
        .enumerate()
        .flat_map(|(y, row)| {
            row.bytes()
                .enumerate()
                .map(move |(x, ch)| ((x as _, y as _), ch == b'#'))
        })
        .collect();

    let mut next_state = State::default();
    let mut part1 = 0xDEAD;
    let mut part2 = 0xBEEF;

    for i in 0..50 {
        next_state.clear();

        let inner: AHashSet<Point> = state.iter().flat_map(|(&pos, _)| shell(pos)).collect();

        next_state.extend(inner.iter().map(|&pos| {
            (
                pos,
                algorithm[shell(pos)
                    .into_iter()
                    .map(|p| state.get(&p).copied().unwrap_or(i % 2 != 0))
                    .fold(0, |a, b| (a << 1) | (b as usize))],
            )
        }));

        swap(&mut next_state, &mut state);

        let lits = || state.values().filter(|&&b| b).count();
        if i == 1 {
            part1 = lits();
        } else if i == 49 {
            part2 = lits();
        }
    }

    (part1, part2)
}
