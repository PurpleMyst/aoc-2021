use std::fmt::Display;

use pathfinding::prelude::{absdiff, astar};

const P1_SIDE: usize = 100;
const P2_SIDE: usize = 500;

fn neighbors<const SIDE: usize>(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    (-1..=1isize).flat_map(move |dx| {
        (-1..=1isize).filter_map(move |dy| {
            if (dx != 0 && dy != 0) || (dx == 0 && dy == 0) {
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

fn foo(mut risk: usize) -> usize {
    while risk > 9 {
        risk -= 9;
    }
    risk
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let map: Vec<_> = include_str!("input.txt")
        .bytes()
        .filter(|ch| ch.is_ascii_digit())
        .map(|b| (b - b'0') as usize)
        .collect();

    let (_, p1) = astar(
        &(0, 0),
        |&(x, y)| neighbors::<P1_SIDE>(x, y).map(|(x, y)| ((x, y), map[y * P1_SIDE + x])),
        |&(x, y)| absdiff(x, P1_SIDE - 1) + absdiff(y, P1_SIDE - 1),
        |&p| p == (P1_SIDE - 1, P1_SIDE - 1),
    )
    .unwrap();

    let (_, p2) = astar(
        &(0, 0),
        |&(x, y)| {
            neighbors::<P2_SIDE>(x, y).map(|(x, y)| {
                (
                    (x, y),
                    foo(map[(y % P1_SIDE) * P1_SIDE + (x % P1_SIDE)] + x / P1_SIDE + y / P1_SIDE),
                )
            })
        },
        |&(x, y)| absdiff(x, P1_SIDE - 1) + absdiff(y, P1_SIDE - 1),
        |&p| p == (P2_SIDE - 1, P2_SIDE - 1),
    )
    .unwrap();

    (p1, p2)
}
