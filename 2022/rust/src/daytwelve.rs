use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::collections::HashSet;

type HeightMap = Vec<Vec<char>>;

pub fn day_twelve(input: String) {
    let map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    println!("{}", part_one(&map).expect("No solution found"));
    println!("{}", part_two(&map).expect("No solution found"));
}

fn a_star<H, F>(
    map: &HeightMap,
    start: (usize, usize),
    goal: char,
    heuristic: H,
    can_reach: F,
) -> Option<u32>
where
    H: Fn(&(usize, usize)) -> i32, // Returns i32 for reasons
    F: Fn(char, char) -> bool,
{
    let mut frontier = PriorityQueue::new();
    let mut visited = HashSet::new();
    let width = map[0].len();
    let height = map.len();

    // Simply transforms S to a and E to z so we can
    // use them in elevation calculations
    let elevation = |c| {
        if c == 'S' {
            'a'
        } else if c == 'E' {
            'z'
        } else {
            c
        }
    };

    // A-star: priorities for exploring each tile are calculated as d + h,
    // where d is the distance from the start to the tile,
    // and h is the heuristic: an estimate of the shortest distance from
    // the tile to the goal.
    frontier.push((start, 0), -(heuristic(&start) as i32));

    while let Some(((last, len), _)) = frontier.pop() {
        visited.insert(last);

        if map[last.0][last.1] == goal {
            return Some(len as u32);
        }

        // Is there a better way of doing this 😅
        for new in [(0, -1), (-1, 0), (1, 0), (0, 1)]
            .iter()
            // Get the possible positions we can move to
            .map(|(yi, xi)| (last.0 as i32 + yi, last.1 as i32 + xi))
            // Make sure they're in bounds
            .filter(|&(ny, nx)| ny >= 0 && ny < height as i32 && nx >= 0 && nx < width as i32)
            // Convert back to usize (needed to be signed for bounds check)
            .map(|(ny, nx)| (ny as usize, nx as usize))
            // Make sure we can actually reach the next tile
            .filter(|&(ny, nx)| {
                let last_char = elevation(map[last.0][last.1]);
                let next_char = elevation(map[ny][nx]);

                can_reach(last_char, next_char)
            })
        {
            if !visited.contains(&new) {
                frontier.push((new, len + 1), -(len + 1 + heuristic(&new)));
            }
        }
    }

    return None;
}

fn part_one(map: &HeightMap) -> Option<u32> {
    let width = map[0].len();
    let height = map.len();
    let mut start = None;
    let mut end = None;

    'outer: for y in 0..height {
        for x in 0..width {
            if map[y][x] == 'S' {
                start = Some((y, x));
            }

            if map[y][x] == 'E' {
                end = Some((y, x));
            }

            if start != None && end != None {
                break 'outer;
            }
        }
    }

    let start = start.unwrap();
    let end = end.unwrap();

    let distance = |&(y, x): &(usize, usize)| {
        (end.0 as i32 - y as i32).abs() + (end.1 as i32 - x as i32).abs()
    };
    let can_reach = |last, next| next as u32 <= last as u32 + 1;
    a_star(map, start, 'E', distance, can_reach)
}

fn part_two(map: &HeightMap) -> Option<u32> {
    let width = map[0].len();
    let height = map.len();
    let mut start = None;

    'outer: for y in 0..height {
        for x in 0..width {
            if map[y][x] == 'E' {
                start = Some((y, x));
                break 'outer;
            }
        }
    }

    let start = start.unwrap();

    let a_s = (0..height)
        .cartesian_product(0..width)
        .filter(|&(y, x)| map[y][x] == 'a' || map[y][x] == 'S')
        .collect_vec();

    let distance = |&(x1, y1): &(usize, usize), &(x2, y2): &(usize, usize)| {
        (y1 as i32 - y2 as i32).abs() + (x1 as i32 - x2 as i32).abs()
    };

    let min_distance = |p: &(usize, usize)| a_s.iter().map(|a| distance(p, a)).min().unwrap();

    let can_reach = |last, next| last as u32 <= next as u32 + 1;
    a_star(map, start, 'a', min_distance, can_reach)
}
