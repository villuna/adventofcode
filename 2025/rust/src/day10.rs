use std::{
    collections::{HashSet, VecDeque},
    convert::identity,
};

use aoc::AOContext;
use aoc::parsers::int;
use good_lp::{
    Expression, ProblemVariables, Solution, SolverModel, constraint, solvers::scip::scip, variable,
};
use itertools::Itertools;
use nom::{
    Finish, IResult, Parser,
    bytes::tag,
    character::streaming::{one_of, space0},
    multi::{many1, separated_list1},
    sequence::{delimited, terminated},
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug)]
struct Entry {
    init: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u32>,
}

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    (
        delimited(tag("["), many1(one_of("#.").map(|c| c == '#')), tag("]")),
        space0,
        many1(terminated(
            delimited(tag("("), separated_list1(tag(","), int::<usize>), tag(")")),
            space0,
        )),
        space0,
        delimited(tag("{"), separated_list1(tag(","), int::<u32>), tag("}")),
    )
        .map(|(init, _, buttons, _, joltages)| Entry {
            init,
            buttons,
            joltages,
        })
        .parse(input)
}

fn part1(entry: &Entry) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((entry.init.clone(), 0));

    while let Some((lights, steps)) = queue.pop_front() {
        visited.insert(lights.clone());
        if lights.iter().cloned().any(identity) {
            for button in &entry.buttons {
                let mut next = lights.clone();
                for &i in button {
                    next[i] ^= true;
                }
                if !visited.contains(&next) {
                    queue.push_back((next, steps + 1));
                }
            }
        } else {
            return steps;
        }
    }

    panic!()
}

fn part2(entry: &Entry) -> usize {
    let mut variables = ProblemVariables::new();
    let mut total_clicks = Expression::from(0);

    let button_clicks = entry
        .buttons
        .iter()
        .map(|_| {
            let button = variables.add(variable().integer().min(0));
            total_clicks += button;
            button
        })
        .collect_vec();

    let constraints = entry
        .joltages
        .iter()
        .enumerate()
        .map(|(i, &target_joltage)| {
            let mut joltage = Expression::from(0);
            for bi in 0..entry.buttons.len() {
                if entry.buttons[bi].contains(&i) {
                    joltage += button_clicks[bi];
                }
            }
            constraint! {joltage == target_joltage}
        });

    let solution = variables
        .minimise(total_clicks)
        .using(scip)
        .with_all(constraints)
        .solve()
        .unwrap();

    button_clicks
        .iter()
        .map(|v| solution.value(*v))
        .sum::<f64>() as usize
}

pub fn day10(input: String, ctx: &mut AOContext) {
    let entries = input
        .lines()
        .map(|l| parse_entry(l).finish().unwrap().1)
        .collect_vec();

    ctx.parsing_done();
    ctx.submit_part1(entries.par_iter().map(part1).sum::<usize>());
    ctx.submit_part2(entries.par_iter().map(part2).sum::<usize>());
}
