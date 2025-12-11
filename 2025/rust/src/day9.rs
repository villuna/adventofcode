use aoc::AOContext;
use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::cmp;

type Coord = (i64, i64);

fn point_intersects_line(point: Coord, line: (Coord, Coord)) -> bool {
    if line.0.0 == line.1.0 {
        // vertical
        point.0 == line.0.0
            && point.1 >= cmp::min(line.0.1, line.1.1)
            && point.1 <= cmp::max(line.0.1, line.1.1)
    } else {
        // horizontal
        point.1 == line.0.1
            && point.0 >= cmp::min(line.0.0, line.1.0)
            && point.0 <= cmp::max(line.0.0, line.1.0)
    }
}

fn lines_intersect(l1: (Coord, Coord), l2: (Coord, Coord)) -> bool {
    let l1_vert = l1.0.0 == l1.1.0;
    let l2_vert = l2.0.0 == l2.1.0;

    if !(l1_vert ^ l2_vert) {
        // We dont need to check for this just trust
        return false;
    }

    let (v, h) = if l1.0.0 == l1.1.0 { (l1, l2) } else { (l2, l1) };

    cmp::min(h.0.0, h.1.0) <= v.0.0
        && cmp::max(h.0.0, h.1.0) >= v.0.0
        && cmp::min(v.0.1, v.1.1) <= h.0.1
        && cmp::max(v.0.1, v.1.1) >= h.0.1
}

fn point_in_area(point: Coord, corners: &[Coord], bounds: (Coord, Coord)) -> bool {
    for line in corners.iter().cloned().circular_tuple_windows::<(_, _)>() {
        if point_intersects_line(point, line) {
            return true;
        }
    }

    'outer: for end in [
        (point.0, bounds.0.1),
        (point.0, bounds.1.1),
        (bounds.0.0, point.1),
        (bounds.1.0, point.1),
    ] {
        let ray = (point, end);

        for line in corners.iter().cloned().circular_tuple_windows::<(_, _)>() {
            if lines_intersect(ray, line) {
                continue 'outer;
            }
        }

        return false;
    }

    true
}

fn rect_in_area(rect: (Coord, Coord), corners: &[Coord], bounds: (Coord, Coord)) -> bool {
    corners_in_area(rect, corners, bounds) && all_points_in_area(rect, corners, bounds)
}

fn all_points_in_area(rect: (Coord, Coord), corners: &[Coord], bounds: (Coord, Coord)) -> bool {
    let min = (cmp::min(rect.0.0, rect.1.0), cmp::min(rect.0.1, rect.1.1));
    let max = (cmp::max(rect.0.0, rect.1.0), cmp::max(rect.0.1, rect.1.1));
    let mut points = Vec::new();
    points.extend((min.0..=max.0).map(|x| (x, min.1)));
    points.extend((min.0..=max.0).map(|x| (x, max.1)));
    points.extend((min.1..=max.1).map(|y| (min.0, y)));
    points.extend((min.1..=max.1).map(|y| (max.0, y)));

    points
        .into_par_iter()
        .all(|p| point_in_area(p, corners, bounds))
}

fn corners_in_area(rect: (Coord, Coord), corners: &[Coord], bounds: (Coord, Coord)) -> bool {
    let points = vec![rect.0, rect.1, (rect.0.0, rect.1.1), (rect.1.0, rect.0.1)];

    points
        .into_iter()
        .all(|p| point_in_area(p, corners, bounds))
}

fn area_of_rect(((x1, y1), (x2, y2)): (Coord, Coord)) -> u64 {
    (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1)
}

pub fn day9(input: String, ctx: &mut AOContext) {
    let corners = input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|n| n.parse::<i64>().unwrap())
                .collect_tuple::<(_, _)>()
                .unwrap()
        })
        .collect_vec();

    let mut rects = corners
        .iter()
        .cloned()
        .cartesian_product(corners.iter().cloned())
        .collect_vec();

    rects.sort_by_key(|rect| cmp::Reverse(area_of_rect(*rect)));

    let p1 = area_of_rect(*rects.first().unwrap());

    ctx.submit_part1(p1);

    let bounds = (
        (
            *corners.iter().map(|(x, _)| x).min().unwrap(),
            *corners.iter().map(|(_, y)| y).min().unwrap(),
        ),
        (
            *corners.iter().map(|(x, _)| x).max().unwrap(),
            *corners.iter().map(|(_, y)| y).max().unwrap(),
        ),
    );

    let p2 = area_of_rect(
        rects
            .into_par_iter()
            .progress()
            .find_first(|&rect| rect_in_area(rect, &corners, bounds))
            .unwrap(),
    );

    ctx.submit_part2(p2);
}
