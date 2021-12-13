use std::fmt::Display;

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
                    Self::Big(hi as u16 - b'A' as u16)
                } else {
                    Self::Small(hi as u16 - b'a' as u16)
                }
            }

            _ => unreachable!(),
        }
    }

    fn id(&self) -> Option<u16> {
        match self {
            Cave::Small(id) | Cave::Big(id) => Some(*id),
            _ => None,
        }
    }
}

#[derive(Clone)]
struct P1Visited {
    visited: [bool; 26],
}

impl P1Visited {
    fn new() -> Self {
        Self {
            visited: [false; 26],
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
    let neighbors = match current {
        Cave::Start => unreachable!(),
        Cave::End => {
            *counter += 1;
            return;
        }
        Cave::Small(current) | Cave::Big(current) => &graph.0[usize::from(current)],
    };

    neighbors
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
    let neighbors = match current {
        Cave::Start => unreachable!(),
        Cave::End => {
            *counter += 1;
            return;
        }
        Cave::Small(current) | Cave::Big(current) => &graph.0[usize::from(current)],
    };

    neighbors
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

struct Graph([Vec<Cave>; 26]);

impl Graph {
    fn new() -> Self {
        Self(array_init::array_init(|_| Vec::new()))
    }

    fn edge(&mut self, from: Cave, to: Cave) {
        if let Some(from) = from.id() {
            self.0[usize::from(from)].push(to);
        }
        if let Some(to) = to.id() {
            self.0[usize::from(to)].push(from);
        }
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut start_neighbors = Vec::new();
    let mut graph = Graph::new();

    include_str!("input.txt").trim().lines().for_each(|line| {
        let (from, to) = line.split_once('-').unwrap();
        let from = Cave::parse(from);
        let to = Cave::parse(to);
        match (from, to) {
            (Cave::Start, to) | (to, Cave::Start) => start_neighbors.push(to),
            (from, to) => graph.edge(from, to),
        }
    });

    let mut p1 = 0;
    let mut p1_visited = P1Visited::new();
    for &cave in &start_neighbors {
        p1_visited.push(cave.id().unwrap());
        p1_walk(&graph, &mut p1_visited, cave, &mut p1);
        p1_visited.pop(cave.id().unwrap());
    }

    let mut p2 = 0;
    let mut p2_visited = P2Visited::new();
    for &cave in &start_neighbors {
        p2_visited.push(cave.id().unwrap());
        p2_walk(&graph, &mut p2_visited, cave, &mut p2);
        p2_visited.pop(cave.id().unwrap());
    }

    (p1, p2)
}
