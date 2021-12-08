use std::fmt::Display;

use arrayvec::ArrayVec;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
struct Segments(u8);

impl std::fmt::Debug for Segments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Segments")
            .field(&format!("{:07b}", self.0))
            .finish()
    }
}

#[derive(Default, Clone, Copy)]
struct LCD {
    signals: [Segments; 10],
    output: [Segments; 4],
}

impl Segments {
    fn parse(s: &str) -> Self {
        Self(
            s.bytes()
                .map(|ch| ch - b'a')
                .fold(0, |acc, b| acc | (1 << b)),
        )
    }

    fn on(&self) -> u32 {
        self.0.count_ones()
    }

    fn in_common(&self, other: &Self) -> u32 {
        (self.0 & other.0).count_ones()
    }
}

fn deduce(bucket: &mut ArrayVec<Segments, 3>, predicate: impl Fn(&Segments) -> bool) -> Segments {
    let nine_pos = bucket.iter().position(predicate).unwrap();
    bucket.swap_remove(nine_pos)
}

impl LCD {
    fn parse(s: &str) -> LCD {
        let mut this = Self::default();

        let (signals, output) = s.split_once(" | ").unwrap();

        signals
            .split_whitespace()
            .map(Segments::parse)
            .zip(this.signals.iter_mut())
            .for_each(|(value, elem)| *elem = value);

        output
            .split_whitespace()
            .map(Segments::parse)
            .zip(this.output.iter_mut())
            .for_each(|(value, elem)| *elem = value);

        this
    }

    fn part1(&self) -> usize {
        self.output
            .iter()
            .filter(|segments| matches!(segments.on(), 2 | 3 | 4 | 7))
            .count()
    }

    fn deduce(&self) -> [Segments; 10] {
        let mut one = None;
        let mut four = None;
        let mut seven = None;
        let mut eight = None;

        let mut five_bucket = ArrayVec::<_, 3>::new();
        let mut six_bucket = ArrayVec::<_, 3>::new();

        for signal in self.signals {
            match signal.on() {
                2 => one = Some(signal),
                3 => seven = Some(signal),
                4 => four = Some(signal),
                5 => five_bucket.push(signal),
                6 => six_bucket.push(signal),
                7 => eight = Some(signal),
                _ => unreachable!(),
            }
        }

        let one = one.unwrap();
        let four = four.unwrap();
        let seven = seven.unwrap();
        let eight = eight.unwrap();

        let three = deduce(&mut five_bucket, |signal| {
            signal.in_common(&one) == one.on()
        });

        let five = deduce(&mut five_bucket, |signal| {
            signal.in_common(&four) == four.on() - 1
        });

        let two = five_bucket.pop().unwrap();

        let nine = deduce(&mut six_bucket, |signal| {
            signal.in_common(&three) == three.on()
        });

        let zero = deduce(&mut six_bucket, |signal| {
            signal.in_common(&seven) == seven.on()
        });

        let six = six_bucket.pop().unwrap();

        [zero, one, two, three, four, five, six, seven, eight, nine]
    }

    fn part2(&self) -> usize {
        let digits = self.deduce();
        self.output
            .iter()
            .map(|signal| digits.iter().position(|signal2| signal == signal2).unwrap())
            .fold(0, |acc, val| 10 * acc + val)
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let lines = include_str!("input.txt").lines().map(LCD::parse);

    let mut p1: usize = 0;
    let mut p2: usize = 0;

    for line in lines {
        p1 += line.part1();
        p2 += line.part2();
    }

    (p1, p2)
}
