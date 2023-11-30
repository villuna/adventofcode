// THIS FINALLY WORKS OH MY GOD
// I spent so much time on this last year and it's what ultimately stopped me
// but it's finally finished
use derivative::Derivative;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::{
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    hash::Hash,
    sync::OnceLock,
};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressDrawTarget};

use itertools::Itertools;

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

type Label = [char; 2];

// This will be constant and the info is important throughout the problem, so I'm making it static
// oncelock means we can set it once at runtime then get immutable references to it whenever!
static ENV: OnceLock<HashMap<Label, Valve>> = OnceLock::new();

const START: Label = ['A', 'A'];
const P1_MAX_TIME: u8 = 31;
const P2_MAX_TIME: u8 = 27;

#[derive(Debug)]
struct Valve {
    rate: u16,
    edges: HashMap<Label, u16>,
}

// Read the graph from file and condense it. The condensed graph will contain only the nodes that
// are "important" - the start node and all the nodes with nonzero pressure rate, and then the
// edges between the nodes will be weighted by the shortest path between them.
fn parse_environment(input: &str) -> HashMap<Label, Valve> {
    let re = regex!(
        r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z]{2}(:?, [A-Z]{2})*)"
    );
    let mut graph: HashMap<Label, (u16, Vec<Label>)> = HashMap::new();

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

        graph.insert(label, (rate, edges));
    }

    // Now calculate distances between important valves
    let mut reduced_graph = HashMap::new();
    let important_valves = graph
        .iter()
        .filter(|&(label, (rate, _edges))| *label == START || *rate > 0);

    for (label, (rate, _edges)) in important_valves {
        // Use dijkstra's algorithm to calculate the shortest path between this and every other
        // node
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut distances: HashMap<[char; 2], u16> = HashMap::new();
        queue.push_back((*label, 0));

        while let Some((next_label, distance)) = queue.pop_front() {
            visited.insert(next_label);

            if next_label != *label {
                distances.insert(next_label, distance);
            }

            let edges = graph.get(&next_label).unwrap().1.iter();

            for edge in edges {
                if !visited.contains(edge) {
                    queue.push_back((*edge, distance + 1));
                }
            }
        }

        // Only keep the important nodes (not including start - we never wanna go back to it)
        distances.retain(|label, _distance| graph.get(label).unwrap().0 > 0 && *label != START);

        reduced_graph.insert(
            *label,
            Valve {
                rate: *rate,
                edges: distances,
            },
        );
    }

    reduced_graph
}

/// Represents the state of a "helper" (you or elephant).
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Action {
    OpeningValve,
    // first argument is destination, second argument is progress
    Moving(Label, u16),
    // This means there's nothing left for you to do (every valve is either open or too far away)
    Done,
}

impl Hash for Action {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        #[derive(Hash)]
        enum HashAction {
            OpeningValve,
            Moving(Label),
            Done,
        }

        let ha = match self {
            Action::OpeningValve => HashAction::OpeningValve,
            Action::Moving(dest, _) => HashAction::Moving(*dest),
            Action::Done => HashAction::Done,
        };

        ha.hash(state);
    }
}

/// A state where there is only a single helper
#[derive(Derivative, Debug, Clone, Eq)]
#[derivative(Hash)]
struct LoneState {
    timestamp: u8,
    position: Label,
    open_valves: BTreeSet<Label>,
    action: Action,
    #[derivative(Hash = "ignore")]
    pressure: u16,
}

impl PartialEq for LoneState {
    // In this case, "equal" means "comparable with a total ordering"
    fn eq(&self, other: &Self) -> bool {
        self.is_better_than(other) || other.is_better_than(self)
    }
}

impl LoneState {
    fn start(target: Label) -> Self {
        Self {
            timestamp: 1,
            position: START,
            open_valves: BTreeSet::new(),
            action: Action::Moving(target, 1),
            pressure: 0,
        }
    }

    fn is_better_than(&self, other: &LoneState) -> bool {
        // This function is much more complicated for HelpedState.
        // This doesnt do much culling but we don't need to because part 1 is very easy and doesnt
        // need too much optimisation
        self.pressure >= other.pressure
    }

