mod dayeight;
mod dayfour;
mod daynine;
mod dayone;
mod dayseven;
mod dayten;
mod daythree;
mod daytwelve;
mod dayfourteen;

use std::io::{self, Write};
use text_io::read;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let day: usize;
    let mut question: usize;

    if args.len() != 3 {
        println!("Hello!");
        print!("Enter the day: ");

        io::stdout().flush().unwrap();

        day = read!("{}\n");

        loop {
            print!("Enter the question (1 or 2): ");
            io::stdout().flush().unwrap();
            question = read!("{}\n");

            if question != 1 && question != 2 {
                println!("Sorry, try again");
            } else {
                break;
            }
        }
    } else {
        day = args[1].parse().unwrap();
        question = args[2].parse().unwrap();
    }

    print!("Enter the input file: ");
    io::stdout().flush().unwrap();
    let filename: String = read!("{}\n");

    // TODO: Use closures or something to make this nicer
    match day {
        1 => dayone::day_one(question, filename),
        3 => daythree::day_three(question, filename),
        4 => dayfour::day_four(question, filename),
        7 => dayseven::day_seven(question, filename),
        8 => dayeight::day_eight(question, filename),
        9 => daynine::day_nine(question, filename),
        10 => dayten::day_ten(question, filename),
        12 => daytwelve::day_twelve(question, filename),
        14 => dayfourteen::day_fourteen(question, filename),
        _ => println!("I haven't solved that day yet :P"),
    }
}
