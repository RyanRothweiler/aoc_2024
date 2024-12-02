pub fn run(input: &str) {
    let ans = solve(input);
    println!("{ans}");
}

fn solve(input: &str) -> i32 {
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

    dist
}

mod test {

    #[test]
    fn simple() {
        let ans = super::solve(include_str!("../resources/day_one/test.txt"));
        assert_eq!(ans, 11);
    }
}
