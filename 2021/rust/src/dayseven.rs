use std::fs::File;
use std::io::Read;

pub fn day_seven(part: usize, filename: String) {
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
    let mut numbers = contents
        .split("\n")
        .nth(0)
        .unwrap()
        .split(",")
        .filter_map(|s| i32::from_str_radix(s, 10).ok())
        .collect::<Vec<i32>>();

    println!("numbers: {:?}", contents.split(","));

    numbers.sort();

    let median = numbers[numbers.len() / 2];

    let answer: i32 = numbers.iter().map(|x| (x - median).abs()).sum();
    println!("The answer is {}", answer);
}

fn part_two(contents: String) {
    let numbers = contents
        .split("\n")
        .nth(0)
        .unwrap()
        .split(",")
        .filter_map(|s| i32::from_str_radix(s, 10).ok())
        .collect::<Vec<i32>>();

    let mean = numbers.iter().sum::<i32>() / numbers.len() as i32;

    // To be entirely sure about the answer, just try [mean - 1, mean + 1].

    let fuel_usage = |x: i32| {
        numbers
            .iter()
            .map(|y| {
                let n = (y - x).abs();
                (n * (n + 1)) / 2
            })
            .sum::<i32>()
    };

    let answer = ((mean - 1)..(mean + 2)).map(fuel_usage).min().unwrap();

    println!("The answer is {}", answer);
}
