use println;
use to_rust_str;
use sqrt;
use atan2;
// Unfortunately because I challenged myself to use no_std for this year's rust solutions I can't
// use a hash map, I have to use BTreeMap, which is pretty sad. I could implement my own hash map.
// But meh.
use alloc::collections::BTreeSet;
use alloc::vec::Vec;
use alloc::vec;
use core::iter::Iterator;
use core::cmp;

// they're asteroids not stars but i love the name StarMap so ill keep it
#[derive(Debug)]
struct StarMap {
    asteroids: BTreeSet<(i16, i16)>,
    dimensions: (usize, usize),
}

impl StarMap {
    fn from(input: &str) -> Option<StarMap> {
        let mut map = BTreeSet::new();
        let width = input.lines().next()?.len();
        let height = input.lines().count();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    map.insert((x as i16, y as i16));
                } else if c != '.' {
                    return None;
                }
            }
        }

        Some(Self {
            asteroids: map,
            dimensions: (width, height),
        })
    }
}

// An iterator to help me scan in a spiral for extra efficiency
struct Spiral {
    start: (i16, i16),
    offset: (i16, i16),
    max: i16,
}

fn spiral(start: (i16, i16), max: i16) -> Spiral {
    Spiral {
        start,
        offset: (0, 0),
        max,
    }
}

impl Iterator for Spiral {
    type Item = ((i16, i16), (i16, i16));

    fn next(&mut self) -> Option<Self::Item> {
        let mag = cmp::max(self.offset.0.abs(), self.offset.1.abs());

        if self.offset == (-mag, mag) {
            if mag == self.max - 1 {
                return None;
            }
            self.offset.1 += 1;
        } else {
            if self.offset.1 == mag && self.offset.0 < mag {
                self.offset.0 += 1;
            } else if self.offset.0 == mag && self.offset.1 > -mag {
                self.offset.1 -= 1;
            } else if self.offset.1 == -mag && self.offset.0 > -mag {
                self.offset.0 -= 1;
            } else if self.offset.0 == -mag && self.offset.1 < mag {
                self.offset.1 += 1;
            }
        }

        Some((
            (self.start.0 + self.offset.0, self.start.1 + self.offset.1),
            self.offset,
        ))
    }
}

fn part1(input: &StarMap) -> (i16, i16) {
    let mut vis_map: Vec<i16> = vec![0; input.dimensions.0 * input.dimensions.1];

    let is_in_bounds = |&(x, y): &(i16, i16)| {
        x >= 0 && y >= 0 && x < input.dimensions.0 as i16 && y < input.dimensions.1 as i16
    };

    let index = |&(x, y): &(i16, i16)| {
        y as usize * input.dimensions.0 + x as usize
    };

    for &asteroid in &input.asteroids {
        let mut visited: BTreeSet<(i16, i16)> = BTreeSet::new();

        let spiral = spiral(asteroid, cmp::max(input.dimensions.0 as i16, input.dimensions.1 as i16))
            .filter(|(p, _)| is_in_bounds(p));

        for (mut point, offset) in spiral {
            if visited.contains(&point) {
                continue;
            }

            let mut blocked = false;

            while is_in_bounds(&point) {
                visited.insert(point);
                
                if !blocked {
                    vis_map[index(&point)] += 1;
                }

                if input.asteroids.contains(&point) {
                    blocked = true;
                }

                point.0 += offset.0;
                point.1 += offset.1;
            }
        }
    }

    let (point, max) = input.asteroids.iter()
        .map(|point| (point, vis_map[index(point)]))
        .max_by(|(_, v1), (_, v2)| v1.cmp(&v2))
        .unwrap();

    println!("part 1: position {point:?} with {max} visible asteroids");

    *point
}

fn part2(input: &StarMap, pos: (i16, i16)) {
    let angle_from = |&(x, y): &(i16, i16)| {
        -atan2(-pos.0 as f32 + x as f32, y as f32 - pos.1 as f32)
    };

    let distance_from = |&(x, y): &(i16, i16)| {
        sqrt((x as f32 - pos.0 as f32)*(x as f32 - pos.0 as f32) + (y as f32 - pos.1 as f32)*(y as f32 - pos.1 as f32))
    };

    let mut ordered_asteroids = input.asteroids
        .iter()
        // I use this pattern so much I feel like it should be a function
        // "zipmap?" idfk
        .map(|aster| (aster, angle_from(&aster)))
        .collect::<Vec<_>>();
    
    ordered_asteroids.sort_by(
        |(aster1, angle1), (aster2, angle2)| {
            (*angle1, distance_from(aster1)).partial_cmp(&(*angle2, distance_from(aster2))).unwrap()
        });

    // The last element of the tuple indicates if the asteroid is destroyed
    let mut ordered_asteroids = ordered_asteroids.into_iter().map(|(aster, angle)| (aster, angle, false)).collect::<Vec<_>>();
    let mut target = 0;
    let mut last = 0;

    for _ in 0..200 {
        ordered_asteroids[target].2 = true;
        last = target; 
        let last_angle = ordered_asteroids[target].1;

        // Eugh this code is kind of nasty but it works and this is just aoc so i dont really
        // care...
        target = ordered_asteroids
            .iter()
            .enumerate()
            .skip(target)
            .find(|(_, (_, angle, destroyed))| !destroyed && *angle != last_angle)
            .unwrap_or_else(|| ordered_asteroids.iter().enumerate().find(|(_, (_, _, destroyed))| !destroyed).unwrap())
            .0;
    }

    let last_asteroid = ordered_asteroids[last].0;
    println!("part 2: position was {:?} => answer is {}", last_asteroid, last_asteroid.0 * 100 + last_asteroid.1);
}

#[no_mangle]
pub extern "C" fn rust_day10(input: *const u8) {
    let input = unsafe { to_rust_str(input).expect("Couldnt read input") };
    let map = StarMap::from(input).expect("Couldnt build starmap");

    let point = part1(&map);
    part2(&map, point);
}
