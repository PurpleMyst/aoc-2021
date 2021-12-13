use std::fmt::Display;

const WIDTH: usize = 1311;
const HEIGHT: usize = 895;

struct Paper {
    width: usize,
    height: usize,
    dots: Vec<bool>,
}

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(
                    f,
                    "{}",
                    if self.dots[y * self.width + x] {
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
        let mut dots = vec![false; WIDTH * HEIGHT];
        for (x, y) in points.lines().map(|p| p.split_once(',').unwrap()) {
            let x = x.parse::<usize>().unwrap();
            let y = y.parse::<usize>().unwrap();
            debug_assert!(x < WIDTH && y < HEIGHT, "{} {}", x, y);
            dots[y * WIDTH + x] = true;
        }
        Self {
            width: WIDTH,
            height: HEIGHT,
            dots,
        }
    }

    fn fold_x(&mut self, n: usize) {
        // folding on vertical line x = n
        let mut folded = vec![false; n * self.height];

        for x in 0..n {
            for y in 0..self.height {
                let xp = self.width - x - 1;

                folded[y * n + x] = self.dots[y * self.width + x] | self.dots[y * self.width + xp];
            }
        }

        self.width = n;
        self.dots = folded;
    }

    fn fold_y(&mut self, n: usize) {
        // folding on horizontal line y = n
        let mut folded = vec![false; self.width * n];

        for x in 0..self.width {
            for y in 0..n {
                let yp = self.height - y - 1;

                folded[y * self.width + x] =
                    self.dots[y * self.width + x] | self.dots[yp * self.width + x];
            }
        }

        self.height = n;
        self.dots = folded;
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
            p1 = Some(paper.dots.iter().filter(|b| **b).count());
        }
    }

    (p1.unwrap(), paper)
}
