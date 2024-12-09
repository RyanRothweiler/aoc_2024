use crate::Solution;

const SAMPLE: &str = include_str!("../resources/3/day_three_sample.txt");
const SAMPLE_TWO: &str = include_str!("../resources/3/day_three_sample_part_two.txt");

pub fn build_solution() -> Solution {
    Solution {
        day: 3,
        input: include_str!("../resources/3/day_three_input.txt").into(),
        part_one: part_one_solve,
        part_two: part_two_solve,
    }
}

fn part_one_solve(input: &str) -> f64 {
    solve(input, false)
}

fn part_two_solve(input: &str) -> f64 {
    solve(input, true)
}

fn solve(input: &str, handle_dont: bool) -> f64 {
    let mut ans: f64 = 0.0;

    let mut tokenizer = Tokenizer {
        data: input.chars().collect(),
        index: 0,
    };

    let mut enabled = true;

    let mut tok = get_next_token(&mut tokenizer);
    while tok != Token::End {
        match tok {
            Token::Word(ref s) => {
                if s.ends_with("mul") {
                    let tok_start = tokenizer.index;

                    let open_paren = get_next_token(&mut tokenizer);
                    let first_val = get_next_token(&mut tokenizer);
                    let comma = get_next_token(&mut tokenizer);
                    let second_val = get_next_token(&mut tokenizer);
                    let close_paren = get_next_token(&mut tokenizer);

                    match (open_paren, first_val, comma, second_val, close_paren) {
                        (
                            Token::OpenParen,
                            Token::Number(x),
                            Token::Comma,
                            Token::Number(y),
                            Token::ClosedParen,
                        ) => {
                            if handle_dont {
                                if enabled {
                                    // println!("valid mul {x} x {y}");
                                    ans += (x * y) as f64;
                                }
                            } else {
                                ans += (x * y) as f64;
                            }
                        }
                        _ => {
                            tokenizer.index = tok_start;
                        }
                    }
                } else if s.ends_with("do") {
                    let tok_start = tokenizer.index;

                    let open_paren = get_next_token(&mut tokenizer);
                    let close_paren = get_next_token(&mut tokenizer);

                    match (open_paren, close_paren) {
                        (Token::OpenParen, Token::ClosedParen) => {
                            // println!("DO");
                            enabled = true;
                        }
                        _ => {
                            tokenizer.index = tok_start;
                        }
                    }
                } else if s.ends_with("don't") {
                    // enabled = false;
                    let tok_start = tokenizer.index;

                    let open_paren = get_next_token(&mut tokenizer);
                    let close_paren = get_next_token(&mut tokenizer);

                    match (open_paren, close_paren) {
                        (Token::OpenParen, Token::ClosedParen) => {
                            // println!("DON'T");
                            enabled = false;
                        }
                        _ => {
                            tokenizer.index = tok_start;
                        }
                    }
                }
            }
            _ => {}
        }

        tok = get_next_token(&mut tokenizer);
    }

    ans
}

#[derive(Debug, Eq, PartialEq)]
enum Token {
    Word(String),
    Number(i32),

    OpenParen,
    ClosedParen,

    Comma,
    End,
}

struct Tokenizer {
    pub data: Vec<char>,
    pub index: usize,
}

impl Tokenizer {
    pub fn current(&self) -> char {
        self.data[self.index]
    }
}

fn is_word(c: char) -> bool {
    !c.is_numeric() && c != '(' && c != ')' && c != ','
}

fn get_next_token(tokenizer: &mut Tokenizer) -> Token {
    loop {
        if tokenizer.index >= tokenizer.data.len() {
            return Token::End;
        }

        if is_word(tokenizer.current()) {
            let start = tokenizer.index;
            let mut word_chars: Vec<char> = vec![];

            while tokenizer.index < tokenizer.data.len() && is_word(tokenizer.current()) {
                word_chars.push(tokenizer.data[tokenizer.index]);
                tokenizer.index += 1;
            }
            let s = String::from_iter(word_chars);
            return Token::Word(s);
        } else if tokenizer.current().is_numeric() {
            let start = tokenizer.index;
            let mut num_chars: Vec<char> = vec![];

            while tokenizer.index < tokenizer.data.len() && tokenizer.current().is_numeric() {
                num_chars.push(tokenizer.data[tokenizer.index]);
                tokenizer.index += 1;
            }

            let num_str = String::from_iter(num_chars);

            return Token::Number(num_str.parse::<i32>().unwrap());
        } else if tokenizer.current() == ',' {
            tokenizer.index += 1;
            return Token::Comma;
        } else if tokenizer.current() == ')' {
            tokenizer.index += 1;
            return Token::ClosedParen;
        } else if tokenizer.current() == '(' {
            tokenizer.index += 1;
            return Token::OpenParen;
        } else {
            tokenizer.index += 1;
        }
    }
}

mod tests {

    #[test]
    fn part_one_sample() {
        let ans = super::part_one_solve(super::SAMPLE);
        assert_eq!(ans, 162.0);
    }

    #[test]
    fn part_two_sample() {
        let ans = super::part_two_solve(super::SAMPLE_TWO);
        assert_eq!(ans, 48.0);
    }

    #[test]
    fn part_one() {
        let sol = super::build_solution();
        let ans = sol.run_part_one();
        assert_eq!(ans, 183669043.0);
    }

    #[test]
    fn part_two() {
        let sol = super::build_solution();
        let ans = sol.run_part_two();
        assert_eq!(ans, 59097164.0);
    }
}
