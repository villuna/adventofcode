use aoc::{parsers::int, utils::Coord, AOContext};
use nom::{bytes::complete::tag, multi::separated_list1, sequence::{separated_pair, tuple}, Parser};

struct Robot {
    p: Coord,
    v: Coord,
}

fn parse(input: &str) -> Vec<Robot> {
    let coord = |input| separated_pair(int, tag(","), int)
        .map(|(x, y)| Coord(x, y)).parse(input);

    let robot = tuple((tag("p="), coord, tag(" v="), coord))
        .map(|(_, p, _, v)| Robot { p, v });

    separated_list1(tag("\n"), robot)(input).unwrap().1
}

fn longest_robot_line(line: &[u8]) -> i32 {
    let mut span = 0;
    let mut longest = 0;

    for &c in line {
        if c == b'#' {
            span += 1;
            longest = std::cmp::max(span, longest);
        } else {
            span = 0;
        }
    }

    longest
}

fn has_uninterrupted_line(robots: &[Robot]) -> bool {
    let mut str: Vec<u8> = (0..HEIGHT).flat_map(|_| (0..=WIDTH).map(|i| if i == WIDTH { b'\n' } else { b'.' }))
        .collect();

    for r in robots {
        str[((WIDTH + 1) * r.p.1 + r.p.0) as usize] = b'#';
    }

    str.split(|c| *c == b'\n')
        .any(|l| longest_robot_line(l) > 30)
}

fn find_robots(mut robots: Vec<Robot>) -> usize {
    for i in 1.. {
        robots = robots.into_iter()
            .map(|r| Robot { p: Coord((r.p.0 + r.v.0).rem_euclid(WIDTH), (r.p.1 + r.v.1).rem_euclid(HEIGHT)), v: r.v })
            .collect();

        if has_uninterrupted_line(&robots) {
            return i;
        }
    }

    panic!("no robots found");
}

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

pub fn day14(input: String, ctx: &mut AOContext) {
    let robots = parse(&input);
    ctx.parsing_done();
    let mut quadrants = [0u32; 4];
    
    for r in robots.iter() {
        let x = (r.p.0 + r.v.0 * 100).rem_euclid(WIDTH);
        let y = (r.p.1 + r.v.1 * 100).rem_euclid(HEIGHT);

        if x == (WIDTH / 2) || y == (HEIGHT / 2) {
            continue;
        }

        let quadrant = (x > WIDTH/2) as usize + 2 * (y > HEIGHT/ 2) as usize;
        quadrants[quadrant] += 1;
    }

    let part1 = quadrants.iter().product::<u32>();
    ctx.submit_part1(part1);
    ctx.submit_part2(find_robots(robots));
}
