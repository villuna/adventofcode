use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use aoc::{
    utils::{Coord, Dir, DIRECTIONS},
    AOContext,
};

fn insert_obstacle(
    dist_map: &mut HashMap<(Coord, Dir), isize>,
    obstacles: &mut HashSet<Coord>,
    new_obstacle: Coord,
    dimensions: (isize, isize),
) {
    if !obstacles.insert(new_obstacle) {
        return;
    }

    for dir in DIRECTIONS {
        let mut current = new_obstacle + dir;
        let mut distance = 0;

        while !obstacles.contains(&current) && current.in_bounds_positive(dimensions) {
            dist_map.insert((current, dir.opposite()), distance);

            distance += 1;
            current = current + dir;
        }
    }
}

pub fn day6(input: String, ctx: &mut AOContext) {
    let mut obstacles = HashSet::new();
    let mut visited = HashSet::new();
    let mut start = Coord(0, 0);
    let mut current;
    let mut dir = Dir::Up;
    let dimensions = (
        input.lines().next().unwrap().bytes().count() as isize,
        input.lines().count() as isize,
    );

    let mut dist_map = HashMap::new();

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.bytes().enumerate() {
            if c == b'#' {
                insert_obstacle(&mut dist_map, &mut obstacles, Coord(x as _, y as _), dimensions);
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

        match dist_map.get(&(current, dir)) {
            None => {
                while current.in_bounds_positive(dimensions) {
                    visited.insert(current);
                    current = current + dir;
                }
                break;
            }

            Some(dist) => {
                for _ in 0..*dist {
                    current = current + dir;
                    visited.insert(current);
                }
            }
        }
    }

    ctx.submit_part1(visited.len());

    let part2 = visited.into_iter().filter(|&c| {
        let mut obstacles = obstacles.clone();
        let mut dist_map = dist_map.clone();

        insert_obstacle(&mut dist_map, &mut obstacles, c, dimensions);

        let mut current = start;
        let mut dir = Dir::Up;
        let mut history = HashSet::new();

        loop {
            if !history.insert((current, dir)) {
                return true;
            }
            while obstacles.contains(&(current + dir)) {
                dir = dir.rotate_cw();
            }

            match dist_map.get(&(current, dir)) {
                None => break,
                Some(&dist) => {
                    current = current + (dir.cincrement() * dist);
                }
            }
        }

        return false;
    }).count();

    ctx.submit_part2(part2);
}
