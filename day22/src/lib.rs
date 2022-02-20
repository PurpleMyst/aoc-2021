use std::{cmp::Ordering, fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy)]
struct Range(i64, i64);

#[derive(Debug, Clone, Copy)]
struct Cuboid {
    x: Range,
    y: Range,
    z: Range,

    sgn: i64,
}

impl Range {
    fn intersect(&self, other: &Self) -> Option<Self> {
        match self.0.cmp(&other.0) {
            Ordering::Less => match self.1.cmp(&other.0) {
                Ordering::Less => None,
                Ordering::Equal => Some(Self(self.1, other.0)),
                Ordering::Greater => Some(Self(other.0, self.1.min(other.1))),
            },
            Ordering::Equal => Some(Self(self.0, self.1.min(other.1))),
            Ordering::Greater => other.intersect(&self),
        }
    }

    fn len(&self) -> i64 {
        (self.1 - self.0).abs() + 1
    }
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, stop) = s.split_once("..").ok_or(())?;
        let start = start.parse().map_err(|_| ())?;
        let stop = stop.parse().map_err(|_| ())?;
        Ok(Self(start, stop))
    }
}

impl Cuboid {
    fn intersect(&self, other: &Self) -> Option<Self> {
        Some(Self {
            x: self.x.intersect(&other.x)?,
            y: self.y.intersect(&other.y)?,
            z: self.z.intersect(&other.z)?,
            sgn: self.sgn * other.sgn * -1,
        })
    }

    fn volume(&self) -> i64 {
        self.x.len() * self.y.len() * self.z.len() * self.sgn
    }
}

impl FromStr for Cuboid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, yz) = s.split_once(',').ok_or(())?;
        let (y, z) = yz.split_once(',').ok_or(())?;
        let x = x[2..].parse().map_err(|_| ())?;
        let y = y[2..].parse().map_err(|_| ())?;
        let z = z[2..].parse().map_err(|_| ())?;
        Ok(Self { x, y, z, sgn: 1 })
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt").lines().map(|line| {
        let (state, cuboid) = line.split_once(' ').unwrap();
        let cuboid: Cuboid = cuboid.parse().unwrap();
        (state == "on", cuboid)
    });

    let mut core: Vec<Cuboid> = vec![];
    for (turn_on, cuboid) in input {
        (0..core.len()).for_each(|i| core.extend(cuboid.intersect(&core[i])));
        if turn_on {
            core.push(cuboid);
        }
    }

    let init_region = Cuboid {
        x: Range(-50, 50),
        y: Range(-50, 50),
        z: Range(-50, 50),
        sgn: 1,
    };
    let part1 = core
        .iter()
        .map(|c| c.intersect(&init_region).map_or(0, |c| -c.volume()))
        .sum::<i64>();
    let part2 = core.into_iter().map(|c| c.volume()).sum::<i64>();

    (part1, part2)
}
