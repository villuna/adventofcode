use std::collections::{HashMap, HashSet};

use aoc::AOContext;

fn remove_accessible(grid: &mut HashMap<(i32, i32), u8>) -> bool {
    let mut removed_any = false;

    for ((x, y), count) in grid.clone().into_iter() {
        if count < 4 {
            grid.remove(&(x, y));

            for i in -1..=1 {
                for j in -1..=1 {
                    if let Some(c) = grid.get_mut(&(x + i, y + j)) {
                        *c -= 1;
                    }
                }
            }

            removed_any = true;
        }
    }

    removed_any
}

fn count_grid(grid: &HashSet<(i32, i32)>) -> HashMap<(i32, i32), u8> {
    let mut map = HashMap::new();

    for &c in grid {
        let mut count = 0;

        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                if grid.contains(&(c.0 + i, c.1 + j)) {
                    count += 1;
                }
            }
        }

        map.insert(c, count);
    }

    map
}

pub fn day4(input: String, ctx: &mut AOContext) {
    let mut grid = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                grid.insert((x as i32, y as i32));
            }
        }
    }

    ctx.parsing_done();

    let mut grid = count_grid(&grid);

    ctx.submit_part1(grid.values().filter(|c| **c < 4).count());

    let original_len = grid.len();
    while remove_accessible(&mut grid) {}
    ctx.submit_part2(original_len - grid.len());
}
