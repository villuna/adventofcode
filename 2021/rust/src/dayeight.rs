use std::fs::File;
use std::io::Read;

pub fn day_eight(part: usize, filename: String) {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    if part == 1 {
        part_one(contents);
    } else {
        part_two(contents);
    }
}

fn part_one(contents: String) {
    let answer = contents
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.split(" | ").collect::<Vec<&str>>()[1])
        .map(|s| s.split(" ").collect::<Vec<&str>>())
        .flatten()
        .filter(|s| [2, 3, 4, 7].contains(&s.len()))
        .collect::<Vec<&str>>()
        .len();

    println!("{:?}", answer);
}

fn part_two(contents: String) {
    let lines = contents
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.split(" | ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let patterns = lines
        .iter()
        .map(|v| v[0].split(" ").collect::<Vec<&str>>())
        .map(parse_pattern)
        .collect::<Vec<[&str; 10]>>();

    let answer: usize = lines
        .into_iter()
        .map(|v| v[1].split(" ").collect::<Vec<&str>>())
        .enumerate()
        .map(|(i, v)| {
            v.iter()
                .map(|s| {
                    let mut num = 0;

                    for j in 0..10 {
                        let mut pattern = patterns[i][j].chars();
                        let mut digit = s.chars();

                        if pattern.all(|c| s.contains(c))
                            && digit.all(|c| patterns[i][j].contains(c))
                        {
                            num = j;
                            break;
                        }
                    }

                    num
                })
                .collect::<Vec<usize>>()
        })
        .map(|v| 1000 * v[0] + 100 * v[1] + 10 * v[2] + v[3])
        .sum();

    println!("The answer is {}", answer);
}

fn parse_pattern(pattern: Vec<&str>) -> [&str; 10] {
    let mut ssd: [char; 7] = ['\0'; 7];
    let mut output = [""; 10];

    // Populate output with the characters we know are of unique length
    for s in pattern.iter() {
        match s.len() {
            2 => output[1] = s,
            3 => output[7] = s,
            4 => output[4] = s,
            7 => output[8] = s,
            _ => {}
        }
    }

    // Segment a will be the character in 7 but not in 4
    for c in output[7].chars() {
        if !output[4].contains(c) {
            ssd[0] = c;
        }
    }

    // Segment f is contained in 9 digits, while segment c is contained
    // in 8 digits.
    {
        let mut count0 = 0;
        let char0 = output[1].chars().nth(0).unwrap();
        let char1 = output[1].chars().nth(1).unwrap();

        for s in pattern.iter() {
            if s.contains(char0) {
                count0 += 1;
            }
        }

        if count0 == 9 {
            ssd[5] = char0;
            ssd[2] = char1;
        } else {
            ssd[5] = char1;
            ssd[2] = char0;
        }
    }

    // 5 is the number that contains 5 segments but not segment c
    // 2 and 3 are the others.
    for s in pattern.iter().filter(|s| s.len() == 5) {
        if !s.contains(ssd[2]) {
            output[5] = s;
        } else {
            if !s.contains(ssd[5]) {
                output[2] = s;
            } else {
                output[3] = s;
            }
        }
    }

    // From this we can deduce segments e and b
    for c in output[2].chars() {
        if !output[3].contains(c) {
            ssd[4] = c;
        }
    }

    for c in output[5].chars() {
        if !output[3].contains(c) {
            ssd[1] = c;
        }
    }

    // 0, 9 and 6 each have 6 segments.
    // 0 and 9 contain c and f, but 9 doesn't contain e.
    for s in pattern.iter().filter(|s| s.len() == 6) {
        if !s.contains(ssd[2]) {
            output[6] = s;
        } else {
            if !s.contains(ssd[4]) {
                output[9] = s;
            } else {
                output[0] = s;
            }
        }
    }

    output
}
