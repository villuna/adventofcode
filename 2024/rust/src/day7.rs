use aoc::{parsers::int, AOContext};
use nom::{bytes::complete::tag, multi::separated_list0, sequence::separated_pair};

fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    separated_list0(
        tag("\n"),
        separated_pair(int::<u64>, tag(": "), separated_list0(tag(" "), int::<u64>)),
    )(input)
    .unwrap()
    .1
}

fn digits_in(int: u64) -> u32 {
    if int == 0 {
        1
    } else {
        int.ilog10() + 1
    }
}

fn unconcat(whole: u64, suffix: u64) -> Option<u64> {
    let suffix_digits = digits_in(suffix);
    (suffix == whole % 10u64.pow(suffix_digits)).then_some(whole / 10u64.pow(suffix_digits))
}

fn is_buildable(target: u64, nums: &[u64], undo_ops: &[fn(u64, u64) -> Option<u64>]) -> bool {
    if nums.len() == 1 {
        nums[0] == target
    } else {
        let n = nums[nums.len() - 1];
        if n > target {
            return false;
        }
        let rest = &nums[..nums.len() - 1];
        undo_ops
            .iter()
            .any(|o| o(target, n).is_some_and(|nt| is_buildable(nt, rest, undo_ops)))
    }
}

fn solve(lines: &[(u64, Vec<u64>)], undo_ops: &[fn(u64, u64) -> Option<u64>]) -> u64 {
    lines
        .iter()
        .filter_map(|(val, nums)| is_buildable(*val, nums, undo_ops).then_some(*val))
        .sum()
}

pub fn day7(input: String, ctx: &mut AOContext) {
    let lines = parse(&input);

    ctx.parsing_done();

    let add = |target, n| Some(target - n);
    let sub = |target, n| (target % n == 0).then_some(target / n);

    ctx.submit_part1(solve(&lines, &[add, sub]));
    ctx.submit_part2(solve(&lines, &[add, sub, unconcat]));
}
