use ahash::{HashMap, HashMapExt};
use aoc::AOContext;
use itertools::Itertools;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

fn next_secret(mut secret: u64) -> u64 {
    const PRUNE: u64 = 16777216;
    secret = (secret ^ (secret * 64)) % PRUNE;
    secret = (secret ^ (secret / 32)) % PRUNE;
    (secret ^ (secret * 2048)) % PRUNE
}

pub fn day22(input: String, ctx: &mut AOContext) {
    let numbers = input.lines().map(|n| n.parse::<u64>().unwrap()).collect_vec();

    ctx.parsing_done();

    let mut nums = Vec::with_capacity(numbers.len());
    let mut monkeys = Vec::with_capacity(numbers.len());

    numbers.into_par_iter().map(|mut n| {
        let mut change_map = HashMap::new();
        let mut change = [0i8; 4];

        for i in 0..2000 {
            let last = n;
            n = next_secret(n);

            change.rotate_right(1);
            change[0] = (n % 10 - last % 10) as _;

            if i >= 3 {
                if !change_map.contains_key(&change) {
                    change_map.insert(change, (n % 10) as _);
                }
            }
        }

        (n, change_map)
    }).unzip_into_vecs(&mut nums, &mut monkeys);

    let part1 = nums.into_iter().sum::<u64>();
    ctx.submit_part1(part1);

    let changes = (0..4).map(|_| (-9..=9)).multi_cartesian_product().collect_vec();

    let part2 = changes.into_par_iter().map(|change| {
        let change = [change[0], change[1], change[2], change[3]];
        monkeys.iter().filter_map(|monkey| monkey.get(&change)).sum::<i32>()
    }).max().unwrap();

    ctx.submit_part2(part2);
}
