use std::fmt::Display;

const SIDE: usize = 5;
type Board = [u8; SIDE * SIDE];
type Drawn = [bool; 100];

fn has_won(drawn: &Drawn, board: &Board) -> bool {
    [
        (0..SIDE).any(|y| {
            board[y * SIDE..(y + 1) * SIDE]
                .into_iter()
                .all(|&cell| drawn[usize::from(cell)])
        }),
        (0..SIDE).any(|x| {
            (0..SIDE)
                .map(|y| board[y * SIDE + x])
                .all(|cell| drawn[usize::from(cell)])
        }),
    ]
    .into_iter()
    .any(|b| b)
}

fn partial_score(drawn: &Drawn, board: &Board) -> u32 {
    board
        .iter()
        .filter(|&&n| !drawn[usize::from(n)])
        .map(|&n| u32::from(n))
        .sum()
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut input = include_str!("input.txt").trim().lines();

    let numbers = input
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u8>().unwrap());

    let mut boards = Vec::new();
    while let Some(..) = input.next() {
        let mut board = Board::default();

        input
            .by_ref()
            .take(5)
            .flat_map(|line| {
                line.split_ascii_whitespace()
                    .map(|n| n.parse::<u8>().unwrap())
            })
            .zip(board.iter_mut())
            .for_each(|(value, elem)| *elem = value);

        boards.push(board);
    }

    let mut drawn = [false; 100];

    let mut p1 = None;
    let mut p2 = None;

    for n in numbers {
        drawn[usize::from(n)] = true;

        boards
            .iter()
            .filter(|board| has_won(&drawn, board))
            .for_each(|winner| {
                let score = partial_score(&drawn, winner) * u32::from(n);
                if p1.is_none() {
                    p1 = Some(score);
                }
                p2 = Some(score);
            });
        boards.retain(|board| !has_won(&drawn, board));
    }

    (p1.unwrap(), p2.unwrap())
}
