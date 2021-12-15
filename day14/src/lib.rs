use std::{collections::Vec, fmt::Display};

struct Rules([Option<u8>; 26 * 26]);

impl Rules {
    fn new(rules: &str) -> Self {
        let mut inserts = [None; 26 * 26];
        rules
            .lines()
            .map(|line| line.split_once(" -> ").unwrap())
            .for_each(|(lhs, rhs)| {
                inserts[usize::from(lhs.as_bytes()[0] - b'A') * 26
                    + usize::from(lhs.as_bytes()[1] - b'A')] = Some(rhs.as_bytes()[0] - b'A');
            });
        Self(inserts)
    }

    fn get(&self, (fst, snd): (u8, u8)) -> Option<u8> {
        self.0[usize::from(fst) * 26 + usize::from(snd)]
    }
}

fn step(rules: &Rules, polymer: &mut Vec<u8>) {
    let mut i = 0;
    while i < polymer.len() - 1 {
        let fst = polymer[i];
        let snd = polymer[i + 1];
        if let Some(new) = rules.get((fst, snd)) {
            polymer.insert(i + 1, new);
            i += 1;
        }
        i += 1;
    }
}

struct Computer {
    lut: [[usize; 26]; 26 * 26],
    rules: Rules,
}

fn tally(frequency: &mut [usize; 26], polymer: &[u8]) {
    for &ch in polymer {
        frequency[usize::from(ch)] += 1;
    }
}

impl Computer {
    fn new(rules: Rules) -> Self {
        Self {
            lut: [[0; 26]; 26 * 26],
            rules,
        }
    }

    fn run_toplevel(&mut self, template: &[u8], generations: usize) {
        template
            .windows(2)
            .for_each(|pair| self.run_pair(pair.try_into().unwrap(), generations));
    }

    fn run_pair(&mut self, [fst, snd]: [u8; 2], generations: usize) {
        let mut polymer = vec![fst, snd];

        for _ in 0..generations {
            step(&self.rules, &mut polymer);
        }

        tally(&mut self.lut[fst as usize * 26 + snd as usize], &polymer);
    }

}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let (template, rules) = include_str!("input.txt").trim().split_once("\n\n").unwrap();

    let rules = Rules::new(rules);

    let mut template = template.as_bytes().to_vec();
    template.iter_mut().for_each(|ch| *ch -= b'A');

    let mut computer = Computer::new(rules);

    computer.run_toplevel(&template, 10);

    ("TODO", "TODO")
}
