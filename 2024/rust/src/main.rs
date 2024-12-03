use clap::Parser;
use once_cell::sync::Lazy;
use paste::paste;
use std::{collections::HashMap, fmt::Display, time::Instant};

macro_rules! days_decl {
    ($daymap_name:ident : $($ds:literal),*) => {
        $( paste!{
            mod [< day $ds >];
            use [< day $ds >]::*;
        })*

        static $daymap_name: Lazy<HashMap<usize, fn(String, &mut AOContext)>> = Lazy::new(|| {
            let mut map: std::collections::HashMap<usize, fn(String, &mut AOContext)> = HashMap::new();
            $( map.insert($ds, paste!{ [< day $ds >] });)*
            map
        });
    }
}

days_decl!(DAYS: 1, 2, 3);

#[derive(Parser)]
struct Args {
    /// the day to solve
    day: usize,
    /// time solutions and print the benchmark with the result
    #[arg(short)]
    time: bool,
}

struct AOContext {
    start: Instant,
    now: Instant,
    laps: Vec<(String, f64)>,
    time: bool,
}

impl AOContext {
    fn new(time: bool) -> AOContext {
        AOContext {
            start: Instant::now(),
            now: Instant::now(),
            laps: Vec::new(),
            time,
        }
    }

    pub fn parsing_done(&mut self) {
        self.lap("parsing");
    }

    pub fn lap(&mut self, lap_name: impl Into<String>) {
        if self.time {
            self.laps.push((lap_name.into(), self.now.elapsed().as_secs_f64() * 1000.0));
            self.now = Instant::now();
        }
    }

    pub fn submit_part1<T: Display>(&mut self, result: T) {
        println!("part 1: {result}");
        self.lap("part 1");
    }

    pub fn submit_part2<T: Display>(&mut self, result: T) {
        println!("part 2: {result}");
        self.lap("part 2");
        self.print_times();
    }

    pub fn submit_both<P1: Display, P2: Display>(&mut self, p1: P1, p2: P2) {
        println!("part 1: {p1}\npart 2: {p2}");
        self.lap("solving");
        self.print_times();
    }

    fn print_times(&self) {
        if self.time {
            let total = self.start.elapsed().as_secs_f64() * 1000.0;
            println!();
            
            for (name, time) in self.laps.iter() {
                println!("{name} took {time:.2}ms");
            }

            println!();
            println!("In total, it took {total:.2}ms to solve");
        }
    }
}

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
