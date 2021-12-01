use std::fmt::Display;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let report: Vec<u16> = include_str!("input.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let p1 = report.windows(2).filter(|w| w[0] < w[1]).count();

    // a + b + c < b + c + d <=> a < d
    let p2 = report.windows(4).filter(|w| w[0] < w[3]).count();

    (p1, p2)
}
