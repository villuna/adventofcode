use std::fs::File;
use std::io::Read;

pub fn day_three(part: usize, filename: String) {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    if part == 1 {
        part_one(contents);
    } else {
        part_two(contents);
    }
}

fn part_one(contents: String) {
    // Get the digits as a 2-dimensional vector of 0s and 1s:
    let splits: Vec<Vec<usize>> = contents
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.as_bytes()
                .iter()
                .map(|c| (*c as char).to_digit(2).expect("not a number!") as usize)
                .collect::<Vec<usize>>()
        })
        .collect();

    let num_bits = splits[0].len();
    let mut totals = vec![0; num_bits];

    // Sum up the columns to get totals
    for i in 0..num_bits {
        for v in splits.iter() {
            totals[i] += v[i];
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..num_bits {
        if totals[num_bits - i - 1] * 2 >= splits.len() {
            gamma += 2usize.pow(i as u32);
        } else {
            epsilon += 2usize.pow(i as u32);
        }
    }

    println!("The result is {}", gamma * epsilon);
}

fn part_two(contents: String) {
    let mut o2: Vec<Vec<char>> = contents
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.as_bytes()
                .iter()
                .map(|c| *c as char)
                .collect::<Vec<char>>()
        })
        .collect();

    let mut co2 = o2.clone();
    let num_bits = o2[0].len();

    for i in 0..num_bits {
        let mut zeros = 0;
        let mut ones = 0;

        for s in o2.iter() {
            match s[i] {
                '0' => zeros += 1,
                '1' => ones += 1,
                _ => {}
            }
        }

        let digit = if zeros > ones { '0' } else { '1' };

        o2 = o2.into_iter().filter(|s| s[i] == digit).collect();

        if o2.len() == 1 {
            break;
        }
    }

    for i in 0..num_bits {
        let mut zeros = 0;
        let mut ones = 0;

        for s in co2.iter() {
            match s[i] {
                '0' => zeros += 1,
                '1' => ones += 1,
                _ => {}
            }
        }

        let digit = if zeros > ones { '1' } else { '0' };

        co2 = co2.into_iter().filter(|s| s[i] == digit).collect();

        if co2.len() == 1 {
            break;
        }
    }

    let o2 = usize::from_str_radix(&o2[0].iter().collect::<String>(), 2).unwrap();

    let co2 = usize::from_str_radix(&co2[0].iter().collect::<String>(), 2).unwrap();

    println!("The answer is {}", o2 * co2);
}

#[allow(unused)]
fn part_one_iter(contents: String) {
    // Collect the data into a vector of rows, each row being a vector of
    // digits in the row.
    let splits: Vec<Vec<u32>> = contents
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.as_bytes()
                .iter()
                .map(|c| (*c as char).to_digit(10).expect("not a number!"))
                .collect::<Vec<u32>>()
        })
        .collect();

    let num_bits = splits[0].len();

    // Turn this vector of rows into its associated vector of columns, and sum
    // up each of these columns.
    let digits = (0..num_bits)
        .map(|i| splits.iter().map(move |s| s[i]).collect::<Vec<u32>>())
        .map(|vec| vec.iter().sum())
        .map(|count: u32| 2 * count >= splits.len() as u32)
        .collect::<Vec<bool>>();

    let max = usize::from_str_radix(
        &digits
            .iter()
            .map(|b| format!("{}", *b as usize))
            .collect::<String>(),
        2,
    )
    .unwrap();

    let min = usize::from_str_radix(
        &digits
            .iter()
            .map(|b| format!("{}", !b as usize))
            .collect::<String>(),
        2,
    )
    .unwrap();

    println!("The result is {}", max * min);
}
