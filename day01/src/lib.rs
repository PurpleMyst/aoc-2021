use std::fmt::Display;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let report: Vec<u16> = include_str!("input.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let p1 = report.windows(2).filter(|w| w[0] < w[1]).count();
    let p2 = report
        .windows(4)
        .filter(|w| w[..3].into_iter().sum::<u16>() < w[1..][..3].into_iter().sum())
        .count();

    (p1, p2)
}
