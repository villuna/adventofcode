use ahash::HashSet;
use nom::{
    bytes::complete::tag, combinator::map, multi::separated_list1, sequence::separated_pair,
};
use std::cmp::Ordering;

use crate::{parsers::int, AOContext};

type Orders = HashSet<(i32, i32)>;
type Updates = Vec<Vec<i32>>;

fn parse(input: &str) -> (Orders, Updates) {
    separated_pair(
        map(
            separated_list1(tag("\n"), separated_pair(int, tag("|"), int)),
            |l| l.into_iter().collect(),
        ),
        tag("\n\n"),
        separated_list1(tag("\n"), separated_list1(tag(","), int)),
    )(input)
    .unwrap()
    .1
}

fn ordering(orders: &Orders, lhs: i32, rhs: i32) -> Ordering {
    if orders.contains(&(lhs, rhs)) {
        Ordering::Less
    } else if orders.contains(&(rhs, lhs)) {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

// Returns true if the entire update is ordered correctly
fn correct_order(update: &[i32], orders: &Orders) -> bool {
    update.is_sorted_by(|&lhs, &rhs| ordering(orders, lhs, rhs).is_lt())
}

pub fn day5(input: String, ctx: &mut AOContext) {
    let (orders, updates) = parse(&input);

    ctx.parsing_done();

    let part1: i32 = updates
        .iter()
        .filter_map(|update| correct_order(update, &orders).then_some(update[update.len() / 2]))
        .sum();

    ctx.submit_part1(part1);

    let part2: i32 = updates
        .into_iter()
        .filter_map(|mut update| {
            if !correct_order(&update, &orders) {
                update.sort_by(|&lhs, &rhs| ordering(&orders, lhs, rhs));
                Some(update[update.len() / 2])
            } else {
                None
            }
        })
        .sum();

    ctx.submit_part2(part2);
}
