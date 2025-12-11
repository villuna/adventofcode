use std::{cmp::Reverse, collections::HashSet};

use aoc::AOContext;
use itertools::Itertools;

type Coord = (i64, i64, i64);

fn distsq(c1: &Coord, c2: &Coord) -> i64 {
    (c1.0 - c2.0).pow(2) + (c1.1 - c2.1).pow(2) + (c1.2 - c2.2).pow(2)
}

fn edges_overlap(e1: &(Coord, Coord), e2: &(Coord, Coord)) -> bool {
    e1.0 == e2.0 || e1.0 == e2.1 || e1.1 == e2.0 || e1.1 == e2.1
}

pub fn day8(input: String, ctx: &mut AOContext) {
    let coords = input
        .lines()
        .map(|c| {
            c.split(",")
                .map(|v| v.parse::<i64>().unwrap())
                .collect_tuple::<(_, _, _)>()
                .unwrap()
        })
        .collect_vec();

    ctx.parsing_done();

    // Create a list of edges sorted by distance
    let mut edges = coords
        .iter()
        .cloned()
        .cartesian_product(coords.iter().cloned())
        .filter(|(e1, e2)| e1 < e2)
        .collect::<Vec<(Coord, Coord)>>();
    edges.sort_by_key(|e| distsq(&e.0, &e.1));

    // Sets contains all the circuits with at least one edge
    let mut circuits: Vec<Vec<(Coord, Coord)>> = Vec::new();
    const P1_EDGE_LIMIT: usize = 1000;

    for e in &edges[0..P1_EDGE_LIMIT] {
        // Create a new circuit with just this edge in it
        let mut new_circuit = vec![*e];

        // Walk through all the circuits, if it connects with the new circuit then join them up
        for i in (0..circuits.len()).rev() {
            if circuits[i].iter().any(|e2| edges_overlap(e, e2)) {
                let mut circuit = circuits.remove(i);
                new_circuit.append(&mut circuit);
            }
        }

        circuits.push(new_circuit);
    }

    // Count how many nodes are in each circuit
    let mut sizes = circuits
        .iter()
        .map(|circuit| {
            let mut unique = HashSet::new();
            for e in circuit {
                unique.insert(e.0);
                unique.insert(e.1);
            }
            unique.len()
        })
        .collect_vec();

    sizes.sort_by_key(|s| Reverse(*s));
    ctx.submit_part1(sizes[0] * sizes[1] * sizes[2]);

    // Keep track of which nodes we've connected up so we know when every node is connected to a
    // circuit
    let nodes = input.trim().lines().count();
    let mut visited = HashSet::new();

    for e in &edges[P1_EDGE_LIMIT..] {
        visited.insert(e.0);
        visited.insert(e.1);
        let mut new_circuit = vec![*e];

        for i in (0..circuits.len()).rev() {
            if circuits[i].iter().any(|e2| edges_overlap(e, e2)) {
                let mut circuit = circuits.remove(i);
                new_circuit.append(&mut circuit);

                // If every node is connected to a circuit and there is only one circuit then the
                // whole network is fully connected and this was the last edge
                if visited.len() == nodes && circuits.is_empty() {
                    ctx.submit_part2(e.0.0 * e.1.0);
                    return;
                }
            }
        }

        circuits.push(new_circuit);
    }

    panic!("Was not able to connect up all the circuits for part 2");
}
