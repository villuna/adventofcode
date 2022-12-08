use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn day_ten(part: usize, filename: String) {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    if part == 1 {
        let (_, sum) = part_one(contents);
        println!("The answer is {}", sum);
    } else {
        part_two(contents);
    }
}

fn part_one(contents: String) -> (Vec<String>, u32) {
    let lines = contents
        .split('\n')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let mut valid_lines = Vec::new();

    let mut sum = 0;

    'outer: for line in lines {
        let mut stack = Vec::new();
        let opening = ['(', '[', '{', '<'];

        let mut map = HashMap::new();
        map.insert('(', ')');
        map.insert('[', ']');
        map.insert('{', '}');
        map.insert('<', '>');

        for c in line.chars() {
            if opening.contains(&c) {
                stack.push(c);
            } else {
                if let Some(open_char) = stack.pop() {
                    if *map.get(&open_char).unwrap() != c {
                        sum += points(c);
                        continue 'outer;
                    }
                } else {
                    sum += points(c);
                    continue 'outer;
                }
            }
        }

        valid_lines.push(line.to_string());
    }

    (valid_lines, sum)
}

fn points(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

fn part_two(contents: String) {
    let (lines, _) = part_one(contents);
    let mut scores = Vec::new();

    for line in lines {
        let mut stack = Vec::new();
        let opening = ['(', '[', '{', '<'];

        for c in line.chars() {
            if opening.contains(&c) {
                stack.push(c);
            } else {
                let _ = stack.pop();
            }
        }

        let mut score = 0;

        while let Some(c) = stack.pop() {
            score *= 5;
            score += points(c) as u64;
        }

        scores.push(score);
    }

    scores.sort();
    println!("Scores: {:?}", scores);
    println!("The answer is {}", scores[scores.len() / 2]);
}