    // returns all the possible states you can go to from the current state.
    // actually, it's more like "all the states that make sense". For example, we cant move to a
    // valve that's already open, because that would be stupid.
    fn edges(&self, max_time: u8) -> Vec<LoneState> {
        let env = ENV.get().unwrap();

        if self.timestamp == max_time {
            return Vec::new();
        }

        let pressure = self.pressure
            + self
                .open_valves
                .iter()
                .map(|valve| env.get(valve).unwrap().rate)
                .sum::<u16>();
        let timestamp = self.timestamp + 1;

        match self.action {
            // If the helper is done, it doesnt do anything
            Action::Done => {
                vec![Self {
                    pressure,
                    timestamp,
                    ..self.clone()
                }]
            }

            // If the helper just opened a valve, it needs to move to a new valve that isnt open.
            Action::OpeningValve => {
                let mut valves = self.open_valves.clone();
                valves.insert(self.position);

                let next = env
                    .keys()
                    .filter(|&label| {
                        // Don't go to AA
                        *label != START
                            // Don't move towards yourself
                            && !valves.contains(label)
                            // Only move towards nodes that are close enough to get to
                            && *env.get(&self.position).unwrap().edges.get(label).unwrap()
                                < (max_time - timestamp) as u16
                    })
                    .map(|label| Self {
                        open_valves: valves.clone(),
                        pressure,
                        timestamp,
                        action: Action::Moving(*label, 1),
                        position: self.position,
                    })
                    .collect::<Vec<_>>();

                if next.is_empty() {
                    // If we can't move to any states, we are done
                    vec![Self {
                        pressure,
                        timestamp,
                        action: Action::Done,
                        open_valves: valves,
                        position: self.position,
                    }]
                } else {
                    next
                }
            }

            // If the helper is moving, it moves towards its goal, and if it gets there it starts
            // opening the valve.
            Action::Moving(target, distance) => {
                let distance_to_cover =
                    *env.get(&self.position).unwrap_or_else(|| panic!("couldnt find {:?}", self.position))
                        .edges.get(&target).unwrap_or_else(|| panic!("couldnt find {:?}", target));

                if distance < distance_to_cover {
                    vec![Self {
                        pressure,
                        timestamp,
                        action: Action::Moving(target, distance + 1),
                        ..self.clone()
                    }]
                } else if distance == distance_to_cover {
                    vec![Self {
                        pressure,
                        timestamp,
                        action: Action::OpeningValve,
                        position: target,
                        ..self.clone()
                    }]
                } else {
                    panic!("should not have overshot the distance");
                }
            }
        }
    }
}

// A state where there are two helpers
#[derive(Debug, Clone, Eq)]
struct HelpedState {
    timestamp: u8,
    positions: [Label; 2],
    open_valves: BTreeSet<Label>,
    actions: [Action; 2],
    pressure: u16,
}

impl Hash for HelpedState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if self.actions != [Action::Done, Action::Done] {
            self.positions.hash(state);
            self.open_valves.hash(state);
        }

        self.actions.hash(state);
    }
}

impl PartialEq for HelpedState {
    fn eq(&self, other: &Self) -> bool {
        self.is_better_than(other) || other.is_better_than(self)
    }
}

impl HelpedState {
    fn start(targets: [Label; 2]) -> Self {
        Self {
            timestamp: 1,
            positions: [START, START],
            open_valves: BTreeSet::new(),
            actions: [Action::Moving(targets[0], 1), Action::Moving(targets[1], 1)],
            pressure: 0,
        }
    }

    fn is_better_than(&self, other: &Self) -> bool {
        // This function will only be called if there is a hash collision. We therefore can assume
        // that the timestamps are equal, and the positions and open valves are *probably* equal,
        // as well as some other conditions.

        // If both states are done, we can calculate what the pressure will be at the end. The
        // highest pressure wins.
        //
        // We do this because our hash function will coalesce all states where both helpers are done.
        if self.actions == [Action::Done, Action::Done]
            && other.actions == [Action::Done, Action::Done]
        {
            let env = ENV.get().unwrap();
            let self_rate = self
                .open_valves
                .iter()
                .map(|label| env.get(label).unwrap().rate)
                .sum::<u16>();

            let other_rate = other
                .open_valves
                .iter()
                .map(|label| env.get(label).unwrap().rate)
                .sum::<u16>();

            return (P2_MAX_TIME - self.timestamp) as u16 * self_rate + self.pressure
                >= (P2_MAX_TIME - other.timestamp) as u16 * other_rate + other.pressure;
        }

        // If all helpers are moving to the same destination (and all else is equal), the winner is
        // the one that is closer to its goal. We do this because the hash function will coalesce
        // all states where the helpers are moving towards the same destinations (respectively).
        //
        // I'm sorry for the nested if, I have to do this to destructure a bunch of enums at once
        // because doing them all in one condition is unstable
        // rustlang pls fix
        if let [Action::Moving(d00, p00), Action::Moving(d01, p01)] = self.actions {
            if let [Action::Moving(d10, p10), Action::Moving(d11, p11)] = other.actions {
                if d00 == d10 && p00 >= p10 && d01 == d11 && p01 >= p11 {
                    return true;
                }
            }
        }
        
        // Otherwise we can just check which has the better pressure
        self.pressure >= other.pressure
    }

