use std::fmt::Display;

use ndarray::Array;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let (template, rules) = include_str!("input.txt").trim().split_once("\n\n").unwrap();

    let mut cake = bimap::BiMap::new();
    let mut fruit = Vec::new();
    let mut counter = 0;

    rules.lines().for_each(|line| {
        let (lhs, rhs) = line.split_once(" -> ").unwrap();

        let lhs: [u8; 2] = lhs.as_bytes().try_into().unwrap();
        cake.insert(lhs, counter);
        fruit.push((counter, lhs, rhs.as_bytes()[0]));

        counter += 1;
    });

    let mut f = ndarray::Array::zeros((counter, counter));
    for (i, [a, b], c) in fruit {
        f[(*cake.get_by_left(&[a, c]).unwrap(), i)] += 1;
        f[(*cake.get_by_left(&[c, b]).unwrap(), i)] += 1;
    }

    let mut t = ndarray::Array::zeros(counter);
    for w in template.as_bytes().windows(2) {
        let w: [u8; 2] = w.try_into().unwrap();
        t[*cake.get_by_left(&w).unwrap()] += 1;
    }

    let solver = |n| {
        let result = (0..n - 1)
            .fold(f.clone(), |acc: Array<u64, _>, _| acc.dot(&f))
            .dot(&t);

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
