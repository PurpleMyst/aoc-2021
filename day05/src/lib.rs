use std::{fmt::Display, ops::RangeInclusive};

type Coord = usize;
type Point = (Coord, Coord);

const SIDE: usize = 1000;

fn parse_point(s: &str) -> Point {
    let (x, y) = s.split_once(",").unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

fn range(start: Coord, end: Coord) -> RangeInclusive<Coord> {
    if start < end {
        start..=end
    } else {
        end..=start
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let lines = include_str!("input.txt").trim().lines().map(|line| {
        let (start, end) = line.split_once(" -> ").unwrap();
        (parse_point(start), parse_point(end))
    });

    let mut p1_space = vec![0u8; SIDE * SIDE];
    let mut p2_space = vec![0u8; SIDE * SIDE];

    for ((x1, y1), (x2, y2)) in lines {
        if x1 == x2 {
            let x = x1;
            for y in range(y1, y2) {
                p1_space[y * SIDE + x] += 1;
                p2_space[y * SIDE + x] += 1;
            }
        } else if y1 == y2 {
            let y = y1;
            for x in range(x1, x2) {
                p1_space[y * SIDE + x] += 1;
                p2_space[y * SIDE + x] += 1;
            }
        } else {
            let m = (y2 as isize - y1 as isize) / (x2 as isize - x1 as isize);
            let q = y1 as isize - m * x1 as isize;
            for x in range(x1, x2) {
                let y = (m * x as isize + q) as usize;
                p2_space[y * SIDE + x] += 1;
            }
        }
    }

    let p1 = p1_space.iter().filter(|&&n| n > 1).count();
    let p2 = p2_space.iter().filter(|&&n| n > 1).count();

    (p1, p2)
}
