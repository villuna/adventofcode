use crate::AOContext;
use itertools::Itertools;

// Checks that a line is safe. Takes in a list of distances
fn check_distances(line: impl Iterator<Item = i32> + Clone) -> bool {
    let descending = line
        .clone()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .all(|d| (-3..=-1).contains(&d));
    let ascending = line
        .tuple_windows()
        .map(|(a, b)| b - a)
        .all(|d| (1..=3).contains(&d));
    descending || ascending
}

// Creates an iterator over the list of integers that skips the ith element
fn skip_iter(line: &[i32], i: usize) -> impl Iterator<Item = i32> + Clone + '_ {
    line.iter()
        .enumerate()
        .filter(move |&(j, _)| j != i)
        .map(|(_, x)| *x)
}

pub fn day2(input: String, ctx: &mut AOContext) {
    let lines = input.lines().map(|l| {
        l.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect_vec()
    });

    let mut safe = 0;
    let mut almost_safe = 0;

    for line in lines {
        if check_distances(line.iter().cloned()) {
            safe += 1;
        } else if (0..line.len()).any(|i| check_distances(skip_iter(&line, i))) {
            almost_safe += 1;
        }
    }

    ctx.submit_both(safe, safe + almost_safe);
}
