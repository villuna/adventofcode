use std::collections::BTreeSet;

use ahash::{HashMap, HashSet, HashSetExt};
use aoc::AOContext;
use itertools::Itertools;

#[derive(Default)]
struct Graph<'a> {
    graph: HashMap<&'a str, HashSet<&'a str>>,
}

fn parse(input: &str) -> Graph {
    let mut g = Graph::default();

    for l in input.lines() {
        let (a, b) = l.split("-").collect_tuple().unwrap();

        g.graph.entry(a).or_default().insert(b);
        g.graph.entry(b).or_default().insert(a);
    }

    g
}

fn part1(graph: &Graph) -> usize {
    let mut sets: HashSet<BTreeSet<&str>> = HashSet::new();

    for (&start, edges) in graph.graph.iter().filter(|(s, _)| s.starts_with("t")) {
        for &e in edges {
            for &e2 in &graph.graph[e] {
                if graph.graph[e2].contains(start) {
                    sets.insert(BTreeSet::from([start, e, e2]));
                }
            }
        }
    }

    sets.len()
}

fn find_complete_helper<'s>(
    start: &'s str,
    graph: &Graph<'s>,
    visited: &mut HashSet<&'s str>,
    res: &mut Vec<&'s str>,
) {
    visited.insert(start);
    res.push(start);

    for &e in &graph.graph[start] {
        if !visited.contains(e) && res.iter().all(|s| graph.graph[e].contains(s)) {
            find_complete_helper(e, graph, visited, res);
        }
    }
}

fn find_complete<'s>(
    start: &'s str,
    graph: &Graph<'s>,
    visited: &mut HashSet<&'s str>,
) -> Vec<&'s str> {
    let mut res = Vec::new();

    find_complete_helper(start, graph, visited, &mut res);

    res
}

fn part2(graph: &Graph) -> String {
    let mut all_sets: Vec<Vec<&str>> = Vec::new();
    let mut visited = HashSet::new();

    for n in graph.graph.keys() {
        if !visited.contains(n) {
            all_sets.push(find_complete(n, &graph, &mut visited));
        }
    }

    let mut biggest = all_sets.into_iter().max_by_key(|s| s.len()).unwrap();
    biggest.sort();
    biggest.join(",")
}

pub fn day23(input: String, ctx: &mut AOContext) {
    let graph = parse(&input);

    ctx.parsing_done();
    ctx.submit_part1(part1(&graph));
    ctx.submit_part2(part2(&graph));
}
