use itertools::Itertools;

pub fn day_three(input: String) {
    println!("The answer to part one is {}", part_one(&input));
    println!("The answer to part two is {}", part_two(&input));
}

fn priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else if c.is_ascii_uppercase() {
        c as u32 - 'A' as u32 + 27
    } else {
        panic!("Not an alphabetical character!");
    }
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|string| {
            let mid = string.len() / 2;
            let (pack1, pack2) = (&string[..mid], &string[mid..]);

            for c in pack1.chars() {
                if pack2.contains([c]) {
                    return c;
                }
            }

            panic!("No common character found!");
        })
        .map(priority)
        .sum()
}

fn part_two(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .tuples::<(_, _, _)>()
        .map(|(p1, p2, p3)| {
            for c in p1.chars() {
                if p2.contains([c]) && p3.contains([c]) {
                    return c;
                }
            }

            panic!("No badge found!");
        })
        .map(priority)
        .sum()
}
