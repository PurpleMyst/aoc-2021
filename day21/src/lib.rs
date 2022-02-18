use ahash::AHashMap;
use std::{fmt::Display, mem::swap};

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
struct Player {
    pos: u16,
    score: u16,
}

impl Player {
    fn new(pos: u16) -> Self {
        Self { pos, score: 0 }
    }

    fn tick(&mut self, die: impl Iterator<Item = u16>) {
        *self = self.do_roll(die.take(3).sum());
    }

    fn do_roll(self, roll: u16) -> Self {
        let pos = (self.pos + roll) % 10;
        Self {
            pos,
            score: self.score + if pos == 0 { 10 } else { pos },
        }
    }
}

fn solve_part1(pos1: u16, pos2: u16) -> u64 {
    const SCORE_THRESHOLD: u16 = 1000;

    let mut p1 = Player::new(pos1);
    let mut p2 = Player::new(pos2);

    let mut die = (1..=100).cycle();
    let mut rolls = 0;

    loop {
        p1.tick(&mut die);
        rolls += 3;
        if p1.score >= SCORE_THRESHOLD {
            break p2.score as u64 * rolls;
        }

        p2.tick(&mut die);
        rolls += 3;
        if p2.score >= SCORE_THRESHOLD {
            break p1.score as u64 * rolls;
        }
    }
}

fn solve_part2(pos1: u16, pos2: u16) -> u64 {
    const SCORE_THRESHOLD: u16 = 21;

    let rolls = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    let mut states = AHashMap::new();
    let mut next_states = AHashMap::new();

    let mut p1_wins = 0;
    let mut p2_wins = 0;

    states.insert((Player::new(pos1), Player::new(pos2)), 1);

    while !states.is_empty() {
        for ((p1, p2), count0) in states.drain() {
            if count0 == 0 {
                continue;
            }
            for (roll1, count1) in rolls {
                let p1 = p1.do_roll(roll1);
                if p1.score >= SCORE_THRESHOLD {
                    p1_wins += count0 * count1;
                    continue;
                }

                for (roll2, count2) in rolls {
                    let p2 = p2.do_roll(roll2);
                    if p2.score >= SCORE_THRESHOLD {
                        p2_wins += count0 * count1 * count2;
                        continue;
                    }

                    *next_states.entry((p1, p2)).or_default() += count0 * count1 * count2;
                }
            }
        }

        swap(&mut states, &mut next_states);
    }

    p1_wins.max(p2_wins)
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let (p1, p2) = include_str!("input.txt").split_once('\n').unwrap();
    let p1 = p1.split_once(": ").unwrap().1.trim().parse().unwrap();
    let p2 = p2.split_once(": ").unwrap().1.trim().parse().unwrap();

    (solve_part1(p1, p2), solve_part2(p1, p2))
}
