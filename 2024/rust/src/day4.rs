use itertools::Itertools;

use crate::AOContext;

const HORIZONTAL: [(usize, usize); 4] = [(0, 0), (0, 1), (0, 2), (0, 3)];
const VERTICAL: [(usize, usize); 4] = [(0, 0), (1, 0), (2, 0), (3, 0)];
const DIAG_BACK: [(usize, usize); 4] = [(0, 0), (1, 1), (2, 2), (3, 3)];
const DIAG_FORWARD: [(usize, usize); 4] = [(3, 0), (2, 1), (1, 2), (0, 3)];

fn check_pattern<const N: usize>(
    grid: &Vec<Vec<u8>>,
    pattern: [(usize, usize); N],
    target: [u8; N],
) -> i32 {
    let max_dx = pattern.iter().map(|(x, _)| *x).max().unwrap();
    let max_dy = pattern.iter().map(|(_, y)| *y).max().unwrap();
    let mut res = 0;
    let mut target_rev: [u8; N] = [0; N];

    for i in 0..N {
        target_rev[i] = target[N - i - 1];
    }

    for y in 0..grid.len() - max_dy {
        for x in 0..grid[0].len() - max_dx {
            let mut word: [u8; N] = [0; N];
            for (i, (dx, dy)) in pattern.iter().enumerate() {
                word[i] = grid[y + dy][x + dx];
            }

            if word == target || word == target_rev {
                res += 1;
            }
        }
    }

    res
}

pub fn day4(input: String, ctx: &mut AOContext) {
    let grid = input.lines().map(|x| x.as_bytes().to_owned()).collect_vec();

    ctx.parsing_done();

    let xmas = *b"XMAS";
    let part1: i32 = [HORIZONTAL, VERTICAL, DIAG_FORWARD, DIAG_BACK].iter()
        .map(|pattern| check_pattern(&grid, *pattern, xmas))
        .sum();

    ctx.submit_part1(part1);

    let mut part2 = 0;

    for i in 0..grid.len() - 2 {
        for j in 0..grid.len() - 2 {
            let word1 = &[grid[i][j], grid[i + 1][j + 1], grid[i + 2][j + 2]];
            let word2 = &[grid[i + 2][j], grid[i + 1][j + 1], grid[i][j + 2]];

            if (word1 == b"MAS" || word1 == b"SAM") && (word2 == b"MAS" || word2 == b"SAM") {
                part2 += 1;
            }
        }
    }

    ctx.submit_part2(part2);
}
