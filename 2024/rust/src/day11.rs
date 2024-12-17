use ahash::{HashMap, HashMapExt};
use aoc::utils::split;
use aoc::AOContext;

fn next(num: u64) -> (u64, Option<u64>) {
    if num == 0 {
        return (1, None);
    }
    if let Some((a, b)) = split(num) {
        return (a, Some(b));
    }
    (num * 2024, None)
}

fn run(stones: &mut HashMap<u64, u64>, next_stones: &mut HashMap<u64, u64>, cycles: u8) {
    for _ in 0..cycles {
        std::mem::swap(next_stones, stones);

        for (n, c) in next_stones.drain() {
            let (a, b) = next(n);
            *stones.entry(a).or_default() += c;
            if let Some(b) = b {
                *stones.entry(b).or_default() += c;
            }
        }

        next_stones.clear();
    }
}

pub fn day11(input: String, ctx: &mut AOContext) {
    let mut stones = input.split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .fold(HashMap::new(), |mut map, n| {
            *map.entry(n).or_default() += 1;
            map
        });

    let mut next_stones = HashMap::new();

    ctx.parsing_done();

    run(&mut stones, &mut next_stones, 25);
    ctx.submit_part1(stones.values().sum::<u64>());


    run(&mut stones, &mut next_stones, 50);
    ctx.submit_part2(stones.values().sum::<u64>());
}
