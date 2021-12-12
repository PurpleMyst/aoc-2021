use std::fmt::Display;

use ahash::AHashMap;

type Graph = AHashMap<Cave, Vec<Cave>>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Cave {
    Start,
    End,
    Small(u16),
    Big(u16),
}

impl Cave {
    fn parse(s: &str) -> Self {
        match s.as_bytes() {
            b"start" => Self::Start,
            b"end" => Self::End,

            &[hi, lo] => {
                if hi.is_ascii_uppercase() {
                    debug_assert!(lo.is_ascii_uppercase());
                    Self::Big((hi as u16 - b'A' as u16) * 26 + (lo as u16 - b'A' as u16))
                } else {
                    Self::Small((hi as u16 - b'a' as u16) * 26 + (lo as u16 - b'a' as u16))
                }
            }

            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
struct P1Visited {
    visited: [bool; 26 * 26],
}

impl P1Visited {
    fn new() -> Self {
        Self {
            visited: [false; 26 * 26],
        }
    }

    fn push(&mut self, cave: u16) -> bool {
        std::mem::replace(&mut self.visited[usize::from(cave)], true)
    }

    fn pop(&mut self, cave: u16) {
        debug_assert!(self.visited[usize::from(cave)]);
        self.visited[usize::from(cave)] = false;
    }
}

#[derive(Clone)]
struct P2Visited {
    single_visit: P1Visited,
    double_visit: Option<u16>,
}

impl P2Visited {
    fn new() -> Self {
        Self {
            single_visit: P1Visited::new(),
            double_visit: None,
        }
    }

    fn push(&mut self, cave: u16) -> bool {
        if self.single_visit.push(cave) {
            if self.double_visit.is_some() {
                return true;
            }

            self.double_visit = Some(cave);
        }

        false
    }

    fn pop(&mut self, cave: u16) {
        if self.double_visit == Some(cave) {
            self.double_visit = None;
        } else {
            self.single_visit.pop(cave);
        }
    }
}

// since we only care about if nodes have been visited or not, we can use a DFS and handle the
// visited map like a stack
fn p1_walk(graph: &Graph, visited: &mut P1Visited, current: Cave, counter: &mut usize) {
    if current == Cave::End {
        *counter += 1;
        return;
    }

    graph[&current]
        .iter()
        .copied()
        .filter(|&cave| cave != Cave::Start)
        .for_each(|next| {
            if let Cave::Small(s) = next {
                if visited.push(s) {
                    return;
                }
            }
            p1_walk(graph, visited, next, counter);
            if let Cave::Small(s) = next {
                visited.pop(s);
            }
        })
}

fn p2_walk(graph: &Graph, visited: &mut P2Visited, current: Cave, counter: &mut usize) {
    if current == Cave::End {
        *counter += 1;
        return;
    }

    graph[&current]
        .iter()
        .copied()
        .filter(|&cave| cave != Cave::Start)
        .for_each(|next| {
            if let Cave::Small(s) = next {
                if visited.push(s) {
                    return;
                }
            }
            p2_walk(graph, visited, next, counter);
            if let Cave::Small(s) = next {
                visited.pop(s);
            }
        })
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut graph: Graph = Default::default();

    include_str!("input.txt").trim().lines().for_each(|line| {
        let (from, to) = line.split_once('-').unwrap();
        let from = Cave::parse(from);
        let to = Cave::parse(to);
        graph.entry(from).or_default().push(to);
        graph.entry(to).or_default().push(from);
    });

    let mut p1 = 0;
    p1_walk(&graph, &mut P1Visited::new(), Cave::Start, &mut p1);

    let mut p2 = 0;
    p2_walk(&graph, &mut P2Visited::new(), Cave::Start, &mut p2);

    (p1, p2)
}
