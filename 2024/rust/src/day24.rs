use ahash::HashMap;
use aoc::{parsers::strip, AOContext};
use itertools::Itertools;
use nom::Parser;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::one_of,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Eq, PartialEq, Debug)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Eq, Debug)]
struct Production<'a>(&'a str, Op, &'a str);

impl<'a> PartialEq for Production<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1 && ((self.0 == other.0 && self.2 == other.2) || (self.0 == other.2 && self.2 == other.0))
    }
}

struct Input<'a> {
    productions: HashMap<&'a str, Production<'a>>,
    atoms: HashMap<&'a str, bool>,
}

fn parse(input: &str) -> IResult<&str, Input> {
    let op = strip(alt((
        tag("AND").map(|_| Op::And),
        tag("OR").map(|_| Op::Or),
        tag("XOR").map(|_| Op::Xor),
    )));
    let production = tuple((is_not(" "), op, is_not(" "), tag(" -> "), is_not("\n")))
        .map(|(a, op, b, _, res)| (res, Production(a, op, b)));

    let atom = tuple((is_not(":"), tag(": "), one_of("01"))).map(|(r, _, v)| (r, v == '1'));

    let mut parser = tuple((
        separated_list1(tag("\n"), atom),
        tag("\n\n"),
        separated_list1(tag("\n"), production),
    ))
    .map(|(atoms, _, productions)| Input {
        atoms: HashMap::from_iter(atoms),
        productions: HashMap::from_iter(productions),
    });

    parser.parse(input)
}

fn calculate<'a>(rules: &Input<'a>, wire: &'a str) -> bool {
    if rules.atoms.contains_key(wire) {
        *rules.atoms.get(wire).unwrap()
    } else {
        let Production(a, op, b) = rules.productions.get(wire).unwrap();
        let a = calculate(rules, a);
        let b = calculate(rules, b);

        match op {
            Op::And => a && b,
            Op::Or => a || b,
            Op::Xor => a ^ b,
        }
    }
}

pub fn day24(input: String, ctx: &mut AOContext) {
    let rules = parse(&input).unwrap().1;
    
    ctx.parsing_done();

    let mut bits = rules
        .productions
        .keys()
        .cloned()
        .filter(|k| k.starts_with("z"))
        .collect_vec();

    bits.sort_by_key(|k| k.strip_prefix("z").unwrap().parse::<u8>().unwrap());

    let part1 = bits.iter().rev().map(|bit| {
        if calculate(&rules, bit) { '1' } else { '0' }
    }).collect::<String>();

    ctx.submit_part1(usize::from_str_radix(&part1, 2).unwrap());
    ctx.submit_part2("Sowwy I did it by hand :3");
}