    fn edges(&self) -> Vec<Self> {
        let env = ENV.get().unwrap();

        if self.timestamp == P2_MAX_TIME {
            return Vec::new();
        }

        let pressure = self.pressure
            + self
                .open_valves
                .iter()
                .map(|valve| env.get(valve).unwrap().rate)
                .sum::<u16>();
        let timestamp = self.timestamp + 1;

        // This is the only time when the fact that there are two helpers impacts what states are
        // possible, so it's written out by hand.
        if matches!(self.actions[0], Action::OpeningValve)
            && matches!(self.actions[1], Action::OpeningValve)
        {
            let mut valves = self.open_valves.clone();
            valves.insert(self.positions[0]);
            valves.insert(self.positions[1]);

            // Both helpers have just opened a valve, so they are going to head to different
            // targets. These are the sets of next targets for helper 0 and helper 1
            let next0 = env.keys().filter(|&label| {
                *label != START
                    && !valves.contains(label)
                    && *env
                        .get(&self.positions[0])
                        .unwrap()
                        .edges
                        .get(label)
                        .unwrap()
                        < (P2_MAX_TIME - timestamp) as u16
            });

            let next1 = env.keys().filter(|&label| {
                *label != START
                    && !valves.contains(label)
                    && *env
                        .get(&self.positions[1])
                        .unwrap()
                        .edges
                        .get(label)
                        .unwrap()
                        < (P2_MAX_TIME - timestamp) as u16
            });

            let mut res = Vec::new();
            for (t1, t2) in next0.cartesian_product(next1) {
                if t1 == t2 {
                    continue;
                }

                res.push(Self {
                    timestamp,
                    positions: self.positions,
                    open_valves: valves.clone(),
                    actions: [Action::Moving(*t1, 1), Action::Moving(*t2, 1)],
                    pressure,
                });
            }

            if res.is_empty() {
                vec![Self {
                    timestamp,
                    positions: self.positions,
                    open_valves: valves,
                    actions: [Action::Done, Action::Done],
                    pressure,
                }]
            } else {
                res
            }
        } else {
            // For all these cases, we can just pretend that each helper is alone and combine their
            // states at the end
            let (l1, l2) = self.mitosis();

            let e1 = l1.edges(P2_MAX_TIME);
            let e2 = l2.edges(P2_MAX_TIME);

            e1.iter()
                .cartesian_product(e2.iter())
                .map(|(s1, s2)| Self::fusion_dance(s1, s2))
                .collect::<Vec<_>>()
        }
    }

    fn mitosis(&self) -> (LoneState, LoneState) {
        (
            LoneState {
                timestamp: self.timestamp,
                position: self.positions[0],
                open_valves: self.open_valves.clone(),
                action: self.actions[0],
                pressure: self.pressure,
            },
            LoneState {
                timestamp: self.timestamp,
                position: self.positions[1],
                open_valves: self.open_valves.clone(),
                action: self.actions[1],
                pressure: self.pressure,
            },
        )
    }

    fn fusion_dance(l1: &LoneState, l2: &LoneState) -> Self {
        assert_eq!(l1.timestamp, l2.timestamp);
        assert_eq!(l1.pressure, l2.pressure);
        Self {
            timestamp: l1.timestamp,
            positions: [l1.position, l2.position],
            open_valves: l1
                .open_valves
                .union(&l2.open_valves)
                .cloned()
                .collect::<BTreeSet<_>>(),
            actions: [l1.action, l2.action],
            pressure: l1.pressure,
        }
    }
}

