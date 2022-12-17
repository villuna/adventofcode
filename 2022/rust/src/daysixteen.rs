use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::{
    collections::{BTreeSet, HashMap, BTreeMap, VecDeque, HashSet},
    hash::Hash,
};
use regex::Regex;

const START_STATE: Label = ['A', 'A'];

// This is a lot of types I know, but it really is worth it I swear.
// I'm not addicted
// I can stop at any time

type Label = [char; 2];

#[derive(Debug, Hash, PartialEq, Eq, Clone, PartialOrd, Ord)]
struct Helper {
    position: Label,
    goal: Option<Label>,
    progress: i8,
    opening_valve: bool,
}

// A struct representing the current state of the world
#[derive(Debug, Eq, Clone)]
struct State {
    done: bool,
    helpers: Vec<Helper>,
    open_valves: BTreeSet<Label>,
    time: i8,
}

#[derive(Debug)]
struct Valve {
    rate: i16,
    edges: Vec<Label>,
}

// A map representing the cave system
struct Environment {
    graph: HashMap<Label, Valve>,
    distances: HashMap<Label, BTreeMap<Label, i8>>,
}

// An iterator that returns a topological ordering of the state graph
// High level cs stuff I know
// thanks COMP3506
//
// The reason this is an iterator is because they can be lazy
// I don't need to store the entire graph of possible states to find the
// best path to the goal on a DAG.
struct TopologicalOrdering<'a> {
    env: &'a Environment,
    max_time: i8,
    frontier: PriorityQueue<State, i8>, // Stores the states to check, sorted by time step
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

        Environment {
            graph,
            distances,
        }
    }

    fn calculate_distances(graph: &HashMap<Label, Valve>) -> HashMap<Label, BTreeMap<Label, i8>> {
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

impl Helper {
    fn next_states(&self, env: &Environment, open_valves: &BTreeSet<Label>) -> Vec<Helper> {
        // If currently travelling, just move. Otherwise consider the state we end up at.
        let current_state = if let Some(label) = self.goal {
            if self.progress < *env.distances.get(&self.position).and_then(|map| map.get(&label)).unwrap() {
                return vec![Helper {
                    progress: self.progress + 1,
                    ..self.clone()
                }];
            } else {
                Helper {
                    position: label,
                    progress: 0,
                    goal: None,
                    opening_valve: false,
                }
            }
        } else {
            self.clone()
        };

        // If the current valve is not open, open it.
        if current_state.position != START_STATE && !open_valves.contains(&current_state.position) {
            return vec![Helper {
                opening_valve: true,
                ..current_state
            }];
        }

        let mut res = Vec::new();
        // Or we could move to any closed, notable valve
        for (&valve, _) in env.distances.get(&current_state.position).unwrap().iter()
            .filter(|(&valve, _)| valve != current_state.position && !open_valves.contains(&valve)){
            res.push(Helper {
                opening_valve: false,
                goal: Some(valve),
                progress: 1,
                ..current_state.clone()
            })
        }

        if res.is_empty() {
            res.push(current_state);
        }

        res
    }
}

impl State {
    // Returns a vector of all the possible states reachable from this state
    fn next_states(&self, env: &Environment, max_time: i8) -> Vec<State> {
        if self.done {
            return Vec::new();
        }

        // if the time reaches 30, we're doneskis
        if self.time == max_time {
            return vec![State::done(max_time)];
        }

        // if all the (meaningful) valves are open, just don't do anything
        if env
            .graph
            .iter()
            .filter(|(_, valve)| valve.rate > 0)
            .all(|(label, _)| self.open_valves.contains(label))
        {
            return vec![State {
                time: max_time,
                ..self.clone()
            }];
        }

        let mut open_valves = self.open_valves.clone();

        for helper in self.helpers.iter() {
            if helper.opening_valve {
                if !open_valves.insert(helper.position) {
                    panic!("Opened a valve twice!!");
                }
            }
        }

        let (helpers, time_step) = if self.helpers.iter().all(|helper| helper.goal.is_some()) {
            let min_distance = self.helpers.iter()
                .map(|helper| env.distances.get(&helper.position).unwrap().get(&helper.goal.unwrap()).unwrap() - helper.progress)
                .min()
                .unwrap();

            (self.helpers.iter()
                .map(|helper| Helper {
                    progress: helper.progress + min_distance,
                    ..helper.clone()
                })
                .collect_vec(), min_distance)
        } else {
            (self.helpers.clone(), 0)
        };

        helpers
            .iter()
            .map(|helper| helper.next_states(env, &open_valves))
            .multi_cartesian_product()
            .filter(|helpers| {
                helpers
                    .iter()
                    .all_unique()
            })
            .map(|helpers| State {
                done: false,
                helpers,
                open_valves: open_valves.clone(),
                time: self.time + time_step + 1,
            })
            .collect_vec()
    }

    fn start(helpers: usize) -> State {
        State {
            done: false,
            helpers: vec![
                Helper {
                    position: START_STATE,
                    goal: None,
                    progress: 0,
                    opening_valve: false,
                };
                helpers
            ],
            open_valves: BTreeSet::new(),
            time: 0,
        }
    }

    fn done(max_time: i8) -> State {
        State {
            done: true,
            helpers: vec![], // Bye bye elephant
            open_valves: BTreeSet::new(),
            time: max_time + 1,
        }
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        let mut self_helpers = self.helpers.clone();
        let mut other_helpers = other.helpers.clone();
        self_helpers.sort();
        other_helpers.sort();

        self.done == other.done
            && self.time == other.time
            && self.open_valves == other.open_valves
            && self_helpers == other_helpers
    }
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.done.hash(state);
        self.time.hash(state);
        self.open_valves.hash(state);
        let mut helpers = self.helpers.clone();
        helpers.sort();
        helpers.hash(state);
    }
}

