use std::fmt::Display;

use bitvec::prelude::*;

const SIDE: usize = 10;

fn neighbors(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    (-1..=1).flat_map(move |dx| {
        (-1..=1).filter_map(move |dy| {
            if dx == 0 && dy == 0 {
                return None;
            }

            let nx = usize::try_from(x as isize + dx).ok()?;
            let ny = usize::try_from(y as isize + dy).ok()?;

            if nx >= SIDE || ny >= SIDE {
                return None;
            }

            Some((nx, ny))
        })
    })
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut map = [0u8; SIDE * SIDE];
    include_str!("input.txt")
        .bytes()
        .filter(|ch| !ch.is_ascii_whitespace())
        .zip(map.iter_mut())
        .for_each(|(value, elem)| *elem = value - b'0');

    let mut steps = std::iter::from_fn(move || {
        let mut flashed = bitarr![0; SIDE * SIDE];

        // increment all octopi
        for y in 0..SIDE {
            for x in 0..SIDE {
                map[y * SIDE + x] += 1;
            }
        }

        loop {
            // count how many octopi have flashed so far
            let previous = flashed.count_ones();

            // flash all octopi that should do so
            for y in 0..SIDE {
                for x in 0..SIDE {
                    let idx = y * SIDE + x;
                    if map[idx] > 9 && !flashed[idx] {
                        flashed.set(idx, true);

                        neighbors(x, y).for_each(|(x, y)| map[y * SIDE + x] += 1);
                    }
                }
            }

            // if no new octopi have flashed, break out
            if flashed.count_ones() == previous {
                break;
            }
        }

        // reset all flashed octopi
        flashed.iter_ones().for_each(|idx| map[idx] = 0);

        Some(flashed.count_ones())
    });

    (
        steps.by_ref().take(100).sum::<usize>(),
        1 + 100 + steps.position(|c| c == SIDE * SIDE).unwrap(),
    )
}
