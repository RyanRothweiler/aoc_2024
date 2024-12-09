use crate::Solution;
use std::sync::LazyLock;

const SAMPLE: &str = include_str!("../resources/4/day_four_sample.txt");
const SAMPLE_TWO: &str = include_str!("../resources/4/day_four_sample.txt");

const WORD: LazyLock<Vec<char>> = LazyLock::new(|| vec!['X', 'M', 'A', 'S']);
const DIRS: LazyLock<Vec<(i32, i32)>> = LazyLock::new(|| {
    vec![
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ]
});

pub fn build_solution() -> Solution {
    Solution {
        day: 4,
        input: include_str!("../resources/4/day_four_input.txt").into(),
        part_one: part_one_solve,
        part_two: part_two_solve,
    }
}

fn part_one_solve(input: &str) -> f64 {
    solve(input)
}

fn part_two_solve(input: &str) -> f64 {
    0.0
}

fn solve(input: &str) -> f64 {
    let mut map: Vec<Vec<char>> = vec![];

    // build map
    let lines: Vec<&str> = input.lines().collect();

    for x in 0..lines.len() {
        map.push(vec![]);

        let ch: Vec<char> = lines[x].chars().collect();
        for y in 0..ch.len() {
            map[x].push(ch[y]);
        }
    }

    let mut count: i32 = 0;

    // check for words
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            for d in &*DIRS {
                if check_dir((x as i32, y as i32), *d, &map) {
                    count += 1;
                }
            }
        }
    }

    count as f64
}

fn check_dir(origin: (i32, i32), dir: (i32, i32), map: &Vec<Vec<char>>) -> bool {
    for w in 0..WORD.len() {
        let mc = map_get(
            (origin.0 + (w as i32 * dir.0)) as i32,
            (origin.1 + (w as i32 * dir.1)) as i32,
            &map,
        );
        if WORD[w] != mc {
            return false;
        }
    }

    true
}

fn map_get(x: i32, y: i32, map: &Vec<Vec<char>>) -> char {
    if x >= 0 && y >= 0 && x < map.len() as i32 && y < map[x as usize].len() as i32 {
        return map[x as usize][y as usize];
    }
    return ' ';
}

mod tests {
    #[test]
    fn part_one_sample() {
        let ans = super::part_one_solve(super::SAMPLE);
        assert_eq!(ans, 18.0);
    }

    #[test]
    fn part_one() {
        let sol = super::build_solution();
        let ans = sol.run_part_one();
        assert_eq!(ans, 2530.0);
    }
}
