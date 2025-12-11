use aoc::AOContext;
use itertools::Itertools;

fn apply_op(op: &str, a: u64, b: u64) -> u64 {
    match op {
        "+" => a + b,
        "*" => a * b,
        _ => unreachable!(),
    }
}

pub fn day6(input: String, ctx: &mut AOContext) {
    let mut rows = Vec::new();
    let mut ops = Vec::new();

    for line in input.trim().lines() {
        if line.chars().next().unwrap().is_ascii_digit() {
            let row = line
                .split_ascii_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect_vec();
            rows.push(row);
        } else {
            ops = line.split_ascii_whitespace().collect_vec();
        }
    }

    let p1 = ops
        .iter()
        .enumerate()
        .map(|(i, op)| {
            rows.iter()
                .map(|row| row[i])
                .reduce(|acc, x| apply_op(op, acc, x))
                .unwrap()
        })
        .sum::<u64>();

    ctx.submit_part1(p1);

    // Transpose the input
    let chars = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let columns = (0..chars[0].len())
        .map(|i| {
            (0..chars.len())
                .map(|idx| chars[idx][i])
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
        })
        .collect_vec();

    let p2 = columns
        .split(|s| s.is_empty())
        .enumerate()
        .map(|(i, set)| {
            set.iter()
                .map(|n| n.parse::<u64>().unwrap())
                .reduce(|acc, x| apply_op(ops[i], acc, x))
                .unwrap()
        })
        .sum::<u64>();

    ctx.submit_part2(p2);
}
