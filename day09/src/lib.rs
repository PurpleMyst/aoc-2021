use std::{cmp::Reverse, fmt::Display};

use ahash::AHashSet;
use arrayvec::ArrayVec;

const SIDE: usize = 100;

type Map = [u8; SIDE * SIDE];

fn neighbors(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
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

fn is_low(map: &Map, x: usize, y: usize) -> bool {
    let height = map[y * SIDE + x];
    neighbors(x, y).all(|(nx, ny)| height < map[ny * SIDE + nx])
}

struct BasinComputer {
    stack: ArrayVec<(usize, usize), { SIDE * SIDE }>,
    visited: AHashSet<(usize, usize)>,
}

impl BasinComputer {
    fn new() -> Self {
        Self {
            stack: ArrayVec::new(),
            visited: AHashSet::with_capacity(SIDE * SIDE),
        }
    }

    fn compute_basin(&mut self, map: &Map, x: usize, y: usize) -> usize {
        self.stack.clear();
        self.visited.clear();
        self.stack.push((x, y));

        while let Some((x, y)) = self.stack.pop() {
            self.visited.insert((x, y));
            let height = map[y * SIDE + x];
            self.stack.extend(neighbors(x, y).filter(|&(nx, ny)| {
                let nheight = map[ny * SIDE + nx];
                nheight != 9 && nheight > height && !self.visited.contains(&(nx, ny))
            }))
        }

        self.visited.len()
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut map = [0u8; SIDE * SIDE];

    include_str!("input.txt")
        .trim()
        .bytes()
        .filter(|ch| !ch.is_ascii_whitespace())
        .zip(map.iter_mut())
        .for_each(|(val, elem)| *elem = val - b'0');

    let mut p1: u64 = 0;
    let mut basins = [Reverse(0); 3];
    let mut computer = BasinComputer::new();

    for y in 0..SIDE {
        for x in 0..SIDE {
            if !is_low(&map, x, y) {
                continue;
            }

            p1 += 1 + u64::from(map[y * SIDE + x]);

            let basin = Reverse(computer.compute_basin(&map, x, y));
            let (Ok(idx) | Err(idx)) = basins.binary_search(&basin);
            if idx == basins.len() {
                continue;
            }
            basins[idx..].rotate_right(1);
            basins[idx] = basin;
        }
    }

    let p2: usize = basins.into_iter().map(|Reverse(n)| n).product();

    (p1, p2)
}
