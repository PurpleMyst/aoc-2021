use std::fmt::Display;

use nom::bits::complete::take;
use nom::combinator::{map, map_opt};
use nom::multi::fold_many0;
use nom::sequence::tuple;
use nom::InputLength;

use either::Either;
use nom_supreme::error::ErrorTree;

type IResult<I, O> = nom::IResult<I, O, ErrorTree<I>>;

#[derive(Debug, PartialEq, Eq)]
struct PacketHeader {
    version: u8,
    type_id: u8,
    length: Length,
}

#[derive(Debug, PartialEq, Eq)]
enum Length {
    InBits(u16),
    Subpackets(u16),
}

#[derive(Debug, PartialEq, Eq)]
struct LiteralPacket {
    version: u8,
    value: u64,
}

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Literal(LiteralPacket),
    Operator {
        header: PacketHeader,
        children: Vec<Packet>,
    },
}

fn packet_header(
    input: (&[u8], usize),
) -> IResult<(&[u8], usize), Either<PacketHeader, LiteralPacket>> {
    let (input, (version, type_id)) = tuple((take(3usize), take(3usize)))(input)?;

    if type_id == 4 {
        let (input, value) = fold_many0(
            map_opt(take(5usize), |value: u64| {
                if value & 0b10000 != 0 {
                    Some(value & !0b10000)
                } else {
                    None
                }
            }),
            || 0,
            |acc, b: u64| (acc << 4) | b,
        )(input)?;

        let (input, last): (_, u64) = take(5usize)(input)?;
        let value = (value << 4) | last;

        return Ok((input, Either::Right(LiteralPacket { version, value })));
    }

    let (input, length_type): (_, u8) = take(1usize)(input)?;

    let (input, length) = if length_type == 0 {
        map(take(15usize), Length::InBits)(input)?
    } else {
        map(take(11usize), Length::Subpackets)(input)?
    };

    Ok((
        input,
        Either::Left(PacketHeader {
            version,
            type_id,
            length,
        }),
    ))
}

fn packet(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    let (input, header) = match packet_header(input)? {
        (input, Either::Left(header)) => (input, header),
        (input, Either::Right(literal)) => return Ok((input, Packet::Literal(literal))),
    };

    let (input, children) = match header.length {
        Length::InBits(l) => {
            let mut children = Vec::new();
            let mut input = input;
            let goal = input.input_len() - usize::from(l);
            while input.input_len() != goal {
                let (next_input, child) = packet(input)?;
                input = next_input;
                children.push(child);
            }
            (input, children)
        }
        Length::Subpackets(l) => {
            let mut children = Vec::new();
            let mut input = input;
            for _ in 0..l {
                let (next_input, child) = packet(input)?;
                input = next_input;
                children.push(child);
            }
            (input, children)
        }
    };

    Ok((input, Packet::Operator { header, children }))
}

fn load_input(input: &str) -> Vec<u8> {
    input
        .trim()
        .as_bytes()
        .chunks_exact(2)
        .map(|c| u8::from_str_radix(std::str::from_utf8(&c).unwrap(), 16).unwrap())
        .collect()
}

fn sum_versions(packet: &Packet) -> usize {
    match packet {
        Packet::Literal(LiteralPacket { version, .. }) => usize::from(*version),
        Packet::Operator {
            header: PacketHeader { version, .. },
            children,
        } => usize::from(*version) + children.iter().map(sum_versions).sum::<usize>(),
    }
}

fn evaluate(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal(LiteralPacket { value, .. }) => u64::from(*value),
        Packet::Operator {
            header: PacketHeader { type_id, .. },
            children,
        } => match type_id {
            0 => children.iter().map(evaluate).sum(),
            1 => children.iter().map(evaluate).product(),
            2 => children.iter().map(evaluate).min().unwrap(),
            3 => children.iter().map(evaluate).max().unwrap(),
            5 => (evaluate(&children[0]) > evaluate(&children[1])) as u64,
            6 => (evaluate(&children[0]) < evaluate(&children[1])) as u64,
            7 => (evaluate(&children[0]) == evaluate(&children[1])) as u64,
            _ => unreachable!(),
        },
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = load_input(include_str!("input.txt"));

    let res: IResult<&[u8], _> = nom::bits::bits(packet)(&input[..]);
    let (_, packet) = res.unwrap();

    (sum_versions(&packet), evaluate(&packet))
}
