use aoc::AOContext;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

fn is_invalid(num: u64, digits: u32) -> bool {
    if !digits.is_multiple_of(2) {
        return false;
    }

    let pow = 10u64.pow(digits / 2);
    let start = num / pow;
    let end = num % pow;

    start == end
}

fn is_invalid_2(num: u64, lengths: &[u32]) -> bool {
    lengths.iter().any(|d| {
        let pow = 10u64.pow(*d);
        let start = num % pow;
        let mut next = num / pow;

        while next > 0 {
            let rem = next % pow;
            next /= pow;

            if rem != start {
                return false;
            }
        }

        true
    })
}

pub fn day2(input: String, ctx: &mut AOContext) {
    let ranges = input
        .trim()
        .split(",")
        .map(|s| {
            let mut s = s.split("-");
            let a = s.next().unwrap().parse::<u64>().unwrap();
            let b = s.next().unwrap().parse::<u64>().unwrap();
            (a, b)
        })
        .collect::<Vec<_>>();

    ctx.parsing_done();

    let part1: u64 = ranges
        .par_iter()
        .flat_map(|&(start, end)| {
            let digits = start.ilog10() + 1;
            (start..=end)
                .into_par_iter()
                .filter(move |x| is_invalid(*x, digits))
        })
        .sum();

    ctx.submit_part1(part1);

    let part2: u64 = ranges
        .par_iter()
        .flat_map(|&(start, end)| {
            let digits = start.ilog10() + 1;
            let lengths = (1..=digits / 2)
                .filter(|d| digits % d == 0)
                .collect::<Vec<_>>()
                .leak();

            (start..=end)
                .into_par_iter()
                .filter(|x| is_invalid_2(*x, lengths))
        })
        .sum();

    ctx.submit_part2(part2);
}
