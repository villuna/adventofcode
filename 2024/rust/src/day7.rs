use aoc::{parsers::int, AOContext};
use nom::{bytes::complete::tag, multi::separated_list0, sequence::separated_pair};

fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    separated_list0(
        tag("\n"),
        separated_pair(int::<u64>, tag(": "), separated_list0(tag(" "), int::<u64>))
    )(input).unwrap().1
}

fn possible_values(target: u64, nums: &[u64], concat: bool) -> Vec<u64> {
    if nums.len() == 1 {
        return nums.to_owned();
    }

    let possible = possible_values(target, &nums[..nums.len() - 1], concat);

    possible.iter()
        .flat_map(|n| {
            let mut res = vec![n * nums[nums.len() - 1], n + nums[nums.len() - 1]];
            if concat {
                res.push(format!("{}{}", n, nums[nums.len() - 1]).parse().unwrap());
            }
            res
        })
        .filter(|n| *n <= target)
        .collect()
}

fn is_buildable(target: u64, nums: &[u64], concat: bool) -> bool {
    possible_values(target, nums, concat).contains(&target)
}

fn solve(lines: &[(u64, Vec<u64>)], concat: bool) -> u64 {
    lines.iter()
        .filter_map(|(val, nums)| {
            is_buildable(*val, nums, concat).then_some(*val)
        })
        .sum()
}

pub fn day7(input: String, ctx: &mut AOContext) {
    let lines = parse(&input);

    ctx.parsing_done();

    ctx.submit_part1(solve(&lines, false));
    ctx.submit_part2(solve(&lines, true));
}
