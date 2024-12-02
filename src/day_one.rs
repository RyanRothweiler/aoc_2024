use crate::Solution;

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
    0.0
}

mod test {

    #[test]
    fn simple() {
        let ans = super::solve(include_str!("../resources/day_one/test.txt"));
        assert_eq!(ans, 11);
    }
}
