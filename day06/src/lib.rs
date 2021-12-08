use std::fmt::Display;

fn run(fishes: &mut [usize; 9], generations: usize) -> usize {
    for _ in 0..generations {
        fishes.rotate_left(1);
        fishes[6] += fishes[8];
    }

    fishes.iter().copied().sum()
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    // each index i in the array represents how many fishes with counter i there are
    let mut fishes = [0; 9];

    include_str!("input.txt")
        .trim()
        .split(',')
        .for_each(|line| fishes[line.parse::<usize>().unwrap()] += 1);

    (run(&mut fishes, 80), run(&mut fishes, 256 - 80))
}
