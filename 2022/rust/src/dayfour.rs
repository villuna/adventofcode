use itertools::Itertools;

type Assignment = ((usize, usize), (usize, usize));

fn parse_input(input: String) -> Vec<Assignment> {
    let mut res = Vec::new();

    for line in input.lines().filter(|line| !line.is_empty()) {
        let pair = line
            .split(&[',', '-'])
            .map(|s| s.parse::<usize>().unwrap())
            .collect_vec();

        res.push(((pair[0], pair[1]), (pair[2], pair[3])));
    }

    return res;
}

pub fn day_four(input: String) {
    let input = parse_input(input);

    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &Vec<Assignment>) -> usize {
    input
        .iter()
        .filter(|((a, b), (c, d))| a <= c && b >= d || c <= a && d >= b)
        .collect_vec()
        .len()
}

fn part_two(input: &Vec<Assignment>) -> usize {
    input
        .iter()
        .filter(|((a, b), (c, d))| !(b < c || d < a))
        .collect_vec()
        .len()
}
