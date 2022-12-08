use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

pub fn day_twelve(part: usize, filename: String) {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let graph = Graph::from_contents(&contents);
    println!("The answer is {}", graph.get_paths(part).len());
}

#[derive(Debug)]
struct Graph<'a>(HashMap<&'a str, HashSet<&'a str>>);

fn is_lower(string: &str) -> bool {
    string.chars().all(|c| matches!(c, 'a'..='z'))
}

// From stackoverflow
fn has_unique_lower_strings(iter: &Vec<&str>) -> bool {
    let mut uniq = HashSet::new();
    iter.iter()
        .filter(|&s| is_lower(s))
        .all(move |&x| uniq.insert(x))
}

impl<'a> Graph<'a> {
    fn from_contents(contents: &'a String) -> Graph {
        let lines = contents
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(|s| {
                let mut splits = s.split('-');
                (splits.next().unwrap(), splits.next().unwrap())
            })
            .collect::<Vec<(&str, &str)>>();

        let mut graph = HashMap::new();

        for (v1, v2) in lines {
            graph.entry(v1).or_insert(HashSet::new()).insert(v2);
            graph.entry(v2).or_insert(HashSet::new()).insert(v1);
        }

        Graph(graph)
    }

    /// Returns all the paths from start to end that goes through each small
    /// cave at most once.
    fn get_paths(&self, part: usize) -> Vec<Vec<&'a str>> {
        self.get_paths_helper(&vec!["start"], part)
    }

    fn get_paths_helper(&self, current_path: &Vec<&'a str>, part: usize) -> Vec<Vec<&'a str>> {
        if current_path.ends_with(&["end"]) {
            return vec![current_path.clone()];
        }

        let mut res = vec![];
        let incident = self
            .incident_vertices(current_path[current_path.len() - 1])
            .unwrap();

        for node in incident.iter() {
            let is_valid = match part {
                1 => !is_lower(*node) || !current_path.contains(node),
                2 => {
                    *node != "start"
                        && (has_unique_lower_strings(current_path)
                            || !(is_lower(*node) && current_path.contains(node)))
                }
                _ => unreachable!(),
            };

            if !is_valid {
                continue;
            }

            let mut new_path = current_path.clone();
            new_path.push(*node);
            res.append(&mut self.get_paths_helper(&new_path, part));
        }

        res
    }

    fn incident_vertices(&self, node: &'a str) -> Option<&HashSet<&'a str>> {
        self.0.get(&node)
    }
}
