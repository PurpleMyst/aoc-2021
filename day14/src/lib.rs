use std::fmt::Display;

use ndarray::{Array, Array2};

fn mat_pow(x: Array2<u64>, n: u64) -> Array2<u64> {
    if n == 0 {
        unreachable!();
    } else if n == 1 {
        x
    } else if n & 1 == 0 {
        mat_pow(x.dot(&x), n / 2)
    } else {
        x.dot(&mat_pow(x.dot(&x), (n - 1) / 2))
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let (template, rules) = include_str!("input.txt").trim().split_once("\n\n").unwrap();

    let mut cake = ahash::AHashMap::new();
    let mut fruit = Vec::new();
    let mut counter = 0;

    rules.lines().for_each(|line| {
        let (lhs, rhs) = line.split_once(" -> ").unwrap();

        let lhs: [u8; 2] = lhs.as_bytes().try_into().unwrap();
        cake.insert(lhs, counter);
        fruit.push((counter, lhs, rhs.as_bytes()[0]));

        counter += 1;
    });

    let mut f = Array::zeros((counter, counter));
    for (i, [a, b], c) in fruit {
        f[(cake[&[a, c]], i)] += 1;
        f[(cake[&[c, b]], i)] += 1;
    }

    let mut t = Array::zeros(counter);
    for w in template.as_bytes().windows(2) {
        let w: [u8; 2] = w.try_into().unwrap();
        t[cake[&w]] += 1;
    }

    let solver = |n: u64| {
        let result = mat_pow(f.clone(), n).dot(&t);

        let mut freq = [0; 26];
        freq[usize::from(template.as_bytes().last().unwrap() - b'A')] += 1;

        for (&k, &v) in cake.iter() {
            freq[usize::from(k[0] - b'A')] += result[v] as u64;
        }

        freq.iter().copied().max().unwrap()
            - freq.iter().copied().filter(|&n| n != 0).min().unwrap()
    };

    (solver(10), solver(40))
}
