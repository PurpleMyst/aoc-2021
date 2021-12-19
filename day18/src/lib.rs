use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Item {
    LeftBracket,
    RightBracket,
    Number(u64),
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::LeftBracket => write!(f, "[ "),
            Item::RightBracket => write!(f, "] "),
            Item::Number(n) => write!(f, "{} ", n),
        }
    }
}

impl Item {
    fn as_number_mut(&mut self) -> Option<&mut u64> {
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
        .filter_map(|b| {
            Some(match b {
                b'[' => Item::LeftBracket,
                b']' => Item::RightBracket,
                b',' => return None,
                _ => Item::Number((b - b'0').into()),
            })
        })
        .collect()
}

fn mag(n: &[Item]) -> u64 {
    let mut stack: Vec<(Option<u64>, Option<u64>)> = Vec::new();

    for item in n {
        match item {
            Item::LeftBracket => stack.push((None, None)),
            Item::RightBracket => {
                let (a, b) = stack.pop().unwrap();
                let m = 3 * a.unwrap() + 2 * b.unwrap();
                match stack.last_mut() {
                    Some((a @ None, _)) => *a = Some(m),
                    Some((_, b @ None)) => *b = Some(m),
                    Some((_, _)) => unreachable!(),
                    None => return m,
                }
            }
            Item::Number(m) => match stack.last_mut().unwrap() {
                (a @ None, _) => *a = Some(*m),
                (_, b @ None) => *b = Some(*m),
                (_, _) => unreachable!(),
            },
        }
    }

    unreachable!();
}

fn explode(n: &mut Vec<Item>) -> bool {
    let mut left = None;
    let mut depth = 0;

    for (idx, item) in n.iter().enumerate() {
        match item {
            Item::LeftBracket => {
                if depth != 4 {
                    depth += 1;
                    continue;
                }

                let a = *n[idx+1].as_number_mut().unwrap();
                let b = *n[idx+2].as_number_mut().unwrap();

                if let Some(left) = left {
                    if let Some(left) = Item::as_number_mut(&mut n[left]) {
                        *left += a;
                    }
                }

                if let Some(right) = n.iter_mut().skip(idx).find_map(Item::as_number_mut) {
                    *right += b;
                }

                n.splice(idx..idx+4, [Item::Number(0)]);
                return true;
            }
            Item::RightBracket => depth -= 1,
            Item::Number(..) => left = Some(idx),
        }
    }

    false
}

fn split(n: &mut Vec<Item>) -> bool {
    for (idx, item) in n.iter().enumerate() {
        match item {
            Item::Number(m) => {
                let m = *m;
                if m < 10 {
                    continue;
                }

                let a = m / 2;
                let b = (m + 1) / 2;
                n.splice(idx..=idx, [
                    Item::LeftBracket,
                    Item::Number(a),
                    Item::Number(b),
                    Item::RightBracket,
                ]);

                return true;
            }

            Item::LeftBracket | Item::RightBracket => {}
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
        let mut terms = terms.iter().cloned();
        let head = terms.next().unwrap();
        let foo = terms.fold(head, |a, b| {
            let mut c = vec![Item::LeftBracket];
            c.extend(a);
            c.extend(b);
            c.push(Item::RightBracket);
            reduce(&mut c);
            c
        });
        mag(&foo)
    };

    let mut p2 = 0;
    for a in &terms {
        for b in &terms {
            if a == b {
                continue;
            }
            let mut c = vec![Item::LeftBracket];
            c.extend(a);
            c.extend(b);
            c.push(Item::RightBracket);
            reduce(&mut c);
            let m = mag(&c);
            p2 = p2.max(m);
        }
    }

    (p1, p2)
}
