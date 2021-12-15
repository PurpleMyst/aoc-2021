use std::{collections::HashMap, fmt::Display};

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let (template, rules) = include_str!("input.txt")
        .trim()
        .split_once("\n\n")
        .unwrap();

    let rules: HashMap<_, _> = rules
        .trim()
        .lines()
        .map(|line| {
            let (k, v) = line.split_once(" -> ").unwrap();
            (k.as_bytes(), v.as_bytes()[0])
        })
        .collect();

    let mut template = template.as_bytes().to_vec();

    for _ in 0..40 {
        let mut i = 0;
        while i < template.len() - 1 {
            if let Some(new) = rules.get(&template[i..i + 2]) {
                template.insert(i + 1, *new);
                i += 1;
            }
            i += 1;
        }
    }

    let mut freq = [None; 26];
    for ch in template {
        *freq[usize::from(ch - b'A')].get_or_insert(0) += 1
    }
    let p1 = freq.iter().filter_map(|&k| k).max().unwrap() - freq.iter().filter_map(|&k| k).min().unwrap();

    (p1, "TODO")
}
