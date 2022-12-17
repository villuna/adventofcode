use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, BTreeMap, VecDeque, HashSet};
use rayon::prelude::*;
use indicatif::{ParallelProgressIterator, ProgressIterator};

const START_STATE: Label = ['A', 'A'];

type Label = [char; 2];

#[derive(Debug)]
struct Valve {
    rate: u16,
    edges: Vec<Label>,
}

// A map representing the cave system
struct Environment {
    graph: HashMap<Label, Valve>,
    important_valves: Vec<Label>,
    distances: HashMap<Label, BTreeMap<Label, u16>>,
}

impl Environment {
    fn parse(input: &str) -> Environment {
        let re = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z]{2}(:?, [A-Z]{2})*)").unwrap();
        let mut graph = HashMap::new();

        for line in input.lines() {
            let captures = re.captures(line).unwrap();
            let (l1, l2) = captures[1].chars().collect_tuple().unwrap();
            let label = [l1, l2];

            let rate = captures[2].parse().unwrap();

            let edges = captures[3]
                .split(", ")
                .map(|s| {
                    let (l1, l2) = s.chars().collect_tuple().unwrap();
                    [l1, l2]
                })
                .collect_vec();

            graph.insert(label, Valve { rate, edges });
        }

        let distances = Self::calculate_distances(&graph);
        let important_valves = graph.iter()
            .filter(|(_, valve)| valve.rate != 0)
            .map(|(l, _)| *l)
            .collect_vec();

        Environment {
            graph,
            important_valves,
            distances,
        }
    }

    fn calculate_distances(graph: &HashMap<Label, Valve>) -> HashMap<Label, BTreeMap<Label, u16>> {
        // Find the shortest path between valves (of note) using dijkstra
        let important_valves = graph.iter().filter(|(&label, valve)| label == START_STATE || valve.rate > 0).map(|x| *x.0).collect_vec();
        let mut res = HashMap::new();

        for start in important_valves.iter() {
            let mut queue = VecDeque::new();
            let mut distances = BTreeMap::new();
            let mut visited = HashSet::new();

            distances.insert(*start, 0);
            queue.push_back(*start);

            while let Some(label) = queue.pop_front() {
                visited.insert(label);
                let distance = *distances.get(&label).unwrap();
                let valve = graph.get(&label).unwrap();

                for edge in valve.edges.iter() {
                    if !visited.contains(edge) && !queue.contains(edge) {
                        distances.insert(*edge, distance + 1);
                        queue.push_back(*edge);
                    }
                }
            }

            distances.retain(|label, _| important_valves.contains(label) && *label != START_STATE);
            res.insert(*start, distances);
        }

        res
    }

    fn get_valve(&self, label: &Label) -> Option<&Valve> {
        self.graph.get(label)
    }
}

pub fn day_sixteen(input: String) {
    /*
    let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
    */
    let env = Environment::parse(&input);

    println!("{}", part_one(&env));
}

fn part_one(env : &Environment) -> u16 {
    let num_valves = env.important_valves.len();
    println!("{:?}", env.important_valves);
    let max_time = 30;
    let len: u64 = (1..=num_valves as u64).product();

    env.important_valves
        .iter()
        .permutations(num_valves)
        .par_bridge()
        .progress_count(len)
        .map(|p| {
            let mut time = 0;
            let mut rate = 0;
            let mut pressure = 0;
            let mut current = START_STATE;

            for valve in p {
                let time_step = env.distances.get(&current).unwrap().get(valve).unwrap() + 1;

                if time + time_step > max_time {
                    break;
                }

                pressure += rate * time_step;
                time += time_step;
                rate += env.graph.get(valve).unwrap().rate;
                current = *valve;
            }

            pressure + rate * (max_time - time)
        })
        .max()
        .unwrap_or(0)
}
