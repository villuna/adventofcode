use itertools::Itertools;
use std::{collections::HashSet, hash::Hash};

pub fn day_six(input: String) {
    part_one(&input);
    part_two(&input);
}

// This is a generic for no reason other than it's fun
fn all_unique<T: Eq + Copy + Hash>(slice: &[T]) -> bool {
    let mut set = HashSet::new();

    for elem in slice.iter() {
        if !set.insert(elem) {
            return false;
        }
    }

    return true;
}

fn find_unique_packet(signal: &str, packet_size: usize) -> Option<usize> {
    for (i, slice) in signal
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect_vec()
        .windows(packet_size)
        .enumerate()
    {
        if all_unique(slice) {
            return Some(i + packet_size);
        }
    }

    None
}

fn part_one(signal: &str) {
    println!("{}", find_unique_packet(signal, 4).unwrap());
}

fn part_two(signal: &str) {
    println!("{}", find_unique_packet(signal, 14).unwrap());
}
