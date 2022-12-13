use std::cmp::Ordering;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Number(u8),
    List(Vec<Packet>),
}

#[derive(Debug, PartialEq, Eq)]
enum Token {
    OpenSquare,
    CloseSquare,
    Comma,
    Number(u8),
}

fn parse_tokens(input: &str) -> Vec<Token> {
    let mut res = Vec::new();
    let mut num = String::new();

    for c in input.chars() {
        if c.is_numeric() {
            num.push(c);
        } else {
            if !num.is_empty() {
                res.push(Token::Number(num.parse().unwrap()));
                num = String::new();
            }

            if c == '[' {
                res.push(Token::OpenSquare);
            } else if c == ']' {
                res.push(Token::CloseSquare);
            } else if c == ',' {
                res.push(Token::Comma);
            }
        }
    }

    res
}

fn parse_number(tokens: &[Token], current: usize) -> Option<(Packet, usize)> {
    match tokens[current] {
        Token::Number(n) => Some((Packet::Number(n), current + 1)),
        _ => None,
    }
}

fn parse_item(tokens: &[Token], current: usize) -> Option<(Packet, usize)> {
    parse_number(tokens, current)
        .or_else(|| parse_blist(tokens, current))
}

fn parse_blist(tokens: &[Token], mut current: usize) -> Option<(Packet, usize)> {
    if tokens[current] == Token::OpenSquare {
        current += 1;

        if tokens[current] == Token::CloseSquare {
            Some((Packet::List(vec![]), current + 1))
        } else {
            let (first, next) = parse_item(tokens, current)?;
            current = next;

            let mut list = vec![first];

            while tokens[current] != Token::CloseSquare {
                if tokens[current] == Token::Comma {
                    current += 1;
                    let (item, next) = parse_item(tokens, current)?;
                    current = next;
                    list.push(item);
                } else {
                    return None
                }
            }

            Some((Packet::List(list), current + 1))
        }
    } else {
        None
    }
}

fn parse_packet(input: &str) -> Packet {
    let tokens = parse_tokens(input);
    parse_blist(&tokens, 0).unwrap().0
}

fn parse_input(input: &str) -> Vec<Packet> {
    input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_packet(line))
        .collect()
}

fn ordering(p1: &Packet, p2: &Packet) -> Ordering {
    match (p1, p2) {
        (Packet::Number(n1), Packet::Number(n2)) => n1.cmp(&n2),
        (Packet::List(l1), Packet::List(l2)) => {
            for (p1, p2) in l1.iter().zip(l2.iter()) {
                let ord = ordering(p1, p2);
                if ord.is_ne() {
                    return ord; 
                }
            }

            l1.len().cmp(&l2.len())
        }
        (Packet::Number(n), l) => ordering(&Packet::List(vec![Packet::Number(*n)]), l),
        (l, Packet::Number(n)) => ordering(l, &Packet::List(vec![Packet::Number(*n)])),
    }
}

pub fn day_thirteen(input: String) {
    let mut packets = parse_input(&input);
    println!("{}", part_one(&packets));
    println!("{}", part_two(&mut packets))
}

fn part_one(packets: &[Packet]) -> usize {
    packets.chunks_exact(2)
        .enumerate()
        .filter(|&(_, p)| ordering(&p[0], &p[1]).is_lt())
        .map(|(i, _)| i + 1)
        .sum()
}

fn part_two(packets: &mut Vec<Packet>) -> usize {
    let first = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    let last = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);
    packets.push(first.clone());
    packets.push(last.clone());

    packets.sort_by(|p1, p2| ordering(p1, p2));

    let (first, _) = packets.iter()
        .find_position(|p| **p == first).unwrap();

    let (last, _) = packets.iter()
        .find_position(|p| **p == last).unwrap();

    (first + 1) * (last + 1)
}
