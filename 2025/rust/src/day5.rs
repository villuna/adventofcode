use aoc::AOContext;
use itertools::Itertools;

pub fn day5(input: String, ctx: &mut AOContext) {
    let (ranges, ids) = input.split("\n\n").collect_tuple().unwrap();

    let mut ranges = ranges
        .trim()
        .lines()
        .map(|line| {
            let (start, end) = line.split("-").collect_tuple().unwrap();
            (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .collect_vec();

    let ids = ids
        .trim()
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect_vec();

    ctx.parsing_done();

    let available = ids
        .iter()
        .filter(|id| {
            ranges
                .iter()
                .any(|&(start, end)| (start..=end).contains(id))
        })
        .count();

    ctx.submit_part1(available);

    ranges.sort_by_key(|(start, _)| *start);

    let mut i = 0;

    loop {
        // Compare the next two ranges
        let Some(r1) = ranges.get(i).copied() else {
            break;
        };
        let Some(r2) = ranges.get(i + 1).copied() else {
            break;
        };

        if r1.1 >= r2.0 {
            // Ranges overlap
            ranges.remove(i + 1);
            ranges.remove(i);

            ranges.insert(i, (r1.0, std::cmp::max(r2.1, r1.1)));
        } else {
            // Ranges dont overlap
            i += 1;
        }
    }

    ctx.submit_part2(
        ranges
            .iter()
            .map(|(start, end)| end - start + 1)
            .sum::<u64>(),
    );
}
