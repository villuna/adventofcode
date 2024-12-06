use clap::Parser;
use std::{fmt::Display, time::Instant};

pub mod parsers;
pub mod utils;

#[macro_export]
macro_rules! days_decl {
    ($daymap_name:ident : $($ds:literal),*) => {
        $( ::paste::paste!{
            mod [< day $ds >];
            use [< day $ds >]::*;
        })*

        static $daymap_name: ::once_cell::sync::Lazy<::std::collections::HashMap<usize, fn(String, &mut ::aoc::AOContext)>> = ::once_cell::sync::Lazy::new(|| {
            let mut map: ::std::collections::HashMap<usize, fn(String, &mut ::aoc::AOContext)> = ::std::collections::HashMap::new();
            $( map.insert($ds, ::paste::paste!{ [< day $ds >] });)*
            map
        });
    }
}

#[derive(Parser)]
pub struct Args {
    /// the day to solve
    pub day: usize,
    /// time solutions and print the benchmark with the result
    #[arg(short)]
    pub time: bool,
}

pub struct AOContext {
    start: Instant,
    now: Instant,
    laps: Vec<(String, f64)>,
    time: bool,
}

impl AOContext {
    pub fn new(time: bool) -> AOContext {
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
            self.laps
                .push((lap_name.into(), self.now.elapsed().as_secs_f64() * 1000.0));
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
                if *time < 100.0 {
                    println!("{name} took {}", format_time(*time));
                } else {
                    println!("{name} took {}", format_time(*time));
                }
            }

            println!();
            println!("In total, it took {} to solve", format_time(total));
        }
    }
}

fn format_time(millis: f64) -> String {
    // I know negative time is impossible but it can't hurt to be correct right
    if millis.abs() > 500.0 {
        format!("{millis:.2} ms ({:.2} s)", millis / 1000.0)
    } else {
        format!("{millis:.2} ms")
    }
}
