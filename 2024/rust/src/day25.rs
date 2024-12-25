use aoc::AOContext;
use itertools::Itertools;

type Pins = [u8; 5];

fn parse(input: &str) -> (Vec<Pins>, Vec<Pins>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for grid in input.split("\n\n") {
        let is_lock = grid.lines().next().unwrap() == "#####";
        let grid = grid.as_bytes();
        let mut seq = Pins::default();

        for i in 0..5 {
            for j in 1..=5 {
                if grid[j * 6 + i] == b'#' {
                    seq[i] += 1;
                }
            }
        }

        if is_lock {
            locks.push(seq);
        } else {
            keys.push(seq);
        }
    }

    (keys, locks)
}

pub fn day25(input: String, ctx: &mut AOContext) {
    let (keys, locks) = parse(&input);

    ctx.parsing_done();

    let part1 = keys.iter().cartesian_product(locks.iter()).filter(|(key, lock)| {
        (0..5).all(|i| key[i] + lock[i] <= 5)
    }).count();

    ctx.submit_both(part1, "Merry Christmas!");
}
