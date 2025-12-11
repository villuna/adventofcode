use std::{collections::HashMap, convert::identity};

use aoc::AOContext;
use itertools::Itertools;

fn dfs<'s>(
    node: &'s str,
    graph: &HashMap<&'s str, Vec<&'s str>>,
    cache: &mut HashMap<(&'s str, &'static [bool]), usize>,
    required: &[&'s str],
    visited: &mut [bool],
) -> usize {
    // Thank you chess programming for teaching me about make and unmake
    // and thank you imperative programming for allowing such things
    for (i, n) in required.iter().enumerate() {
        if node == *n {
            assert!(!visited[i]);
            visited[i] = true;
        }
    }

    // Sorry this is objectively terrible but it's all i got
    let paths = (|| {
        if let Some(&res) = cache.get(&(node, visited)) {
            return res;
        };

        if node == "out" {
            if visited.iter().cloned().all(identity) {
                return 1;
            } else {
                return 0;
            }
        }

        let mut paths = 0;
        for edge in graph.get(node).unwrap() {
            paths += dfs(edge, graph, cache, required, visited);
        }

        cache.insert((node, visited.to_owned().leak()), paths);

        paths
    })();

    for (i, n) in required.iter().enumerate() {
        if node == *n {
            visited[i] = false;
        }
    }

    paths
}

fn part1(graph: &HashMap<&str, Vec<&str>>) -> usize {
    let mut cache = HashMap::new();
    dfs("you", graph, &mut cache, &[], &mut [])
}

fn part2(graph: &HashMap<&str, Vec<&str>>) -> usize {
    let mut cache = HashMap::new();
    dfs(
        "svr",
        graph,
        &mut cache,
        &["dac", "fft"],
        &mut [false, false],
    )
}

pub fn day11(input: String, ctx: &mut AOContext) {
    let graph = input
        .lines()
        .map(|l| {
            let (node, outputs) = l.split(": ").collect_tuple().unwrap();
            (node, outputs.split_whitespace().collect_vec())
        })
        .collect::<HashMap<_, _>>();

    ctx.submit_part1(part1(&graph));
    ctx.submit_part2(part2(&graph));
}
