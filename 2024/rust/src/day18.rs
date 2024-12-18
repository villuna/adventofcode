use std::cmp::Reverse;

use ahash::{HashSet, HashSetExt};
use aoc::{utils::{Coord, DIRECTIONS}, AOContext};
use itertools::Itertools;
use priority_queue::PriorityQueue;

#[derive(Hash, Eq, PartialEq, Ord)]
struct Cost(usize, usize);

impl Cost {
    fn new(cost: usize, from: Coord, to: Coord) -> Self {
        Cost(cost, ((from.0 - to.0).abs() + (from.1 - to.1).abs()) as usize)
    }
}

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.0 + self.1).partial_cmp(&(other.0 + other.1))
    }
}

fn astar(map: &HashSet<Coord>, start: Coord, end: Coord) -> Option<usize> {
    let mut frontier = PriorityQueue::new();
    let mut visited = HashSet::new();
    frontier.push(start, Reverse(Cost::new(0, start, end)));

    while let Some((pos, Reverse(Cost(cost, _)))) = frontier.pop() {
        visited.insert(pos);

        for d in DIRECTIONS {
            let next = pos + d;
            
            if next == end {
                return Some(cost + 1);
            }

            let in_bounds = next.0 >= 0 && next.1 >= 0 && next.0 <= 70 && next.1 <= 70;

            if in_bounds && !map.contains(&next) && !visited.contains(&next) {
                frontier.push_increase(next, Reverse(Cost::new(cost + 1, next, end)));
            }
        }
    }

    None
}

pub fn day18(input: String, ctx: &mut AOContext) {
    let parse_coord = |l: &str| -> Coord {
        let (x, y) = l.split(",").map(|n| n.parse().unwrap()).collect_tuple().unwrap();
        Coord(x, y)
    };
    let mut map: HashSet<Coord> = input.lines().take(1024)
        .map(parse_coord)
        .collect();

    ctx.submit_part1(astar(&map, Coord(0, 0), Coord(70, 70)).unwrap());

    for c@Coord(x, y) in input.lines().skip(1024).map(parse_coord) {
        map.insert(c);

        if astar(&map, Coord(0, 0), Coord(70, 70)).is_none() {
            ctx.submit_part2(format!("{x},{y}"));
            break;
        }
    }
}
