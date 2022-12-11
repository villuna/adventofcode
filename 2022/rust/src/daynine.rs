use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

fn parse_input(input: &str) -> Vec<(Direction, i32)> {
    let mut res = Vec::new();

    for line in input.lines().filter(|line| !line.is_empty()) {
        let (dir, amount) = line.split(" ").collect_tuple().unwrap();

        let dir = match dir {
            "R" => Direction::Right,
            "D" => Direction::Down,
            "U" => Direction::Up,
            "L" => Direction::Left,
            _ => panic!("Invalid direction"),
        };

        let amount = amount.parse::<i32>().unwrap();
        res.push((dir, amount));
    }

    res
}

pub fn day_nine(input: String) {
    let moves = parse_input(&input);

    // Part one
    println!("{}", simulate_rope(&moves, 2));

    // Part 2
    println!("{}", simulate_rope(&moves, 10));
}

fn move_knot(pos: &mut (i32, i32), direction: &Direction) {
    match *direction {
        Direction::Right => pos.0 += 1,
        Direction::Down => pos.1 -= 1,
        Direction::Left => pos.0 -= 1,
        Direction::Up => pos.1 += 1,
    }
}

fn are_adjacent(head: (i32, i32), tail: (i32, i32)) -> bool {
    i32::max((head.0 - tail.0).abs(), (head.1 - tail.1).abs()) <= 1
}

fn simulate_rope(moves: &[(Direction, i32)], num_knots: usize) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut knots = vec![(0, 0); num_knots];
    visited.insert((0, 0));

    for (direction, distance) in moves {
        for _ in 0..*distance {
            move_knot(&mut knots[0], direction);

            for i in 1..num_knots {
                // This is required because rust doesn't understand that it's okay to mutably borrow
                // two different elements of the same vector at once
                let (a, b) = knots.split_at_mut(i);
                let head = &mut a[i - 1];
                let tail = &mut b[0];

                if !are_adjacent(*head, *tail) {
                    // Need to move towards the head
                    tail.0 += (head.0 - tail.0).signum();
                    tail.1 += (head.1 - tail.1).signum();
                }
            }

            visited.insert(knots[num_knots - 1]);
        }
    }

    visited.len()
}
