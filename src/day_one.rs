use crate::Solution;
use std::collections::HashMap;

pub fn build_solution() -> Solution {
    Solution {
        day: 1,
        input: include_str!("../resources/day_one/day_one_input.txt").into(),
        part_one: part_one_solve,
        part_two: part_two_solve,
    }
}

fn part_one_solve(input: &str) -> f64 {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for l in input.lines() {
        let parts: Vec<&str> = l.split(' ').collect();

        left.push(parts[0].parse::<i32>().unwrap());
        right.push(parts[3].parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    let mut dist: i32 = 0;
    for i in 0..left.len() {
        dist += (left[i] - right[i]).abs();
    }

    dist as f64
}

fn part_two_solve(input: &str) -> f64 {
    let mut left: Vec<i32> = vec![];
    // let mut right: Vec<i32> = vec![];

    let mut right_hash: HashMap<i32, i32> = HashMap::new();

    for l in input.lines() {
        let parts: Vec<&str> = l.split(' ').collect();

        left.push(parts[0].parse::<i32>().unwrap());

        let r_num = parts[3].parse::<i32>().unwrap();
        *right_hash.entry(r_num).or_insert(0) += 1;
    }

    let mut ans: i32 = 0;
    for i in 0..left.len() {
        let n = left[i];
        ans += n * *right_hash.entry(n).or_insert(0);
    }

    ans as f64
}

mod test {

    #[test]
    fn part_one_sample() {
        let ans = super::part_one_solve(include_str!("../resources/day_one/test.txt"));
        assert_eq!(ans, 11.0);
    }

    #[test]
    fn part_two_sample() {
        let ans = super::part_two_solve(include_str!("../resources/day_one/test.txt"));
        assert_eq!(ans, 31.0);
    }

    #[test]
    fn part_one() {
        let sol = super::build_solution();
        let ans = sol.run_part_one();
        assert_eq!(ans, 1580061.0);
    }
}
