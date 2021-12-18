use std::fmt::Display;
use std::ops::ControlFlow;

use slab::Slab;

// TODO: utilize a tree so that we can mutate it directly

#[derive(Debug, Clone, PartialEq)]
enum Structure {
    Single(usize),
    Pair(Box<Structure>, Box<Structure>),
}

fn load_input<'a>(input: &'a [u8], slab: &mut Slab<u64>) -> (&'a [u8], Structure) {
    let (head, input) = input.split_first().unwrap();
    match head {
        b'[' => {
            let (input, fst) = load_input(input, slab);
            let (&ch, input) = input.split_first().unwrap();
            assert_eq!(ch, b',');
            let (input, snd) = load_input(input, slab);
            let (&ch, input) = input.split_first().unwrap();
            assert_eq!(ch, b']');
            (input, Structure::Pair(Box::new(fst), Box::new(snd)))
        }
        b']' => unreachable!(),
        b',' => unreachable!(),
        &n => (input, Structure::Single(slab.insert(u64::from(n - b'0')))),
    }
}

#[derive(Debug, Default)]
struct State {
    left: Option<usize>,
    carry: Option<usize>,
    level: usize,
}

fn mag(slab: &Slab<u64>, cake: &Structure) -> u64 {
    match cake {
        Structure::Single(n) => slab[*n],
        Structure::Pair(fst, snd) => 3 * mag(slab, fst) + 2 * mag(slab, snd),
    }
}

fn explode_step(
    slab: &mut Slab<u64>,
    cake: &mut Structure,
    state: State,
) -> ControlFlow<(), State> {
    match cake {
        Structure::Single(n) => {
            if let Some(m) = state.carry {
                slab[*n] += slab[m];
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(State {
                    left: Some(*n),
                    ..state
                })
            }
        }

        Structure::Pair(fst, snd) => {
            if state.carry.is_none() && state.level == 4 {
                if let Some(left) = state.left {
                    slab[left] += match &**fst {
                        Structure::Single(n) => slab[*n],
                        _ => unreachable!(),
                    }
                }
                let right = match &**snd {
                    Structure::Single(n) => *n,
                    _ => unreachable!(),
                };
                *cake = Structure::Single(slab.insert(0));
                ControlFlow::Continue(State {
                    carry: Some(right),
                    ..state
                })
            } else {
                let state = State {
                    level: state.level + 1,
                    ..state
                };
                let state = explode_step(slab, fst, state)?;
                let mut state = explode_step(slab, snd, state)?;
                state.level -= 1;
                ControlFlow::Continue(state)
            }
        }
    }
}

fn split_step(slab: &mut Slab<u64>, cake: &mut Structure) -> ControlFlow<()> {
    match cake {
        Structure::Single(n) => {
            let n = *n;
            if slab[n] >= 10 {
                *cake = Structure::Pair(
                    Box::new(Structure::Single(slab.insert(slab[n] / 2))),
                    Box::new(Structure::Single(slab.insert((slab[n] + 1) / 2))),
                );
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        }

        Structure::Pair(fst, snd) => {
            split_step(slab, fst)?;
            split_step(slab, snd)
        }
    }
}

fn reduce(slab: &mut Slab<u64>, cake: &mut Structure) {
    loop {
        if matches!(
            explode_step(slab, cake, State::default()),
            ControlFlow::Continue(..)
        ) && split_step(slab, cake) == ControlFlow::Continue(())
        {
            break;
        }
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut slab = Slab::new();
    let terms = include_str!("input.txt")
        .trim()
        .lines()
        .map(|line| load_input(line.as_bytes(), &mut slab).1)
        .collect::<Vec<_>>();
    let slab = slab;

    let p1 = {
        let mut slab = slab.clone();
        let mut terms = terms.iter().cloned();
        let head = terms.next().unwrap();
        let foo = terms.fold(head, |a, b| {
            let mut c = Structure::Pair(Box::new(a), Box::new(b));
            reduce(&mut slab, &mut c);
            c
        });
        mag(&slab, &foo)
    };

    let mut p2 = 0;
    for a in &terms {
        for b in &terms {
            if a == b {
                continue;
            }
            let mut slab = slab.clone();
            let mut c = Structure::Pair(Box::new(a.clone()), Box::new(b.clone()));
            reduce(&mut slab, &mut c);
            let m = mag(&slab, &c);
            p2 = p2.max(m);
        }
    }

    (p1, p2)
}
