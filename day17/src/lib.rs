use std::fmt::Display;

fn parse_range(s: &str) -> (i64, i64) {
    let (l, r) = s[2..].split_once("..").unwrap();
    (l.parse().unwrap(), r.parse().unwrap())
}

fn simulate(mut vx: i64, mut vy: i64) -> impl Iterator<Item = (i64, i64)> {
    let mut x = 0;
    let mut y = 0;

    std::iter::from_fn(move || {
        x += vx;
        y += vy;
        vx -= vx.signum();
        vy -= 1;
        Some((x, y))
    })
}

fn predict(vx: i64, vy: i64, tx1: i64, tx2: i64, ty1: i64, ty2: i64) -> Option<i64> {
    let mut ymax = 0;
    simulate(vx, vy)
        .inspect(|&(_, y)| ymax = ymax.max(y))
        .find_map(|(x, y)| {
            ((tx1..=tx2).contains(&x) && (ty1..=ty2).contains(&y))
                .then(|| true)
                .or_else(|| (x > tx2 || (y < 0 && y.abs() > ty1.abs())).then(|| false))
        })
        .unwrap()
        .then(|| ymax)
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let (tx, ty) = include_str!("input.txt")
        .trim()
        .split_once(": ")
        .unwrap()
        .1
        .split_once(", ")
        .unwrap();

    let (tx1, tx2) = parse_range(tx);
    let (ty1, ty2) = parse_range(ty);
    debug_assert!(ty2 < 0);

    let mut p1 = 0;
    let mut p2 = 0;
    for vx in 0..=tx2 {
        for vy in ty2 * 2..=ty2 * -2 {
            if let Some(ymax) = predict(vx, vy, tx1, tx2, ty1, ty2) {
                p1 = p1.max(ymax);
                p2 += 1;
            }
        }
    }

    (p1, p2)
}
