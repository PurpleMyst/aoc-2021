use std::fmt::Display;

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
struct SegmentDisplay {
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

impl SegmentDisplay {
    fn parse(s: &str) -> SegmentDisplay {
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

    fn count_trivial(&self) -> usize {
        self.output
            .iter()
            .filter(|segments| matches!(segments.on(), 2 | 3 | 4 | 7))
            .count()
    }

    fn deduce(&self) -> (Segments, Segments) {
        let mut one = None;
        let mut four = None;

        for segments in self.signals {
            match segments.on() {
                2 => one = Some(segments),
                4 => four = Some(segments),
                _ => {}
            }
        }

        (one.unwrap(), four.unwrap())
    }

    fn output_value(&self) -> usize {
        let (one, four) = self.deduce();
        self.output
            .iter()
            .map(|segments| match (segments.on(), segments.in_common(&one), segments.in_common(&four)) {
                (6, 2, 3) => 0,
                (2, _, _) => 1,
                (5, 1, 2) => 2,
                (5, 2, 3) => 3,
                (4, _, _) => 4,
                (5, 1, 3) => 5,
                (6, 1, 3) => 6,
                (3, _, _) => 7,
                (7, _, _) => 8,
                (6, 2, 4) => 9,
                _ => unreachable!(),
            })
            .fold(0, |acc, val| 10 * acc + val)
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let lines = include_str!("input.txt").lines().map(SegmentDisplay::parse);

    let mut p1: usize = 0;
    let mut p2: usize = 0;

    for line in lines {
        p1 += line.count_trivial();
        p2 += line.output_value();
    }

    (p1, p2)
}
