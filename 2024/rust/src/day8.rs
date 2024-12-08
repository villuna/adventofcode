use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use aoc::{utils::Coord, AOContext};
use itertools::Itertools;

fn part1(grid: &HashMap<Coord, char>, dimensions: (isize, isize)) -> usize {
    let mut antinodes = HashSet::new();

    for ((&start, _), (&target, _)) in grid.iter()
        .cartesian_product(grid.iter())
        .filter(|((p1, c1), (p2, c2))| p1 != p2 && c1 == c2)
    {
        let node = start + (start - target);
        if node.in_bounds_positive(dimensions) {
            antinodes.insert(node);
        }
    }

    antinodes.len()
}

fn part2(grid: &HashMap<Coord, char>, dimensions: (isize, isize)) -> usize {
    let mut antinodes = HashSet::new();

    for ((&start, _), (&target, _)) in grid.iter()
        .cartesian_product(grid.iter())
        .filter(|((p1, c1), (p2, c2))| p1 != p2 && c1 == c2)
    {
        let mut node = start;
        while node.in_bounds_positive(dimensions) {
            antinodes.insert(node);
            node = node + (start - target);
        }
    }

    antinodes.len()
}

pub fn day8(input: String, ctx: &mut AOContext) {
    let dimensions = (input.lines().next().unwrap().chars().count() as _, input.lines().count() as _);
    let mut grid = HashMap::new();
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c.is_alphanumeric() {
                grid.insert(Coord(x as _, y as _), c);
            }
        }
    }

    ctx.parsing_done();
    
    ctx.submit_part1(part1(&grid, dimensions));
    ctx.submit_part2(part2(&grid, dimensions));
}
