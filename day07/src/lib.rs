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

fn p2_fuel(crabs: &[usize; 2000], pos: usize) -> usize {
    crabs
        .iter()
        .enumerate()
        .map(|(pos2, n)| sum_up_to(abs_diff(pos, pos2)) * n)
        .sum()
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut crabs = [0; 2000];

    include_str!("input.txt")
        .trim()
        .split(',')
        .for_each(|crab| crabs[crab.parse::<usize>().unwrap()] += 1);

    let p1: usize = crabs
        .iter()
        .enumerate()
        .filter(|&(_, &n)| n != 0)
        .map(|(pos, _)| {
            crabs
                .iter()
                .enumerate()
                .map(|(pos2, n)| abs_diff(pos, pos2) * n)
                .sum()
        })
        .min()
        .unwrap();

    let mut seed = 1000;
    let mut current = p2_fuel(&crabs, seed);

    loop {
        let left = p2_fuel(&crabs, seed - 1);
        let right = p2_fuel(&crabs, seed + 1);

        if left < current {
            seed -= 1;
            current = left;
        } else if right < current {
            seed += 1;
            current = right;
        } else {
            break;
        }
    }

    (p1, p2_fuel(&crabs, seed))
}
