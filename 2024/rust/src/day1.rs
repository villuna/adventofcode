use crate::AOContext;
use itertools::Itertools;

fn count(x: i32, list: &[i32]) -> i32 {
    list.iter().filter(|y| **y == x).count() as i32
}

pub fn day1(input: String, ctx: &mut AOContext) {
    let (mut l1, mut l2): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect_tuple().unwrap())
        .unzip();

    ctx.parsing_done();

    l1.sort();
    l2.sort();

    let part1: u32 = l1.iter().zip(l2.iter()).map(|(a, b)| a.abs_diff(*b)).sum();
    ctx.submit_part1(part1);

    let part2: i32 = l1.into_iter().map(|n| count(n, &l2) * n).sum();
    ctx.submit_part2(part2);
}
