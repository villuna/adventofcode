use std::{cmp::Reverse, collections::VecDeque};

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use aoc::{utils::{Coord, DIRECTIONS}, AOContext};
use itertools::Itertools;
use priority_queue::PriorityQueue;

#[derive(Default, Debug)]
struct Grid {
    walls: HashSet<Coord>,
    dimensions: (isize, isize),
    start: Coord,
    end: Coord,
}

fn parse(input: &str) -> Grid {
    let mut res = Grid::default();
    res.dimensions = (input.lines().next().unwrap().chars().count() as _, input.lines().count() as _);

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

fn distance_tree(grid: &Grid, start: Coord) -> HashMap<Coord, usize> {
    let mut frontier = VecDeque::new();
    let mut tree = HashMap::new();
    frontier.push_back((start, 0));

    while let Some((pos, cost)) = frontier.pop_front() {
        if !tree.contains_key(&pos) {
            tree.insert(pos, cost); 
        } else {
            continue;
        }

        for d in DIRECTIONS {
            let next = pos + d;

            if !grid.walls.contains(&next) && !tree.contains_key(&next) {
                frontier.push_back((next, cost + 1));
            }
        }
    }

    tree
}

fn find_cheats(max_length: usize, cheats: &mut HashMap<(Coord, Coord), usize>, start: Coord, grid: &Grid) {
    let mut frontier = PriorityQueue::new();    
    let mut visited = HashSet::new();
    frontier.push(start, Reverse(0));

    while let Some((pos, Reverse(cost))) = frontier.pop() {
        if !visited.contains(&pos) {
            visited.insert(pos);
        } else {
            continue;
        }

        if pos != start && !grid.walls.contains(&pos) {
            cheats.insert((start, pos), cost);
        }

        if cost < max_length {
            for d in DIRECTIONS {
                let next = pos + d;

                if !visited.contains(&next) {
                    frontier.push_increase(next, Reverse(cost + 1));
                }
            }
        }
    }
}

fn solve(max_length: usize, grid: &Grid, start_tree: &HashMap<Coord, usize>, end_tree: &HashMap<Coord, usize>) -> usize {
    let mut cheats = HashMap::new();

    for x in 0..grid.dimensions.0 {
        for y in 0..grid.dimensions.1 {
            let pos = Coord(x,y);
            if grid.walls.contains(&pos) {
                continue;
            }

            find_cheats(max_length, &mut cheats, pos, &grid);
        }
    }

    let no_cheat_dist = *start_tree.get(&grid.end).unwrap();

    let saves = cheats.iter().filter_map(|((start, end), cost)| {
        let d1 = start_tree.get(start)?;
        let d2 = end_tree.get(end)?;
        let dist = d1 + d2 + cost;

        (no_cheat_dist > dist).then_some(no_cheat_dist - dist)
    }).collect_vec();

    saves.iter().filter(|&&c| c >= 100).count()
}

pub fn day20(input: String, ctx: &mut AOContext) {
    let grid = parse(&input);
    let start_tree = distance_tree(&grid, grid.start);
    let end_tree = distance_tree(&grid, grid.end);

    ctx.submit_part1(solve(2, &grid, &start_tree, &end_tree));
    ctx.submit_part2(solve(20, &grid, &start_tree, &end_tree));
}