pub fn day_sixteen(input: String) {
    let env = parse_environment(&input);
    ENV.set(env).expect("should be able to set environment");

    println!("note the answers this program gives are not correct 100% of the time. the algorithm is non-deterministic.");
    println!("if you get the wrong answer, just run it again, it'll probably work eventually");
    println!("part 1: {}", part_one());
    println!("part 2: {}", part_two());
}

fn part_one() -> u16 {
    let env = ENV.get().unwrap();

    env.keys()
        .cloned()
        .filter(|label| *label != START)
        .collect::<Vec<_>>()
        .into_par_iter()
        .progress_with(ProgressBar::with_draw_target(Some(env.keys().len() as u64), ProgressDrawTarget::stdout()))
        .map(|start| {
            // The core of this algorithm is using a collision-heavy hash function to cull the state space.
            // The Hash and PartialEq implementations are such that two states are equal implies they are
            // *comparable* - that is, one of them is objectively better than another. Therefore, only one
            // of them need get a spot in the frontier. The best one will win after all the states are
            // expanded.
            //
            // We use two hashmaps to simulate a queue.
            // The frontier is all the states at the current timestep. We don't care about the order the
            // states are in within the timesteps, we just care that we expand all of them before we expand
            // their child states.
            let mut frontier: HashSet<LoneState> = HashSet::new();
            // The spare contains all the child states that we will want to expand later. Once the frontier
            // empties, we swap them out and make this the new frontier. This is how the "queue" works.
            let mut spare: HashSet<LoneState> = HashSet::new();
            let mut max = 0;

            frontier.insert(LoneState::start(start));

            // for superspeed we split up the start states and expand each on a separate thread using rayon
            // this isn't optimally efficient, but it is faster
            // the more compute power we throw at the problem more than makes up for the slightly harder problem
            loop {
                let state = if !frontier.is_empty() {
                    frontier.iter().next().unwrap()
                } else if !spare.is_empty() {
                    // If the frontier is empty we want to swap it for the spare
                    // then expand that
                    std::mem::swap(&mut spare, &mut frontier);
                    frontier.iter().next().unwrap()
                } else {
                    // If the frontier and spare are both empty, we've searched every state
                    break;
                }
                .clone();

                frontier.remove(&state);

                if state.pressure > max {
                    max = state.pressure;
                }

                let edges = state.edges(P1_MAX_TIME);

                for next_state in edges {
                    match spare.get(&next_state) {
                        // Here is the important part
                        // if there exists a comparable state in the frontier, then we compare the two
                        // and keep only the best state in the frontier.
                        Some(entry) => {
                            if next_state.is_better_than(entry) {
                                spare.replace(next_state.clone());
                            } else {
                                continue;
                            }
                        }

                        // otherwise, we just insert our new state
                        None => {
                            spare.insert(next_state.clone());
                        }
                    }
                }
            }

            max
        })
        .max()
        .unwrap()
}

fn part_two() -> u16 {
    // See part_one for comments explaining the process
    let env = ENV.get().unwrap();

    // Start by moving towards all the combinations of two unique valves.
    // the order doesnt matter. man and elephant are equals 🧑🤝🐘
    let start_states = env
        .keys()
        .cloned()
        .filter(|label| *label != START)
        .tuple_combinations()
        .map(|(k1, k2)| [k1, k2])
        .collect::<Vec<_>>();
    let len = start_states.len();
    
    start_states
        .into_par_iter()
        .progress_with(ProgressBar::with_draw_target(Some(len as u64), ProgressDrawTarget::stdout()))
        .map(|start| {
            let mut max = 0;
            let mut frontier: HashSet<HelpedState> = HashSet::new();
            let mut spare: HashSet<HelpedState> = HashSet::new();

            frontier.insert(HelpedState::start(start));

            loop {
                let state = if !frontier.is_empty() {
                    frontier.iter().next().unwrap()
                } else if !spare.is_empty() {
                    std::mem::swap(&mut spare, &mut frontier);
                    frontier.iter().next().unwrap()
                } else {
                    break;
                }
                .clone();

                frontier.remove(&state);

                if state.pressure > max {
                    max = state.pressure;
                }

                let edges = state.edges();

                for next_state in edges {
                    match spare.get(&next_state) {
                        Some(entry) => {
                            if next_state.is_better_than(entry) {
                                spare.replace(next_state.clone());
                            } else {
                                continue;
                            }
                        }

                        None => {
                            spare.insert(next_state.clone());
                        }
                    }
                }
            }

            max
        })
        .max()
        .unwrap()
}
