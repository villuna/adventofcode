use std::collections::{HashMap, HashSet};

use aoc::AOContext;

fn dfs(
    splitters: &HashSet<(i32, i32)>,
    mut start: (i32, i32),
    visited: &mut HashSet<(i32, i32)>,
    splitters_visited: &mut HashSet<(i32, i32)>,
    max_y: i32,
) {
    if visited.contains(&start) {
        return;
    }
    visited.insert(start);

    while start.1 <= max_y {
        start.1 += 1;

        if splitters.contains(&start) {
            splitters_visited.insert(start);
            dfs(
                splitters,
                (start.0 + 1, start.1),
                visited,
                splitters_visited,
                max_y,
            );
            dfs(
                splitters,
                (start.0 - 1, start.1),
                visited,
                splitters_visited,
                max_y,
            );
            return;
        }
    }
}

fn part1(splitters: &HashSet<(i32, i32)>, start: (i32, i32), max_y: i32) -> usize {
    let mut visited = HashSet::new();
    let mut splitters_visited = HashSet::new();
    dfs(
        splitters,
        start,
        &mut visited,
        &mut splitters_visited,
        max_y,
    );
    splitters_visited.len()
}

fn part2(
    splitters: &HashSet<(i32, i32)>,
    start: (i32, i32),
    max_y: i32,
    cache: &mut HashMap<(i32, i32), usize>,
) -> usize {
    if let Some(&res) = cache.get(&start) {
        return res;
    }
    let mut current = start;

    while current.1 <= max_y {
        current.1 += 1;

        if splitters.contains(&current) {
            let res = part2(splitters, (current.0 + 1, current.1), max_y, cache)
                + part2(splitters, (current.0 - 1, current.1), max_y, cache);

            cache.insert(start, res);
            return res;
        }
    }

    cache.insert(start, 1);
    1
}

pub fn day7(input: String, ctx: &mut AOContext) {
    let mut splitters = HashSet::new();
    let mut start = None;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '^' {
                splitters.insert((x as i32, y as i32));
            } else if c == 'S' {
                start = Some((x as i32, y as i32));
            }
        }
    }

    let start = start.unwrap();
    let max_y = input.lines().count() as i32;

    ctx.parsing_done();

    ctx.submit_part1(part1(&splitters, start, max_y));
    let mut cache = HashMap::new();
    ctx.submit_part2(part2(&splitters, start, max_y, &mut cache));
}
