use itertools::Itertools;
use std::fmt::Display;

struct Code([i8; 14]);

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for n in self.0 {
            write!(f, "{}", n)?;
        }
        Ok(())
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut part1 = Code([0; 14]);
    let mut part2 = Code([0; 14]);

    let mut stack = Vec::with_capacity(7);

    // Produced by massage.py
    include_str!("input_params.txt")
        .lines()
        .map(|line| {
            let mut it = line.split(' ').map(|s| s.parse().unwrap());
            let d = it.next().unwrap();
            let n = it.next().unwrap();
            let m = it.next().unwrap();
            (d, n, m)
        })
        .enumerate()
        .for_each(|(i, (d, n, m))| match d {
            1 => stack.push((i, m)),
            26 => {
                let (j, m) = stack.pop().unwrap();
                let m = m + n;

                let ((a1, b1), (a2, b2)) = (1..=9)
                    .filter(|k| (1..=9).contains(&(k + m)))
                    .map(|k| (k, k + m))
                    .minmax_by_key(|&(fst, snd)| if i > j { (fst, snd) } else { (snd, fst) })
                    .into_option()
                    .unwrap();

                part2.0[j] = a1;
                part2.0[i] = b1;
                part1.0[j] = a2;
                part1.0[i] = b2;
            }
            _ => unreachable!(),
        });

    (part1, part2)
}
