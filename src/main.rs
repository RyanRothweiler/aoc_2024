#![allow(clippy::all)]
#![allow(unused_variables, unused_imports)]

use std::{
    include_str,
    time::{Duration, Instant},
};

pub struct Solution {
    pub day: i32,
    pub input: String,
    pub part_one: fn(&str) -> f64,
    pub part_two: fn(&str) -> f64,
}

mod day_one;

fn main() {
    // build solutions
    let mut solutions: Vec<Solution> = vec![];
    solutions.push(day_one::build_solution());

    // run solutions
    for s in solutions {
        println!("Day {:?} ---------------------", s.day);

        let start = Instant::now();
        println!("  P1: {:?}", (s.part_one)(&s.input));
        duration_display(start.elapsed());

        println!("");

        let start = Instant::now();
        println!("  P2: {:?}", (s.part_two)(&s.input));
        duration_display(start.elapsed());
    }
}

fn duration_display(duration: Duration) {
    let seconds: f64 = (duration.as_secs() % 60) as f64;
    let minutes: f64 = ((duration.as_secs() / 60) % 60) as f64;

    let ms = (duration.as_millis() as f64) - (seconds * 1000.0);

    println!("  {}(m):{}(s):{}(ms)", minutes, seconds, ms);
}
