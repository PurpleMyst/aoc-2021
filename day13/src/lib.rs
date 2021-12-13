use std::fmt::Display;

use ahash::AHashSet;

const WIDTH: usize = 1311;
const HEIGHT: usize = 895;

struct Paper {
    width: usize,
    height: usize,
    dots: AHashSet<(usize, usize)>,
}

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(
                    f,
                    "{}",
                    if self.dots.contains(&(x, y)) {
                        '#'
                    } else {
                        ' '
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Paper {
    fn new(points: &str) -> Self {
        let mut dots = AHashSet::new();
        for (x, y) in points.lines().map(|p| p.split_once(',').unwrap()) {
            let x = x.parse::<usize>().unwrap();
            let y = y.parse::<usize>().unwrap();
            debug_assert!(x < WIDTH && y < HEIGHT, "{} {}", x, y);
            dots.insert((x, y));
        }
        Self {
            width: WIDTH,
            height: HEIGHT,
            dots,
        }
    }

    fn fold_x(&mut self, n: usize) {
        // folding on vertical line x = n
        for (x, y) in std::mem::take(&mut self.dots) {
            if x < n {
                self.dots.insert((x, y));
            } else {
                self.dots.insert((2 * n - x, y));
            }
        }

        self.width = n;
    }

    fn fold_y(&mut self, n: usize) {
        // folding on horizontal line y = n
        for (x, y) in std::mem::take(&mut self.dots) {
            if y < n {
                self.dots.insert((x, y));
            } else {
                self.dots.insert((x, 2 * n - y));
            }
        }

        self.height = n;
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let (points, instructions) = include_str!("input.txt").trim().split_once("\n\n").unwrap();
    let mut paper = Paper::new(points);

    let mut p1: Option<usize> = None;
    for instruction in instructions.lines() {
        let (_, line) = instruction.rsplit_once(' ').unwrap();
        let (axis, coord) = line.split_once('=').unwrap();
        let coord = coord.parse().unwrap();
        match axis {
            "x" => paper.fold_x(coord),
            "y" => paper.fold_y(coord),
            _ => unreachable!(),
        }

        if p1.is_none() {
            p1 = Some(paper.dots.len());
        }
    }

    (p1.unwrap(), paper)
}
