use ahash::{HashMap, HashMapExt};
use aoc::AOContext;
use itertools::Itertools;

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut sections = input.split("\n\n");

    let patterns = sections.next().unwrap().trim().split(", ").collect();
    let targets = sections.next().unwrap().trim().split("\n").collect();

    (patterns, targets)
}

fn ways_to_make<'a>(
    target: &'a str,
    patterns: &'a [&'a str],
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if cache.contains_key(target) {
        *cache.get(target).unwrap()
    } else if target.is_empty() {
        1
    } else {
        let mut ways = 0;

        for pattern in patterns {
            if target.starts_with(pattern) {
                ways += ways_to_make(target.strip_prefix(pattern).unwrap(), patterns, cache);
            }
        }

        cache.insert(target, ways);
        ways
    }
}

pub fn day19(input: String, ctx: &mut AOContext) {
    let (patterns, targets) = parse(&input);
    ctx.parsing_done();

    let mut cache = HashMap::new();
    let res = targets
        .iter()
        .map(|target| ways_to_make(target, &patterns, &mut cache))
        .collect_vec();

    ctx.submit_both(
        res.iter().filter(|&&w| w != 0).count(),
        res.iter().sum::<usize>(),
    );
}
