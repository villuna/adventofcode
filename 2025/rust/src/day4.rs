use std::collections::HashSet;

use aoc::AOContext;

fn get_accessible(grid: &HashSet<(i32, i32)>, width: usize, height: usize) -> Vec<(i32, i32)> {
    let mut res = Vec::new();

    for x in 0..width as i32 {
        for y in 0..height as i32 {
            if !grid.contains(&(x, y)) {
                continue;
            }

            let mut count = 0;

            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }

                    if grid.contains(&(x + i, y + j)) {
                        count += 1;
                    }
                }
            }

            if count < 4 {
                res.push((x, y));
            }
        }
    }

    res
}

pub fn day4(input: String, ctx: &mut AOContext) {
    let mut grid = HashSet::new();
    let width = input.lines().count();
    let height = input.lines().next().unwrap().chars().count();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                grid.insert((x as i32, y as i32));
            }
        }
    }

    ctx.parsing_done();

    let mut accessible = get_accessible(&grid, width, height);

    ctx.submit_part1(accessible.len());

    let original_len = grid.len();

    while !accessible.is_empty() {
        for c in accessible {
            grid.remove(&c);
        }

        accessible = get_accessible(&grid, width, height)
    }

    ctx.submit_part2(original_len - grid.len());
}
