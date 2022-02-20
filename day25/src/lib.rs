use std::cmp::Ordering;
use std::fmt::Display;
use std::iter::Peekable;
use std::mem::{swap, take};

type Plane = Vec<Vec<usize>>;

fn occupied(pos1: usize, it: &mut Peekable<impl Iterator<Item = usize>>) -> bool {
    while let Some(pos2) = it.peek() {
        match pos2.cmp(&pos1) {
            Ordering::Less => {
                it.next();
            }
            Ordering::Equal => return true,
            Ordering::Greater => return false,
        }
    }

    false
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut east_herd = Plane::new();
    let mut south_herd = Plane::new();

    let mut max_x = 0;

    include_str!("input.txt").trim().lines().for_each(|line| {
        let mut east_row = Vec::new();
        let mut south_row = Vec::new();
        line.bytes().enumerate().for_each(|(x, cell)| match cell {
            b'>' => east_row.push(x),
            b'v' => south_row.push(x),
            _ => {
                max_x = max_x.max(x);
            }
        });
        east_herd.push(east_row);
        south_herd.push(south_row);
    });

    let mut cur_e_row_buf = Vec::new();
    let mut cur_s_row_buf = Vec::new();
    let mut next_s_row_buf = Vec::new();
    let mut next_south_herd: Plane = Vec::new();

    for i in 1.. {
        let mut moved = false;

        // For each row of cucumbers, we'll move the east ones first.
        for (e_row, s_row) in east_herd.iter_mut().zip(south_herd.iter()) {
            cur_e_row_buf.clear();

            let mut e_it = e_row.iter().copied().peekable();
            let mut s_it = s_row.iter().copied().peekable();

            // Check a priori if the first spot in the row is occupied.
            let first_occupied = (e_it.peek() == Some(&0)) || (s_it.peek() == Some(&0));

            // For each east-cucumber in this row...
            while let Some(x1) = e_it.next() {
                // Check if there's an east-cucumber after this one.
                if let Some(&x2) = e_it.peek() {
                    // If there is, check if the next east-cucumber is occupying the spot we wanna
                    // move to, otherwise check if there's a south-cucumber occupying the spot we
                    // want.
                    let can_move = (x1 + 1 != x2) && !occupied(x1 + 1, &mut s_it);
                    moved = moved || can_move;
                    // If we can move, do so, otherwise stay were we are.
                    cur_e_row_buf.push(if can_move { x1 + 1 } else { x1 });
                } else {
                    // If there's no more east-cucumbers after this one, we can be in one of two cases:
                    // * We're at the end of the row. If so, check if the first spot in the row is
                    //   occupied and move there if it isn't, otherwise stay where we are.
                    // * We're not at the end of the row and we're just the rightmost
                    //   east-cucumber. If so, check if there's no south-cucumber in the spot we
                    //   wanna move to and move if we can.
                    if x1 == max_x {
                        if first_occupied {
                            cur_e_row_buf.push(x1);
                        } else {
                            moved = true;
                            cur_e_row_buf.insert(0, 0);
                        }
                    } else {
                        cur_e_row_buf.push(if !occupied(x1 + 1, &mut s_it) {
                            moved = true;
                            x1 + 1
                        } else {
                            x1
                        });
                    }
                }
            }

            e_row.clone_from(&cur_e_row_buf);
        }

        let mut s_herd_it = south_herd.iter().peekable();
        let mut e_herd_it = east_herd.iter().skip(1);

        // For each row of south-cucumbers...
        while let Some(cur_s_row) = s_herd_it.next() {
            // Get the east and south rows that we'll be falling onto.
            let mut next_e_row = e_herd_it
                .next()
                .unwrap_or(&east_herd[0])
                .iter()
                .copied()
                .peekable();
            let mut next_s_row = s_herd_it
                .peek()
                .unwrap_or(&&south_herd[0])
                .iter()
                .copied()
                .peekable();

            // For each south-cucumber, we can reutilize the `occupied` function to compute if we
            // can move into the spot we want to.
            for &x in cur_s_row {
                let can_move = !occupied(x, &mut next_e_row) && !occupied(x, &mut next_s_row);
                if can_move {
                    moved = true;
                    next_s_row_buf.push(x);
                } else {
                    cur_s_row_buf.push(x);
                }
            }

            // Seeing as we'll be inserting stuff in two separate iterations, it'll be wise to sort before inserting into the next herd.
            cur_s_row_buf.sort_unstable();
            next_south_herd.push(take(&mut cur_s_row_buf));
            swap(&mut cur_s_row_buf, &mut next_s_row_buf);
        }

        // Once all iterations are over, our `cur_s_row_buf` will contain the south-cucumbers that
        // go off the screen like pacman.
        next_south_herd[0].append(&mut cur_s_row_buf);
        next_south_herd[0].sort_unstable();

        south_herd.clone_from(&next_south_herd);
        next_south_herd.clear();

        if !moved {
            return (i, "Merry Christmas!");
        }
    }

    unreachable!();
}
