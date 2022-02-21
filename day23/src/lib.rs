use std::{fmt::Display, ops::RangeInclusive};

use arrayvec::ArrayVec;

const ENTRANCES: [usize; 4] = [2, 4, 6, 8];
const PARKING: [usize; 11 - 4] = [0, 1, 3, 5, 7, 9, 10];

const PART2_SURPRISE: [[usize; 2]; 4] = [[3, 3], [2, 1], [1, 0], [0, 2]];

#[derive(Debug, Hash, PartialEq, Eq, Clone, Default)]
struct State<const N: usize> {
    hallway: [Option<usize>; 11],
    rooms: [ArrayVec<usize, N>; 4],
}

fn steps_it(a: usize, b: usize) -> RangeInclusive<usize> {
    if a < b {
        a..=b
    } else {
        b..=a
    }
}

impl<const N: usize> State<N> {
    fn advance(&self) -> Vec<(Self, usize)> {
        let mut next_states = Vec::new();

        // Amphipods have two macro-actions:
        // * Leave their current room and choose a spot in the hallway to occupy.
        // * Leave the hallway and enter their home if conditions are met.
        // An amphipod may leave their home room if a foreign amphipod has to leave.
        'homeloop: for (x, &pod) in self.hallway.iter().enumerate() {
            let pod = match pod {
                Some(pod) => pod,
                None => continue,
            };

            let all_friends = self.rooms[pod].iter().all(|&occupant| occupant == pod);
            if self.rooms[pod].len() == N {
                continue;
            }

            let multiplier = 10usize.pow(pod as _);

            let mut steps = (N - 1) - self.rooms[pod].len();
            for k in steps_it(x, ENTRANCES[pod]) {
                if k == x || self.hallway[k].is_none() {
                    steps += 1;
                } else {
                    continue 'homeloop;
                }
            }

            if all_friends {
                let mut next_state = self.clone();
                next_state.rooms[pod].push(pod);
                next_state.hallway[x] = None;
                next_states.push((next_state, multiplier * steps));
            }
        }

        for (native_pod, room) in self.rooms.iter().enumerate() {
            let mut next_room = room.clone();

            let base_steps = N - next_room.len();

            let moving_pod = match next_room.pop() {
                Some(v) => v,
                None => continue,
            };

            let multiplier: usize = 10usize.pow(moving_pod as _);

            'parkloop: for x in PARKING {
                if self.hallway[x].is_some() {
                    continue;
                }

                let mut steps = base_steps;
                for k in steps_it(ENTRANCES[native_pod], x) {
                    if self.hallway[k].is_none() {
                        steps += 1;
                    } else {
                        continue 'parkloop;
                    }
                }

                let mut next_state = self.clone();
                next_state.hallway[x] = Some(moving_pod);
                next_state.rooms[native_pod] = next_room.clone();
                next_states.push((next_state, multiplier * steps));
            }
        }

        next_states
    }

    fn homed(&self) -> usize {
        let mut result = 0;
        for (native_pod, room) in self.rooms.iter().enumerate() {
            result += room
                .iter()
                .take_while(|&&occupant| occupant == native_pod)
                .count();
        }
        result
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut part1_state = State::<2>::default();
    let mut part2_state = State::<4>::default();
    include_str!("input.txt")
        .lines()
        .skip(2)
        .take(2)
        .for_each(|line| {
            line.bytes().enumerate().for_each(|(x, cell)| {
                let pod = match cell {
                    b'A' => 0,
                    b'B' => 1,
                    b'C' => 2,
                    b'D' => 3,
                    _ => return,
                };
                part1_state.rooms[(x - 2) / 2].push(pod);
                part2_state.rooms[(x - 2) / 2].push(pod);
            });
        });
    part1_state.rooms.iter_mut().for_each(|room| room.reverse());
    part2_state
        .rooms
        .iter_mut()
        .zip(PART2_SURPRISE)
        .for_each(|(room, [surprise1, surprise2])| {
            room.reverse();
            room.insert(1, surprise1);
            room.insert(1, surprise2);
        });

    let (_, part1) = pathfinding::prelude::dijkstra(
        &part1_state,
        |state| state.advance(),
        |state| state.homed() == 4 * 2,
    )
    .unwrap();

    let (_, part2) = pathfinding::prelude::dijkstra(
        &part2_state,
        |state| state.advance(),
        |state| state.homed() == 4 * 4,
    )
    .unwrap();

    (part1, part2)
}
