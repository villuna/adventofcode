use ahash::{HashSet, HashSetExt};
use aoc::{utils::{Coord, Dir}, AOContext};

pub fn day6(input: String, ctx: &mut AOContext) {
    let mut obstacles = HashSet::new();
    let mut visited = HashSet::new();
    let mut start = Coord(0, 0);
    let mut current;
    let mut dir = Dir::Up;
    let dimensions = (input.lines().next().unwrap().bytes().count() as isize, input.lines().count() as isize);

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.bytes().enumerate() {
            if c == b'#' {
                obstacles.insert(Coord(x as _, y as _));
            } else if c == b'^' {
                start = Coord(x as _, y as _);
            }
        }
    }

    ctx.parsing_done();

    current = start;

    loop {
        visited.insert(current);
        while obstacles.contains(&(current + dir)) {
            dir = dir.rotate_cw();
        }

        current = current + dir;

        if current.0 < 0 || current.0 >= dimensions.0 || current.1 < 0 || current.1 >= dimensions.1 {
            break;
        }
    }

    ctx.submit_part1(visited.len());

    let mut loops = 0;

    for x in 0..dimensions.0 {
        'inner: for y in 0..dimensions.1 {
            let mut obstacles = obstacles.clone();
            if !obstacles.insert(Coord(x, y)) {
                continue;
            }

            current = start;
            dir = Dir::Up;
            let mut history = HashSet::new();

            loop {
                if !history.insert((current, dir)) {
                    loops += 1;
                    continue 'inner;
                }
                while obstacles.contains(&(current + dir)) {
                    dir = dir.rotate_cw();
                }

                current = current + dir;

                if current.0 < 0 || current.0 >= dimensions.0 || current.1 < 0 || current.1 >= dimensions.1 {
                    break;
                }
            }
        }
    }

    ctx.submit_part2(loops);
}
