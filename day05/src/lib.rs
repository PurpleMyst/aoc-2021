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

    let mut space = vec![0u8; SIDE * SIDE];

    for ((x1, y1), (x2, y2)) in lines {
        if x1 == x2 {
            let x = x1;
            for y in range(y1, y2) {
                space[y * SIDE + x] += 0x01;
            }
        } else if y1 == y2 {
            let y = y1;
            for x in range(x1, x2) {
                space[y * SIDE + x] += 0x01;
            }
        } else {
            let m = (y2 as isize - y1 as isize) / (x2 as isize - x1 as isize);
            let q = y1 as isize - m * x1 as isize;
            for x in range(x1, x2) {
                let y = (m * x as isize + q) as usize;
                space[y * SIDE + x] += 0x10;
            }
        }
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
