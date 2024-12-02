use itertools::Itertools;
use crate::AOContext;

fn check_distances(line: impl Iterator<Item = (i32, i32)> + Clone) -> bool {
    let descending = line.clone()
        .map(|w| w.1 - w.0)
        .all(|d| (-3..=-1).contains(&d));
    let ascending = line
        .map(|w| w.1 - w.0)
        .all(|d| (1..=3).contains(&d));
    descending || ascending
}

pub fn day2(input: String, ctx: &mut AOContext) {
    let lines = input.lines()
        .map(|l| l.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect_vec());

    let mut safe = 0;
    let mut almost_safe = 0;

    for line in lines {
        if check_distances(line.iter().cloned().tuple_windows()) {
            safe += 1;
        } else if (0..line.len()).any(|i| check_distances(line.iter().enumerate().filter(|&(j, _)| j != i).map(|(_, x)| *x).tuple_windows())) {
            // iterators are so readable and concise trust me bro just put that above line into
            // your code everyone will love it
            almost_safe += 1;
        }
    }

    ctx.submit_both(safe, safe + almost_safe);
}
