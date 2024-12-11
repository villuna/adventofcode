use ahash::{HashMap, HashMapExt};
use aoc::utils::split;
use aoc::AOContext;
use smallvec::SmallVec;
use smallvec::smallvec;

fn next(num: u64) -> SmallVec<[u64; 2]> {
    if num == 0 {
        return smallvec![1];
    }
    if let Some((a, b)) = split(num) {
        return smallvec![a, b];
    }
    smallvec![num * 2024]
}

fn run(stones: &mut HashMap<u64, u64>, cycles: u8) {
    for _ in 0..cycles {
        *stones = std::mem::take(stones).into_iter()
            .fold(HashMap::new(), |mut map, (n, c)| {
                for m in next(n) {
                    *map.entry(m).or_default() += c;
                }
                map
            });

    }
}

pub fn day11(input: String, ctx: &mut AOContext) {
    let mut ints: HashMap<u64, u64> = input.split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .fold(HashMap::new(), |mut map, n| {
            *map.entry(n).or_default() += 1;
            map
        });

    ctx.parsing_done();

    run(&mut ints, 25);
    ctx.submit_part1(ints.values().sum::<u64>());


    run(&mut ints, 50);
    ctx.submit_part2(ints.values().sum::<u64>());
}

