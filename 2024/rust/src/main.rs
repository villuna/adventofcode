use clap::Parser;
use aoc::{days_decl, AOContext, Args};

// Merychippus!
days_decl!(DAYS: 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25);

fn main() {
    let args = Args::parse();
    let day = args.day;

    match DAYS.get(&day) {
        None => {
            eprintln!("Day invalid or not completed!");
        }

        Some(function) => {
            let Ok(input) = std::fs::read_to_string(format!("../input/day{day}.txt")) else {
                eprintln!(
                    "input file not found! please create it at [REPO ROOT]/input/day{day}.txt"
                );
                return;
            };

            let mut ctx = AOContext::new(args.time);
            function(input, &mut ctx);
        }
    }
}
