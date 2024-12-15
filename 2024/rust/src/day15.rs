use ahash::HashSet;
use aoc::{utils::{Coord, Dir}, AOContext};

fn dir(c: char) -> Dir {
    match c {
        '>' => Dir::Right,
        '^' => Dir::Up,
        '<' => Dir::Left,
        'v' => Dir::Down,
        _ => panic!(),
    }
}

#[derive(Default, Clone)]
struct Input {
    walls: HashSet<Coord>,
    boxes: HashSet<Coord>,
    start: Coord,
    moves: Vec<Dir>,
}

fn parse(input: &str) -> Input {
    let mut sections = input.split("\n\n");

    let grid = sections.next().unwrap();

    let mut res = Input::default();

    for (y, l) in grid.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let coord = Coord(x as _, y as _);
            if c == '@' {
                res.start = coord;
            } else if c == '#' {
                res.walls.insert(coord);
            } else if c == 'O' {
                res.boxes.insert(coord);
            }
        }
    }

    res.moves = sections.next().unwrap().lines().map(|l| l.chars()).flatten().map(|c| dir(c)).collect();
    res 
}

fn gps_coord(c: &Coord) -> isize {
    c.1 * 100 + c.0
}

fn move_box(next: Coord, dir: Dir, input: &mut Input) -> bool {
    let nn = next + dir;

    if input.walls.contains(&nn) {
        false
    } else if input.boxes.contains(&nn) {
        if move_box(nn, dir, input) {
            input.boxes.remove(&next);
            input.boxes.insert(nn);
            true
        } else {
            false
        }
    } else {
        input.boxes.remove(&next);
        input.boxes.insert(nn);
        true
    }
}

fn adj(c: Coord) -> Coord {
    Coord(c.0 + 1, c.1)
}

fn part1(mut input: Input) -> isize {
    let mut pos = input.start;

    for d in input.moves.clone() {
        let next = pos + d;

        if input.walls.contains(&next) {
            continue;
        } else if input.boxes.contains(&next) {
            if move_box(next, d, &mut input) {
                pos = next;
            }
        } else {
            pos = next;
        }
    }

    input.boxes.iter().map(gps_coord).sum::<isize>()
}

fn pushed_box(coord: Coord, dir: Dir, input: &Input) -> Option<Coord> {
    match dir {
        Dir::Up | Dir::Down => {
            if input.boxes.contains(&coord) {
                Some(coord)
            } else if input.boxes.contains(&Coord(coord.0 - 1, coord.1)) {
                Some(Coord(coord.0 - 1, coord.1))
            } else {
                None
            }
        },

        Dir::Left => {
            input.boxes.contains(&Coord(coord.0 - 1, coord.1)).then_some(Coord(coord.0 - 1, coord.1))
        },

        Dir::Right => {
            input.boxes.contains(&coord).then_some(coord)
        }
    }
}

fn can_move_box_p2(coord: Coord, dir: Dir, input: &Input) -> bool {
    let next = coord + dir;

    if input.walls.contains(&next) || input.walls.contains(&adj(next)) {
        false
    } else {
        let mut can_move = true;

        let boxes = if matches!(dir, Dir::Up | Dir::Down) {
            vec![Coord(next.0 - 1, next.1), next, adj(next)]
        } else {
            vec![next + dir]
        };

        for bc in boxes {
            if input.boxes.contains(&bc) {
                can_move &= can_move_box_p2(bc, dir, input);
            }
        }

        can_move
    }
}

fn move_box_p2(coord: Coord, dir: Dir, input: &mut Input) -> bool {
    let next = coord + dir;

    if input.walls.contains(&next) || input.walls.contains(&adj(next)) {
        false
    } else {
        let mut can_move = true;

        let boxes = if matches!(dir, Dir::Up | Dir::Down) {
            vec![Coord(next.0 - 1, next.1), next, adj(next)]
        } else {
            vec![next + dir]
        };

        for bc in boxes {
            if input.boxes.contains(&bc) {
                can_move &= move_box_p2(bc, dir, input);
            }
        }

        if can_move {
            input.boxes.remove(&coord);
            input.boxes.insert(next);
        }

        can_move
    }
}

fn part2(input: Input) -> isize {
    let mut new_input = Input::default();

    for wall in input.walls {
        new_input.walls.insert(Coord(2 * wall.0, wall.1));
        new_input.walls.insert(Coord(2 * wall.0 + 1, wall.1));
    }

    for b in input.boxes {
        new_input.boxes.insert(Coord(2 * b.0, b.1));
    }

    new_input.moves = input.moves;
    new_input.start = Coord(input.start.0 * 2, input.start.1);

    let mut input = new_input;
    let mut pos = input.start;

    for d in input.moves.clone().into_iter() {
        let next = pos + d;

        if input.walls.contains(&next) {
            continue;
        } else {
            match pushed_box(next, d, &input) {
                Some(b) => {
                    if can_move_box_p2(b, d, &input) {
                        assert!(move_box_p2(b, d, &mut input));
                        pos = next;
                    }
                }
                None => {
                    pos = next;
                }
            }
        }
    }

    input.boxes.iter().map(gps_coord).sum::<isize>()
}

pub fn day15(input: String, ctx: &mut AOContext) {
    let data = parse(&input);
    ctx.parsing_done();
    ctx.submit_part1(part1(data.clone()));
    ctx.submit_part2(part2(data));
}