impl TopologicalOrdering<'_> {
    fn new(env: &Environment, max_time: i8, helpers: usize) -> TopologicalOrdering {
        let mut frontier = PriorityQueue::new();

        let first_state = State::start(helpers);

        frontier.push(first_state, 0);

        TopologicalOrdering {
            env,
            frontier,
            max_time,
        }
    }
}

impl Iterator for TopologicalOrdering<'_> {
    // Returns the next state to check, and all the predecessors of that state
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        let (next, _) = self.frontier.pop()?;

        for state in next.next_states(self.env, self.max_time).into_iter() {
            let time = state.time;
            self.frontier.push(state, -time);
        }

        Some(next)
    }
}

pub fn day_sixteen(input: String) {
    let env = Environment::parse(&input);

    // Part 1
    //println!("{}", best_path(&env, 30, false));

    // Part 2
    println!("{}", best_path(&env, 26, true));
}

fn best_path(env: &Environment, max_time: i8, elephant: bool) -> i16 {
    // Since the graph of states is a directed acyclic graph (as you can't go back in time :P)
    // we can find a "shortest path" using negative values!
    let helpers = if elephant { 2 } else { 1 };

    let states = TopologicalOrdering::new(env, max_time, helpers);
    let mut pressures: HashMap<State, i16> = HashMap::new();
    let mut time = -1;

    pressures.insert(State::start(helpers), 0);

    for state in states {
        if state.time > time {
            time = state.time;
            println!("t = {time}");
        }
        let current_pressure = pressures.remove(&state).unwrap();

        if state.done {
            return -1 * current_pressure;
        }

        let pressure_released: i16 = state
            .open_valves
            .iter()
            .map(|label| env.get_valve(label).unwrap().rate)
            .sum();

        for new_state in state.next_states(env, max_time) {
            let new_time = new_state.time;

            if !pressures.contains_key(&new_state)
                || *pressures.get(&new_state).unwrap() > current_pressure - pressure_released * (new_time - state.time) as i16
            {
                pressures.insert(new_state, current_pressure - pressure_released * (new_time - state.time) as i16);
            }
        }
    }

    panic!("Ran out of states!");
}
