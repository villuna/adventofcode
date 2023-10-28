use core::cmp;
use alloc::vec::Vec;

use to_rust_str;
use println;

#[derive(Debug)]
struct Interval {
    start: (i32, i32),
    end:  (i32, i32),
    horizontal: bool,
}

impl Interval {
    fn span_and_point(&self) -> ((i32, i32), i32) {
        if self.horizontal {
            let span = (cmp::min(self.start.0, self.end.0), cmp::max(self.start.0, self.end.0));
            let point = self.start.1;

            (span, point)
        } else {
            let span = (cmp::min(self.start.1, self.end.1), cmp::max(self.start.1, self.end.1));
            let point = self.start.0;

            (span, point)
        }
    }

    fn distance_to_intersection(&self, point: (i32, i32)) -> Option<i32> {
        let (s, p) = self.span_and_point();

        if self.horizontal {
            if p == point.1 && s.0 < point.0 && point.0 < s.1 {
                Some((self.start.0 - point.0).abs())
            } else {
                None
            }
        } else {
            if p == point.0 && s.0 < point.1 && point.1 < s.1 {
                Some((self.start.1 - point.1).abs())
            } else {
                None
            }
        }
    }

    fn distance(&self) -> i32 {
        (self.start.0 - self.end.0).abs() + (self.start.1 - self.end.1).abs()
    }

    fn intersection(&self, other: &Interval) -> (i32, i32) {
        if self.horizontal {
            (other.start.0, self.start.1)
        } else {
            (self.start.0, other.start.1)
        }
    }
}

fn read_command(input: &str) -> (char, i32) {
    let mut chars = input.chars();
    
    let dir = chars.next().unwrap();

    if !['U', 'D', 'L', 'R'].contains(&dir) {
        panic!("invalid direction character!");
    }

    let num = chars.as_str().parse::<i32>().unwrap();

    (dir, num)
}

fn manhattan_norm(point: (i32, i32)) -> i32 {
    point.0.abs() + point.1.abs()
}

#[no_mangle]
pub extern "C" fn rust_day3(input: *const u8) {
    println!("Hi from rust :3");
    let input = unsafe { to_rust_str(input).unwrap() };
    let intervals = input.lines()
        .map(|line| {
            let mut start = (0,0);

            line.split(",")
                .map(read_command)
                .map(|(dir, dist)| {
                    let (end, horizontal) = match dir {
                        'U' => ((start.0, start.1 + dist), false),
                        'D' => ((start.0, start.1 - dist), false),
                        'L' => ((start.0 - dist, start.1), true),
                        'R' => ((start.0 + dist, start.1), true),
                        _ => unreachable!(),
                    };

                    let interval = Interval { start, end, horizontal };
                    start = end;
                    interval
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut intersections = Vec::new();

    for i1 in intervals[0].iter() {
        for i2 in intervals[1].iter().filter(|i| i.horizontal != i1.horizontal) {
            let (span1, point1) = i1.span_and_point();
            let (span2, point2) = i2.span_and_point();

            if span1.0 <= point2 && point2 <= span1.1 && span2.0 <= point1 && point1 <= span2.1 {
                intersections.push(i1.intersection(&i2));
            }
        }
    }

    // Part 1
    let p1 = intersections
        .iter()
        .cloned()
        .map(manhattan_norm)
        .min()
        .unwrap();

    println!("part 1: {p1}");

    // Part 2
    let p2 = part2(&intervals[0], &intervals[1], &intersections);

    println!("part 2: {p2}");
}

fn travel_to_intersection(ivs: &[Interval], is: (i32, i32)) -> i32 {
    let mut res = 0;

    for iv in ivs {
        if let Some(dist) = iv.distance_to_intersection(is) {
            res += dist;
            return res
        } else {
            res += iv.distance();
        }
    }

    panic!("couldn't find intersection on the path!")
}

fn part2(iv1: &[Interval], iv2: &[Interval], is: &[(i32, i32)]) -> i32 {
    is.iter()
        .map(|i| travel_to_intersection(iv1, *i) + travel_to_intersection(iv2, *i))
        .min()
        .unwrap()
}
