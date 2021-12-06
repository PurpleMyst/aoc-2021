use std::{cmp::Ordering, fmt::Display};

const SIDE: usize = 1000;

fn parse_point(s: &str) -> (usize, usize) {
    let (x, y) = s.split_once(",").unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

fn delta(start: usize, end: usize) -> isize {
    match start.cmp(&end) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let lines = include_str!("input.txt").trim().lines().map(|line| {
        let (start, end) = line.split_once(" -> ").unwrap();
        (parse_point(start), parse_point(end))
    });

    let mut space = vec![0u8; SIDE * SIDE];

    for ((x1, y1), (x2, y2)) in lines {
        let marker = if x1 == x2 || y1 == y2 { 0x01 } else { 0x10 };
        let dx = delta(x1, x2);
        let dy = delta(y1, y2);
        let mut x = x1;
        let mut y = y1;
        while x != x2 || y != y2 {
            space[y * SIDE + x] += marker;
            x = (x as isize + dx) as usize;
            y = (y as isize + dy) as usize;
        }

        // extra iteration for (x2, y2)
        space[y * SIDE + x] += marker;
    }

    let mut p1 = 0;
    let mut p2 = 0;

    for hits in space {
        let nondiagonal = hits & 0x0F;
        let diagonal = (hits & 0xF0) >> 4;

        if nondiagonal > 1 {
            p1 += 1;
        } else if nondiagonal + diagonal > 1 {
            p2 += 1;
        }
    }

    (p1, p1 + p2)
}
