// CARGO FMT KEEPS SORTING THESE IN ALPHABETICAL ORDER
// AAAAAAAAAAAAAAAAAAAAAAAAAA
mod dayeight;
mod dayeleven;
mod dayfive;
mod dayfour;
mod daynine;
mod dayone;
mod dayseven;
mod daysix;
mod daysixteen;
mod daythirteen;
mod daythree;
mod daytwelve;
mod daytwo;

use dayeight::*;
use dayeleven::*;
use dayfive::*;
use dayfour::*;
use daynine::*;
use dayone::*;
use dayseven::*;
use daysix::*;
use daysixteen::*;
use daythirteen::*;
use daythree::*;
use daytwelve::*;
use daytwo::*;
use std::env;
use std::fs::File;
use std::io::{self, Read};

fn input(day: usize) -> Result<String, io::Error> {
    let mut file = File::open(&format!("../input/day{day}.txt"))?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    Ok(input)
}

fn main() {
    let Some(day) = env::args().nth(1).and_then(|s| s.parse::<usize>().ok()) else {
        println!("Please pass the day you want to solve");
        return;
    };

    let input = match input(day) {
        Ok(string) => string,
        Err(e) => panic!("Error reading file: {e:?}"),
    };

    match day {
        1 => day_one(input),
        2 => day_two(input),
        3 => day_three(input),
        4 => day_four(input),
        5 => day_five(input),
        6 => day_six(input),
        7 => day_seven(input),
        8 => day_eight(input),
        9 => day_nine(input),
        11 => day_eleven(input),
        12 => day_twelve(input),
        13 => day_thirteen(input),
        16 => day_sixteen(input),
        _ => println!("Not a valid day!"),
    }
}
