use std::cmp::Reverse;

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use aoc::{
    utils::{Coord, Dir, DIRECTIONS},
    AOContext,
};
use priority_queue::PriorityQueue;

#[derive(Default)]
struct Grid {
    walls: HashSet<Coord>,
    start: Coord,
    end: Coord,
}

fn parse(input: &str) -> Grid {
    let mut res = Grid::default();

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let coord = Coord(x as _, y as _);
            if c == '#' {
                res.walls.insert(coord);
            } else if c == 'S' {
                res.start = coord;
            } else if c == 'E' {
                res.end = coord;
            }
        }
    }

    res
}

fn bfs(grid: &Grid, start: Coord, start_dirs: &[Dir]) -> HashMap<(Coord, Dir), usize> {
    let mut frontier = PriorityQueue::new();
    let mut visited = HashSet::new();
    let mut res = HashMap::new();

    for &d in start_dirs {
        frontier.push((start, d), Reverse(0));
    }

    while let Some(((pos, dir), Reverse(cost))) = frontier.pop() {
        visited.insert((pos, dir));
        res.insert((pos, dir), cost);

        let mut edges = vec![(pos + dir, dir)];
        for d in DIRECTIONS.into_iter().filter(|&d| d != dir && d != dir.opposite()) {
            edges.push((pos, d)); 
        }
        for (next, d) in edges {
            let step_cost = if dir == d { 1 } else { 1000 };

            if !visited.contains(&(next, d)) && !grid.walls.contains(&next) {
                frontier.push_increase((next, d), Reverse(cost + step_cost));
            }
        }
    }

    res
}

fn part2(start_map: HashMap<(Coord, Dir), usize>, end_map: HashMap<(Coord, Dir), usize>, target: usize) -> HashSet<Coord> {
    start_map.into_iter().filter_map(|((c, d), cost)| {
        let Some(ecost) = end_map.get(&(c, d.opposite())) else { return None };
        (ecost + cost == target).then_some(c)
    }).collect::<HashSet<Coord>>()
}

pub fn day16(input: String, ctx: &mut AOContext) {
    let grid = parse(&input);
    ctx.parsing_done();

    let start_map = bfs(&grid, grid.start, &[Dir::Right]);
    let part1 = DIRECTIONS.iter().filter_map(|d| start_map.get(&(grid.end, *d)).cloned()).min().unwrap();

    ctx.submit_part1(part1);

    let end_map = bfs(&grid, grid.end, &DIRECTIONS);

    let good_tiles = part2(start_map, end_map, part1);
    ctx.submit_part2(good_tiles.len());
}
