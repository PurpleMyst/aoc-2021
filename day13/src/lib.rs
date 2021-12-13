use std::fmt::Display;

struct Paper {
    dots: Vec<(usize, usize)>,
}

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut xmax = 0;
        let mut ymax = 0;
        for &(x, y) in &self.dots {
            xmax = xmax.max(x);
            ymax = ymax.max(y);
        }

        for y in 0..=ymax {
            for x in 0..=xmax {
                write!(
                    f,
                    "{}",
                    if self.dots.contains(&(x, y)) {
                        '\u{2588}'
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
        let mut dots = Vec::new();
        for (x, y) in points.lines().map(|p| p.split_once(',').unwrap()) {
            let x = x.parse::<usize>().unwrap();
            let y = y.parse::<usize>().unwrap();
            dots.push((x, y));
        }
        Self {
            dots,
        }
    }

    fn fold_x(&mut self, n: usize) {
        // folding on vertical line x = n
        for (x, _) in &mut self.dots {
            if *x > n {
                *x = 2 * n - *x;
            }
        }
    }

    fn fold_y(&mut self, n: usize) {
        // folding on horizontal line y = n
        for (_, y) in &mut self.dots {
            if *y > n {
                *y = 2 * n - *y;
            }
        }
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
