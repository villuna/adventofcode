use std::fs::File;
use std::io::Read;

pub fn day_one(part: usize, filename: String) {
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
    /*
    let splits: Vec<i32> = contents.split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let count: i32 = splits.windows(2)
        .map(|w| (w[0] < w[1]) as i32)
        .sum();
    */

    // That solution is good but this is epic rust mode

    let count: i32 = contents
        .split("\n")
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>()
        .windows(2)
        .map(|w| (w[0] < w[1]) as i32)
        .sum();

    println!("The count is {}", count);
}

fn part_two(contents: String) {
    let count: i32 = contents
        .split("\n")
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>()
        .windows(3)
        .map(|s| s.into_iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .map(|w| (w[0] < w[1]) as i32)
        .sum();

    println!("The count is {}", count);
}
