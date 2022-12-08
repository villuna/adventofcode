use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

pub fn day_fourteen(part: usize, filename: String) {
    let mut contents = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut contents).unwrap();

    if part == 1 {
        general_solution(contents, 10);
    } else {
        general_solution(contents, 40);
    }
}

fn general_solution(contents: String, iterations: usize) {
    let mut lines = contents.lines();
    let template = lines.next().unwrap().to_string();

    let rule_chars = lines.filter(|s| !s.is_empty()).map(|s| -> ((char, char), char) {
        let mut splits = s.split(" -> ");

        let mut pair_chars = splits.next().unwrap().chars();
        let pair = (pair_chars.next().unwrap(), pair_chars.next().unwrap());

        let insert_char = splits.next().unwrap().chars().next().unwrap();

        (pair, insert_char)
    });

    let mut rules = HashMap::new();

    for rule in rule_chars {
        rules.insert(rule.0, rule.1);
    }

    // Counts how many occurences of a given pair there are in the string
    let mut polymer: HashMap<(char, char), usize> = HashMap::new();

    for cs in template.chars().collect::<Vec<char>>().windows(2) {
        polymer.insert((cs[0], cs[1]), 1);
    }

    for _ in 0..iterations {
        let mut temp = HashMap::new();

        for (pair, count) in polymer.iter() {
            match rules.get(pair) {
                Some(&insert) => {
                    let p1 = (pair.0, insert);
                    let p2 = (insert, pair.1);

                    *temp.entry(p1).or_insert(0) += count;
                    *temp.entry(p2).or_insert(0) += count;
                },

                None => {
                    *temp.entry(*pair).or_insert(0) += count;
                }
            }
        }

        polymer = temp;
    }

    let mut char_counts = HashMap::new();

    for (pair, count) in polymer.iter() {
        *char_counts.entry(pair.0).or_insert(0) += count;
        *char_counts.entry(pair.1).or_insert(0) += count;
    }

    *char_counts.get_mut(&template.chars().next().unwrap()).unwrap() += 1;
    *char_counts.get_mut(&template.chars().last().unwrap()).unwrap() += 1;

    let mut counts = char_counts.into_values().collect::<Vec<usize>>();
    counts.sort();

    let answer = (counts[counts.len() - 1] - counts[0]) / 2;

    println!("The answer is {}", answer);
}

// Do not run this function unless you want to use up all of your RAM
#[allow(unused)]
fn part_two_naive(contents: String) {
    let mut lines = contents.lines();

    let template = lines.next().unwrap().to_string();

    let rule_strs = lines.filter(|s| !s.is_empty()).map(|s| -> (String, String) {
        let mut splits = s.split(" -> ");
        (splits.next().unwrap().to_string(), splits.next().unwrap().to_string())
    });

    let mut rules = HashMap::new();

    for rule in rule_strs {
        rules.insert(rule.0, rule.1);
    }

    let mut answer = template;

    for _ in 0..40 {
        let mut temp = String::new();

        for cs in answer.chars().collect::<Vec<char>>().windows(2) {
            temp.push(cs[0]);

            let mut pair = String::with_capacity(2);
            pair.insert(0, cs[0]);
            pair.insert(1, cs[1]);

            if let Some(c) = rules.get(&pair) {
                temp.push_str(c);
            }
        }

        temp.push(answer.chars().last().unwrap());
        answer = temp;
    }

    let mut char_map = HashMap::new();

    for c in answer.chars() {
        *char_map .entry(c).or_insert(0usize) += 1;
    }

    let mut counts = char_map.into_values().collect::<Vec<usize>>();
    counts.sort();

    let answer = counts[counts.len() - 1] - counts[0];

    println!("The answer is {}", answer);
}
