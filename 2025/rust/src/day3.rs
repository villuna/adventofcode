use aoc::AOContext;
use itertools::Itertools;

pub fn solve<const DIGITS: usize>(rows: &[Vec<u32>]) -> u64 {
    let mut res = 0;

    for row in rows {
        let mut vals = [0; DIGITS];
        let mut idx = -1;

        for n in (0..DIGITS).rev() {
            let (i, val) = row
                .iter()
                .enumerate()
                .skip((idx + 1) as usize)
                .rev()
                .skip(n)
                .max_by_key(|(_, k)| *k)
                .unwrap();

            idx = i as i32;
            vals[n] = *val;
        }

        for i in 0..DIGITS {
            res += vals[i] as u64 * 10u64.pow(i as u32);
        }
    }

    res
}

pub fn day3(input: String, ctx: &mut AOContext) {
    let rows = input
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();
    ctx.parsing_done();

    ctx.submit_part1(solve::<2>(&rows));
    ctx.submit_part2(solve::<12>(&rows));
}
