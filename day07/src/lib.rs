use std::fmt::Display;

fn abs_diff(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}

fn sum_up_to(n: usize) -> usize {
    n * (n + 1) / 2
}

fn p2_fuel(crabs: &[usize], pos: usize) -> usize {
    crabs
        .iter()
        .map(|&pos2| sum_up_to(abs_diff(pos, pos2)))
        .sum()
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut crabs: Vec<_> = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|crab| crab.parse::<usize>().unwrap())
        .collect();
    crabs.sort_unstable();

    let median = crabs[crabs.len() / 2];
    let mean = crabs.iter().sum::<usize>() / crabs.len();

    let p1: usize = crabs.iter().map(|&pos| abs_diff(pos, median)).sum();
    let p2: usize = p2_fuel(&crabs, mean).min(p2_fuel(&crabs, mean + 1));

    (p1, p2)
}
