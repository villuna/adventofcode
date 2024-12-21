use ahash::{HashMap, HashMapExt, HashSet};
use aoc::{
    utils::{Coord, Dir},
    AOContext,
};
use itertools::Itertools;
use std::hash::Hash;

// A key on a directional keypad. Basically either a direction or 'A'
#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
enum DirKey {
    Dir(Dir),
    A,
}

fn find_paths_helper(from: Coord, to: Coord, dirs: &[Dir], map: &HashSet<Coord>) -> Vec<Vec<Dir>> {
    if from == to {
        return vec![vec![]];
    }

    let mut res = Vec::new();

    for &d in dirs {
        // Stop ourselves from going too far
        if matches!(d, Dir::Right | Dir::Left) {
            if from.0 == to.0 {
                continue;
            }
        } else {
            if from.1 == to.1 {
                continue;
            }
        }

        let next = from + d;

        if map.contains(&next) {
            let paths = find_paths_helper(next, to, dirs, map);

            for mut p in paths {
                p.insert(0, d);
                res.push(p);
            }
        }
    }

    res
}

// Finds all the possible shortest paths from one coord to another coord, in a given map.
// (The paths are only allowed to go through coordinates that are in the 'coords' set).
fn find_paths(from: Coord, to: Coord, coords: &HashSet<Coord>) -> Vec<Vec<Dir>> {
    let vert = if from.1 < to.1 {
        Some(Dir::Down)
    } else if from.1 > to.1 {
        Some(Dir::Up)
    } else {
        None
    };

    let horiz = if from.0 > to.0 {
        Some(Dir::Left)
    } else if from.0 < to.0 {
        Some(Dir::Right)
    } else {
        None
    };

    find_paths_helper(
        from,
        to,
        &[vert, horiz].iter().filter_map(|d| *d).collect_vec(),
        &coords,
    )
}

// Takes in a set of coordinates and returns the fewest keys the *human* has to press on the keypad
// in order to get the robot on that level to press those buttons. Caches results in the given
// hashmap.
fn fewest(
    max_level: u8,
    // How many levels removed from the human we are
    level: u8,
    // The coordinates of buttons to press. We don't press the button at the first coordinate: it's
    // just where we start.
    keys: &[Coord],
    // Maps each direction to its position on the directional keyboard
    dir_map: &HashMap<DirKey, Coord>,
    // all the coordinates of buttons in the numpad
    num_coords: &HashSet<Coord>,
    // all the coordinates of buttons in the directional keyboard
    dir_coords: &HashSet<Coord>,
    cache: &mut HashMap<(Vec<Coord>, u8), usize>,
) -> usize {
    if level == 0 {
        // If this is the human keyboard, we just need to press all the keys in the path,
        // except for the first one which we start on (hence the -1).
        keys.len() - 1
    } else if cache.contains_key(&(keys.into(), level)) {
        // Return the answer if we have already figured it out
        cache[&(keys.into(), level)]
    } else {
        // When we call this the first time, the coordinates will be for the numpad. Otherwise
        // they'll be on the keypad
        let coords = if level == max_level {
            num_coords
        } else {
            dir_coords
        };

        // Okay, this is a scary iterator but it's really not so bad.
        let res = keys
            // For every pair (a, b) of two keys in the sequence...
            .windows(2)
            // We have to find every possible path from a to b...
            .map(|meow| find_paths(meow[0], meow[1], coords))
            .map(|paths| {
                // now for each possible path between a and b, we turn that path 
                // into a series of dirpad presses that get the arm to follow that path and press.
                let paths = paths
                    .into_iter()
                    .map(|path| {
                        // Obviously we have to press each corresponding direction on the keypad
                        let mut path = path.into_iter().map(DirKey::Dir).collect_vec();
                        // ...and we always start at A
                        path.insert(0, DirKey::A);
                        // ...and at the end we have to press A to get the next robot to press the
                        // button
                        path.push(DirKey::A);
                        path.into_iter().map(|dk| dir_map[&dk]).collect_vec()
                    })
                    .collect_vec();

                // Recursive step:
                // we have a list of buttons for this robot to press. We have to now find which
                // path takes the fewest human button presses.
                paths
                    .into_iter()
                    .map(|path| {
                        fewest(
                            max_level,
                            level - 1,
                            &path,
                            dir_map,
                            num_coords,
                            dir_coords,
                            cache,
                        )
                    })
                    .min()
                    .unwrap()
            })
            // And now that we have found the lengths of the shortest paths between every pair of
            // buttons, we can sum them up to get the length of the entire path in sequence.
            .sum();

        // And cache that result for later :)
        cache.insert((keys.into(), level), res);
        res
    }
}

pub fn day21(input: String, ctx: &mut AOContext) {
    let sequences = input.lines().map(|l| l.as_bytes().to_owned()).collect_vec();
    ctx.parsing_done();
    let mut num_kb = HashMap::new();

    for i in (0..3).rev() {
        let start = 3 * i + 1;
        for j in 0..3 {
            num_kb.insert((start + j) as u8 + b'0', Coord(j, 2 - i));
        }
    }

    num_kb.insert(b'0', Coord(1, 3));
    num_kb.insert(b'A', Coord(2, 3));

    let num_kb = num_kb;
    let num_coords = num_kb.values().copied().collect::<HashSet<Coord>>();

    let dir_kb = HashMap::from_iter([
        (DirKey::Dir(Dir::Up), Coord(1, 0)),
        (DirKey::A, Coord(2, 0)),
        (DirKey::Dir(Dir::Left), Coord(0, 1)),
        (DirKey::Dir(Dir::Down), Coord(1, 1)),
        (DirKey::Dir(Dir::Right), Coord(2, 1)),
    ]);
    let dir_coords = dir_kb.values().copied().collect::<HashSet<Coord>>();

    let mut cache = HashMap::new();
    let mut solve = |levels| {
        sequences
            .iter()
            .map(|sequence| {
                let mut buttons = vec![num_kb[&b'A']];
                buttons.extend(sequence.iter().map(|b| num_kb[&b]));
                let path = fewest(
                    levels,
                    levels,
                    &buttons,
                    &dir_kb,
                    &num_coords,
                    &dir_coords,
                    &mut cache,
                );
                let num = String::from_utf8(
                    sequence
                        .clone()
                        .into_iter()
                        .filter(|b| *b != b'A')
                        .collect_vec(),
                )
                .unwrap()
                .parse::<usize>()
                .unwrap();
                num * path
            })
            .sum::<usize>()
    };

    ctx.submit_part1(solve(3));
    ctx.submit_part2(solve(26));
}
