use std::{
    collections::{BinaryHeap, HashSet},
    fmt::Display,
};

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
    let value = map[y * SIDE + x];
    neighbors(x, y).all(|(nx, ny)| value < map[ny * SIDE + nx])
}

fn compute_basin(map: &Map, x: usize, y: usize) -> usize {
    let mut q = vec![(x, y)];
    let mut visited = HashSet::new();

    while let Some((x, y)) = q.pop() {
        if !visited.insert((x, y)) {
            continue;
        }

        let value = map[y * SIDE + x];
        q.extend(neighbors(x, y).filter(|&(nx, ny)| {
            let nheight = map[ny * SIDE + nx];
            nheight != 9 && nheight > value
        }))
    }

    visited.len()
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
    let mut basins = BinaryHeap::<usize>::new();

    for y in 0..SIDE {
        for x in 0..SIDE {
            if !is_low(&map, x, y) {
                continue;
            }

            p1 += 1 + u64::from(map[y * SIDE + x]);

            basins.push(compute_basin(&map, x, y));
        }
    }

    let p2: usize = (0..3).map(|_| basins.pop().unwrap()).product();

    (p1, p2)
}
