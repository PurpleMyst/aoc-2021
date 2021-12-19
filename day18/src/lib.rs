use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Item {
    OpenPair,
    ClosePair,
    Number(u16),
}

impl Item {
    fn as_number_mut(&mut self) -> Option<&mut u16> {
        if let Self::Number(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

fn load_input(input: &[u8]) -> Vec<Item> {
    input
        .iter()
        .filter_map(|ch| {
            Some(match ch {
                b'[' => Item::OpenPair,
                b']' => Item::ClosePair,
                b',' => return None,
                _ => Item::Number((ch - b'0').into()),
            })
        })
        .collect()
}

fn mag(n: &[Item]) -> u16 {
    let mut total = 0;
    let mut k = 1;
    for item in n {
        match item {
            Item::OpenPair => k *= 3,
            Item::ClosePair => k /= 2,
            Item::Number(n) => {
                total += k * n;
                k = (k / 3) * 2;
            }
        }
    }
    total
}

fn explode(items: &mut Vec<Item>) -> bool {
    let mut left = None;
    let mut depth = 0;

    for (idx, item) in items.iter().enumerate() {
        match item {
            Item::OpenPair => {
                if depth != 4 {
                    depth += 1;
                    continue;
                }

                let a = *items[idx + 1].as_number_mut().unwrap();
                let b = *items[idx + 2].as_number_mut().unwrap();

                if let Some(left) = left {
                    if let Some(left) = Item::as_number_mut(&mut items[left]) {
                        *left += a;
                    }
                }

                if let Some(right) = items.iter_mut().skip(idx + 4).find_map(Item::as_number_mut) {
                    *right += b;
                }

                items.splice(idx..idx + 4, [Item::Number(0)]);
                return true;
            }
            Item::ClosePair => depth -= 1,
            Item::Number(..) => left = Some(idx),
        }
    }

    false
}

fn split(items: &mut Vec<Item>) -> bool {
    for (idx, item) in items.iter().enumerate() {
        match item {
            Item::Number(n) => {
                let n = *n;
                if n < 10 {
                    continue;
                }

                let a = n / 2;
                let b = (n + 1) / 2;
                items.splice(
                    idx..=idx,
                    [
                        Item::OpenPair,
                        Item::Number(a),
                        Item::Number(b),
                        Item::ClosePair,
                    ],
                );

                return true;
            }

            Item::OpenPair | Item::ClosePair => {}
        }
    }
    false
}

fn reduce(n: &mut Vec<Item>) {
    while explode(n) || split(n) {}
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let terms = include_str!("input.txt")
        .trim()
        .lines()
        .map(|line| load_input(line.as_bytes()))
        .collect::<Vec<_>>();

    let p1 = {
        let mut terms = terms.iter();
        let head = terms.next().unwrap().clone();
        let total = terms.fold(head, |mut acc, current| {
            acc.insert(0, Item::OpenPair);
            acc.extend_from_slice(&current);
            acc.push(Item::ClosePair);
            reduce(&mut acc);
            acc
        });
        mag(&total)
    };

    let mut p2 = 0;
    let mut current = Vec::new();
    for a in &terms {
        for b in &terms {
            if a == b {
                continue;
            }
            current.clear();
            current.push(Item::OpenPair);
            current.extend_from_slice(&a);
            current.extend_from_slice(&b);
            current.push(Item::ClosePair);
            reduce(&mut current);
            p2 = p2.max(mag(&current));
        }
    }

    (p1, p2)
}
