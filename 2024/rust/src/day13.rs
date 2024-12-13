use aoc::{parsers::int, AOContext};
use nom::{bytes::complete::{is_not, tag}, multi::{many0_count, separated_list1}, sequence::tuple, Parser};
use nalgebra::{matrix, vector};

#[derive(Debug)]
struct Machine {
    a_button: (i64, i64),
    b_button: (i64, i64),
    target: (i64, i64),
}

fn parse(input: &str) -> Vec<Machine> {
    let line = |input| {
        tuple((
            many0_count(is_not("0123456789")),
            int::<i64>,
            many0_count(is_not("0123456789")),
            int::<i64>,
            many0_count(is_not("\n")),
            tag("\n"),
        )).map(|(_, x, _, y, _, _)| (x, y)).parse(input)
    };

    let machine = tuple((line, line, line))
        .map(|(a_button, b_button, target)| Machine { a_button, b_button, target });

    separated_list1(tag("\n"), machine)(input)
        .unwrap()
        .1
}

fn f64_is_integral(f: f64) -> bool {
    (f.round() - f).abs() < 0.001
}

fn solve(machines: &[Machine], offset: i64) -> i64 {
    machines.iter().filter_map(|m| {
        let a = matrix![m.a_button.0 as f64, m.b_button.0 as f64; m.a_button.1 as f64, m.b_button.1 as f64];
        let v = vector![(m.target.0 + offset) as f64, (m.target.1 + offset) as f64];
        let clicks = a.try_inverse().expect("Uninvertable matrix??????") * v;

        (f64_is_integral(clicks[0]) && f64_is_integral(clicks[1]))
            .then_some(clicks[0].round() as i64 * 3 + clicks[1].round() as i64)
    }).sum()
}

pub fn day13(input: String, ctx: &mut AOContext) {
    let machines = parse(&input);
    ctx.parsing_done();
    ctx.submit_part1(solve(&machines, 0));
    ctx.submit_part2(solve(&machines, 10000000000000));
}
