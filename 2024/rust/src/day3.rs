use regex::Regex;

use crate::AOContext;

pub fn day3(input: String, ctx: &mut AOContext) {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let re2 = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|don't\(\)|do\(\)").unwrap();

    ctx.lap("compiling regex");

    let mut part1 = 0;

    for (_, [n1, n2]) in re.captures_iter(&input).map(|c| c.extract()) {
        let n1: i32 = n1.parse().unwrap();
        let n2: i32 = n2.parse().unwrap();

        part1 += n1 * n2;
    }

    ctx.submit_part1(part1);

    let mut part2 = 0;
    let mut doing = true;

    for cap in re2.captures_iter(&input) {
        if &cap[0] == "do()" {
            doing = true;
        } else if &cap[0] == "don\'t()" {
            doing = false;
        } else if doing {
            let n1: i32 = cap[1].parse().unwrap();
            let n2: i32 = cap[2].parse().unwrap();

            part2 += n1 * n2;
        }
    }

    ctx.submit_part2(part2);
}
