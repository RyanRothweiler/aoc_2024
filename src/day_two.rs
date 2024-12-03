use crate::Solution;

const SAMPLE: &str = include_str!("../resources/day_two/day_two_sample.txt");

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

        if parts.len() < 2 {
            panic!("Not enough numbers");
        }

        let dir;
        if parts[0] < parts[1] {
            dir = Dir::Increasing;
        } else {
            dir = Dir::Decreasing;
        }

        let mut valid = true;

        for i in 1..parts.len() {
            let cur = parts[i];
            let prev = parts[i - 1];

            match dir {
                Dir::Increasing => {
                    if cur < prev {
                        valid = false;
                        break;
                    }
                }
                Dir::Decreasing => {
                    if cur > prev {
                        valid = false;
                        break;
                    }
                }
            }

            let dif = (cur - prev).abs();
            if dif < 1 || dif > 3 {
                valid = false;
            }
        }

        if valid {
            ans += 1;
        }
    }

    ans as f64
}

fn part_two_solve(input: &str) -> f64 {
    0.0
}

mod test {
    #[test]
    fn part_one_sample() {
        let ans = super::part_one_solve(super::SAMPLE);
        assert_eq!(ans, 2.0);
    }

    #[test]
    fn part_one() {
        let sol = super::build_solution();
        let ans = sol.run_part_one();
        assert_eq!(ans, 624.0);
    }
}
