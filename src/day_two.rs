// A sliding window of size 3 would be much better here.

use crate::Solution;

const SAMPLE: &str = include_str!("../resources/day_two/day_two_sample.txt");
const EDGE_CASES: &str = include_str!("../resources/day_two/day_two_edge_cases.txt");

pub fn build_solution() -> Solution {
    Solution {
        day: 1,
        input: include_str!("../resources/day_two/day_two_input.txt").into(),
        part_one: part_one_solve,
        part_two: part_two_solve,
    }
}

enum Dir {
    Increasing,
    Decreasing,
}

fn part_one_solve(input: &str) -> f64 {
    let mut ans: i32 = 0;

    for l in input.lines() {
        let parts: Vec<i32> = l.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();

        if line_valid(&parts) == -1 {
            ans += 1;
        }
    }

    ans as f64
}

fn part_two_solve(input: &str) -> f64 {
    let mut ans: i32 = 0;

    for l in input.lines() {
        let parts: Vec<i32> = l.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();

        let lv = line_valid(&parts);

        // check if anything needs to be removed
        if lv == -1 {
            ans += 1;
            continue;
        }

        // try removing the incorrect ones
        let mut first = parts.clone();
        first.remove(lv as usize);

        let mut second = parts.clone();
        second.remove((lv - 1) as usize);

        if line_valid(&first) == -1 || line_valid(&second) == -1 {
            ans += 1;
            continue;
        }

        // try removing the first entry
        let mut first = parts.clone();
        first.remove(0);
        if line_valid(&first) == -1 {
            ans += 1;
            continue;
        }
    }

    ans as f64
}

// returns the entry that causes the invalid, otherwise -1
fn line_valid(nums: &Vec<i32>) -> i32 {
    if nums.len() < 2 {
        panic!("Not enough numbers");
    }

    let dir;
    if nums[0] < nums[1] {
        dir = Dir::Increasing;
    } else {
        dir = Dir::Decreasing;
    }

    for i in 1..nums.len() {
        let cur = nums[i];
        let prev = nums[i - 1];

        match dir {
            Dir::Increasing => {
                if cur < prev {
                    return i as i32;
                }
            }
            Dir::Decreasing => {
                if cur > prev {
                    return i as i32;
                }
            }
        }

        let dif = (cur - prev).abs();
        if dif < 1 || dif > 3 {
            return i as i32;
        }
    }

    -1
}

mod test {
    #[test]
    fn part_one_sample() {
        let ans = super::part_one_solve(super::SAMPLE);
        assert_eq!(ans, 2.0);
    }

    #[test]
    fn part_two_sample() {
        let ans = super::part_two_solve(super::SAMPLE);
        assert_eq!(ans, 4.0);
    }

    #[test]
    fn part_two_edge_cases() {
        let ans = super::part_two_solve(super::EDGE_CASES);
        assert_eq!(ans, 10.0);
    }

    #[test]
    fn part_one() {
        let sol = super::build_solution();
        let ans = sol.run_part_one();
        assert_eq!(ans, 624.0);
    }

    #[test]
    fn part_two() {
        let sol = super::build_solution();
        let ans = sol.run_part_two();
        assert_eq!(ans, 658.0);
    }
}
