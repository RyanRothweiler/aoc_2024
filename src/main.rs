#![allow(clippy::all)]
#![allow(unused_variables, unused_imports)]

use std::include_str;

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
        println!("  P1: {:?}", (s.part_one)(&s.input));
        println!("  P2: {:?}", (s.part_two)(&s.input));
    }
}
