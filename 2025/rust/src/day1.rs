use aoc::AOContext;

fn part1(rotations: &[i16]) -> i16 {
    let mut start = 50;
    let mut count = 0;

    for &r in rotations {
        start += r;
        start = start.rem_euclid(100);
        if start == 0 {
            count += 1;
        }
    }

    count
}

fn part2(rotations: &[i16]) -> i16 {
    let mut start = 50;
    let mut count = 0;

    for &r in rotations {
        if r < 0 {
            let mut r = r;
            if -r >= start && start != 0 {
                count += 1;
                r += start;
            }
            count += -r / 100;
        } else {
            count += (start + r) / 100;
        }
        start = (start + r).rem_euclid(100);
    }

    count
}

pub fn day1(input: String, ctx: &mut AOContext) {
    let rotations = input
        .trim()
        .as_bytes()
        .split(|&c| c == b'\n')
        .map(|s| {
            let left = s[0] == b'L';
            let mut val = 0;
            for c in &s[1..] {
                val = 10 * val + (c - b'0') as i16;
            }
            if left { -val } else { val }
        })
        .collect::<Vec<_>>();

    ctx.parsing_done();

    ctx.submit_part1(part1(&rotations));
    ctx.submit_part2(part2(&rotations));
}
