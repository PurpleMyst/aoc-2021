use std::fmt::Display;

fn opening(closing: u8) -> u8 {
    match closing {
        b')' => b'(',
        b']' => b'[',
        b'}' => b'{',
        b'>' => b'<',
        _ => unreachable!(),
    }
}

fn p1_score(closing: u8) -> u64 {
    match closing {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => unreachable!(),
    }
}

fn p2_score(b: u8) -> u64 {
    match b {
        b'(' => 1,
        b'[' => 2,
        b'{' => 3,
        b'<' => 4,
        _ => unreachable!(),
    }
}

fn is_opening(closing: u8) -> bool {
    matches!(closing, b'(' | b'[' | b'{' | b'<')
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut p1: u64 = 0;
    let mut stack = Vec::new();

    let mut p2_scores: Vec<u64> = include_str!("input.txt")
        .trim()
        .lines()
        .filter_map(|line| {
            stack.clear();
            for b in line.bytes() {
                if is_opening(b) {
                    stack.push(b);
                } else if stack.pop().unwrap() != opening(b) {
                    p1 += p1_score(b);
                    return None;
                }
            }

            Some(
                stack
                    .drain(..)
                    .rev()
                    .map(p2_score)
                    .fold(0, |acc, s| 5 * acc + s),
            )
        })
        .collect();

    p2_scores.sort_unstable();
    let p2 = p2_scores[p2_scores.len() / 2];

    (p1, p2)
}
