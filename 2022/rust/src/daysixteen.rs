use derivative::Derivative;
use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
};

use itertools::Itertools;

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

type Label = [char; 2];

const START: Label = ['A', 'A'];
const P1_MAX_TIME: u8 = 31;
const P2_MAX_TIME: u8 = 27;

#[derive(Debug)]
struct Valve {
    rate: u16,
    edges: HashMap<Label, u16>,
}

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
        // Use dijkstra's algorithm to calculate the shortest path between nodes
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

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
enum Action {
    OpeningValve,
    Moving(Label, u16),
    Done,
}

impl PartialOrd for Action {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // This implementation assumes the actions are coming from the same position
        // if moving, the valve at the current position is assumed to be open
        match self {
            Action::OpeningValve => {
                if self == other {
                    Some(Ordering::Equal)
                } else {
                    Some(Ordering::Less)
                }
            }
            Action::Moving(target, progress) => match other {
                Action::Moving(other_target, other_progress) => {
                    if target == other_target {
                        Some(progress.cmp(other_progress))
                    } else {
                        None
                    }
                }

                Action::Done => Some(Ordering::Less),

                Action::OpeningValve => Some(Ordering::Greater),
            },
            Action::Done => {
                if self == other {
                    Some(Ordering::Equal)
                } else {
                    Some(Ordering::Greater)
                }
            }
        }
    }
}

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
        self.pressure >= other.pressure
    }

    fn edges(&self, env: &HashMap<Label, Valve>, max_time: u8) -> Vec<LoneState> {
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
            Action::Done => {
                vec![Self {
                    pressure,
                    timestamp,
                    ..self.clone()
                }]
            }

            Action::OpeningValve => {
                let mut valves = self.open_valves.clone();
                valves.insert(self.position);

                let next = env
                    .keys()
                    .filter(|&label| {
                        *label != START
                            && !valves.contains(label)
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

            Action::Moving(target, distance) => {
                let distance_to_cover =
                    *env.get(&self.position).unwrap().edges.get(&target).unwrap();

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

#[derive(Derivative, Debug, Clone, Eq)]
#[derivative(Hash)]
struct HelpedState {
    timestamp: u8,
    positions: [Label; 2],
    open_valves: BTreeSet<Label>,
    actions: [Action; 2],
    #[derivative(Hash = "ignore")]
    pressure: u16,
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

    /*
    fn eq_real(&self, other: &Self) -> bool {
        let positions_equal = (self.positions[0] == other.positions[0]
            && self.positions[1] == other.positions[1])
            || (self.positions[0] == other.positions[1] && self.positions[1] == other.positions[0]);
        let actions_equal = (self.actions[0] == other.actions[0]
            && self.actions[1] == other.actions[1])
            || (self.actions[0] == other.actions[1] && self.actions[1] == other.actions[0]);

        self.timestamp == other.timestamp
            && positions_equal
            && self.open_valves == other.open_valves
            && actions_equal
            && self.pressure == other.pressure
    }
    */

    fn is_better_than(&self, other: &Self) -> bool {
        self.pressure >= other.pressure
    }

    fn edges(&self, env: &HashMap<Label, Valve>) -> Vec<Self> {
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

        if matches!(self.actions[0], Action::OpeningValve)
            && matches!(self.actions[1], Action::OpeningValve)
        {
            let mut valves = self.open_valves.clone();
            valves.insert(self.positions[0]);
            valves.insert(self.positions[1]);

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
                vec![Self { timestamp, positions: self.positions, open_valves: valves, actions: [Action::Done, Action::Done], pressure }]
            } else {
                res
            }
        } else {
            let (l1, l2) = self.mitosis();

            let e1 = l1.edges(env, P2_MAX_TIME);
            let e2 = l2.edges(env, P2_MAX_TIME);

            e1.iter().cartesian_product(e2.iter())
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
            }
        )
    }

    fn fusion_dance(l1: &LoneState, l2: &LoneState) -> Self {
        Self {
            timestamp: l1.timestamp,
            positions: [l1.position, l2.position],
            open_valves: l1.open_valves.union(&l2.open_valves).cloned().collect::<BTreeSet<_>>(),
            actions: [l1.action, l2.action],
            pressure: l1.pressure,
        }
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
    let env = parse_environment(&input);
    println!("part 1: {}", part_one(&env));
    println!("part 2: {}", part_two(&env));
}

fn part_one(env: &HashMap<Label, Valve>) -> u16 {
    // Use BFS to find the optimal solution
    let mut max = 0;
    let mut max_spare_size = 0;
    // Our frontier contains all the states that we want to consider expanding next.
    let mut frontier: HashSet<LoneState> = HashSet::new();
    let mut spare: HashSet<LoneState> = HashSet::new();

    let mut level = 0;

    for edge in env.keys() {
        if *edge != START {
            frontier.insert(LoneState::start(*edge));
            spare.insert(LoneState::start(*edge));
        }
    }

    loop {
        let state = if !frontier.is_empty() {
            frontier.iter().next().unwrap()
        } else if !spare.is_empty() {
            level += 1;
            if spare.len() > max_spare_size {
                max_spare_size = spare.len();
            }
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

        let edges = state.edges(env, P1_MAX_TIME);

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

    println!("max spare size: {max_spare_size}");
    max
}

fn part_two(env: &HashMap<Label, Valve>) -> u16 {
    // Use BFS to find the optimal solution
    let mut max = 0;
    let mut max_spare_size = 0;
    // Our frontier contains all the states that we want to consider expanding next.
    let mut frontier: HashSet<HelpedState> = HashSet::new();
    let mut spare: HashSet<HelpedState> = HashSet::new();

    let mut level = 0;

    let keys = env.keys().cloned().collect::<Vec<_>>();

    for i1 in 1..keys.len() {
        if keys[i1] == START {
            continue;
        }

        for i2 in 0..i1 {
            if keys[i2] == START {
                continue;
            }

            frontier.insert(HelpedState::start([keys[i1], keys[i2]]));
            spare.insert(HelpedState::start([keys[i1], keys[i2]]));
        }
    }

    loop {
        let state = if !frontier.is_empty() {
            frontier.iter().next().unwrap()
        } else if !spare.is_empty() {
            level += 1;
            if spare.len() > max_spare_size {
                max_spare_size = spare.len();
            }
            println!("part {level} - spare size {}", spare.len());
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

        let edges = state.edges(env);

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

    println!("max spare size: {max_spare_size}");
    max
}
