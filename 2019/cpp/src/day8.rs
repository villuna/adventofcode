// day 8 - haskell

use alloc::vec::Vec;
use alloc::string::String;
use println;
use to_rust_str;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

type Image<'a> = Vec<&'a [&'a [char]]>;

#[no_mangle]
pub extern "C" fn rust_day8(input: *const u8) {
    let input = unsafe { to_rust_str(input).unwrap() };

    let chars = input
        .trim()
        .chars()
        .collect::<Vec<_>>();
    let rows = chars.chunks(WIDTH)
        .collect::<Vec<_>>();
    let image = rows.chunks(HEIGHT)
        .collect::<Vec<_>>();

    part1(&image);
    part2(&image);
}

fn part1(image: &Image) {
    let num_count = |table: &[&[char]], c: char| {
        table.iter()
            .map(|r| r.iter())
            .flatten()
            .filter(|i| **i == c)
            .count()
    };

    let min_index = image.iter()
        .map(|layer| num_count(layer, '0'))
        .enumerate()
        .min_by(|(_, c1), (_, c2)| c1.cmp(&c2))
        .unwrap()
        .0;

    let num_ones = num_count(&image[min_index], '1');
    let num_twos = num_count(&image[min_index], '2');

    println!("part 1: {}", num_ones * num_twos);
}

fn part2(image: &Image) {
    let mut res = String::new();

    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            let mut pixel = '2';

            for layer in 0..image.len() {
                if image[layer][row][col] != '2' {
                    pixel = image[layer][row][col];
                    break;
                }
            }

            if pixel == '0' {
                res.push(' ');
            } else if pixel == '1' {
                res.push('#');
            } else {
                res.push('?');
            }
        }
        res.push('\n');
    }

    println!("{res}");
}
