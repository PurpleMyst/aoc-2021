use std::{cmp::Ordering, fmt::Display};

const ITEM_LEN: usize = 12;
type Item = u32;

fn calculate_frequency(numbers: &[Item], pos: u32) -> (usize, usize) {
    let mut zeros = 0;
    let mut ones = 0;
    let mask = 1 << pos;
    numbers.iter().for_each(|n| {
        if n & mask == mask {
            ones += 1;
        } else {
            zeros += 1;
        }
    });

    (zeros, ones)
}

fn pass(numbers: &mut Vec<Item>, pos: usize, choose: impl Fn(usize, usize) -> u32) {
    let (z, o) = calculate_frequency(numbers, pos as u32);
    let desired = choose(z, o) << pos;
    numbers.retain(|n| n & (1 << pos) == desired);
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut zeros = [0u16; ITEM_LEN];
    let mut ones = [0u16; ITEM_LEN];

    let numbers: Vec<Item> = include_str!("input.txt")
        .trim()
        .lines()
        .map(|line| {
            line.bytes()
                .zip(zeros.iter_mut().zip(ones.iter_mut()))
                .for_each(|(ch, (z, o))| if ch != b'1' { *z += 1 } else { *o += 1 });

            line.bytes()
                .fold(0, |acc, ch| (acc << 1) | Item::from(ch - b'0'))
        })
        .collect();

    let mut gamma = 0;
    let mut epsilon = 0;

    zeros
        .into_iter()
        .zip(ones.into_iter())
        .for_each(|(z, o)| match z.cmp(&o) {
            Ordering::Less => {
                gamma = gamma << 1 | 1;
                epsilon <<= 1;
            }

            Ordering::Equal | Ordering::Greater => {
                gamma <<= 1;
                epsilon = epsilon << 1 | 1;
            }
        });

    let mut oxy_numbers = numbers.clone();
    let mut co2_numbers = numbers;
    for i in (0..ITEM_LEN).rev() {
        let do_oxy = oxy_numbers.len() != 1;
        let do_co2 = co2_numbers.len() != 1;
        if do_oxy {
            pass(&mut oxy_numbers, i, |z, o| if z <= o { 1 } else { 0 });
        }
        if do_co2 {
            pass(&mut co2_numbers, i, |z, o| if z <= o { 0 } else { 1 });
        }
        if !do_oxy && !do_co2 {
            break;
        }
    }
    debug_assert!(oxy_numbers.len() == 1 && co2_numbers.len() == 1);

    (
        gamma * epsilon,
        oxy_numbers.pop().unwrap() * co2_numbers.pop().unwrap(),
    )
}
